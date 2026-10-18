#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="$(grep -E '^version\s*=' "$REPO_ROOT/Cargo.toml" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')"
ARCH="$(uname -m)"
WORK_DIR="${FLATPAK_WORK_DIR:-$REPO_ROOT/target/flatpak-work}"
OUT_DIR="${FLATPAK_OUT_DIR:-$REPO_ROOT/target/release}"
BUNDLE_NAME="duckydeck-studio-v${VERSION}-${ARCH}.flatpak"

echo "==> Building DuckyDeck Studio v${VERSION} Flatpak (${ARCH})..."
rm -rf "$WORK_DIR/build-dir" "$WORK_DIR/repo"
mkdir -p "$WORK_DIR" "$OUT_DIR"

flatpak build-init \
  --sdk-extension=org.freedesktop.Sdk.Extension.rust-stable \
  --sdk-extension=org.freedesktop.Sdk.Extension.llvm18 \
  "$WORK_DIR/build-dir" \
  io.github.duckydeck.Studio \
  org.kde.Sdk \
  org.kde.Platform \
  6.8

CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"

flatpak build \
  --share=network \
  --filesystem="$REPO_ROOT":ro \
  --filesystem="$CARGO_HOME_DIR":ro \
  --filesystem="$WORK_DIR":rw \
  --env=PATH=/usr/lib/sdk/rust-stable/bin:/usr/lib/sdk/llvm18/bin:/usr/bin:/bin \
  --env=CARGO_HOME="$CARGO_HOME_DIR" \
  --env=CARGO_TARGET_DIR="$WORK_DIR/target" \
  --env=LIBCLANG_PATH=/usr/lib/sdk/llvm18/lib \
  --env=QMAKE=/usr/bin/qmake6 \
  --env=QT_VERSION_MAJOR=6 \
  "$WORK_DIR/build-dir" \
  sh -c "
    set -euo pipefail
    cargo build --release --manifest-path '$REPO_ROOT/Cargo.toml'
    install -Dm755 \"\$CARGO_TARGET_DIR/release/duckydeck-studio\" -t /app/bin/
    install -Dm644 '$REPO_ROOT/data/io.github.duckydeck.Studio.desktop' -t /app/share/applications/
    install -Dm644 '$REPO_ROOT/data/io.github.duckydeck.Studio.metainfo.xml' -t /app/share/metainfo/
    install -Dm644 '$REPO_ROOT/data/icons/io.github.duckydeck.Studio.svg' -t /app/share/icons/hicolor/scalable/apps/
    install -Dm644 '$REPO_ROOT/data/udev/70-duckypad.rules' -t /app/share/udev/rules.d/
    install -Dm755 '$REPO_ROOT/data/udev/install-udev-rules.sh' -t /app/bin/
    install -Dm644 '$REPO_ROOT/data/syntax/duckyscript3.xml' -t /app/share/org.kde.syntax-highlighting/syntax/
    install -Dm644 '$REPO_ROOT/docs/user-guide.html' -t /app/share/duckydeck-studio/docs/
    install -Dm644 '$REPO_ROOT/firmware/releases/DPP_FW_1.0.0_ad2acc81.bin' -t /app/share/duckydeck-studio/firmware/releases/
  "

flatpak build-finish "$WORK_DIR/build-dir" \
  --command=duckydeck-studio \
  --share=ipc \
  --socket=wayland \
  --socket=fallback-x11 \
  --device=dri \
  --device=all \
  --share=network \
  --filesystem=xdg-run/gvfs:ro \
  --filesystem=/run/media:rw \
  --filesystem=/media:rw \
  --filesystem=xdg-config/kglobalshortcutsrc:ro \
  --filesystem=xdg-data/kxmlgui5:ro \
  --filesystem=xdg-run/duckydeck-kwin6-hook.js:create \
  --talk-name=org.kde.KWin \
  --talk-name=org.kde.StatusNotifierWatcher \
  --talk-name=org.freedesktop.Notifications \
  --talk-name=org.freedesktop.secrets \
  --talk-name=org.freedesktop.Flatpak

flatpak build-export "$WORK_DIR/repo" "$WORK_DIR/build-dir" stable
flatpak build-bundle "$WORK_DIR/repo" "$OUT_DIR/$BUNDLE_NAME" io.github.duckydeck.Studio stable --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
(cd "$OUT_DIR" && sha256sum "$BUNDLE_NAME" > "${BUNDLE_NAME}.sha256" && ln -sf "$BUNDLE_NAME" duckydeck-studio.flatpak)

echo "==> Built Flatpak bundle: $OUT_DIR/$BUNDLE_NAME"
