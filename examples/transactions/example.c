#include "../../libsql.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
        "DROP TABLE IF EXISTS users;"
        "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);"
        "INSERT INTO users (name) VALUES ('Iku Turso');";

    libsql_batch_t batch = libsql_connection_batch(conn, setup_sql);
    if (batch.err) {
        fprintf(
            stderr,
            "Error executing setup batch: %s\n",
            libsql_error_message(batch.err)
        );
        return 1;
    }

    libsql_transaction_t tx = libsql_connection_transaction(conn);
    if (tx.err) {
        fprintf(
            stderr,
            "Error starting transaction: %s\n",
            libsql_error_message(tx.err)
        );
        return 1;
    }

    const char *forenames[] = {"John", "Mary", "Alice", "Mark"};
    const char *surnames[] = {"Doe", "Smith", "Jones", "Taylor"};
    int forename_count = sizeof(forenames) / sizeof(forenames[0]);
    int surname_count = sizeof(surnames) / sizeof(surnames[0]);

    libsql_statement_t stmt =
        libsql_transaction_prepare(tx, "INSERT INTO users (name) VALUES (?)");
    if (stmt.err) {
        fprintf(
            stderr,
            "Error preparing statement: %s\n",
            libsql_error_message(stmt.err)
        );
        return 1;
    }

    for (int i = 0; i < forename_count; i++) {
        for (int j = 0; j < surname_count; j++) {
            char full_name[100];
            snprintf(
                full_name, sizeof(full_name), "%s %s", forenames[i], surnames[j]
            );

            libsql_statement_reset(stmt);
            libsql_statement_bind_value(
                stmt, libsql_text(full_name, strlen(full_name))
            );

            libsql_execute_t result = libsql_statement_execute(stmt);
            if (result.err) {
                fprintf(
                    stderr,
                    "Error inserting %s: %s\n",
                    full_name,
                    libsql_error_message(result.err)
                );
            }
        }
    }

    libsql_statement_deinit(stmt);

    libsql_transaction_rollback(tx);
    printf("Transaction rolled back.\n");

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

    libsql_row_t row;
    while (!(row = libsql_rows_next(rows)).err && !libsql_row_empty(row)) {
        libsql_result_value_t id = libsql_row_value(row, 0);
        libsql_result_value_t name = libsql_row_value(row, 1);

        if (id.err || name.err) {
            fprintf(stderr, "Error retrieving row values\n");
            continue;
        }

        printf(
            "%lld %s\n", id.ok.value.integer, (char *)name.ok.value.text.ptr
        );

        libsql_row_deinit(row);
    }

    libsql_rows_deinit(rows);
    libsql_statement_deinit(query_stmt);
    libsql_connection_deinit(conn);
    libsql_database_deinit(db);

    return 0;
}
