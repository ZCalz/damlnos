# 07 · Variants

> **Coming soon** — exercises not yet written.

## Preview

Variants (sum types) let a value be one of several named constructors.

```daml
data Direction = North | South | East | West

data Shape
  = Circle    with radius : Decimal
  | Rectangle with width : Decimal; height : Decimal
  | Triangle  with base : Decimal;  height : Decimal
```

### What you will learn

- Simple enum-style variants
- Variants with payload fields
- Pattern matching on variants
- `deriving (Eq, Show)`
- Using variants to model domain states (e.g. `Pending | Active | Settled`)

## Further Reading

- [Variants in DAML](https://docs.daml.com/daml/intro/3_Data.html#variants)
