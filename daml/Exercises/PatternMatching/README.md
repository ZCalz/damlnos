# 04 · Pattern Matching

> **Coming soon** — exercises not yet written.

## Preview

`case` expressions let you branch on the structure of a value.

```daml
describe : Optional Int -> Text
describe opt = case opt of
  None   -> "nothing here"
  Some 0 -> "zero"
  Some n -> "got " <> show n
```

### What you will learn

- `case` expression syntax
- Matching on constructors and literals
- Wildcard patterns (`_`)
- Tuple patterns
- Exhaustiveness — DAML requires all cases to be covered

## Further Reading

- [Pattern matching](https://docs.daml.com/daml/intro/3_Data.html#variants)
