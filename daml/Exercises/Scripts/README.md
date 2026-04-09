# 11 · Scripts

> **Coming soon** — exercises not yet written.

## Preview

`Daml.Script` is DAML's simulation and testing framework. These exercises go deeper into the Script API.

```daml
test : Script ()
test = script do
  alice <- allocateParty "Alice"
  cid   <- submit alice do createCmd MyTemplate with owner = alice
  submit alice do exerciseCmd cid MyChoice with arg = 42
  submitMustFail alice do exerciseCmd cid MyChoice with arg = -1
```

### What you will learn

- `submit` / `submitMustFail` — test both success and expected failure paths
- `submitMulti` — submit on behalf of multiple parties simultaneously
- `query` / `queryContractId` — inspect ledger state
- `passTime` — advance the ledger clock
- Structuring multi-step test scenarios

## Further Reading

- [Daml.Script reference](https://docs.daml.com/daml-script/index.html)
