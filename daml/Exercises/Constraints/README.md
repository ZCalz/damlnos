# 13 · Constraints

> **Coming soon** — exercises not yet written.

## Preview

Constraints let you enforce invariants on contract data at creation time.

```daml
template BankAccount
  with
    owner   : Party
    balance : Decimal
  where
    signatory owner
    ensure balance >= 0.0   -- contracts with negative balance cannot be created
```

### What you will learn

- `ensure` — a Boolean precondition checked on `create`
- `assertMsg` inside choice `do` blocks — runtime checks
- The difference between `ensure` (creation) and choice-level assertions (exercise)
- Writing meaningful error messages

## Further Reading

- [Constraints in DAML](https://docs.daml.com/daml/intro/5_Restrictions.html)
