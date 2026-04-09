# 15 · Propose / Accept

> **Coming soon** — exercises not yet written.

## Preview

The propose/accept pattern is the standard DAML idiom for getting two parties to agree on a contract without a trusted intermediary.

```daml
-- Step 1: Alice creates a proposal
template TransferProposal
  with
    iou      : Iou
    newOwner : Party
  where
    signatory iou.owner
    observer  newOwner

    choice Accept : ContractId Iou
      controller newOwner
      do create iou with owner = newOwner

    choice Reject : ()
      controller newOwner
      do pure ()
```

### What you will learn

- Why multi-signatory contracts need a workflow
- Modelling proposals as separate contracts
- `Accept` and `Reject` choices controlled by the counterparty
- Chaining ContractIds across workflow steps

## Further Reading

- [Propose/accept pattern](https://docs.daml.com/daml/intro/6_Parties.html#the-propose-accept-pattern)
