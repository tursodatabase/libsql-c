# Encryption

This example demonstrates how to create and use an encrypted SQLite database with libSQL.

## Building

```bash
make
```

## Running

```bash
./example
```

This example will:

1. Create an encrypted SQLite database file named `encrypted.db`.
2. Create a table called `secrets`.
3. Insert some sample data into the `secrets` table.
4. Query and display all secrets in the table.
