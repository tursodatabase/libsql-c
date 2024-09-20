use anyhow::bail;
use c::libsql_log_t;
use lazy_static::lazy_static;
use libsql::{replication::Replicated, Connection, Database, Row, Rows, Transaction};
use once_cell::sync::OnceCell;
use std::{
    ffi::{c_char, c_void, CStr, CString},
    mem::ManuallyDrop,
    ops::Not,
    ptr, slice,
    sync::Once,
    time::Duration,
};
use tokio::runtime::Runtime;

use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::format::Writer,
    layer::{Context, SubscriberExt},
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    use core::slice;
    use std::{
        ffi::{c_void, CString},
        mem::ManuallyDrop,
    };

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl From<libsql::Value> for libsql_value_t {
        fn from(value: libsql::Value) -> Self {
            match value {
                libsql::Value::Null => libsql_value_t {
                    type_: libsql_type_t::LIBSQL_TYPE_NULL,
                    ..Default::default()
                },
                libsql::Value::Integer(integer) => libsql_value_t {
                    type_: libsql_type_t::LIBSQL_TYPE_INTEGER,
                    value: libsql_value_union_t { integer },
                },
                libsql::Value::Real(real) => libsql_value_t {
                    type_: libsql_type_t::LIBSQL_TYPE_REAL,
                    value: libsql_value_union_t { real },
                },
                libsql::Value::Text(text) => {
                    let text = match text.find('\0') {
                        Some(i) => ManuallyDrop::new(CString::new(&text[0..i]).unwrap()),
                        None => ManuallyDrop::new(CString::new(text).unwrap()),
                    };
                    libsql_value_t {
                        type_: libsql_type_t::LIBSQL_TYPE_TEXT,
                        value: libsql_value_union_t {
                            text: libsql_slice_t {
                                ptr: text.as_bytes_with_nul().as_ptr() as *mut c_void,
                                len: text.as_bytes_with_nul().len(),
                            },
                        },
                    }
                }
                libsql::Value::Blob(blob) => {
                    let blob = ManuallyDrop::new(blob);
                    libsql_value_t {
                        type_: libsql_type_t::LIBSQL_TYPE_BLOB,
                        value: libsql_value_union_t {
                            blob: libsql_slice_t {
                                ptr: blob.as_ptr() as *const c_void,
                                len: blob.len(),
                            },
                        },
                    }
                }
            }
        }
    }

    impl TryFrom<libsql_value_t> for libsql::Value {
        type Error = anyhow::Error;

        fn try_from(value: libsql_value_t) -> Result<Self, Self::Error> {
            Ok(match value.type_ {
                libsql_type_t::LIBSQL_TYPE_INTEGER => {
                    libsql::Value::Integer(unsafe { value.value.integer })
                }
                libsql_type_t::LIBSQL_TYPE_REAL => libsql::Value::Real(unsafe { value.value.real }),
                libsql_type_t::LIBSQL_TYPE_TEXT => {
                    libsql::Value::Text(String::from_utf8(unsafe {
                        slice::from_raw_parts(value.value.text.ptr as *mut u8, value.value.text.len)
                            .to_vec()
                    })?)
                }
                libsql_type_t::LIBSQL_TYPE_BLOB => libsql::Value::Blob(unsafe {
                    slice::from_raw_parts(value.value.text.ptr as *mut u8, value.value.text.len)
                        .to_vec()
                }),
                libsql_type_t::LIBSQL_TYPE_NULL => libsql::Value::Null,
            })
        }
    }
}

lazy_static! {
    static ref RT: Runtime = Runtime::new().unwrap();
}

#[derive(Clone)]
enum Params {
    None,
    Positional(Vec<libsql::Value>),
    Named(Vec<(String, libsql::Value)>),
}

struct Statement {
    inner: libsql::Statement,
    params: Params,
}

impl Statement {
    pub fn bind_named(&mut self, name: String, value: libsql::Value) -> anyhow::Result<()> {
        match self.params {
            Params::None => {
                self.params = Params::Named(vec![(name, value)]);
                Ok(())
            }
            Params::Named(ref mut params) => {
                params.push((name, value));
                Ok(())
            }
            Params::Positional(_) => {
                bail!("binding names and positional arguments in the same statement is unsupported")
            }
        }
    }

    pub fn bind_value(&mut self, value: libsql::Value) -> anyhow::Result<()> {
        match self.params {
            Params::None => {
                self.params = Params::Positional(vec![value]);
                Ok(())
            }
            Params::Positional(ref mut params) => {
                params.push(value);
                Ok(())
            }
            Params::Named(_) => {
                bail!("binding names and positional arguments in the same statement is unsupported")
            }
        }
    }
}

struct CallbackLayer<F>
where
    F: Fn(libsql_log_t) + Send + Sync + 'static,
{
    callback: F,
}

impl<S, F> Layer<S> for CallbackLayer<F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    F: Fn(libsql_log_t) + Send + Sync + 'static,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut buffer = String::new();
        let mut visitor =
            tracing_subscriber::fmt::format::DefaultVisitor::new(Writer::new(&mut buffer), true);

        let level = match *event.metadata().level() {
            Level::ERROR => c::libsql_tracing_level_t::LIBSQL_TRACING_LEVEL_ERROR,
            Level::WARN => c::libsql_tracing_level_t::LIBSQL_TRACING_LEVEL_WARN,
            Level::INFO => c::libsql_tracing_level_t::LIBSQL_TRACING_LEVEL_INFO,
            Level::DEBUG => c::libsql_tracing_level_t::LIBSQL_TRACING_LEVEL_DEBUG,
            Level::TRACE => c::libsql_tracing_level_t::LIBSQL_TRACING_LEVEL_TRACE,
        };

        event.record(&mut visitor);

        let log = libsql_log_t {
            level,
            ..Default::default()
        };

        (self.callback)(log);
    }
}

static LOGGER: OnceCell<unsafe extern "C" fn(c::libsql_log_t)> = OnceCell::new();
static SETUP: Once = Once::new();

#[no_mangle]
pub extern "C" fn libsql_setup(config: c::libsql_config_t) -> *const c::libsql_error_t {
    let callback = |log: libsql_log_t| {
        if let Some(logger) = LOGGER.get() {
            unsafe { logger(log) }
        }
    };

    if let Some(logger) = config.logger.as_ref() {
        if let Err(_) = LOGGER.set(*logger) {
            return CString::new("attempted to set the logger more than once")
                .unwrap()
                .into_raw() as *mut c::libsql_error_t;
        }
    }

    SETUP.call_once(|| {
        tracing_subscriber::registry()
            .with(EnvFilter::builder().parse_lossy("trace"))
            .with(CallbackLayer { callback })
            .init();
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn libsql_error_message(err: *mut c::libsql_error_t) -> *const c_char {
    err as *mut c_char
}

#[no_mangle]
pub extern "C" fn libsql_database_init(desc: c::libsql_database_desc_t) -> c::libsql_database_t {
    match (|| -> anyhow::Result<Database> {
        let path = desc
            .path
            .is_null()
            .not()
            .then(|| unsafe { CStr::from_ptr(desc.path) });

        let url = desc
            .url
            .is_null()
            .not()
            .then(|| unsafe { CStr::from_ptr(desc.url) });

        let auth_token = desc
            .auth_token
            .is_null()
            .not()
            .then(|| unsafe { CStr::from_ptr(desc.auth_token) });

        let encryption_key = desc
            .encryption_key
            .is_null()
            .not()
            .then(|| unsafe { CStr::from_ptr(desc.encryption_key) });

        let db = match (path, url, auth_token) {
            (None, None, None) => {
                let db = libsql::Builder::new_local(":memory:");
                RT.block_on(db.build())
            }
            (Some(path), None, None) => {
                let db = libsql::Builder::new_local(path.to_str()?);

                let db = match (desc.cypher, encryption_key) {
                    (
                        c::libsql_cypher_t::LIBSQL_CYPHER_AES256
                        | c::libsql_cypher_t::LIBSQL_CYPHER_DEFAULT,
                        Some(key),
                    ) => db.encryption_config(libsql::EncryptionConfig {
                        cipher: libsql::Cipher::Aes256Cbc,
                        encryption_key: key.to_bytes().into(),
                    }),
                    _ => db,
                };

                RT.block_on(db.build())
            }
            (None, Some(url), Some(auth_token)) => {
                let db = libsql::Builder::new_remote(
                    url.to_str()?.to_string(),
                    auth_token.to_str()?.to_string(),
                );

                let db = if desc.webpki {
                    let connector = hyper_rustls::HttpsConnectorBuilder::new()
                        .with_webpki_roots()
                        .https_or_http()
                        .enable_http1()
                        .build();

                    db.connector(connector)
                } else {
                    db
                };

                RT.block_on(db.build())
            }
            (Some(path), Some(url), Some(auth_token)) => {
                let db = libsql::Builder::new_remote_replica(
                    path.to_str()?,
                    url.to_str()?.to_string(),
                    auth_token.to_str()?.to_string(),
                )
                // NOTE: This is done so that the default zero initialization respects that
                // read_your_writes is true by default.
                .read_your_writes(desc.not_read_your_writes.not());

                let db = match (desc.cypher, encryption_key) {
                    (
                        c::libsql_cypher_t::LIBSQL_CYPHER_AES256
                        | c::libsql_cypher_t::LIBSQL_CYPHER_DEFAULT,
                        Some(key),
                    ) => db.encryption_config(libsql::EncryptionConfig {
                        cipher: libsql::Cipher::Aes256Cbc,
                        encryption_key: key.to_bytes().into(),
                    }),
                    _ => db,
                };

                let db = if desc.sync_interval != 0 {
                    db.sync_interval(Duration::from_millis(desc.sync_interval))
                } else {
                    db
                };

                let db = if desc.webpki {
                    let connector = hyper_rustls::HttpsConnectorBuilder::new()
                        .with_webpki_roots()
                        .https_or_http()
                        .enable_http1()
                        .build();

                    db.connector(connector)
                } else {
                    db
                };

                RT.block_on(db.build())
            }
            _ => bail!("invalid database description"),
        };

        Ok(db?)
    })() {
        Ok(db) => c::libsql_database_t {
            inner: Box::into_raw(Box::new(db)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_database_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_database_sync(db: c::libsql_database_t) -> c::libsql_sync_t {
    match (|| -> anyhow::Result<Replicated> {
        if db.inner.is_null() {
            bail!("attempted to sync a null database")
        }

        let db = ManuallyDrop::new(unsafe { Box::from_raw(db.inner as *mut Database) });

        Ok(RT.block_on(db.sync())?)
    })() {
        Ok(replicated) => c::libsql_sync_t {
            frame_no: replicated.frame_no().unwrap_or(0),
            frames_synced: replicated
                .frames_synced()
                .try_into()
                .expect("usize should be no more than 64 bits"),
            ..Default::default()
        },
        Err(err) => c::libsql_sync_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_database_connect(db: c::libsql_database_t) -> c::libsql_connection_t {
    match (|| -> anyhow::Result<Connection> {
        if db.inner.is_null() {
            bail!("attempted to init connection with a null database")
        }

        let db = ManuallyDrop::new(unsafe { Box::from_raw(db.inner as *mut Database) });

        Ok(db.connect()?)
    })() {
        Ok(conn) => c::libsql_connection_t {
            inner: Box::into_raw(Box::new(conn)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_connection_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_connection_transaction(
    conn: c::libsql_connection_t,
) -> c::libsql_transaction_t {
    match (move || -> anyhow::Result<Transaction> {
        if conn.inner.is_null() {
            bail!("attempted to init a statement with a null connection")
        }

        let conn = ManuallyDrop::new(unsafe { Box::from_raw(conn.inner as *mut Connection) });

        Ok(RT.block_on(conn.transaction())?)
    })() {
        Ok(tx) => c::libsql_transaction_t {
            inner: Box::into_raw(Box::new(tx)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_transaction_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_connection_batch(
    conn: c::libsql_connection_t,
    sql: *const c_char,
) -> c::libsql_batch_t {
    match (move || -> anyhow::Result<_> {
        if conn.inner.is_null() {
            bail!("attempted to init a statement with a null connection")
        }

        if sql.is_null() {
            bail!("execute a null sql query")
        }

        let sql = unsafe { CStr::from_ptr(sql) }.to_str()?;

        let conn = ManuallyDrop::new(unsafe { Box::from_raw(conn.inner as *mut Connection) });

        Ok(RT.block_on(conn.execute_batch(sql))?)
    })() {
        Ok(_) => c::libsql_batch_t {
            ..Default::default()
        },
        Err(err) => c::libsql_batch_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_transaction_batch(
    tx: c::libsql_transaction_t,
    sql: *const c_char,
) -> c::libsql_batch_t {
    match (move || -> anyhow::Result<_> {
        if tx.inner.is_null() {
            bail!("attempted to init a statement with a null connection")
        }

        if sql.is_null() {
            bail!("execute a null sql query")
        }

        let sql = unsafe { CStr::from_ptr(sql) }.to_str()?;

        let tx = ManuallyDrop::new(unsafe { Box::from_raw(tx.inner as *mut Transaction) });

        Ok(RT.block_on(tx.execute_batch(sql))?)
    })() {
        Ok(_) => c::libsql_batch_t {
            ..Default::default()
        },
        Err(err) => c::libsql_batch_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_connection_prepare(
    conn: c::libsql_connection_t,
    sql: *const c_char,
) -> c::libsql_statement_t {
    match (move || -> anyhow::Result<Statement> {
        if conn.inner.is_null() {
            bail!("attempted to init a statement with a null connection")
        }

        if sql.is_null() {
            bail!("attempted to init a statement with a null sql query")
        }

        let sql = unsafe { CStr::from_ptr(sql) }.to_str()?;

        let conn = ManuallyDrop::new(unsafe { Box::from_raw(conn.inner as *mut Connection) });

        Ok(Statement {
            inner: RT.block_on(conn.prepare(sql))?,
            params: Params::None,
        })
    })() {
        Ok(stmt) => c::libsql_statement_t {
            inner: Box::into_raw(Box::new(stmt)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_statement_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_transaction_prepare(
    tx: c::libsql_transaction_t,
    sql: *const c_char,
) -> c::libsql_statement_t {
    match (move || -> anyhow::Result<Statement> {
        if tx.inner.is_null() {
            bail!("attempted to init a statement with a null transaction")
        }

        if sql.is_null() {
            bail!("attempted to init a statement with a null sql query")
        }

        let sql = unsafe { CStr::from_ptr(sql) }.to_str()?;

        let tx = ManuallyDrop::new(unsafe { Box::from_raw(tx.inner as *mut Transaction) });

        Ok(Statement {
            inner: RT.block_on(tx.prepare(sql))?,
            params: Params::None,
        })
    })() {
        Ok(stmt) => c::libsql_statement_t {
            inner: Box::into_raw(Box::new(stmt)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_statement_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_statement_execute(stmt: c::libsql_statement_t) -> c::libsql_execute_t {
    match (move || -> anyhow::Result<usize> {
        if stmt.inner.is_null() {
            bail!("attempted to execute a null statement")
        }

        let mut stmt = ManuallyDrop::new(unsafe { Box::from_raw(stmt.inner as *mut Statement) });

        let params = stmt.params.clone();

        Ok(match params {
            Params::None => RT.block_on(stmt.inner.execute(()))?,
            Params::Named(named) => RT.block_on(stmt.inner.execute(named))?,
            Params::Positional(positional) => RT.block_on(stmt.inner.execute(positional))?,
        })
    })() {
        Ok(rows_changed) => c::libsql_execute_t {
            rows_changed: rows_changed
                .try_into()
                .expect("usize should be no more than 64 bits"),
            ..Default::default()
        },
        Err(err) => c::libsql_execute_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_statement_query(stmt: c::libsql_statement_t) -> c::libsql_rows_t {
    match (move || -> anyhow::Result<Rows> {
        if stmt.inner.is_null() {
            bail!("attempted to query a null statement")
        }

        let mut stmt = ManuallyDrop::new(unsafe { Box::from_raw(stmt.inner as *mut Statement) });

        let params = stmt.params.clone();

        Ok(match params {
            Params::None => RT.block_on(stmt.inner.query(()))?,
            Params::Named(named) => RT.block_on(stmt.inner.query(named))?,
            Params::Positional(positional) => RT.block_on(stmt.inner.query(positional))?,
        })
    })() {
        Ok(rows) => c::libsql_rows_t {
            inner: Box::into_raw(Box::new(rows)) as *mut c_void,
            ..Default::default()
        },
        Err(err) => c::libsql_rows_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_statement_reset(stmt: c::libsql_statement_t) {
    if stmt.inner.is_null() {
        // TODO: Should we just panic! here?
        return;
    }

    let mut stmt = ManuallyDrop::new(unsafe { Box::from_raw(stmt.inner as *mut Statement) });

    stmt.inner.reset();
    stmt.params = Params::None;
}

#[no_mangle]
pub extern "C" fn libsql_rows_next(rows: c::libsql_rows_t) -> c::libsql_row_t {
    match (move || -> anyhow::Result<Option<Row>> {
        if rows.inner.is_null() {
            bail!("attempted get a row from a null rows")
        }

        let mut rows = ManuallyDrop::new(unsafe { Box::from_raw(rows.inner as *mut Rows) });

        Ok(RT.block_on(rows.next())?)
    })() {
        Ok(Some(row)) => c::libsql_row_t {
            inner: Box::into_raw(Box::new(row)) as *mut c_void,
            ..Default::default()
        },
        Ok(None) => c::libsql_row_t {
            inner: ptr::null_mut(),
            ..Default::default()
        },
        Err(err) => c::libsql_row_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_row_value(row: c::libsql_row_t, idx: i32) -> c::libsql_result_value_t {
    match (move || -> anyhow::Result<libsql::Value> {
        if row.inner.is_null() {
            bail!("attempted get a row from a null rows")
        }

        let row = ManuallyDrop::new(unsafe { Box::from_raw(row.inner as *mut Row) });

        Ok(row.get_value(idx)?)
    })() {
        Ok(value) => c::libsql_result_value_t {
            ok: value.into(),
            ..Default::default()
        },
        Err(err) => c::libsql_result_value_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_row_name(row: c::libsql_row_t, idx: i32) -> c::libsql_slice_t {
    if row.inner.is_null() {
        return c::libsql_slice_t {
            ptr: ptr::null(),
            len: 0,
        };
    }

    let row = ManuallyDrop::new(unsafe { Box::from_raw(row.inner as *mut Row) });

    match row.column_name(idx) {
        None => c::libsql_slice_t {
            ptr: ptr::null(),
            len: 0,
        },
        Some(name) => {
            let name = ManuallyDrop::new(CString::new(name).unwrap());

            c::libsql_slice_t {
                ptr: name.as_bytes_with_nul().as_ptr() as *mut c_void,
                len: name.as_bytes_with_nul().len(),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn libsql_row_length(row: c::libsql_row_t) -> i32 {
    if row.inner.is_null() {
        return 0;
    }

    let row = ManuallyDrop::new(unsafe { Box::from_raw(row.inner as *mut Row) });

    // TODO: Why `column_count` returns usize if `get` only accepts a i32?
    row.column_count().try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn libsql_row_empty(row: c::libsql_row_t) -> bool {
    row.inner.is_null()
}

#[no_mangle]
pub extern "C" fn libsql_statement_bind_named(
    stmt: c::libsql_statement_t,
    name: *const c_char,
    value: c::libsql_value_t,
) -> c::libsql_bind_t {
    match (move || -> anyhow::Result<()> {
        if stmt.inner.is_null() {
            bail!("attempted to bind a null statement")
        }

        if name.is_null() {
            bail!("attempted to bind a statement with a null name")
        }

        let name = unsafe { CStr::from_ptr(name) }.to_str()?;

        let mut stmt = ManuallyDrop::new(unsafe { Box::from_raw(stmt.inner as *mut Statement) });

        stmt.bind_named(name.to_owned(), value.try_into()?)?;

        Ok(())
    })() {
        Ok(_) => c::libsql_bind_t {
            ..Default::default()
        },
        Err(err) => c::libsql_bind_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_statement_bind_value(
    stmt: c::libsql_statement_t,
    value: c::libsql_value_t,
) -> c::libsql_bind_t {
    match (move || -> anyhow::Result<()> {
        if stmt.inner.is_null() {
            bail!("attempted to bind a null statement")
        }

        let mut stmt = ManuallyDrop::new(unsafe { Box::from_raw(stmt.inner as *mut Statement) });

        stmt.bind_value(value.try_into()?)?;

        Ok(())
    })() {
        Ok(_) => c::libsql_bind_t {
            ..Default::default()
        },
        Err(err) => c::libsql_bind_t {
            err: CString::new(err.to_string()).unwrap().into_raw() as *mut c::libsql_error_t,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_integer(value: i64) -> c::libsql_value_t {
    c::libsql_value_t {
        type_: c::libsql_type_t::LIBSQL_TYPE_INTEGER,
        value: c::libsql_value_union_t { integer: value },
    }
}

#[no_mangle]
pub extern "C" fn libsql_real(value: f64) -> c::libsql_value_t {
    c::libsql_value_t {
        type_: c::libsql_type_t::LIBSQL_TYPE_REAL,
        value: c::libsql_value_union_t { real: value },
    }
}

#[no_mangle]
pub extern "C" fn libsql_text(ptr: *const c_char, len: usize) -> c::libsql_value_t {
    c::libsql_value_t {
        type_: c::libsql_type_t::LIBSQL_TYPE_TEXT,
        value: c::libsql_value_union_t {
            text: c::libsql_slice_t {
                ptr: ptr as *const c_void,
                len,
            },
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_blob(ptr: *const u8, len: usize) -> c::libsql_value_t {
    c::libsql_value_t {
        type_: c::libsql_type_t::LIBSQL_TYPE_BLOB,
        value: c::libsql_value_union_t {
            text: c::libsql_slice_t {
                ptr: ptr as *const c_void,
                len,
            },
        },
    }
}

#[no_mangle]
pub extern "C" fn libsql_null() -> c::libsql_value_t {
    c::libsql_value_t {
        type_: c::libsql_type_t::LIBSQL_TYPE_NULL,
        ..Default::default()
    }
}

// == Destructors ==

#[no_mangle]
pub extern "C" fn libsql_error_deinit(err: *mut c::libsql_error_t) {
    drop(unsafe { CString::from_raw(err as *mut c_char) })
}

#[no_mangle]
pub extern "C" fn libsql_database_deinit(db: c::libsql_database_t) {
    drop(unsafe { Box::from_raw(db.inner as *mut Database) })
}

#[no_mangle]
pub extern "C" fn libsql_connection_deinit(conn: c::libsql_connection_t) {
    drop(unsafe { Box::from_raw(conn.inner as *mut Connection) })
}

#[no_mangle]
pub extern "C" fn libsql_statement_deinit(db: c::libsql_statement_t) {
    drop(unsafe { Box::from_raw(db.inner as *mut Statement) })
}

#[no_mangle]
pub extern "C" fn libsql_transaction_commit(db: c::libsql_transaction_t) {
    RT.block_on(unsafe { Box::from_raw(db.inner as *mut Transaction) }.commit())
        .unwrap()
}

#[no_mangle]
pub extern "C" fn libsql_transaction_rollback(db: c::libsql_transaction_t) {
    RT.block_on(unsafe { Box::from_raw(db.inner as *mut Transaction) }.rollback())
        .unwrap()
}

#[no_mangle]
pub extern "C" fn libsql_rows_deinit(rows: c::libsql_rows_t) {
    drop(unsafe { Box::from_raw(rows.inner as *mut Rows) })
}

#[no_mangle]
pub extern "C" fn libsql_row_deinit(row: c::libsql_row_t) {
    drop(unsafe { Box::from_raw(row.inner as *mut Row) })
}

#[no_mangle]
pub extern "C" fn libsql_slice_deinit(s: c::libsql_slice_t) {
    let s = unsafe { slice::from_raw_parts_mut(s.ptr as *mut u8, s.len) };
    drop(unsafe { Box::from_raw(s) })
}

const _: () = {
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_setup, libsql_setup];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_error_message, libsql_error_message];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_database_init, libsql_database_init];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_database_sync, libsql_database_sync];

    let _: [unsafe extern "C" fn(_) -> _; 2] =
        [c::libsql_database_connect, libsql_database_connect];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [
        c::libsql_connection_transaction,
        libsql_connection_transaction,
    ];

    let _: [unsafe extern "C" fn(_, _) -> _; 2] =
        [c::libsql_connection_batch, libsql_connection_batch];

    let _: [unsafe extern "C" fn(_, _) -> _; 2] =
        [c::libsql_transaction_batch, libsql_transaction_batch];

    let _: [unsafe extern "C" fn(_, _) -> _; 2] =
        [c::libsql_connection_prepare, libsql_connection_prepare];
    let _: [unsafe extern "C" fn(_, _) -> _; 2] =
        [c::libsql_transaction_prepare, libsql_transaction_prepare];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_statement_query, libsql_statement_query];
    let _: [unsafe extern "C" fn(_) -> _; 2] =
        [c::libsql_statement_execute, libsql_statement_execute];

    let _: [unsafe extern "C" fn(_, _, _) -> _; 2] =
        [c::libsql_statement_bind_named, libsql_statement_bind_named];

    let _: [unsafe extern "C" fn(_, _) -> _; 2] =
        [c::libsql_statement_bind_value, libsql_statement_bind_value];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_rows_next, libsql_rows_next];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_row_empty, libsql_row_empty];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_row_length, libsql_row_length];
    let _: [unsafe extern "C" fn(_, _) -> _; 2] = [c::libsql_row_value, libsql_row_value];
    let _: [unsafe extern "C" fn(_, _) -> _; 2] = [c::libsql_row_name, libsql_row_name];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_integer, libsql_integer];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_real, libsql_real];
    let _: [unsafe extern "C" fn(_, _) -> _; 2] = [c::libsql_text, libsql_text];
    let _: [unsafe extern "C" fn(_, _) -> _; 2] = [c::libsql_blob, libsql_blob];
    let _: [unsafe extern "C" fn() -> _; 2] = [c::libsql_null, libsql_null];

    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_error_deinit, libsql_error_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_database_deinit, libsql_database_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] =
        [c::libsql_connection_deinit, libsql_connection_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_rows_deinit, libsql_rows_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_row_deinit, libsql_row_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] = [c::libsql_slice_deinit, libsql_slice_deinit];
    let _: [unsafe extern "C" fn(_) -> _; 2] =
        [c::libsql_transaction_rollback, libsql_transaction_rollback];
    let _: [unsafe extern "C" fn(_) -> _; 2] =
        [c::libsql_transaction_commit, libsql_transaction_commit];
};

#[cfg(test)]
mod tests {
    use crate::{libsql_database_deinit, libsql_setup};

    use super::c::*;
    use anyhow::Result;
    use std::{
        ffi::{c_char, CStr, CString},
        ops::Not,
    };

    #[test]
    fn memory_database() -> Result<()> {
        unsafe {
            println!("hello");
            let path = CString::new(":memory:")?;

            libsql_setup(libsql_config_t {
                ..Default::default()
            });

            let desc = libsql_database_desc_t {
                ..Default::default()
            };

            let db = libsql_database_init(desc);
            assert!(db.err.is_null());

            let conn = libsql_database_connect(db);
            assert!(conn.err.is_null());

            let sql = CString::new("select :named")?;
            let stmt = libsql_connection_prepare(conn, sql.as_ptr());
            assert!(stmt.err.is_null());

            let name = CString::new(":named")?;
            let bind = libsql_statement_bind_named(stmt, name.as_ptr(), libsql_integer(1));
            assert!(bind.err.is_null());

            let rows = libsql_statement_query(stmt);
            assert!(rows.err.is_null());

            let row = libsql_rows_next(rows);
            assert!(row.err.is_null());
            assert_eq!(libsql_row_empty(row), false);

            let name = libsql_row_name(row, 0);
            assert!(name.ptr.is_null().not());
            assert_ne!(name.len, 0);

            assert_eq!(
                ":named",
                CStr::from_ptr(name.ptr as *const c_char).to_str()?
            );
            libsql_slice_deinit(name);

            let value = libsql_row_value(row, 0);
            assert!(value.err.is_null());

            assert_eq!(value.ok.type_, libsql_type_t::LIBSQL_TYPE_INTEGER);
            assert_eq!(value.ok.value.integer, 1);

            libsql_database_deinit(db);
            libsql_connection_deinit(conn);
            libsql_statement_deinit(stmt);
            libsql_rows_deinit(rows);
            libsql_row_deinit(row);

            Ok(())
        }
    }

    #[test]
    fn test_database() -> Result<()> {
        unsafe {
            println!("hello");
            let path = CString::new("./test.db")?;

            let setup = libsql_setup(libsql_config_t {
                ..Default::default()
            });
            assert!(setup.is_null());

            let desc = libsql_database_desc_t {
                path: path.as_ptr(),
                ..Default::default()
            };

            let db = libsql_database_init(desc);
            assert!(db.err.is_null());

            let conn = libsql_database_connect(db);
            assert!(conn.err.is_null());

            let sql = CString::new(
                "create table if not exists test (i integer, r real, t text, b blob);",
            )?;
            let batch = libsql_connection_batch(conn, sql.as_ptr());
            assert!(batch.err.is_null());

            let sql = CString::new("insert into test values (:i, :r, :t, :b)")?;
            let stmt = libsql_connection_prepare(conn, sql.as_ptr());
            assert!(stmt.err.is_null());

            let name = CString::new(":i")?;
            let bind = libsql_statement_bind_named(stmt, name.as_ptr(), libsql_integer(1));
            assert!(bind.err.is_null());

            let name = CString::new(":r")?;
            let bind = libsql_statement_bind_named(stmt, name.as_ptr(), libsql_real(1.5));
            assert!(bind.err.is_null());

            let name = CString::new(":t")?;
            let value = CString::new("test")?;
            let bind = libsql_statement_bind_named(
                stmt,
                name.as_ptr(),
                libsql_text(
                    value.as_bytes_with_nul().as_ptr() as _,
                    value.as_bytes_with_nul().len(),
                ),
            );
            assert!(bind.err.is_null());

            let name = CString::new(":b")?;
            let value = vec![69u8, 42u8, 00u8];
            let bind = libsql_statement_bind_named(
                stmt,
                name.as_ptr(),
                libsql_blob(value.as_slice().as_ptr() as _, value.as_slice().len()),
            );
            assert!(bind.err.is_null());

            let exec = libsql_statement_execute(stmt);
            assert!(exec.err.is_null());
            assert_eq!(exec.rows_changed, 1);

            libsql_database_deinit(db);
            libsql_connection_deinit(conn);
            libsql_statement_deinit(stmt);

            Ok(())
        }
    }
}
