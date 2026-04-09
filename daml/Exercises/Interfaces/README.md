# 17 · Interfaces

> **Coming soon** — exercises not yet written.

## Preview

Interfaces define a common API that different templates can implement, enabling polymorphism across contract types.

```daml
interface Token where
  getOwner  : Party
  getAmount : Decimal

  choice Transfer : ContractId Token
    with newOwner : Party
    controller getOwner this
    do ...
```

### What you will learn

- Declaring an `interface` with view and choice signatures
- `implements` — making a template satisfy an interface
- `toInterface` / `fromInterface` — upcasting and safe downcasting
- Interface dispatch — calling interface choices on any implementing template
- `requires` — interface inheritance

## Further Reading

- [DAML interfaces](https://docs.daml.com/daml/reference/interfaces.html)
