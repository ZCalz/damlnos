# Contributing to Damlnos

## Adding a new exercise

1. **Pick the module** — find the right directory under `daml/Exercises/` or create a new one.
2. **Write the DAML file** — follow the exercise template below.
3. **Register it** — add an entry to `EXERCISES` in `src/exercises.rs`.
4. **Write the solution** — mirror the file under `solutions/Exercises/`.
5. **Test it** — run `daml test --files daml/Exercises/<Module>/<File>.daml` to confirm the broken version fails and the solution passes.

## Exercise file template

```daml
-- Copyright (c) 2026 Damlnos Contributors
-- SPDX-License-Identifier: Apache-2.0

-- Exercise: <Section> N — <Title>
--
-- CONCEPT ─────────────────────────────────────────────────────────────────────
-- <2-4 sentences explaining the concept. Keep it beginner-friendly.>
--
-- DOCS ────────────────────────────────────────────────────────────────────────
-- <one or two links to the official DAML docs>
--
-- HINT ────────────────────────────────────────────────────────────────────────
-- <one-line hint — also copied into src/exercises.rs>
-- ─────────────────────────────────────────────────────────────────────────────

module Exercises.<Module>.Ex<N> where

import Daml.Script

<scriptName> : Script ()
<scriptName> = script do
  -- <broken code or TODOs>
```

## Exercise design rules

- **One concept per exercise.** Don't mix template syntax and pattern matching in the same file.
- **Broken state must fail.** The file as committed must either fail to compile or fail its assertions.
- **Hint is one line.** It goes in `src/exercises.rs` and is printed by `damlnos hint`.
- **No spoilers in hints.** Point at the right DAML concept; don't give the exact fix.
- **Assert the right things.** The test should only pass once the user actually understands the exercise — not just because they deleted the assert.

## Adding a new exercise module (section)

1. Create `daml/Exercises/<NewModule>/README.md` explaining the concept and listing planned exercises.
2. Write the `.daml` exercise files.
3. Add a matching `solutions/Exercises/<NewModule>/` directory.
4. Update the table in the root `README.md`.

## Running all exercises

```bash
dpm test    # runs every script in the project (expect failures — exercises are broken by design)
```

CI validates reference solutions without touching the committed exercise files. The check copies the project to a temporary directory, overlays `solutions/Exercises/` there, and runs `dpm test` on each non-quiz file. Run the same check locally:

```bash
bash scripts/verify-solutions.sh
```

Your local `daml/Exercises/` files stay intentionally broken for learners.

## Building the CLI runner

```bash
cargo build --release
./target/release/damlnos list
```
