# Damlnos

> Learn DAML by fixing broken code.

```
               ___________     ___________
              /\           \  /\           \
             /  \___________\   \___________\
            /   /           /   /           /
           /   /   •   •   /   /   •   •   /
          /   /     •     /   /           /
         /\  /   •   •   /\  /     •     /
        /  \/___________/  \/___________/
   __| | __/  ___ ___  | | /_    ___   /__ 
  / _` |/ _` | '_ ` _ \| | '_ \ / _ \ / __|
 | (_| | (_| | | | | | | | | | | (_) |\__ \
  \__,_|\__,_|_| |_| |_|_|_| |_|\___/ |___/
     \/___________/  \/___________/

 ```

Damlnos is a collection of small, hands-on exercises that teach the [DAML](https://www.digitalasset.com/developers) smart contract language from the ground up. Inspired by [Rustlings](https://github.com/rust-lang/rustlings), each exercise contains intentional errors or blanks that you fix to make the tests pass.

No prior DAML knowledge required. The exercises start from pure functional programming basics (DAML is built on Haskell) and build up to full contract workflows, multi-party authorization, and interface design.

---

## Prerequisites

### DAML

[DAML](https://www.digitalasset.com/developers) is a smart contract language for building multi-party applications on distributed ledgers. You define templates (contracts), choices (actions on those contracts), and scripts (tests) in DAML; the compiler packages them into DAR archives that run on [Canton](https://www.canton.network/).

Damlnos compiles and runs exercises using the DAML SDK, installed and managed by **dpm** (Daml Package Manager). Follow the official installation guide:

**[Install dpm →](https://docs.canton.network/sdks-tools/cli-tools/dpm#installation)**

Once `dpm` is on your `PATH`, install the SDK version this project expects (see `daml.yaml`):

```bash
dpm install package
```

Verify the installation:

```bash
dpm --version
dpm version --active
```

### Other requirements

- [Rust](https://rustup.rs/) — to build the `damlnos` CLI watcher
- A text editor with DAML support (VS Code + Daml Studio recommended; launch with `dpm studio`)

---

## Quick Start

```bash
git clone https://github.com/your-org/damlnos
cd damlnos

# Build the CLI runner
cargo build --release

# Start the watcher — it will guide you through exercises in order
./target/release/damlnos watch
```

The watcher compiles and runs the current exercise's tests after every file save. When all tests pass, it advances to the next exercise automatically.

### Shorter command

If you prefer `damlnos watch` over `./target/release/damlnos watch`, install the binary once:

```bash
cargo install --path .
damlnos watch
```

This places `damlnos` in `~/.cargo/bin`. If that directory is on your `PATH` (it usually is after installing Rust via rustup), you can use `damlnos` from any directory.

Without installing, you can also run:

```bash
cargo run --release -- watch
```

### Other commands

```bash
damlnos list          # Show all exercises and your progress
damlnos hint          # Print a hint for the current exercise
damlnos run <name>    # Run a specific exercise (e.g. damlnos run ex01_types1)
damlnos reset <name>  # Reset an exercise back to its original broken state
```

---

## How Exercises Work

Each exercise is a DAML module file under `daml/exercises/`. Every file has:

- A brief comment explaining the concept
- A link to the relevant DAML docs
- Intentional errors, missing keywords, or `-- TODO` blanks for you to fill in
- One or more `Script` tests at the bottom that must pass

Example structure of an exercise file:

```
daml/exercises/02_records/
  README.md        ← concept explanation and links
  ex02_records1.daml  ← exercise 1: define a record type
  ex02_records2.daml  ← exercise 2: access and update fields
  ex02_records3.daml  ← exercise 3: nested records
```

Fix the `-- TODO` items in each `.daml` file until `damlnos watch` shows all tests green.

---

## Exercise Track

### Part 1 — Functional Programming Foundations

DAML's expression language is based on Haskell. These exercises cover the building blocks before any contract-specific concepts.

| # | Module | Exercises | What You Learn |
|---|--------|-----------|----------------|
| 00 | `00_intro` | 2 | What DAML is, ledger model overview, running your first Script |
| 01 | `01_types` | 4 | Primitive types: `Text`, `Int`, `Decimal`, `Bool`, `Party`, `Date`, `Time` |
| 02 | `02_records` | 4 | `data` records, field access (dot syntax), record update syntax |
| 03 | `03_functions` | 5 | Defining functions, `let`/`in`, `if`/`then`/`else`, guards, `where` |
| 04 | `04_pattern_matching` | 4 | `case` expressions, pattern matching on constructors and tuples |
| 05 | `05_lists` | 5 | List literals, `map`, `filter`, `foldl`, list comprehensions |
| 06 | `06_optional` | 3 | `Optional`, `Some`/`None`, `fromSome`, `mapOptional` |
| 07 | `07_variants` | 4 | Sum types (`data T = A | B with ...`), `deriving (Eq, Show)` |
| 08 | `08_functional_101` | 3 | Higher-order functions, function composition, `$` operator |

### Part 2 — DAML Contract Model

| # | Module | Exercises | What You Learn |
|---|--------|-----------|----------------|
| 09 | `09_templates` | 4 | `template`, `with`, `where`, `signatory`, `observer` |
| 10 | `10_choices` | 5 | Consuming vs non-consuming choices, `controller`, `do` blocks |
| 11 | `11_scripts` | 4 | `Daml.Script`, `allocateParty`, `submit`, `submitMustFail` |
| 12 | `12_parties` | 4 | Multi-signatory contracts, `submitMulti`, authorization rules |
| 13 | `13_constraints` | 3 | `ensure`, `assertMsg`, precondition failures |
| 14 | `14_keys` | 4 | Tracking `ContractId`s, `queryContractId`, exercising known contracts |
| 15 | `15_propose_accept` | 4 | Proposal/accept pattern, two-step workflows, `ContractId` chaining |
| 16 | `16_compose` | 4 | Multi-contract workflows, delegation, role templates |
| 17 | `17_interfaces` | 5 | `interface`, `requires`, `implements`, `toInterface`, dispatch |
| 18 | `18_exceptions` | 3 | `try`/`catch`, `throw`, custom exception types |
| 19 | `19_upgrades` | 3 | Contract upgrade patterns, data migration, backwards compatibility |

### Quizzes

After every few modules there is a quiz exercise — a larger scenario with no hints that combines concepts from the preceding sections.

| Quiz | After Module | Scenario |
|------|-------------|----------|
| `quiz_01` | 08 | Build a simple bank account using records, variants, and functions |
| `quiz_02` | 13 | Write an IOU contract with transfer and settle choices |
| `quiz_03` | 19 | Design a token swap workflow with proposal/accept and interface dispatch |

---

## Project Layout

```
      _______________________________
     / \              \               \
     \  \   •          \           •   \
      \  \       •      \      •        \
    __| | __    ___ ___ | |_ __   ___   ___
   / _` |/ _` | '_ ` _ \| | '_ \ / _ \ / __|
  | (_| | (_| | | | | | | | | | | (_) |\__ \
   \__,_|\__,_|_| |_| |_|_|_| |_|\___/ |___/

Damlnos/
├── README.md                  ← you are here
├── daml.yaml                  ← single DAML project config (SDK 3.4.x)
├── Cargo.toml                 ← CLI runner (Rust)
├── src/
│   └── main.rs                ← damlnos CLI: watch, hint, list, run, reset
├── daml/
│   └── exercises/
│       ├── 00_intro/
│       │   ├── README.md
│       │   ├── ex00_intro1.daml
│       │   └── ex00_intro2.daml
│       ├── 01_types/
│       │   ├── README.md
│       │   ├── ex01_types1.daml
│       │   ├── ex01_types2.daml
│       │   ├── ex01_types3.daml
│       │   └── ex01_types4.daml
│       ├── ...
│       └── quiz_01/
│           ├── README.md
│           └── quiz_01.daml
├── solutions/
│   └── exercises/             ← completed reference solutions (same layout)
└── .damlnos/
    └── state.json             ← tracks your current exercise and progress
```

---

## How the CLI Runner Works

The `damlnos` binary is a small Rust program that:

1. Reads `.damlnos/state.json` to find the current exercise
2. Spawns `daml test --files daml/exercises/<current>/<file>.daml` as a subprocess
3. Watches the exercise file for changes using `notify` (cross-platform file watcher)
4. On each save, re-runs the test and parses the output:
   - **All scripts pass** → prints success, advances state, shows the next exercise
   - **Compile error or test failure** → prints the error with a friendly header
5. `damlnos hint` reads the `-- HINT:` comment block at the top of each exercise

No separate ledger process is needed — DAML Script tests run entirely in the DAML interpreter (`daml test`).

---

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add new exercises, fix hints, or improve the runner.

---

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the full text and [NOTICE](NOTICE) for copyright attribution.
