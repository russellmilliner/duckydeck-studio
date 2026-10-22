#!/usr/bin/env bash
# Cross-compiles DuckyDeck Studio for Windows x64 on Linux and packages:
#   target/release/DuckyDeck-Studio-Setup-v<ver>-x86_64.exe          (Inno Setup installer)
#   target/release/duckydeck-studio-v<ver>-windows-x86_64-portable.zip (portable zip)
#   target/release/*.sha256
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET="x86_64-pc-windows-gnullvm"
VERSION="$(grep -E '^version\s*=' "$REPO_ROOT/Cargo.toml" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')"

TOOLCHAIN_ENV="${LLU_WIN_TOOLCHAIN_ENV:-$HOME/.local/share/llu-wintoolchain/env.sh}"
if [[ -f "$TOOLCHAIN_ENV" ]]; then
    # shellcheck source=/dev/null
    . "$TOOLCHAIN_ENV"
fi

QT_BASE_DIR="${DUCKYDECK_WIN_QT_BASE:-$HOME/.local/share/llu-wintoolchain/qt6}"
QT_VER="6.8.3"
export DUCKYDECK_WIN_QT_ROOT="$QT_BASE_DIR/$QT_VER"
WIN_QT="$DUCKYDECK_WIN_QT_ROOT/llvm-mingw_64"
export DUCKYDECK_WIN_QT_DIR="$WIN_QT"
HOST_QT="$DUCKYDECK_WIN_QT_ROOT/gcc_64"

if [[ ! -d "$WIN_QT/lib" ]]; then
    echo "==> Installing Windows Qt $QT_VER (win64_llvm_mingw) via aqtinstall..."
    mkdir -p "$QT_BASE_DIR"
    uvx --from aqtinstall aqt install-qt windows desktop "$QT_VER" win64_llvm_mingw -O "$QT_BASE_DIR" -m qtimageformats qtshadertools
fi
if [[ ! -d "$HOST_QT/libexec" ]]; then
    echo "==> Installing Linux host Qt $QT_VER tools via aqtinstall..."
    mkdir -p "$QT_BASE_DIR"
    uvx --from aqtinstall aqt install-qt linux desktop "$QT_VER" linux_gcc_64 -O "$QT_BASE_DIR" -m qtshadertools
fi

if [[ -f "$HOST_QT/libexec/rcc" && ! -f "$HOST_QT/libexec/rcc.real" ]]; then
    mv "$HOST_QT/libexec/rcc" "$HOST_QT/libexec/rcc.real"
    cat > "$HOST_QT/libexec/rcc" << 'EOF'
#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
for arg in "$@"; do
    case "$arg" in
        -h|--help|-help|-v|--version|-version)
            exec "$DIR/rcc.real" "$@"
            ;;
    esac
done
exec "$DIR/rcc.real" --no-zstd "$@"
EOF
    chmod +x "$HOST_QT/libexec/rcc"
    touch "$REPO_ROOT/build.rs"
fi

export QMAKE="$SCRIPT_DIR/qmake-mingw64-wrapper.sh"
export QT_VERSION_MAJOR=6
export LD_LIBRARY_PATH="$HOST_QT/lib:${LD_LIBRARY_PATH:-}"

echo "==> Building DuckyDeck Studio v$VERSION for Windows ($TARGET)..."
cargo build --release --target "$TARGET" --manifest-path "$REPO_ROOT/Cargo.toml"

EXE="$REPO_ROOT/target/$TARGET/release/duckydeck-studio.exe"
STAGE_DIR="$REPO_ROOT/target/windows-dist/bundle"
OUT_DIR="$REPO_ROOT/target/release"

rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR" "$OUT_DIR"

cp "$EXE" "$STAGE_DIR/duckydeck-studio.exe"
x86_64-w64-mingw32-strip --strip-unneeded "$STAGE_DIR/duckydeck-studio.exe" || true

# Copy Qt 6 runtime DLLs
QT_DLLS=(
    Qt6Core.dll
    Qt6Gui.dll
    Qt6Network.dll
    Qt6OpenGL.dll
    Qt6Qml.dll
    Qt6QmlCore.dll
    Qt6QmlMeta.dll
    Qt6QmlModels.dll
    Qt6QmlWorkerScript.dll
    Qt6Quick.dll
    Qt6QuickControls2.dll
    Qt6QuickControls2Basic.dll
    Qt6QuickControls2BasicStyleImpl.dll
    Qt6QuickControls2Fusion.dll
    Qt6QuickControls2FusionStyleImpl.dll
    Qt6QuickControls2Impl.dll
    Qt6QuickDialogs2.dll
    Qt6QuickDialogs2QuickImpl.dll
    Qt6QuickDialogs2Utils.dll
    Qt6QuickEffects.dll
    Qt6QuickLayouts.dll
    Qt6QuickShapes.dll
    Qt6QuickTemplates2.dll
    Qt6Svg.dll
    Qt6Widgets.dll
    d3dcompiler_47.dll
    opengl32sw.dll
)
for dll in "${QT_DLLS[@]}"; do
    if [[ -f "$WIN_QT/bin/$dll" ]]; then
        cp "$WIN_QT/bin/$dll" "$STAGE_DIR/"
    fi
done

# Copy LLVM-MinGW C++ runtime DLLs (UCRT)
for rtdll in libc++.dll libunwind.dll libgcc_s_seh-1.dll libstdc++-6.dll libwinpthread-1.dll; do
    if [[ -f "$WIN_QT/bin/$rtdll" ]]; then
        cp "$WIN_QT/bin/$rtdll" "$STAGE_DIR/"
    elif [[ -f "$HOME/.local/share/llu-wintoolchain/llvm-mingw/x86_64-w64-mingw32/bin/$rtdll" ]]; then
        cp "$HOME/.local/share/llu-wintoolchain/llvm-mingw/x86_64-w64-mingw32/bin/$rtdll" "$STAGE_DIR/"
    fi
done

# Copy Qt 6 plugins
mkdir -p "$STAGE_DIR/platforms" "$STAGE_DIR/styles" "$STAGE_DIR/iconengines" "$STAGE_DIR/imageformats" "$STAGE_DIR/tls"
for platp in qwindows.dll qoffscreen.dll qminimal.dll; do
    [[ -f "$WIN_QT/plugins/platforms/$platp" ]] && cp "$WIN_QT/plugins/platforms/$platp" "$STAGE_DIR/platforms/"
done
[[ -f "$WIN_QT/plugins/styles/qmodernwindowsstyle.dll" ]] && cp "$WIN_QT/plugins/styles/qmodernwindowsstyle.dll" "$STAGE_DIR/styles/"
[[ -f "$WIN_QT/plugins/iconengines/qsvgicon.dll" ]] && cp "$WIN_QT/plugins/iconengines/qsvgicon.dll" "$STAGE_DIR/iconengines/"
for imgp in qsvg.dll qico.dll qjpeg.dll qgif.dll; do
    [[ -f "$WIN_QT/plugins/imageformats/$imgp" ]] && cp "$WIN_QT/plugins/imageformats/$imgp" "$STAGE_DIR/imageformats/"
done
[[ -f "$WIN_QT/plugins/tls/qschannelbackend.dll" ]] && cp "$WIN_QT/plugins/tls/qschannelbackend.dll" "$STAGE_DIR/tls/"

# Copy required QML modules
mkdir -p "$STAGE_DIR/qml"
for qmod in QtQuick QtQml QtCore; do
    if [[ -d "$WIN_QT/qml/$qmod" ]]; then
        cp -r "$WIN_QT/qml/$qmod" "$STAGE_DIR/qml/"
        find "$STAGE_DIR/qml/$qmod" -name "*d.dll" -o -name "*.pdb" | xargs -r rm -f
    fi
done

cat > "$STAGE_DIR/qt.conf" << 'EOF'
[Paths]
Prefix = .
Plugins = .
QmlImports = qml
EOF

# Copy docs and firmware assets
mkdir -p "$STAGE_DIR/docs" "$STAGE_DIR/firmware/releases"
cp "$REPO_ROOT/README.md" "$STAGE_DIR/README.md"
cp "$REPO_ROOT/docs/"*.html "$STAGE_DIR/docs/"
cp "$REPO_ROOT/firmware/releases/"*.bin "$STAGE_DIR/firmware/releases/"

export WINEPREFIX="${WINEPREFIX:-$HOME/.local/share/llu-wintoolchain/wineprefix}"
export WINEDEBUG="${WINEDEBUG:--all}"
export WINEDLLOVERRIDES="${WINEDLLOVERRIDES:-mscoree,mshtml=}"

if command -v wine >/dev/null 2>&1; then
    echo "==> Smoke testing duckydeck-studio.exe --version under Wine..."
    wine "$STAGE_DIR/duckydeck-studio.exe" --version | tr -d '\r'
fi

ISCC="$WINEPREFIX/drive_c/InnoSetup/ISCC.exe"
if [[ -f "$ISCC" ]] && command -v wine >/dev/null 2>&1; then
    echo "==> Building Windows Setup installer via Inno Setup (Wine)..."
    (cd "$REPO_ROOT/packaging/windows" && wine "$ISCC" /Q \
        "/DAppVersion=$VERSION" \
        "/DSourceDir=$(winepath -w "$STAGE_DIR")" \
        "/DOutputDir=$(winepath -w "$OUT_DIR")" \
        DuckyDeckStudio.iss)
fi

PORTABLE_ZIP="$OUT_DIR/duckydeck-studio-v${VERSION}-windows-x86_64-portable.zip"
echo "==> Packaging portable zip: $PORTABLE_ZIP"
rm -f "$PORTABLE_ZIP"
(cd "$STAGE_DIR" && zip -q -9 -r "$PORTABLE_ZIP" .)

(cd "$OUT_DIR" && for f in "DuckyDeck-Studio-Setup-v${VERSION}-x86_64.exe" "duckydeck-studio-v${VERSION}-windows-x86_64-portable.zip"; do
    if [[ -f "$f" ]]; then
        sha256sum "$f" > "${f}.sha256"
    fi
done)

echo "==> Windows build artifacts ready in $OUT_DIR:"
ls -lh "$OUT_DIR"/DuckyDeck-Studio-Setup-v"${VERSION}"* "$OUT_DIR"/duckydeck-studio-v"${VERSION}"-windows* 2>/dev/null || true
