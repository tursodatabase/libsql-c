#include "../../libsql.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    libsql_setup((libsql_config_t){0});

    const char *url = getenv("TURSO_DATABASE_URL");
    const char *auth_token = getenv("TURSO_AUTH_TOKEN");

    if (!url || !auth_token) {
        fprintf(
            stderr,
            "Error: TURSO_DATABASE_URL and TURSO_AUTH_TOKEN environment "
            "variables must be set.\n"
        );
        return 1;
    }

    libsql_database_t db = libsql_database_init((libsql_database_desc_t
    ){.path = "local.db",
      .url = url,
      .auth_token = auth_token,
      .sync_interval = 60000});

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
        "CREATE TABLE IF NOT EXISTS sync_test (id INTEGER PRIMARY KEY "
        "AUTOINCREMENT, value TEXT);"
        "INSERT INTO sync_test (value) VALUES ('Initial value');";

    libsql_batch_t batch = libsql_connection_batch(conn, setup_sql);
    if (batch.err) {
        fprintf(
            stderr,
            "Error executing setup batch: %s\n",
            libsql_error_message(batch.err)
        );
        return 1;
    }

    printf("Initial data inserted. Waiting for sync...\n");

    sleep(15); // Wait for at least one sync has occurred

    // Manual sync
    libsql_sync_t sync = libsql_database_sync(db);
    if (sync.err) {
        fprintf(
            stderr,
            "Error syncing database: %s\n",
            libsql_error_message(sync.err)
        );
    } else {
        printf(
            "Manual sync completed. Frame number: %llu, Frames synced: %llu\n",
            (unsigned long long)sync.frame_no,
            (unsigned long long)sync.frames_synced
        );
    }

    const char *insert_sql =
        "INSERT INTO sync_test (value) VALUES ('New value after sync');";
    batch = libsql_connection_batch(conn, insert_sql);
    if (batch.err) {
        fprintf(
            stderr,
            "Error inserting new data: %s\n",
            libsql_error_message(batch.err)
        );
    } else {
        printf("New data inserted.\n");
    }

    libsql_statement_t query_stmt =
        libsql_connection_prepare(conn, "SELECT * FROM sync_test");
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

    printf("Current data in sync_test table:\n");
    libsql_row_t row;
    while (!(row = libsql_rows_next(rows)).err && !libsql_row_empty(row)) {
        libsql_result_value_t id = libsql_row_value(row, 0);
        libsql_result_value_t value = libsql_row_value(row, 1);

        if (id.err || value.err) {
            fprintf(stderr, "Error retrieving row values\n");
            continue;
        }

        printf(
            "%lld: %s\n",
            (long long)id.ok.value.integer,
            (char *)value.ok.value.text.ptr
        );

        libsql_row_deinit(row);
    }

    libsql_rows_deinit(rows);
    libsql_statement_deinit(query_stmt);
    libsql_connection_deinit(conn);
    libsql_database_deinit(db);

    return 0;
}
