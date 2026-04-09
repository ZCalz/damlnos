# 10 · Choices

> **Coming soon** — exercises not yet written.

## Preview

Choices are the only way to transform contracts on the ledger. Each choice has a controller who must authorise it.

```daml
template SimpleIou
  with
    issuer : Party
    owner  : Party
    amount : Decimal
  where
    signatory issuer
    observer  owner

    choice Transfer : ContractId SimpleIou
      with newOwner : Party
      controller owner
      do create this with owner = newOwner
```

### What you will learn

- Consuming choices (archive the contract, return a new one)
- Non-consuming choices (`nonconsuming` keyword — contract survives)
- The `controller` field and authorization rules
- `do` blocks inside choices: `create`, `archive`, `fetch`, `assert`
- Returning values from choices

## Further Reading

- [Choices](https://docs.daml.com/daml/intro/4_Transformations.html)
