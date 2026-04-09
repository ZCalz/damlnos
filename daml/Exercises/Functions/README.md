# 03 · Functions

> **Coming soon** — exercises not yet written.

## Preview

DAML functions are pure and first-class. There are no side effects outside of the `Script` (or `Update`) monad.

```daml
double : Int -> Int
double x = x * 2

clamp : Int -> Int -> Int -> Int
clamp lo hi x = if x < lo then lo else if x > hi then hi else x
```

### What you will learn

- Top-level function definitions and type signatures
- `if / then / else` expressions
- Guards (`| condition = expr`)
- `let / in` and `where` clauses
- Partial application and currying

## Further Reading

- [Functions in DAML](https://docs.daml.com/daml/intro/2_DamlScript.html)
