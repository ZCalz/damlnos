# 14 · Contract IDs

> **Coming soon** — exercises not yet written.

## Preview

This SDK target does not support contract keys in this project, so these exercises
focus on the most common workflow: keep the `ContractId` returned by `createCmd`
and use it later with `queryContractId` or `exerciseCmd`.

```daml
template Account
  with
    owner      : Party
    accountNum : Text
  where
    signatory owner

cid <- submit alice do
  createCmd Account with owner = alice, accountNum = "ACC-001"

res <- queryContractId alice cid
```

### What you will learn

- Capturing the `ContractId` returned by `createCmd`
- `queryContractId` — fetch a contract by known id
- `exerciseCmd` — exercise a choice on a known contract
- Chaining workflows through newly-created contract ids

## Further Reading

- [Templates and choices](https://docs.daml.com/daml/intro/3_Templates.html)
