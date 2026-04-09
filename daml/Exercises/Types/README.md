# 01 · Types

DAML is a strongly-typed language. Every value has an exact type and mixing types is a compile error — which you will fix in these exercises.

## Core Primitive Types

| Type      | Example literals          | Notes |
|-----------|--------------------------|-------|
| `Text`    | `"hello"`, `""`          | Unicode strings; concatenate with `<>` |
| `Int`     | `42`, `-7`, `0`          | 64-bit signed integer |
| `Decimal` | `3.14`, `100.0`, `-0.5`  | Fixed-point; 38 digits, 10 decimal places |
| `Bool`    | `True`, `False`          | |
| `Party`   | (allocated at runtime)   | Opaque ledger identity |
| `Date`    | `date 2026 Jan 1`        | Calendar date |
| `Time`    | (from `DA.Time`)         | UTC timestamp |

## Exercises

| File | Concept |
|------|---------|
| `Ex1.daml` | Primitive literals and type annotations |
| `Ex2.daml` | `Int` vs `Decimal`; `intToDecimal` |
| `Ex3.daml` | `Text` concatenation with `<>`; `show`; `DA.Text` functions |
| `Ex4.daml` | `Party`; `allocateParty`; `<-` bind syntax |

## Further Reading

- [DAML built-in types](https://docs.daml.com/daml/intro/1_Token.html)
- [DA.Text module](https://docs.daml.com/daml/stdlib/DA-Text.html)
- [Prelude](https://docs.daml.com/daml/stdlib/Prelude.html)
