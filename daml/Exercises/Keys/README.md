# 14 · Keys

> **Coming soon** — exercises not yet written.

## Preview

Contract keys give contracts a stable, business-meaningful identity beyond their `ContractId`.

```daml
template Account
  with
    owner      : Party
    accountNum : Text
  where
    signatory owner
    key (owner, accountNum) : (Party, Text)
    maintainer key._1

-- Look up by key instead of ContractId:
-- fetchByKey @Account (alice, "ACC-001")
```

### What you will learn

- Declaring `key` and `maintainer`
- `fetchByKey` — fetch a contract by its key
- `lookupByKey` — returns `Optional (ContractId t, t)`
- `exerciseByKey` — exercise a choice without holding the `ContractId`
- Key uniqueness: only one active contract per key

## Further Reading

- [Contract keys](https://docs.daml.com/daml/reference/contract-keys.html)
