#!/usr/bin/env bash
set -euo pipefail

QT_ROOT="${DUCKYDECK_WIN_QT_ROOT:-$HOME/.local/share/llu-wintoolchain/qt6/6.8.3}"
WIN_QT="${DUCKYDECK_WIN_QT_DIR:-$QT_ROOT/llvm-mingw_64}"
HOST_QT="$QT_ROOT/gcc_64"

if [[ "${1:-}" == "-query" ]]; then
    VAR="${2:-}"
    case "$VAR" in
        QT_VERSION) echo "6.8.3" ;;
        QT_INSTALL_PREFIX|QT_INSTALL_PREFIX/get) echo "$WIN_QT" ;;
        QT_INSTALL_HEADERS|QT_INSTALL_HEADERS/get) echo "$WIN_QT/include" ;;
        QT_INSTALL_LIBS|QT_INSTALL_LIBS/get) echo "$WIN_QT/lib" ;;
        QT_INSTALL_BINS|QT_INSTALL_BINS/get) echo "$WIN_QT/bin" ;;
        QT_INSTALL_LIBEXECS|QT_INSTALL_LIBEXECS/get) echo "$WIN_QT/bin" ;;
        QT_INSTALL_PLUGINS|QT_INSTALL_PLUGINS/get) echo "$WIN_QT/plugins" ;;
        QT_INSTALL_QML|QT_INSTALL_QML/get) echo "$WIN_QT/qml" ;;
        QT_INSTALL_DATA|QT_INSTALL_DATA/get|QT_INSTALL_ARCHDATA|QT_INSTALL_ARCHDATA/get) echo "$WIN_QT" ;;
        QT_HOST_PREFIX|QT_HOST_PREFIX/get) echo "$HOST_QT" ;;
        QT_HOST_BINS|QT_HOST_BINS/get) echo "$HOST_QT/bin" ;;
        QT_HOST_LIBEXECS|QT_HOST_LIBEXECS/get) echo "$HOST_QT/libexec" ;;
        QT_HOST_LIBS|QT_HOST_LIBS/get) echo "$HOST_QT/lib" ;;
        QT_HOST_DATA|QT_HOST_DATA/get) echo "$HOST_QT" ;;
        *) exec "$HOST_QT/bin/qmake" "$@" ;;
    esac
    exit 0
fi

exec "$HOST_QT/bin/qmake" "$@"
