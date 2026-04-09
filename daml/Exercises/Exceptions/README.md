# 18 · Exceptions

> **Coming soon** — exercises not yet written.

## Preview

DAML exceptions let you handle and recover from errors within a transaction.

```daml
exception InsufficientFunds
  with balance : Decimal; requested : Decimal
  where message "Insufficient funds: have " <> show balance <> ", need " <> show requested

withdraw : Decimal -> Update Decimal
withdraw amount = do
  balance <- ...
  if balance < amount
    then throw InsufficientFunds with balance; requested = amount
    else ...
```

### What you will learn

- Declaring custom `exception` types
- `throw` — raise an exception
- `try / catch` — handle exceptions and recover
- The difference between `abort` (no recovery) and `throw` (recoverable)
- Transaction rollback semantics

## Further Reading

- [DAML exceptions](https://docs.daml.com/daml/reference/exceptions.html)
