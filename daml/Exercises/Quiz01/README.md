# Quiz 01 — Bank Account

> **Coming soon** — quiz not yet written.

## When to attempt

Complete modules **00 Intro** through **08 Functional 101** before attempting this quiz.

## Scenario

You will build a simple bank account system from scratch:

- A `BankAccount` record holding owner, account number, and balance
- Functions to deposit, withdraw, and check balance
- A variant `TransactionType` with `Deposit`, `Withdrawal`, and `Transfer`
- A `Transaction` record pairing an amount with a `TransactionType`
- A function `applyTransactions : BankAccount -> [Transaction] -> BankAccount`
- Script tests that verify the resulting balances

No hints. No TODOs. Just a specification and a set of assertions to satisfy.
