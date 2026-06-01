#!/usr/bin/env bash
set -euo pipefail

# Verify reference solutions compile without including the intentionally broken
# learner exercises under daml/.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

if ! command -v dpm >/dev/null 2>&1; then
  echo "error: dpm not found on PATH" >&2
  exit 1
fi

if [[ ! -d "$ROOT/Solutions/Exercises" ]]; then
  echo "error: Solutions/Exercises not found" >&2
  exit 1
fi

cat > "$WORK/daml.yaml" <<'YAML'
sdk-version: 3.4.11
name: damlings-solutions-check
version: 0.0.1
dependencies:
  - daml-prim
  - daml-stdlib
  - daml-script
source: .
build-options:
  - -Wno-upgrade-interfaces
  - -Wno-upgrade-exceptions
YAML

cp -R "$ROOT/Solutions" "$WORK/Solutions"

cd "$WORK"

echo "Running dpm build for Solutions/ only…"
dpm build

echo ""
echo "Solutions compile successfully."
