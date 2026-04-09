# 02 · Records

> **Coming soon** — exercises not yet written.

## Preview

DAML records are product types — named collections of fields.

```daml
data Cash = Cash with
  currency : Text
  amount   : Decimal
    deriving (Eq, Show)
```

### What you will learn

- Defining `data` record types
- Field access with dot syntax: `cash.amount`
- Record update syntax: `cash with amount = 50.0`
- Nested records
- `deriving (Eq, Show)` — structural equality and printing

## Further Reading

- [DAML data types](https://docs.daml.com/daml/intro/3_Data.html)
