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

    const char *setup_sql =
        "DROP TABLE IF EXISTS movies;"
        "CREATE TABLE movies (title TEXT, year INT, embedding F32_BLOB(3));"
        "CREATE INDEX movies_idx ON movies (libsql_vector_idx(embedding));"
        "INSERT INTO movies (title, year, embedding) VALUES"
        "('Napoleon', 2023, vector32('[1,2,3]')),"
        "('Black Hawk Down', 2001, vector32('[10,11,12]')),"
        "('Gladiator', 2000, vector32('[7,8,9]')),"
        "('Blade Runner', 1982, vector32('[4,5,6]'));";

    libsql_batch_t batch = libsql_connection_batch(conn, setup_sql);
    if (batch.err) {
        fprintf(
            stderr,
            "Error executing setup batch: %s\n",
            libsql_error_message(batch.err)
        );
        return 1;
    }

    const char *query_sql = "SELECT title, year "
                            "FROM vector_top_k('movies_idx', '[4,5,6]', 3) "
                            "JOIN movies ON movies.rowid = id";

    libsql_statement_t stmt = libsql_connection_prepare(conn, query_sql);
    if (stmt.err) {
        fprintf(
            stderr,
            "Error preparing statement: %s\n",
            libsql_error_message(stmt.err)
        );
        return 1;
    }

    libsql_rows_t rows = libsql_statement_query(stmt);
    if (rows.err) {
        fprintf(
            stderr,
            "Error executing query: %s\n",
            libsql_error_message(rows.err)
        );
        return 1;
    }

    printf("Vector similarity search results:\n");
    libsql_row_t row;
    while (!(row = libsql_rows_next(rows)).err && !libsql_row_empty(row)) {
        libsql_result_value_t title = libsql_row_value(row, 0);
        libsql_result_value_t year = libsql_row_value(row, 1);

        if (title.err || year.err) {
            fprintf(stderr, "Error retrieving row values\n");
            continue;
        }

        printf(
            "%s (%lld)\n",
            (char *)title.ok.value.text.ptr,
            (long long)year.ok.value.integer
        );

        libsql_row_deinit(row);
    }

    libsql_rows_deinit(rows);
    libsql_statement_deinit(stmt);
    libsql_connection_deinit(conn);
    libsql_database_deinit(db);

    return 0;
}
