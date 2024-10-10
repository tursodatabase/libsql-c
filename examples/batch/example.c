#include "../../libsql.h"
#include <stdio.h>

int main() {
    libsql_setup((libsql_config_t){0});

    libsql_database_t db =
        libsql_database_init((libsql_database_desc_t){.path = "local.db"});
    if (db.err) {
        fprintf(
            stderr,
            "Error initializing database: %s\n",
            libsql_error_message(db.err)
        );
        return 1;
    }

    libsql_connection_t conn = libsql_database_connect(db);
    if (conn.err) {
        fprintf(
            stderr,
            "Error connecting to database: %s\n",
            libsql_error_message(conn.err)
        );
        return 1;
    }

    const char *batch_sql =
        "DROP TABLE IF EXISTS users;"
        "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);"
        "INSERT INTO users (name) VALUES ('Alice'), ('Bob'), ('Charlie');"
        "CREATE INDEX idx_name ON users (name);";

    libsql_batch_t batch = libsql_connection_batch(conn, batch_sql);
    if (batch.err) {
        fprintf(
            stderr,
            "Error executing batch: %s\n",
            libsql_error_message(batch.err)
        );
        return 1;
    }

    printf("Batch execution successful.\n");

    libsql_statement_t query_stmt =
        libsql_connection_prepare(conn, "SELECT * FROM users");
    if (query_stmt.err) {
        fprintf(
            stderr,
            "Error preparing query: %s\n",
            libsql_error_message(query_stmt.err)
        );
        return 1;
    }

    libsql_rows_t rows = libsql_statement_query(query_stmt);
    if (rows.err) {
        fprintf(
            stderr,
            "Error executing query: %s\n",
            libsql_error_message(rows.err)
        );
        return 1;
    }

    printf("Users in the database:\n");
    libsql_row_t row;
    while (!(row = libsql_rows_next(rows)).err && !libsql_row_empty(row)) {
        libsql_result_value_t id = libsql_row_value(row, 0);
        libsql_result_value_t name = libsql_row_value(row, 1);

        if (id.err || name.err) {
            fprintf(stderr, "Error retrieving row values\n");
            continue;
        }

        printf(
            "%lld: %s\n",
            (long long)id.ok.value.integer,
            (char *)name.ok.value.text.ptr
        );

        libsql_row_deinit(row);
    }

    libsql_rows_deinit(rows);
    libsql_statement_deinit(query_stmt);
    libsql_connection_deinit(conn);
    libsql_database_deinit(db);

    return 0;
}
