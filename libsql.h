
#ifndef LIBSQL_H
#define LIBSQL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <assert.h>

#ifdef __clang__
#define nullable _Nullable
#define nonnull _Nonnull
#else
#define nullable
#define nonnull
#endif

#define LEN_PTR(xs) sizeof(xs) / sizeof(xs[0]), (xs)

typedef struct libsql_error_t libsql_error_t;

typedef enum __attribute__((__packed__)) {
    LIBSQL_CYPHER_DEFAULT = 0,
    LIBSQL_CYPHER_AES256,
} libsql_cypher_t;

typedef enum __attribute__((__packed__)) {
    LIBSQL_TYPE_INTEGER = 1,
    LIBSQL_TYPE_REAL = 2,
    LIBSQL_TYPE_TEXT = 3,
    LIBSQL_TYPE_BLOB = 4,
    LIBSQL_TYPE_NULL = 5,
} libsql_type_t;

typedef enum __attribute__((__packed__)) {
    LIBSQL_TRACING_LEVEL_ERROR = 1,
    LIBSQL_TRACING_LEVEL_WARN,
    LIBSQL_TRACING_LEVEL_INFO,
    LIBSQL_TRACING_LEVEL_DEBUG,
    LIBSQL_TRACING_LEVEL_TRACE,
} libsql_tracing_level_t;

static_assert(sizeof(libsql_type_t) == 1, "expect libsql_type_t to be 1 byte");
static_assert(sizeof(libsql_cypher_t) == 1, "expect libsql_cypher_t to be 1 byte");
static_assert(sizeof(libsql_tracing_level_t) == 1, "expect libsql_tracing_level_t to be 1 byte");

typedef struct {
    const char* message;
    const char* target;
    const char* file;
    uint64_t timestamp;
    size_t line;
    libsql_tracing_level_t level;
} libsql_log_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_database_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_connection_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_parameters_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_statement_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_transaction_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_rows_t;

typedef struct {
    libsql_error_t *err;
    void *inner;
} libsql_row_t;

typedef struct {
    libsql_error_t *err;
} libsql_batch_t;

typedef struct {
    const void *ptr;
    size_t len;
} libsql_slice_t;

typedef union {
    int64_t integer;
    double real;
    libsql_slice_t text;
    libsql_slice_t blob;
} libsql_value_union_t;

typedef struct {
    libsql_value_union_t value;
    libsql_type_t type;
} libsql_value_t;

typedef struct {
    libsql_value_t ok;
    libsql_error_t *err;
} libsql_result_value_t;

typedef struct {
    uint64_t frame_no;
    uint64_t frames_synced;
    libsql_error_t *err;
} libsql_sync_t;

typedef struct {
    libsql_error_t *err;
} libsql_bind_t;

typedef struct {
    uint64_t rows_changed;
    libsql_error_t *err;
} libsql_execute_t;

/// If only path is set, try to open a local database or a memory one if path ==
/// ":memory:". A encryption key and cypher can be set to the database if the
/// crate is compiled with support for it.
///
/// If there url and auth_token are set, a remote only database is created and
/// any other options are ignored.
///
/// If all path, url and auth_token are set, a embedded replica database is
/// created. And dont_read_your_writes and sync_interval are applicable now.

typedef struct {
    const char *nullable url;
    const char *nullable path;
    const char *nullable auth_token;
    const char *nullable key;
    uint64_t sync_interval;
    libsql_cypher_t cypher;
    bool not_read_your_writes;
    bool webpki;
} libsql_database_desc_t;

typedef struct {
    void (*nullable logger)(libsql_log_t log);
} libsql_config_t;

void libsql_setup(libsql_config_t config);

const char *libsql_error_message(libsql_error_t *self);

/// If trying to open a database with a NULL description, a error will be
/// returned.
libsql_database_t
libsql_database_init(libsql_database_desc_t desc);

/// Sync frames from the primary
///
/// @return If sucessful, ruturns the amount of frames that where synced and the
/// last frame number.
libsql_sync_t libsql_database_sync(libsql_database_t self);

libsql_connection_t libsql_database_connect(libsql_database_t self);

libsql_transaction_t libsql_connection_transaction(libsql_connection_t self);

libsql_batch_t libsql_connection_batch(
    libsql_connection_t self,
    const char *nullable sql
);

libsql_batch_t libsql_transaction_batch(
    libsql_transaction_t self,
    const char *nullable sql
);

libsql_statement_t
libsql_connection_prepare(libsql_connection_t self, const char *nullable sql);
libsql_statement_t
libsql_transaction_prepare(libsql_transaction_t self, const char *nullable sql);

libsql_execute_t libsql_statement_execute(libsql_statement_t self);
libsql_rows_t libsql_statement_query(libsql_statement_t self);
void libsql_statement_reset(libsql_statement_t self);

libsql_row_t libsql_rows_next(libsql_rows_t self);

libsql_result_value_t libsql_row_value(libsql_row_t self, int32_t index);
libsql_slice_t libsql_row_name(libsql_row_t self, int32_t index);
int32_t libsql_row_length(libsql_row_t self);
bool libsql_row_empty(libsql_row_t self);

libsql_bind_t libsql_statement_bind_named(
    libsql_statement_t self,
    const char *name,
    libsql_value_t value
);
libsql_bind_t
libsql_statement_bind_value(libsql_statement_t self, libsql_value_t value);

libsql_value_t libsql_integer(int64_t integer);
libsql_value_t libsql_real(double real);
libsql_value_t libsql_text(const char *ptr, size_t len);
libsql_value_t libsql_blob(const uint8_t *ptr, size_t len);

void libsql_error_deinit(libsql_error_t *self);
void libsql_database_deinit(libsql_database_t self);
void libsql_connection_deinit(libsql_connection_t self);
void libsql_statement_deinit(libsql_statement_t self);
void libsql_transaction_commit(libsql_transaction_t self);
void libsql_transaction_rollback(libsql_transaction_t self);
void libsql_rows_deinit(libsql_rows_t self);
void libsql_row_deinit(libsql_row_t self);
void libsql_slice_deinit(libsql_slice_t value);

#endif /* LIBSQL_H */
