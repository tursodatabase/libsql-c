/* automatically generated by rust-bindgen 0.70.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_error_t {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum libsql_cypher_t {
    LIBSQL_CYPHER_DEFAULT = 0,
    LIBSQL_CYPHER_AES256 = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum libsql_type_t {
    LIBSQL_TYPE_INTEGER = 1,
    LIBSQL_TYPE_REAL = 2,
    LIBSQL_TYPE_TEXT = 3,
    LIBSQL_TYPE_BLOB = 4,
    LIBSQL_TYPE_NULL = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum libsql_tracing_level_t {
    LIBSQL_TRACING_LEVEL_ERROR = 1,
    LIBSQL_TRACING_LEVEL_WARN = 2,
    LIBSQL_TRACING_LEVEL_INFO = 3,
    LIBSQL_TRACING_LEVEL_DEBUG = 4,
    LIBSQL_TRACING_LEVEL_TRACE = 5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_log_t {
    pub message: *const ::std::os::raw::c_char,
    pub target: *const ::std::os::raw::c_char,
    pub file: *const ::std::os::raw::c_char,
    pub timestamp: u64,
    pub line: usize,
    pub level: libsql_tracing_level_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_log_t"][::std::mem::size_of::<libsql_log_t>() - 48usize];
    ["Alignment of libsql_log_t"][::std::mem::align_of::<libsql_log_t>() - 8usize];
    ["Offset of field: libsql_log_t::message"]
        [::std::mem::offset_of!(libsql_log_t, message) - 0usize];
    ["Offset of field: libsql_log_t::target"]
        [::std::mem::offset_of!(libsql_log_t, target) - 8usize];
    ["Offset of field: libsql_log_t::file"][::std::mem::offset_of!(libsql_log_t, file) - 16usize];
    ["Offset of field: libsql_log_t::timestamp"]
        [::std::mem::offset_of!(libsql_log_t, timestamp) - 24usize];
    ["Offset of field: libsql_log_t::line"][::std::mem::offset_of!(libsql_log_t, line) - 32usize];
    ["Offset of field: libsql_log_t::level"][::std::mem::offset_of!(libsql_log_t, level) - 40usize];
};
impl Default for libsql_log_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_database_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_database_t"][::std::mem::size_of::<libsql_database_t>() - 16usize];
    ["Alignment of libsql_database_t"][::std::mem::align_of::<libsql_database_t>() - 8usize];
    ["Offset of field: libsql_database_t::err"]
        [::std::mem::offset_of!(libsql_database_t, err) - 0usize];
    ["Offset of field: libsql_database_t::inner"]
        [::std::mem::offset_of!(libsql_database_t, inner) - 8usize];
};
impl Default for libsql_database_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_connection_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_connection_t"][::std::mem::size_of::<libsql_connection_t>() - 16usize];
    ["Alignment of libsql_connection_t"][::std::mem::align_of::<libsql_connection_t>() - 8usize];
    ["Offset of field: libsql_connection_t::err"]
        [::std::mem::offset_of!(libsql_connection_t, err) - 0usize];
    ["Offset of field: libsql_connection_t::inner"]
        [::std::mem::offset_of!(libsql_connection_t, inner) - 8usize];
};
impl Default for libsql_connection_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_statement_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_statement_t"][::std::mem::size_of::<libsql_statement_t>() - 16usize];
    ["Alignment of libsql_statement_t"][::std::mem::align_of::<libsql_statement_t>() - 8usize];
    ["Offset of field: libsql_statement_t::err"]
        [::std::mem::offset_of!(libsql_statement_t, err) - 0usize];
    ["Offset of field: libsql_statement_t::inner"]
        [::std::mem::offset_of!(libsql_statement_t, inner) - 8usize];
};
impl Default for libsql_statement_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_transaction_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_transaction_t"][::std::mem::size_of::<libsql_transaction_t>() - 16usize];
    ["Alignment of libsql_transaction_t"][::std::mem::align_of::<libsql_transaction_t>() - 8usize];
    ["Offset of field: libsql_transaction_t::err"]
        [::std::mem::offset_of!(libsql_transaction_t, err) - 0usize];
    ["Offset of field: libsql_transaction_t::inner"]
        [::std::mem::offset_of!(libsql_transaction_t, inner) - 8usize];
};
impl Default for libsql_transaction_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_rows_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_rows_t"][::std::mem::size_of::<libsql_rows_t>() - 16usize];
    ["Alignment of libsql_rows_t"][::std::mem::align_of::<libsql_rows_t>() - 8usize];
    ["Offset of field: libsql_rows_t::err"][::std::mem::offset_of!(libsql_rows_t, err) - 0usize];
    ["Offset of field: libsql_rows_t::inner"]
        [::std::mem::offset_of!(libsql_rows_t, inner) - 8usize];
};
impl Default for libsql_rows_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_row_t {
    pub err: *mut libsql_error_t,
    pub inner: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_row_t"][::std::mem::size_of::<libsql_row_t>() - 16usize];
    ["Alignment of libsql_row_t"][::std::mem::align_of::<libsql_row_t>() - 8usize];
    ["Offset of field: libsql_row_t::err"][::std::mem::offset_of!(libsql_row_t, err) - 0usize];
    ["Offset of field: libsql_row_t::inner"][::std::mem::offset_of!(libsql_row_t, inner) - 8usize];
};
impl Default for libsql_row_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_batch_t {
    pub err: *mut libsql_error_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_batch_t"][::std::mem::size_of::<libsql_batch_t>() - 8usize];
    ["Alignment of libsql_batch_t"][::std::mem::align_of::<libsql_batch_t>() - 8usize];
    ["Offset of field: libsql_batch_t::err"][::std::mem::offset_of!(libsql_batch_t, err) - 0usize];
};
impl Default for libsql_batch_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_slice_t {
    pub ptr: *const ::std::os::raw::c_void,
    pub len: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_slice_t"][::std::mem::size_of::<libsql_slice_t>() - 16usize];
    ["Alignment of libsql_slice_t"][::std::mem::align_of::<libsql_slice_t>() - 8usize];
    ["Offset of field: libsql_slice_t::ptr"][::std::mem::offset_of!(libsql_slice_t, ptr) - 0usize];
    ["Offset of field: libsql_slice_t::len"][::std::mem::offset_of!(libsql_slice_t, len) - 8usize];
};
impl Default for libsql_slice_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union libsql_value_union_t {
    pub integer: i64,
    pub real: f64,
    pub text: libsql_slice_t,
    pub blob: libsql_slice_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_value_union_t"][::std::mem::size_of::<libsql_value_union_t>() - 16usize];
    ["Alignment of libsql_value_union_t"][::std::mem::align_of::<libsql_value_union_t>() - 8usize];
    ["Offset of field: libsql_value_union_t::integer"]
        [::std::mem::offset_of!(libsql_value_union_t, integer) - 0usize];
    ["Offset of field: libsql_value_union_t::real"]
        [::std::mem::offset_of!(libsql_value_union_t, real) - 0usize];
    ["Offset of field: libsql_value_union_t::text"]
        [::std::mem::offset_of!(libsql_value_union_t, text) - 0usize];
    ["Offset of field: libsql_value_union_t::blob"]
        [::std::mem::offset_of!(libsql_value_union_t, blob) - 0usize];
};
impl Default for libsql_value_union_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libsql_value_t {
    pub value: libsql_value_union_t,
    pub type_: libsql_type_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_value_t"][::std::mem::size_of::<libsql_value_t>() - 24usize];
    ["Alignment of libsql_value_t"][::std::mem::align_of::<libsql_value_t>() - 8usize];
    ["Offset of field: libsql_value_t::value"]
        [::std::mem::offset_of!(libsql_value_t, value) - 0usize];
    ["Offset of field: libsql_value_t::type_"]
        [::std::mem::offset_of!(libsql_value_t, type_) - 16usize];
};
impl Default for libsql_value_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libsql_result_value_t {
    pub err: *mut libsql_error_t,
    pub ok: libsql_value_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_result_value_t"][::std::mem::size_of::<libsql_result_value_t>() - 32usize];
    ["Alignment of libsql_result_value_t"]
        [::std::mem::align_of::<libsql_result_value_t>() - 8usize];
    ["Offset of field: libsql_result_value_t::err"]
        [::std::mem::offset_of!(libsql_result_value_t, err) - 0usize];
    ["Offset of field: libsql_result_value_t::ok"]
        [::std::mem::offset_of!(libsql_result_value_t, ok) - 8usize];
};
impl Default for libsql_result_value_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_sync_t {
    pub err: *mut libsql_error_t,
    pub frame_no: u64,
    pub frames_synced: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_sync_t"][::std::mem::size_of::<libsql_sync_t>() - 24usize];
    ["Alignment of libsql_sync_t"][::std::mem::align_of::<libsql_sync_t>() - 8usize];
    ["Offset of field: libsql_sync_t::err"][::std::mem::offset_of!(libsql_sync_t, err) - 0usize];
    ["Offset of field: libsql_sync_t::frame_no"]
        [::std::mem::offset_of!(libsql_sync_t, frame_no) - 8usize];
    ["Offset of field: libsql_sync_t::frames_synced"]
        [::std::mem::offset_of!(libsql_sync_t, frames_synced) - 16usize];
};
impl Default for libsql_sync_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_bind_t {
    pub err: *mut libsql_error_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_bind_t"][::std::mem::size_of::<libsql_bind_t>() - 8usize];
    ["Alignment of libsql_bind_t"][::std::mem::align_of::<libsql_bind_t>() - 8usize];
    ["Offset of field: libsql_bind_t::err"][::std::mem::offset_of!(libsql_bind_t, err) - 0usize];
};
impl Default for libsql_bind_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_execute_t {
    pub err: *mut libsql_error_t,
    pub rows_changed: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_execute_t"][::std::mem::size_of::<libsql_execute_t>() - 16usize];
    ["Alignment of libsql_execute_t"][::std::mem::align_of::<libsql_execute_t>() - 8usize];
    ["Offset of field: libsql_execute_t::err"]
        [::std::mem::offset_of!(libsql_execute_t, err) - 0usize];
    ["Offset of field: libsql_execute_t::rows_changed"]
        [::std::mem::offset_of!(libsql_execute_t, rows_changed) - 8usize];
};
impl Default for libsql_execute_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_connection_info_t {
    pub err: *mut libsql_error_t,
    pub last_inserted_rowid: i64,
    pub total_changes: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_connection_info_t"]
        [::std::mem::size_of::<libsql_connection_info_t>() - 24usize];
    ["Alignment of libsql_connection_info_t"]
        [::std::mem::align_of::<libsql_connection_info_t>() - 8usize];
    ["Offset of field: libsql_connection_info_t::err"]
        [::std::mem::offset_of!(libsql_connection_info_t, err) - 0usize];
    ["Offset of field: libsql_connection_info_t::last_inserted_rowid"]
        [::std::mem::offset_of!(libsql_connection_info_t, last_inserted_rowid) - 8usize];
    ["Offset of field: libsql_connection_info_t::total_changes"]
        [::std::mem::offset_of!(libsql_connection_info_t, total_changes) - 16usize];
};
impl Default for libsql_connection_info_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Database description."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_database_desc_t {
    #[doc = " The url to the primary database"]
    pub url: *const ::std::os::raw::c_char,
    #[doc = " Path to the database file or `:memory:`"]
    pub path: *const ::std::os::raw::c_char,
    #[doc = " Auth token to access the primary"]
    pub auth_token: *const ::std::os::raw::c_char,
    #[doc = " Encryption key to encrypt and decrypt the database in `path`"]
    pub encryption_key: *const ::std::os::raw::c_char,
    #[doc = " Interval to periodicaly sync with primary"]
    pub sync_interval: u64,
    #[doc = " Cypher to be used with `encryption_key`"]
    pub cypher: libsql_cypher_t,
    #[doc = " If set, disable `read_your_writes`. To mantain consistency."]
    pub disable_read_your_writes: bool,
    #[doc = " Enable Webpki connector"]
    pub webpki: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_database_desc_t"][::std::mem::size_of::<libsql_database_desc_t>() - 48usize];
    ["Alignment of libsql_database_desc_t"]
        [::std::mem::align_of::<libsql_database_desc_t>() - 8usize];
    ["Offset of field: libsql_database_desc_t::url"]
        [::std::mem::offset_of!(libsql_database_desc_t, url) - 0usize];
    ["Offset of field: libsql_database_desc_t::path"]
        [::std::mem::offset_of!(libsql_database_desc_t, path) - 8usize];
    ["Offset of field: libsql_database_desc_t::auth_token"]
        [::std::mem::offset_of!(libsql_database_desc_t, auth_token) - 16usize];
    ["Offset of field: libsql_database_desc_t::encryption_key"]
        [::std::mem::offset_of!(libsql_database_desc_t, encryption_key) - 24usize];
    ["Offset of field: libsql_database_desc_t::sync_interval"]
        [::std::mem::offset_of!(libsql_database_desc_t, sync_interval) - 32usize];
    ["Offset of field: libsql_database_desc_t::cypher"]
        [::std::mem::offset_of!(libsql_database_desc_t, cypher) - 40usize];
    ["Offset of field: libsql_database_desc_t::disable_read_your_writes"]
        [::std::mem::offset_of!(libsql_database_desc_t, disable_read_your_writes) - 44usize];
    ["Offset of field: libsql_database_desc_t::webpki"]
        [::std::mem::offset_of!(libsql_database_desc_t, webpki) - 45usize];
};
impl Default for libsql_database_desc_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libsql_config_t {
    pub logger: ::std::option::Option<unsafe extern "C" fn(log: libsql_log_t)>,
    pub version: *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of libsql_config_t"][::std::mem::size_of::<libsql_config_t>() - 16usize];
    ["Alignment of libsql_config_t"][::std::mem::align_of::<libsql_config_t>() - 8usize];
    ["Offset of field: libsql_config_t::logger"]
        [::std::mem::offset_of!(libsql_config_t, logger) - 0usize];
    ["Offset of field: libsql_config_t::version"]
        [::std::mem::offset_of!(libsql_config_t, version) - 8usize];
};
impl Default for libsql_config_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[doc = " Setup some global info"]
    pub fn libsql_setup(config: libsql_config_t) -> *const libsql_error_t;
}
extern "C" {
    #[doc = " Get the error message from a error"]
    pub fn libsql_error_message(self_: *mut libsql_error_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Create or open a database"]
    pub fn libsql_database_init(desc: libsql_database_desc_t) -> libsql_database_t;
}
extern "C" {
    #[doc = " Sync frames with the primary"]
    pub fn libsql_database_sync(self_: libsql_database_t) -> libsql_sync_t;
}
extern "C" {
    #[doc = " Connect with the database"]
    pub fn libsql_database_connect(self_: libsql_database_t) -> libsql_connection_t;
}
extern "C" {
    #[doc = " Begin a transaction"]
    pub fn libsql_connection_transaction(self_: libsql_connection_t) -> libsql_transaction_t;
}
extern "C" {
    #[doc = " Send a batch statement in a connection"]
    pub fn libsql_connection_batch(
        self_: libsql_connection_t,
        sql: *const ::std::os::raw::c_char,
    ) -> libsql_batch_t;
}
extern "C" {
    #[doc = " Send a batch statement in a connection"]
    pub fn libsql_connection_info(self_: libsql_connection_t) -> libsql_connection_info_t;
}
extern "C" {
    #[doc = " Send a batch statement in a transaction"]
    pub fn libsql_transaction_batch(
        self_: libsql_transaction_t,
        sql: *const ::std::os::raw::c_char,
    ) -> libsql_batch_t;
}
extern "C" {
    #[doc = " Prepare a statement in a connection"]
    pub fn libsql_connection_prepare(
        self_: libsql_connection_t,
        sql: *const ::std::os::raw::c_char,
    ) -> libsql_statement_t;
}
extern "C" {
    #[doc = " Prepare a statement in a transaction"]
    pub fn libsql_transaction_prepare(
        self_: libsql_transaction_t,
        sql: *const ::std::os::raw::c_char,
    ) -> libsql_statement_t;
}
extern "C" {
    #[doc = " Execute a statement"]
    pub fn libsql_statement_execute(self_: libsql_statement_t) -> libsql_execute_t;
}
extern "C" {
    #[doc = " Query a statement"]
    pub fn libsql_statement_query(self_: libsql_statement_t) -> libsql_rows_t;
}
extern "C" {
    #[doc = " Reset a statement"]
    pub fn libsql_statement_reset(self_: libsql_statement_t);
}
extern "C" {
    #[doc = " Column count"]
    pub fn libsql_statement_column_count(self_: libsql_statement_t) -> usize;
}
extern "C" {
    #[doc = " Get the next row from rows"]
    pub fn libsql_rows_next(self_: libsql_rows_t) -> libsql_row_t;
}
extern "C" {
    #[doc = " Get the column name at the index"]
    pub fn libsql_rows_column_name(self_: libsql_rows_t, index: i32) -> libsql_slice_t;
}
extern "C" {
    #[doc = " Get rows column count"]
    pub fn libsql_rows_column_count(self_: libsql_rows_t) -> i32;
}
extern "C" {
    #[doc = " Get the value at the the index"]
    pub fn libsql_row_value(self_: libsql_row_t, index: i32) -> libsql_result_value_t;
}
extern "C" {
    #[doc = " Get the column name at the the index"]
    pub fn libsql_row_name(self_: libsql_row_t, index: i32) -> libsql_slice_t;
}
extern "C" {
    #[doc = " Get row column count"]
    pub fn libsql_row_length(self_: libsql_row_t) -> i32;
}
extern "C" {
    #[doc = " Check if the row is empty, indicating the end of `libsql_rows_next`"]
    pub fn libsql_row_empty(self_: libsql_row_t) -> bool;
}
extern "C" {
    #[doc = " Bind a named argument to a statement"]
    pub fn libsql_statement_bind_named(
        self_: libsql_statement_t,
        name: *const ::std::os::raw::c_char,
        value: libsql_value_t,
    ) -> libsql_bind_t;
}
extern "C" {
    #[doc = " Bind a positional argument to a statement"]
    pub fn libsql_statement_bind_value(
        self_: libsql_statement_t,
        value: libsql_value_t,
    ) -> libsql_bind_t;
}
extern "C" {
    #[doc = " Create a libsql integer value"]
    pub fn libsql_integer(integer: i64) -> libsql_value_t;
}
extern "C" {
    #[doc = " Create a libsql real value"]
    pub fn libsql_real(real: f64) -> libsql_value_t;
}
extern "C" {
    #[doc = " Create a libsql text value"]
    pub fn libsql_text(ptr: *const ::std::os::raw::c_char, len: usize) -> libsql_value_t;
}
extern "C" {
    #[doc = " Create a libsql blob value"]
    pub fn libsql_blob(ptr: *const u8, len: usize) -> libsql_value_t;
}
extern "C" {
    #[doc = " Create a libsql null value"]
    pub fn libsql_null() -> libsql_value_t;
}
extern "C" {
    #[doc = " Deallocate and close a error"]
    pub fn libsql_error_deinit(self_: *mut libsql_error_t);
}
extern "C" {
    #[doc = " Deallocate and close a database"]
    pub fn libsql_database_deinit(self_: libsql_database_t);
}
extern "C" {
    #[doc = " Deallocate and close a connection"]
    pub fn libsql_connection_deinit(self_: libsql_connection_t);
}
extern "C" {
    #[doc = " Deallocate and close a statement"]
    pub fn libsql_statement_deinit(self_: libsql_statement_t);
}
extern "C" {
    #[doc = " Deallocate and commit a transaction (transaction becomes invalid)"]
    pub fn libsql_transaction_commit(self_: libsql_transaction_t);
}
extern "C" {
    #[doc = " Deallocate and rollback a transaction (transaction becomes invalid)"]
    pub fn libsql_transaction_rollback(self_: libsql_transaction_t);
}
extern "C" {
    #[doc = " Deallocate and close rows"]
    pub fn libsql_rows_deinit(self_: libsql_rows_t);
}
extern "C" {
    #[doc = " Deallocate and close a row"]
    pub fn libsql_row_deinit(self_: libsql_row_t);
}
extern "C" {
    #[doc = " Deallocate a slice"]
    pub fn libsql_slice_deinit(value: libsql_slice_t);
}
