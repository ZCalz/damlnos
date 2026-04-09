# Quiz 02 — IOU Contract

> **Coming soon** — quiz not yet written.

## When to attempt

Complete modules **09 Templates** through **13 Constraints** before attempting this quiz.

## Scenario

Design a full IOU (I-Owe-You) contract system:

- An `Iou` template with `issuer`, `owner`, `amount`, and `currency`
- A `Transfer` choice that proposes transfer to a new owner via a separate `IouTransferProposal` template
- An `Accept` choice on `IouTransferProposal` that completes the transfer
- A `Settle` choice that archives the IOU (only the owner can settle with the issuer)
- `ensure` constraints preventing negative amounts or self-transfer
- A multi-step Script test: issue → propose transfer → accept → settle

No hints. Design the templates, choices, and constraints yourself.
