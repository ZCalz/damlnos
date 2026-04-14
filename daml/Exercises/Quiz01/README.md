# Quiz 01 — Bank Account

## When to attempt

Complete modules **00 Intro** through **08 Functional 101** before attempting this quiz.

## Rules

No hints. No bug comments. Just a specification and a set of assertions to satisfy.

## Specification

Open `Quiz01.daml`. The type definitions are already there:

- `BankAccount` — record with `owner : Party`, `acctNum : Text`, `balance : Decimal`
- `TransactionType` — variant: `Deposit | Withdrawal | Transfer { recipient : Text }`
- `Transaction` — record with `amount : Decimal` and `txType : TransactionType`

The function stubs compile but return wrong results. Implement them correctly:

| Function | Type | Description |
|---|---|---|
| `deposit` | `Decimal -> BankAccount -> BankAccount` | Add to balance |
| `withdraw` | `Decimal -> BankAccount -> BankAccount` | Subtract from balance |
| `getBalance` | `BankAccount -> Decimal` | Return current balance |
| `applyTransactions` | `BankAccount -> [Transaction] -> BankAccount` | Apply each transaction in order; `Transfer` subtracts like a withdrawal |

The `quiz01` script asserts the expected balances. All four assertions must pass.
