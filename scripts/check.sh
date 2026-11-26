#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> 1. Checking Rust formatting (cargo fmt --all -- --check)..."
cargo fmt --all -- --check

echo "==> 2. Checking Rust clippy (cargo clippy --workspace --all-targets -- -D warnings)..."
cargo clippy --workspace --all-targets -- -D warnings

echo "==> 3. Running workspace tests (cargo test --workspace)..."
cargo test --workspace

echo "==> 4. Linting QML files with Qt 6 qmllint..."
QMLLINT_BIN=""
for cand in /usr/lib/qt6/bin/qmllint "$(command -v qmllint6 2>/dev/null || true)" "$(command -v qmllint 2>/dev/null || true)"; do
    [[ -z "$cand" || ! -x "$cand" ]] && continue
    if "$cand" --version 2>/dev/null | grep -qE '^qmllint 6\.'; then
        QMLLINT_BIN="$cand"
        break
    fi
done
if [[ -n "$QMLLINT_BIN" ]]; then
    QML_IMPORT_DIR=""
    for qm in "$(command -v qmake6 2>/dev/null || true)" /usr/lib/qt6/bin/qmake "$(command -v qmake 2>/dev/null || true)"; do
        [[ -z "$qm" || ! -x "$qm" ]] && continue
        QML_IMPORT_DIR="$("$qm" -query QT_INSTALL_QML 2>/dev/null || true)"
        [[ -n "$QML_IMPORT_DIR" && -d "$QML_IMPORT_DIR" ]] && break || QML_IMPORT_DIR=""
    done
    if [[ -z "$QML_IMPORT_DIR" ]]; then
        for d in /usr/lib/qt6/qml /usr/lib/x86_64-linux-gnu/qt6/qml; do
            [[ -d "$d" ]] && QML_IMPORT_DIR="$d" && break
        done
    fi
    if [[ -n "$QML_IMPORT_DIR" ]]; then
        "$QMLLINT_BIN" -I "$QML_IMPORT_DIR" -I src/qml src/qml/*.qml
    else
        "$QMLLINT_BIN" -I src/qml src/qml/*.qml
    fi
else
    echo "  SKIP: Qt 6 qmllint not installed"
fi

echo "==> 5. Checking C++ headers with -Wall -Wextra -Werror..."
if command -v g++ >/dev/null 2>&1 && pkg-config --exists Qt6Core Qt6Gui Qt6Widgets Qt6Quick Qt6Qml Qt6Svg Qt6Network 2>/dev/null; then
    QT_CFLAGS="$(pkg-config --cflags Qt6Core Qt6Gui Qt6Widgets Qt6Quick Qt6Qml Qt6Svg Qt6Network)"
    KF6_CFLAGS="$(pkg-config --cflags KF6ColorScheme KF6ConfigCore KF6IconThemes 2>/dev/null || true)"
    for base in "${KF6_INCLUDE_DIR:-}" /usr/include/KF6 /usr/local/include/KF6 /usr/include/x86_64-linux-gnu/KF6; do
        [[ -z "$base" || ! -d "$base" ]] && continue
        for mod in KColorScheme KConfig KConfigCore KIconThemes; do
            if [[ -d "$base/$mod" && "$KF6_CFLAGS" != *"-I$base/$mod"* ]]; then
                KF6_CFLAGS="$KF6_CFLAGS -I$base/$mod"
            fi
        done
    done
    if [[ -n "$KF6_CFLAGS" ]]; then
        # shellcheck disable=SC2086
        echo '#include "app_helper.h"' | g++ -std=c++17 -fsyntax-only -Wall -Wextra -Werror -Isrc/cpp $QT_CFLAGS $KF6_CFLAGS -x c++ -
    else
        echo "  SKIP app_helper.h: KF6 headers not found"
    fi
    # shellcheck disable=SC2086
    echo '#include "kirigami_compat.h"' | g++ -std=c++17 -fsyntax-only -Wall -Wextra -Werror -Isrc/cpp $QT_CFLAGS -x c++ -
else
    echo "  SKIP: g++ or Qt6 pkg-config development headers not installed"
fi

echo "==> 6. Checking shell scripts..."
for shf in scripts/*.sh data/udev/*.sh; do
    bash -n "$shf"
done
if command -v shellcheck >/dev/null 2>&1; then
    shellcheck -S warning scripts/*.sh data/udev/*.sh
else
    echo "  NOTE: shellcheck not installed (verified bash -n on all scripts)"
fi

echo "==> 7. Checking embedded JavaScript in data/esp32_ha_web_ui.html..."
if command -v node >/dev/null 2>&1; then
    python3 - <<'PY'
import pathlib, re, subprocess, tempfile
html = pathlib.Path("data/esp32_ha_web_ui.html").read_text(encoding="utf-8")
scripts = re.findall(r"<script[^>]*>(.*?)</script>", html, re.S)
assert scripts, "No <script> block found in data/esp32_ha_web_ui.html"
for i, body in enumerate(scripts):
    with tempfile.NamedTemporaryFile("w", suffix=".js", delete=True) as tf:
        tf.write(body)
        tf.flush()
        proc = subprocess.run(["node", "--check", tf.name], capture_output=True, text=True)
        if proc.returncode != 0:
            raise SystemExit(f"node --check failed for <script>[{i}]:\n{proc.stderr}")
print(f"  OK: node --check passed for {len(scripts)} embedded script block(s)")
PY
else
    echo "  SKIP: node not installed"
fi

echo "==> 8. Verifying all src/ and crates/ paths referenced in docs/ARCHITECTURE.md and docs/architecture.html exist..."
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
done < <(grep -hoE '(crates/[A-Za-z0-9_./-]+|src/[A-Za-z0-9_./-]+)' docs/ARCHITECTURE.md docs/architecture.html | sed 's/[.,:;)]*$//' | sort -u)

if [[ "$missing" -ne 0 ]]; then
    exit 1
fi

echo "==> All checks passed!"
