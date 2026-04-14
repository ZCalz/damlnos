# Quiz 02 — IOU Contract

## When to attempt

Complete modules **09 Templates** through **13 Constraints** before attempting this quiz.

## Rules

No hints. No bug comments. Just a specification and a set of assertions to satisfy.

## Specification

Open `Quiz02.daml`. The template skeletons are there but contain bugs. Fix them to satisfy the spec:

**`Iou`** — fields: `issuer : Party`, `owner : Party`, `amount : Decimal`, `currency : Text`
- `signatory issuer`, `observer owner`
- `ensure` rejects non-positive amounts
- `Transfer` choice: owner proposes a transfer to a `newOwner`; creates an `IouTransferProposal`
- `Settle` choice: owner archives the IOU

**`IouTransferProposal`** — fields: `iou : Iou`, `newOwner : Party`
- `Accept` choice: `newOwner` accepts; creates a new `Iou` with ownership transferred

**Script assertions** in `quiz02`:
1. Bank issues a 100 USD IOU to Alice
2. Creating a negative-amount IOU must fail
3. Alice transfers to Bob via `Transfer` → `Accept`
4. The transferred IOU's owner is Bob, issuer and amount unchanged
5. Bob settles the IOU
6. The original IOU is gone after the transfer
