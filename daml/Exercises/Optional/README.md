# 06 · Optional

> **Coming soon** — exercises not yet written.

## Preview

`Optional` represents a value that may or may not be present — DAML's equivalent of `Maybe` in Haskell or `Option` in Rust.

```daml
safeDivide : Int -> Int -> Optional Decimal
safeDivide _ 0 = None
safeDivide x y = Some (intToDecimal x / intToDecimal y)
```

### What you will learn

- `Optional a`, `Some a`, `None`
- `fromSome`, `fromOptional`, `isNone`, `isSome`
- `mapOptional` (like `map` for Optional)
- Pattern matching on Optional
- Using Optional as a safe replacement for partial functions

## Further Reading

- [DA.Optional module](https://docs.daml.com/daml/stdlib/DA-Optional.html)
