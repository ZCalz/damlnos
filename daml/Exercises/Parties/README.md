# 12 · Parties

> **Coming soon** — exercises not yet written.

## Preview

Authorization is the heart of DAML's security model. A contract action is only valid if all required parties have authorised it.

```daml
template Iou
  with
    issuer : Party
    owner  : Party
    amount : Decimal
  where
    -- Both parties must sign — neither can create this unilaterally.
    signatory issuer, owner
```

### What you will learn

- Multi-signatory contracts — all signatories must agree
- `submitMulti` — acting as multiple parties at once in tests
- Divulgence — what observers can and cannot do
- The `maintainer` keyword for contract keys
- Common authorization errors and how to fix them

## Further Reading

- [Authorization in DAML](https://docs.daml.com/daml/intro/6_Parties.html)
