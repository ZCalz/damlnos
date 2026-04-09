# 09 · Templates

> **Coming soon** — exercises not yet written.

## Preview

A `template` is the core DAML construct — a blueprint for a contract that can live on the ledger.

```daml
template Token
  with
    owner : Party
  where
    signatory owner
```

### What you will learn

- `template` / `with` / `where` structure
- `signatory` — who must authorise creation and archival
- `observer` — who can see the contract without signing it
- `ensure` — a precondition on contract data
- Creating contracts with `createCmd` in Scripts
- Archiving contracts with `archiveCmd`

## Further Reading

- [DAML templates](https://docs.daml.com/daml/intro/1_Token.html)
- [Signatories and observers](https://docs.daml.com/daml/intro/4_Transformations.html)
