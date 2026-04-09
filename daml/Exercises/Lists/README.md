# 05 · Lists

> **Coming soon** — exercises not yet written.

## Preview

Lists are the primary collection type in DAML.

```daml
let nums = [1, 2, 3, 4, 5]
let doubled = map (*2) nums           -- [2, 4, 6, 8, 10]
let evens   = filter even nums        -- [2, 4]
let total   = foldl (+) 0 nums        -- 15
```

### What you will learn

- List literals and the `::` cons operator
- `map`, `filter`, `foldl`, `foldr`
- `length`, `head`, `tail`, `last`, `null`
- List comprehensions
- `DA.List` utility functions

## Further Reading

- [DA.List module](https://docs.daml.com/daml/stdlib/DA-List.html)
