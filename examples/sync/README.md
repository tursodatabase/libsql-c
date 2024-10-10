# Sync

This example demonstrates how to use libSQL with a synced database (local file synced with a remote database).

## Building

```bash
make
```

## Running

```bash
TURSO_DATABASE_URL="..." TURSO_AUTH_TOKEN="..." ./example
```

This will create a local database file that syncs with a remote database, insert some data, and query it.
