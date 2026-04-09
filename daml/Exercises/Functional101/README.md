# 08 · Functional 101

> **Coming soon** — exercises not yet written.

## Preview

Higher-order functions and function composition are central to idiomatic DAML.

```daml
applyTwice : (a -> a) -> a -> a
applyTwice f x = f (f x)

pipeline : [Int] -> Int
pipeline = foldl (+) 0 . filter even . map (*3)
```

### What you will learn

- Functions as values (passing functions to other functions)
- Anonymous functions (`\x -> x + 1`)
- Function composition with `(.)`
- The `$` operator (low-precedence application — avoids parentheses)
- `flip`, `const`, `id`

## Further Reading

- [Functional programming patterns in DAML](https://docs.daml.com/daml/intro/2_DamlScript.html)
