#!/usr/bin/env bash
set -euo pipefail

# Verify reference solutions compile and pass without modifying the committed
# exercise files (those stay intentionally broken for learners).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

if ! command -v dpm >/dev/null 2>&1; then
  echo "error: dpm not found on PATH" >&2
  exit 1
fi

cp "$ROOT/daml.yaml" "$WORK/"
cp -R "$ROOT/daml" "$WORK/daml"

echo "Overlaying solutions in a temporary project copy (daml/Exercises/ in the repo is unchanged)…"

while IFS= read -r -d '' solution; do
  rel="${solution#"$ROOT"/solutions/Exercises/}"
  target="$WORK/daml/Exercises/${rel}"
  exercise="$ROOT/daml/Exercises/${rel}"

  if [[ ! -f "$exercise" ]]; then
    echo "error: no exercise file for solution: $solution (expected $exercise)" >&2
    exit 1
  fi

  sed \
    -e 's/^module Solutions\.Exercises\./module Exercises./' \
    -e '/^-- SOLUTION:/d' \
    "$solution" > "$target"

  echo "  $rel"
done < <(find "$ROOT/solutions/Exercises" -name '*.daml' -print0)

cd "$WORK"

echo ""
echo "Running dpm test for non-quiz exercises…"

failed=0
while IFS= read -r -d '' file; do
  echo "--- $file"
  if ! dpm test --files "$file"; then
    failed=1
  fi
done < <(find daml/Exercises -name '*.daml' ! -path '*/Quiz*/*' -print0)

if [[ "$failed" -ne 0 ]]; then
  echo ""
  echo "error: one or more solution tests failed" >&2
  exit 1
fi

echo ""
echo "All solution tests passed."
