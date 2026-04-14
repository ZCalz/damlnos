# Quiz 03 — Token Swap with Interface

## When to attempt

Complete all modules **00** through **18 Exceptions** before attempting this quiz.

## Rules

No hints. No bug comments. Just a specification and a set of assertions to satisfy.

## Specification

Open `Quiz03.daml`. The skeletons are there but contain bugs. Fix them so that:

**`Fungible` interface**
- Methods: `getOwner : Party`, `getAmount : Decimal`
- `Transfer` choice: controlled by `getOwner this`, delegates to `transferImpl`

**`GoldToken`** and **`SilverToken`** — fields: `issuer : Party`, `owner : Party`, `amount : Decimal`
- Each must implement `Fungible` correctly
- `Transfer` produces a new contract with `owner = newOwner` and returns it as `ContractId Fungible`

**`SwapProposal`** — Alice offers her gold for Bob's silver
- `AcceptSwap` choice: Bob accepts; atomically transfers gold to Bob and silver to Alice
- `SwapProposal` must only reference `ContractId Fungible` — not `GoldToken` or `SilverToken` directly

**Script assertions** in `quiz03`:
1. Bank mints 10 gold for Alice and 50 silver for Bob
2. Alice creates a `SwapProposal`; Bob calls `AcceptSwap`
3. Bob now owns the gold (amount 10.0)
4. Alice now owns the silver (amount 50.0)
5. A second swap using already-consumed contract ids must fail
