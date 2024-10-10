# Transactions

This example demonstrates how to use transactions with libSQL.

## Building

```bash
make
```

## Running

```bash
./example
```

This example will:

1. Create a new table called `users`.
2. Start a transaction.
3. Insert multiple users within the transaction.
4. Demonstrate how to rollback a transaction.
5. Start another transaction.
6. Insert more users and commit the transaction.
7. Query and display the final state of the `users` table.
