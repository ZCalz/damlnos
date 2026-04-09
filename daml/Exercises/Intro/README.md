# 00 · Intro

Welcome to Damlers! These two exercises get you up and running.

## What is DAML?

DAML is a smart contract language built on top of Haskell. A DAML program defines *contract templates* — blueprints for agreements between parties on a shared ledger. Contracts can be created, queried, and transformed, but only by the parties that have been authorized to do so.

## What is a Script?

`Daml.Script` is DAML's built-in testing and simulation framework. A `Script` lets you simulate ledger operations — allocating parties, creating contracts, exercising choices — without needing a running ledger. All exercises in Damlers use `Daml.Script`.

## Exercises

| File | Concept |
|------|---------|
| `Ex1.daml` | Running your first script; `debug` and `assert` |
| `Ex2.daml` | `assertMsg`, arithmetic, equality |

## Further Reading

- [DAML Introduction](https://docs.daml.com/daml/intro/0_Intro.html)
- [Daml.Script reference](https://docs.daml.com/daml-script/index.html)
