#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Checking formatting (cargo fmt --check)..."
cargo fmt --check

echo "==> Checking clippy (cargo clippy --all-targets -- -D warnings)..."
cargo clippy --all-targets -- -D warnings

echo "==> Running tests (cargo test)..."
cargo test

echo "==> Verifying all src/ paths referenced in docs/ARCHITECTURE.md and docs/architecture.html exist..."
if [[ ! -f docs/architecture.html ]]; then
    echo "ERROR: docs/architecture.html is missing!" >&2
    exit 1
fi
missing=0
while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    [[ "$path" == *"*"* ]] && continue
    if [[ ! -e "$path" ]]; then
        echo "ERROR: Path mentioned in architecture docs does not exist: $path" >&2
        missing=1
    else
        echo "  OK: $path"
    fi
done < <(grep -hoE 'src/[A-Za-z0-9_./-]+' docs/ARCHITECTURE.md docs/architecture.html | sed 's/[.,:;)]*$//' | sort -u)

if [[ "$missing" -ne 0 ]]; then
    exit 1
fi

echo "==> All checks passed!"
