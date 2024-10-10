<p align="center">
  <a href="https://tur.so/turso-c">
    <picture>
      <img src="/.github/cover.png" alt="libSQL C" />
    </picture>
  </a>
  <h1 align="center">libSQL C</h1>
</p>

<p align="center">
  Databases for C multi-tenant AI Apps.
</p>

<p align="center">
  <a href="https://tur.so/turso-c"><strong>Turso</strong></a> ·
  <a href="https://docs.turso.tech"><strong>Docs</strong></a> ·
  <a href="https://docs.turso.tech/sdk/c/quickstart"><strong>Quickstart</strong></a> ·
  <a href="https://docs.turso.tech/sdk/c/reference"><strong>SDK Reference</strong></a> ·
  <a href="https://turso.tech/blog"><strong>Blog &amp; Tutorials</strong></a>
</p>

<p align="center">
  <a href="LICENSE">
    <picture>
      <img src="https://img.shields.io/github/license/tursodatabase/libsql-c?color=0F624B" alt="MIT License" />
    </picture>
  </a>
  <a href="https://tur.so/discord-c">
    <picture>
      <img src="https://img.shields.io/discord/933071162680958986?color=0F624B" alt="Discord" />
    </picture>
  </a>
  <a href="#contributors">
    <picture>
      <img src="https://img.shields.io/github/contributors/tursodatabase/libsql-c?color=0F624B" alt="Contributors" />
    </picture>
  </a>
  <a href="https://packagist.org/packages/turso/libsql">
    <picture>
      <img src="https://img.shields.io/packagist/dt/turso/libsql?color=0F624B" alt="Total downloads" />
    </picture>
  </a>
  <a href="/examples">
    <picture>
      <img src="https://img.shields.io/badge/browse-examples-0F624B" alt="Examples" />
    </picture>
  </a>
</p>

## Features

- 🔌 Works offline with [Embedded Replicas](https://docs.turso.tech/features/embedded-replicas/introduction)
- 🌎 Works with remote Turso databases
- ✨ Works with Turso [AI & Vector Search](https://docs.turso.tech/features/ai-and-embeddings)

> [!WARNING]
> This SDK is currently in technical preview, and mostly used for internal use when building other libSQL SDKs. <a href="https://tur.so/discord-c">Join us in Discord</a> to report any issues.

## Install

1. Clone the repository:

   ```bash
   git clone https://github.com/your-repo/libsql-c.git
   cd libsql-c
   ```

2. Build the library:

   ```bash
   cargo build --release
   ```

3. The compiled library will be in `target/release/`:

   - `liblibsql.so` (Linux)
   - `liblibsql.dylib` (macOS)
   - `liblibsql.dll` (Windows)

4. Copy `libsql.h` and the compiled library to your project directory or a standard system location.

## Quickstart

1. Write your program:

   ```c
   #include <stdio.h>
   #include "libsql.h"

   int main() {
       libsql_setup((libsql_config_t){0});

       libsql_database_t db = libsql_database_init((libsql_database_desc_t){
           .path = "local.db"
       });

       if (db.err) {
           fprintf(stderr, "Error: %s\n", libsql_error_message(db.err));
           return 1;
       }

       libsql_connection_t conn = libsql_database_connect(db);
       if (conn.err) {
           fprintf(stderr, "Connection error: %s\n", libsql_error_message(conn.err));
           return 1;
       }

       const char* sql = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT);"
                         "INSERT INTO users (name) VALUES ('Alice');";

       libsql_batch_t batch = libsql_connection_batch(conn, sql);
       if (batch.err) {
           fprintf(stderr, "Batch error: %s\n", libsql_error_message(batch.err));
           return 1;
       }

       printf("Database operations completed successfully.\n");

       libsql_connection_deinit(conn);
       libsql_database_deinit(db);

       return 0;
   }
   ```

2. Compile your program, linking against the libsql library:

   ```
   gcc -o example example.c -L/path/to/libsql -llibsql
   ```

3. Run your program:
   ```
   ./example
   ```

## Examples

| Example                               | Description                                                                                                                                                                      |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [local](examples/local)               | Demonstrates how to use libsql with a local SQLite database file. Creates a database, inserts data, and performs queries.                                                        |
| [remote](examples/remote)             | Shows how to connect to a remote database using libsql. Requires setting up environment variables for the database URL and authentication token.                                 |
| [sync](examples/sync)                 | Illustrates the use of libsql's synchronization features. Creates a local database that syncs with a remote database, demonstrating how to handle offline and online operations. |
| [batch](examples/batch)               | Demonstrates how to execute multiple SQL statements in a single batch operation using libsql. Useful for efficient execution of multiple related operations.                     |
| [transactions](examples/transactions) | Shows how to use transactions in libsql. Demonstrates starting a transaction, performing multiple operations, and committing or rolling back changes.                            |
| [memory](examples/memory)             | Illustrates the use of an in-memory SQLite database with libsql. Useful for temporary storage, testing, or scenarios requiring fast access without data persistence.             |
| [vector](examples/vector)             | Demonstrates how to work with vector embeddings in libsql, including storing and querying vector data for similarity search.                                                     |
| [encryption](examples/encryption)     | Demonstrates how to create and use an encrypted SQLite database with libsql. Shows setting up encryption, writing data, and reading from an encrypted database.                  |

## Documentation

Visit our [official documentation](https://docs.turso.tech/sdk/c).

## Support

Join us [on Discord](https://tur.so/discord-c) to get help using this SDK. Report security issues [via email](mailto:security@turso.tech).

## Contributors

See the [contributing guide](CONTRIBUTING.md) to learn how to get involved.

![Contributors](https://contrib.nn.ci/api?repo=tursodatabase/libsql-c)

<a href="https://github.com/tursodatabase/libsql-c/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22">
  <picture>
    <img src="https://img.shields.io/github/issues-search/tursodatabase/libsql-c?label=good%20first%20issue&query=label%3A%22good%20first%20issue%22%20&color=0F624B" alt="good first issue" />
  </picture>
</a>
