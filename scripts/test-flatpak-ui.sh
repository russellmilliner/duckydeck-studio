#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if ! flatpak info io.github.duckydeck.Studio >/dev/null 2>&1; then
    echo "Error: io.github.duckydeck.Studio Flatpak is not installed. Run 'bash scripts/build-flatpak.sh' and 'flatpak install --user -y --reinstall target/release/duckydeck-studio.flatpak' first." >&2
    exit 1
fi

echo "==> Running Flatpak QML UI Overflow, Contrast & Behavioral Test Suite..."
cargo test --bin duckydeck-studio ui_test_suite::test_flatpak_qml_ui_overflow_and_contrast_suite -- --ignored --nocapture --exact
echo "==> Flatpak UI test suite passed!"
