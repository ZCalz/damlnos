# Quiz 03 — Token Swap with Interface

> **Coming soon** — quiz not yet written.

## When to attempt

Complete all modules **00** through **19** before attempting this quiz.

## Scenario

Build a decentralised token swap:

- A `Fungible` interface with `getOwner : Party`, `getAmount : Decimal`, and a `Transfer` choice
- Two implementing templates: `GoldToken` and `SilverToken`
- A `SwapProposal` template where Alice offers N gold tokens for M silver tokens from Bob
- Bob accepts by exercising `AcceptSwap`, which atomically transfers both tokens
- The swap must work via the `Fungible` interface — `SwapProposal` should not import `GoldToken` or `SilverToken` directly
- Script tests covering the happy path and rejection

This quiz tests interfaces, propose/accept, and multi-party authorization together.
