# DuckyDeck Studio (Rust + CXX-Qt + KDE Plasma 6 Kirigami)

A modern, native Linux configurator and AI Macro Architect for the **duckyPad Pro (`0x0483:0xd11d`)** and **duckyPad 2020 (`0x0483:0xd11c`)** built with **Rust**, **CXX-Qt 0.7**, and **KDE Frameworks 6 (Kirigami + KSyntaxHighlighting)**.

## Project Layout

```text
duckydeck-studio/
├── Cargo.toml                             # Rust dependencies (cxx-qt, hidapi, zbus, tokio, serde)
├── build.rs                               # CXX-Qt QML module & QObject bridge compiler
├── io.github.duckydeck.Studio.yml         # Flathub-ready Flatpak manifest
├── data/
│   ├── io.github.duckydeck.Studio.desktop      # XDG Desktop entry
│   ├── io.github.duckydeck.Studio.metainfo.xml # AppStream metadata with USB modalias
│   ├── icons/io.github.duckydeck.Studio.svg    # Scalable application icon
│   ├── syntax/duckyscript3.xml                 # KDE KSyntaxHighlighting grammar for duckyScript 3
│   └── udev/
│       ├── 70-duckypad.rules                   # uaccess udev rules for 0x0483:0xd11d & 0xd11c
│       └── install-udev-rules.sh               # Helper script to install & reload udev rules
└── src/
    ├── main.rs                            # Qt 6 / Breeze entrypoint
    ├── bridge/device_controller.rs        # #[cxx_qt::bridge] QObject exposed to QML
    ├── hardware/
    │   ├── permissions.rs                 # /sys/class/hidraw permission checker & Polkit installer
    │   └── storage.rs                     # USB HID PGVs (0x18/0x19) & microSD config.txt/key*.txt parser
    ├── kwin/auto_switcher.rs              # Pure-Rust zbus listener for Plasma 6 KWin Wayland window changes
    ├── ai/shortcut_scraper.rs             # Local ~/.config/kglobalshortcutsrc parser for AI grounding
    └── qml/
        ├── Main.qml                       # ApplicationWindow shell + KSyntaxHighlighting editor
        ├── UdevBanner.qml                 # Polkit 1-click udev setup inline warning
        ├── HardwareCanvas.qml             # 4x5 RGB matrix, 2 rotary encoders, 128x128 OLED & 32 expansion slots
        ├── PgvInspectorDrawer.qml         # Live Signed/Unsigned/Hex editor for _GV0.._GV31
        └── AiArchitectModal.qml           # AI Profile Architect modal dialog
```

---

## 1. Building Natively on Linux (Arch Linux / Fedora / KDE Neon)

### Prerequisites (Arch Linux)
```bash
sudo pacman -S --needed rust clang cmake ninja qt6-base qt6-declarative kirigami syntax-highlighting hidapi libusb
```

### Install Syntax Highlighting & Build
```bash
# Install duckyScript 3 syntax definition for your local user
mkdir -p ~/.local/share/org.kde.syntax-highlighting/syntax/
cp data/syntax/duckyscript3.xml ~/.local/share/org.kde.syntax-highlighting/syntax/

# Install udev rules (or click "Install Rules" inside the app's Polkit banner)
sudo ./data/udev/install-udev-rules.sh

# Build & run
cargo run --release
```

---

## 2. Building as a Flatpak

1. Generate `cargo-sources.json` for offline Flatpak compilation:
   ```bash
   cargo generate-lockfile
   curl -LO https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
   pip install aiohttp tomlkit
   python3 flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json
   ```
2. Install the KDE 6.8 SDK & Rust extensions and build:
   ```bash
   flatpak install flathub org.kde.Platform//6.8 org.kde.Sdk//6.8 \
       org.freedesktop.Sdk.Extension.rust-stable//24.08 \
       org.freedesktop.Sdk.Extension.llvm18//24.08

   flatpak-builder --user --install --force-clean build-dir io.github.duckydeck.Studio.yml
   flatpak run io.github.duckydeck.Studio
   ```
