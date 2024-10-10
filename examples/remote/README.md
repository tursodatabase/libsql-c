# Remote

This example demonstrates how to use libSQL with a remote database.

## Building

```bash
make
```

## Running

```bash
TURSO_DATABASE_URL="..." TURSO_AUTH_TOKEN="..." ./example
```

This will connect to a remote database, insert some data, and query it.
