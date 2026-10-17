# <img src="data/icons/io.github.duckydeck.Studio.svg" width="36" height="36" align="top" alt="DuckyDeck Studio Icon" /> DuckyDeck Studio

[![Ko-fi](https://img.shields.io/badge/Ko--fi-Buy%20Me%20a%20Coffee-ff5e5b?logo=ko-fi&logoColor=white&style=for-the-badge)](https://ko-fi.com/russellmilliner)
[![duckyPad Official Site](https://img.shields.io/badge/duckyPad-Official%20Site-f59e0b?style=for-the-badge)](https://www.duckypad.com)
[![Tindie Store](https://img.shields.io/badge/Tindie-dekuNukem%20Store-10b981?style=for-the-badge)](https://www.tindie.com/stores/dekunukem/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Linux / KDE Plasma 6](https://img.shields.io/badge/Platform-Linux%20%7C%20KDE%20Plasma%206-3daee9?logo=kde&logoColor=white&style=for-the-badge)](https://kde.org/)

**DuckyDeck Studio** is a native Linux desktop configurator, **DSVM v2 bytecode compiler**, **KDE Plasma 6 Wayland Auto-Switcher**, and **AI Macro Architect** for the **duckyPad Pro (`0x0483:0xd11d`)** and **duckyPad 2020 (`0x0483:0xd11c`)**, built with **Rust**, **CXX-Qt 0.7**, and **KDE Frameworks 6 (Kirigami + KSyntaxHighlighting)**.

> ☕ **Enjoying DuckyDeck Studio?** Support development by buying Russell Milliner a coffee on Ko-fi: **[https://ko-fi.com/russellmilliner](https://ko-fi.com/russellmilliner)**
>
> 🙏 **Special Thanks to [dekuNukem](https://github.com/dekuNukem)** for creating the **[duckyPad & duckyPad Pro](https://www.duckypad.com)**! Get official hardware and accessories at **[dekuNukem's Official Tindie Store](https://www.tindie.com/stores/dekunukem/)**.

---

## 🎬 Video Demo & Screenshots

- 🎥 **[Watch the Full 1080p Application Demo Video (`docs/duckydeck-studio-demo.mp4`)](docs/duckydeck-studio-demo.mp4)**

| Dolphin File Manager & DSVM v2 IDE | OBS Studio & Web API Webhook Builder |
| :---: | :---: |
| ![Dolphin File Manager Overview](docs/screenshots/01-dolphin-overview.png) | ![OBS Studio Web API Builder](docs/screenshots/05-obs-studio-web-api.png) |
| **Desktop Application Launcher** | **Custom Key Color Picker & Palettes** |
| ![Desktop Application Launcher](docs/screenshots/09-app-launcher.png) | ![Color Picker & Palettes](docs/screenshots/04-color-picker-palette.png) |
| **KWin Wayland Auto-Switcher Rules** | **Live System Event Log** |
| ![Wayland Auto-Switcher Rules](docs/screenshots/10-wayland-auto-switcher.png) | ![System Event Log](docs/screenshots/11-system-event-log.png) |

---

## ✨ Key Features

- **🎹 Interactive Hardware Canvas & 128×128 OLED Simulator:**
  - Full visual editor for the **20-key RGB matrix (4×5)**, **Dual Rotary Encoders (Slots 21–26)**, and **32-channel Expansion Modules (Slots 27–58)**.
  - Drag-and-drop keys to **swap** slots, or hold <kbd>Shift</kbd> while dragging to **insert & shift** keys.
  - Live 128×128 monochrome OLED display simulator supporting both the 4×5 key label grid and custom `OLED_LINE` / `OLED_RECT` / `OLED_CIRCLE` / `OLED_PRINT` drawing scripts.
  - Right-click any keycap to **Copy**, **Paste** (across keys and profiles), **Set Key X Color**, or **Clear / Reset**.
- **🎨 Custom Color Picker with Saved & Used Palettes:**
  - Interactive 2D Saturation/Value color square, vertical Hue slider, R/G/B (`0–255`) sliders & spinboxes, `#RRGGBB` hex input, and native OS eyedropper integration.
  - **Saved Colors Palette:** Pin your favorite RGB colors to a persistent palette (`~/.config/duckydeck-studio/kwin_rules.json`).
  - **Used Colors Palette:** Automatically scans and displays every unique color currently used in any profile (`BG_COLOR`) or key (`SWCOLOR`) for 1-click reuse.
- **📜 Built-In duckyScript 3 IDE & DSVM v2 Bytecode Compiler:**
  - Pure-Rust two-pass compiler that compiles `duckyScript 3` (`keyN.txt` & `keyN-release.txt`) directly into **DSVM v2 (`.dsb`)** bytecode binaries.
  - Live syntax verification with byte count & exact error line reporting, KDE `KSyntaxHighlighting`, and a searchable **Command Reference Drawer**.
- **🚀 Launch Application Builder (No-Code Desktop App Launcher):**
  - Search installed Linux desktop applications (`.desktop` entries from system, user, and Flatpak directories with icons and categories) or configure custom CLI commands, arguments, working directories, and terminal execution.
  - Triggers instantaneously over USB HID via Persistent Global Variable `_GV19` (no keyboard shortcut conflicts!) or via `F13–F24` global hotkeys (`evdev`).
- **🌐 Web API / Webhook Builder & 🏠 Home Assistant Entity Search:**
  - Trigger HTTP `GET`, `POST`, `PUT`, `PATCH`, and `DELETE` requests (Home Assistant, Miniflux, OBS, custom webhooks) with custom headers and JSON bodies directly from any duckyPad key via USB HID (`_GV18`) or `F13–F24`.
  - **Built-in Home Assistant Web API Browser:** Connect to your Home Assistant instance (`base_url` + Long-Lived Access Token) to search devices and entities across domains (`light`, `switch`, `scene`, `script`, `automation`, `button`, `input_boolean`, `media_player`, `cover`, `fan`, `climate`, `lock`) and bind `toggle` / `turn_on` / `turn_off` / `trigger` service calls to any key in one click.
  - **🔔 Profile Change Webhooks:** Automatically fire a per-profile HTTP webhook (configured in **Profile Properties**) and/or a global profile-change webhook (configured in **Settings**) whenever the active profile changes via UI, hardware buttons, Window Auto-Switcher, or the REST API, with template placeholders (`{{profile_number}}`, `{{profile_name}}`, `{{previous_profile_number}}`, `{{previous_profile_name}}`, `{{source}}`, `{{device_serial}}`, `{{timestamp}}`).
- **🔌 Built-In HTTP REST API Server (`/api/v1/*`) for Remote Control & Programming:**
  - Optional embedded HTTP/1.1 REST API server configurable in **Settings → HTTP REST API Server** (defaults to `127.0.0.1:28491`, disabled by default, with 160-bit `ddk_...` Bearer token authentication) for controlling and programming every function of the duckyPad remotely (switching/creating/editing/deleting profiles, programming key labels/colors/icons/scripts, reading/writing `_GV0.._GV31`, setting LEDs/OLED, triggering slots, and syncing to microSD).
- **🪟 KDE Plasma 6 Wayland Auto-Switcher:**
  - Pure-Rust `zbus` D-Bus listener integrated with KWin Wayland to automatically switch duckyPad hardware profiles when you focus different applications or browser tabs.
  - Includes a **🎯 Crosshair Window Picker** to bind any open window or web app tab in one click, and automatically pauses while the Key Editor is open.
- **💾 Offline Profile Cache, Diff Summary & Validated microSD Sync:**
  - Edits are saved immediately to a local per-device cache (`~/.local/share/duckydeck-studio/profiles_cache/`).
  - Inspect a detailed **Unsynced Changes Summary** before syncing, then write, flush, and verify `.dsb` bytecode and `config.txt` files on the duckyPad microSD card.
  - Export and import individual profiles (`.duckyprofile.json` or folder) or complete **Profile Groups (`.duckygroup.json`)**.
- **🔢 Live PGV Inspector (`_GV0`..`_GV31`) & ✨ AI Macro Architect:**
  - Read and write all 32 non-volatile 32-bit Persistent Global Variables (`_GV0.._GV31`) over USB HID (`0x18`/`0x19`) in Signed, Unsigned, or Hex format.
  - Generate duckyScript 3 macros from natural language grounded in your local KDE shortcuts (`~/.config/kglobalshortcutsrc`).
- **📋 System Event Log, Per-Area UI Text Scaling & System Tray:**
  - Built-in **System Event Log** (`DEVICE`, `PROFILE`, `AUTOSWITCH`, `SD_SYNC`, `SCRIPT`, `WEB_API`, `APP_LAUNCH`, `SYSTEM`) with search, category/severity filters, and file export.
  - Independent text size scaling (`75%–160%`) for every area of the application, plus a dynamic KDE `StatusNotifierItem` System Tray icon showing the active profile number and color badge.
- **🗂️ Reusable Profile Templates:**
  - Create new profiles from built-in templates (*Blank*, *Media & Streaming*, *IDE & Developer*, *Desktop App & Web API Launcher*) or copy any existing profile into a reusable custom template (`~/.config/duckydeck-studio/profile_templates.json`).
- **🖥️ Desktop OLED Companion Widget & Instant Independent Orientation (`0° / 90° / 180° / 270°`):**
  - Floating, multi-monitor-aware desktop replica of the duckyPad OLED screen with persistent screen/position/transparency settings, optional 24-character extended labels (`wzN`/`wxN`), and independent **Desktop Widget Orientation** (`Follow OLED`, `0°`, `90°`, `180°`, `270°`) configurable per profile in **Profile Properties** that applies immediately without requiring a microSD card sync.
- **🔧 DuckyDeck Studio Firmware `v1.0.0` & Official Firmware Catalog Downloader:**
  - Custom firmware build (`v1.0.0`, badged with the DuckyDeck Studio icon in the status bar) supporting `0°`, `90°`, `180°`, and `270°` hardware OLED orientations and crisp 1-bit monochrome key icons.
  - **Official Firmware Catalog Downloader:** Fetch the list of official dekuNukem firmwares directly inside the **Firmware Manager** window, download `.bin` images with IEEE CRC32 verification, and install them in one click.

---

## ⚠️ Hardware Portability vs. Host-Dependent Capabilities

DuckyDeck Studio clearly badges capabilities in the UI as either **✅ Hardware-Portable** (compiled to the microSD card and executed autonomously by the duckyPad on any computer) or **💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable)**:

| Capability / Feature | Portability | How It Works & What Happens Without DuckyDeck Studio |
| :--- | :--- | :--- |
| **Standard DuckyScript 3 Macros** (`STRING`, `KEYDOWN`, `DELAY`, `MOUSE_MOVE`, `LMOUSE`, loops, `IF`/`WHILE`) | ✅ **Hardware-Portable** | Compiles to `.dsb` bytecode on the microSD card. Works on **any PC/OS** with zero host software installed. |
| **Static Profile & Key RGB Colors** (`BG_COLOR`, `SWCOLOR`, `KEYDOWN_COLOR`, `DIM_UNUSED_KEYS`) | ✅ **Hardware-Portable** | Stored in `config.txt` on the microSD card and rendered directly by duckyPad hardware. |
| **Embedded Key Script RGB Animations** (`SWC_SET`, `SWC_FILL`, `SWC_RESET`) | ✅ **Hardware-Portable** | Compiled directly into `keyN.dsb` bytecode on the microSD card; runs standalone on hardware. |
| **5-Char Physical OLED Key Labels (`zN` / `xN`) & `OLED_*` Script Graphics** | ✅ **Hardware-Portable** | Stored on microSD and rendered by the physical 128×128 OLED display. |
| **Physical OLED Orientation (`0° / 90° / 180° / 270°`) & Monochrome Icons** | ✅ **Hardware-Portable** *(180°/270° & icons require DuckyDeck Studio Firmware v1.0.0)* | Stored in `config.txt` (`IS_LANDSCAPE` / `OLED_ROTATION`) and rendered on the hardware OLED screen. |
| **Manual Profile Switching** (`GOTO_PROFILE`, `NEXT_PROFILE`, `PREV_PROFILE`, `+`/`-` button hold) | ✅ **Hardware-Portable** | Handled natively on the duckyPad hardware. |
| **🚀 Launch Desktop Application (`💻APP` badge)** | 💻 **Requires DuckyDeck Studio Running** | Triggered over USB HID (`_GV19`/`_GV31`) or `evdev` (`F13–F24`) by the DuckyDeck Studio host process. **Will not launch apps if DuckyDeck Studio is closed or on another PC.** |
| **🌐 Web API / HTTP Webhooks & Home Assistant Triggers (`💻API` badge)** | 💻 **Requires DuckyDeck Studio Running** | Outbound HTTP requests are dispatched by DuckyDeck Studio on the host PC. **The duckyPad hardware has no network stack and cannot fire webhooks standalone.** |
| **🔔 Profile Change Webhooks (Per-Profile & Global)** | 💻 **Requires DuckyDeck Studio Running** | Fired by DuckyDeck Studio when a profile switch is initiated or detected on the host PC. Does not require a microSD card sync. |
| **🔌 HTTP REST API Server (`/api/v1/*`)** | 💻 **Requires DuckyDeck Studio Running** | Hosted by the DuckyDeck Studio process (`127.0.0.1:28491` by default). |
| **🪟 KWin Wayland Active-Window Auto-Switcher** | 💻 **Requires DuckyDeck Studio Running** | Uses KDE Plasma 6 KWin D-Bus tracking on the host PC to send USB HID profile-switch commands. Without the host app, switch profiles manually on the keypad. |
| **🖥️ Desktop OLED Widget, 24-Char Extended Widget Labels (`wzN`/`wxN`) & Widget Orientation (`DW_ROTATION`)** | 💻 **Requires DuckyDeck Studio Running** | Displayed on your computer monitor by DuckyDeck Studio; orientation changes apply immediately without requiring a microSD card sync. |
| **✨ Live USB HID RGB Animation Streaming** | 💻 **Requires DuckyDeck Studio Running** | Streams RGB frames over USB HID (`0x14`) from DuckyDeck Studio. Use the *"Embed standalone DuckyScript 3 SWC_SET"* option if you want portable on-device key lighting. |

---

## 🔌 HTTP REST API Server & Security Implications

DuckyDeck Studio includes a full-featured **HTTP REST API Server** (configurable in **Settings → HTTP REST API Server**) for remotely controlling and programming all functions of the duckyPad from scripts, Home Assistant, Node-RED, Stream Deck bridges, or custom tools.

- **Default Listen Address & Port:** `127.0.0.1:28491` (Disabled by default; Bearer Token Authentication enabled by default).
- **Authentication Header:** Pass your generated 160-bit token (`ddk_<40 hex chars>`) via `Authorization: Bearer <token>` or `X-API-Token: <token>`.

### ⚠️ Security Implications & Best Practices
1. **Full Hardware & Host Control:** An authenticated REST API client has complete administrative control over the connected duckyPad and host integrations. It can switch/create/modify/delete profiles and key scripts, sync changes to the microSD card, compile and execute DuckyScript keystroke/mouse macros, manipulate LEDs/OLED/PGVs, and invoke host actions (`APP_LAUNCH`, `WEB_API`, `SYS_ACTION`) configured on slots.
2. **Loopback (`127.0.0.1`) vs. Network Binding (`0.0.0.0`):**
   - Keeping the listen address set to **`127.0.0.1` (default)** ensures that only local processes running on your computer can connect to the API.
   - Binding to **`0.0.0.0`** or a LAN IP address exposes the server over **unencrypted HTTP** to every device on your local network.
3. **Token Protection & Transport Security:**
   - Always keep **Require Bearer Token Authentication** enabled, especially when binding to a non-loopback address.
   - Treat your API token (`ddk_...`) as a sensitive secret and rotate it immediately using **Generate Token** in Settings if you suspect it was exposed.
   - Never expose the plain-HTTP port directly to the public internet or untrusted networks; place it behind a TLS/HTTPS reverse proxy (Caddy, Nginx) or use an encrypted tunnel (SSH port forwarding, WireGuard, Tailscale).

### REST API Endpoint Summary (`/api/v1/*`)
| Method & Path | Description |
| :--- | :--- |
| `GET /api/v1/health` | Unauthenticated liveness probe (`{"ok":true,"status":"healthy"}`). |
| `GET /api/v1/status` | Full device connection, firmware, active profile, SD sync, and auto-switcher state. |
| `POST /api/v1/device/reconnect` | Re-scan USB HID devices and reload profiles. |
| `POST /api/v1/device/sleep` \| `/wake` \| `/reboot` | Put duckyPad to sleep, wake it up, or reboot it (`{"bootloader":false}`). |
| `POST /api/v1/device/sync-sd` | Compile all `.dsb` binaries and synchronize profiles to the microSD card. |
| `GET /api/v1/profiles` | List all profiles and their properties. |
| `POST /api/v1/profiles` | Create a new profile (`{"name":"Streaming"}`). |
| `POST /api/v1/profiles/select` | Switch active profile (`{"profile":2}`). |
| `POST /api/v1/profiles/next` \| `/prev` | Step to next or previous profile. |
| `GET /api/v1/profiles/{num}` | Get full profile details, webhook config, and all 58 slots. |
| `PUT /api/v1/profiles/{num}` | Update profile properties (`name`, `bgColor`, `oledRotation`, `widgetRotation`, `webhook`, etc.). |
| `DELETE /api/v1/profiles/{num}` | Delete profile `{num}`. |
| `POST /api/v1/profiles/{num}/duplicate` | Duplicate profile `{num}`. |
| `POST /api/v1/profiles/{num}/move` | Reorder profile (`{"direction":-1}` or `1`). |
| `GET /api/v1/profiles/{num}/slots` | List all 58 slots in profile `{num}`. |
| `GET /api/v1/profiles/{num}/slots/{slot}` | Get configuration and scripts for a specific slot (`1..58`). |
| `PUT /api/v1/profiles/{num}/slots/{slot}` | Program a slot (`line1`, `line2`, `widgetLine1`, `widgetLine2`, `colorHex`, `iconName`, `script`, `releaseScript`, `allowAbort`, `dontRepeat`). |
| `DELETE /api/v1/profiles/{num}/slots/{slot}` | Clear/reset slot `{slot}` in profile `{num}`. |
| `POST /api/v1/profiles/{num}/slots/{slot}/trigger` | Remotely execute the slot's configured action/webhook/app-launch/macro. |
| `GET /api/v1/pgv` \| `POST /api/v1/pgv` | Read all 32 PGVs (`_GV0.._GV31`) or write one (`{"index":0,"value":42}`). |
| `POST /api/v1/leds` \| `POST /api/v1/leds/restore` | Override RGB LEDs (`{"fill":"#38bdf8"}` or `{"slot":1,"color":"#ff0055"}`) or restore profile LEDs. |
| `POST /api/v1/oled/text` \| `/clear` \| `/restore` | Draw text on the OLED (`{"x":0,"y":0,"text":"Hello"}`), clear, or restore profile OLED. |
| `POST /api/v1/compile` | Validate/compile DuckyScript 3 source (`{"script":"STRING Hello\nENTER"}`). |
| `POST /api/v1/auto-switcher` | Enable/disable the KWin Wayland Auto-Switcher (`{"enabled":true}`). |

---

## 📚 Documentation

- **📖 [Web User Documentation & Manual (`docs/user-guide.html`)](docs/user-guide.html)** — Interactive dark/light web manual covering every feature, workflow, and troubleshooting tip (also accessible inside the app via **☰ → User Documentation** or **About → User Guide**).
- **🏗️ [Architecture & Design Developer Documentation (`docs/ARCHITECTURE.md`)](docs/ARCHITECTURE.md)** — In-depth technical documentation with Mermaid diagrams detailing the CXX-Qt bridge, USB HID protocol, DSVM v2 bytecode compiler, HTTP REST API server, and KWin Wayland Auto-Switcher.

---

## 📂 Project Layout

```text
duckydeck-studio/
├── Cargo.toml                             # Rust dependencies (cxx-qt, hidapi, zbus, tokio, ksni, evdev, ureq)
├── build.rs                               # CXX-Qt QML module & QObject bridge compiler
├── io.github.duckydeck.Studio.yml         # Flatpak manifest (org.kde.Platform 6.8 + Rust SDK)
├── docs/
│   ├── user-guide.html                    # Interactive Web User Documentation & Manual
│   └── ARCHITECTURE.md                    # Developer Architecture & Design Documentation
├── data/
│   ├── io.github.duckydeck.Studio.desktop      # XDG Desktop entry
│   ├── io.github.duckydeck.Studio.metainfo.xml # AppStream metadata with USB modalias
│   ├── icons/io.github.duckydeck.Studio.svg    # Scalable application icon
│   ├── syntax/duckyscript3.xml                 # KDE KSyntaxHighlighting grammar for duckyScript 3
│   └── udev/
│       ├── 70-duckypad.rules                   # uaccess udev rules for 0x0483:0xd11d & 0xd11c
│       └── install-udev-rules.sh               # Helper script to install & reload udev rules
└── src/
    ├── main.rs                            # Qt 6 / Breeze entrypoint & syntax registration
    ├── api_server.rs                      # Embedded HTTP/1.1 REST API Server (/api/v1/*) with Bearer auth
    ├── tray.rs                            # StatusNotifierItem system tray & dynamic profile icon
    ├── bridge/device_controller.rs        # #[cxx_qt::bridge] DeviceController QObject
    ├── hardware/
    │   ├── dsvm.rs                        # Pure-Rust duckyScript 3 -> DSVM v2 (.dsb) bytecode compiler
    │   ├── permissions.rs                 # /sys/class/hidraw permission checker & Polkit installer
    │   └── storage.rs                     # USB HID protocol (0x18/0x19/0x1B), HA client & microSD sync
    ├── kwin/auto_switcher.rs              # Pure-Rust zbus listener for Plasma 6 KWin Wayland window focus
    ├── ai/shortcut_scraper.rs             # Local ~/.config/kglobalshortcutsrc parser for AI grounding
    └── qml/
        ├── Main.qml                       # ApplicationWindow shell, Color Picker, Builders, Event Log
        ├── UdevBanner.qml                 # Polkit 1-click udev setup inline warning
        ├── HardwareCanvas.qml             # 4x5 RGB matrix, 2 rotary encoders, 128x128 OLED & 32ch expansion
        ├── AutoSwitchDrawer.qml           # KWin Wayland Auto-Switcher rules & window picker dialog
        ├── PgvInspectorDrawer.qml         # Live Signed/Unsigned/Hex editor for _GV0.._GV31
        └── AiArchitectModal.qml           # AI Profile Architect modal dialog
```

---

## 🛠️ Build & Installation Instructions

### 1. Install Prerequisites

#### Arch Linux / CachyOS / EndeavourOS / Manjaro
```bash
sudo pacman -S --needed base-devel rust clang cmake ninja pkgconf \
    qt6-base qt6-declarative kirigami syntax-highlighting hidapi libusb
```

#### Fedora (40 / 41+)
```bash
sudo dnf install cargo rust clang cmake ninja-build pkgconf-pkg-config \
    qt6-qtbase-devel qt6-qtdeclarative-devel kf6-kirigami-devel \
    kf6-syntax-highlighting-devel hidapi-devel libusb1-devel systemd-devel
```

#### Ubuntu (24.04+) / Debian / KDE Neon
```bash
sudo apt install build-essential cargo rustc clang cmake ninja-build pkg-config \
    qt6-base-dev qt6-declarative-dev libkf6kirigami-dev \
    libkf6syntaxhighlighting-dev libhidapi-dev libusb-1.0-0-dev libudev-dev
```

---

### 2. Configure USB HID Permissions (`udev` Rules)

To allow non-root access to `/dev/hidraw*` for the duckyPad Pro (`0x0483:0xd11d`) and duckyPad 2020 (`0x0483:0xd11c`), install the included `udev` rules:

```bash
sudo ./data/udev/install-udev-rules.sh
```
*(Alternatively, you can click the **Install Rules** button inside the app's top Polkit banner on first launch.)*

---

### 3. Local Build & User Installation (`~/.local/bin`)

Build the optimized release binary and install the executable, desktop launcher, icon, and `duckyScript 3` syntax highlighting definition for your user account:

```bash
# 1. Compile optimized release binary
cargo build --release

# 2. Install binary to ~/.local/bin
install -Dm755 target/release/duckydeck-studio ~/.local/bin/duckydeck-studio

# 3. Install duckyScript 3 syntax highlighting for KDE KSyntaxHighlighting
install -Dm644 data/syntax/duckyscript3.xml \
    ~/.local/share/org.kde.syntax-highlighting/syntax/duckyscript3.xml

# 4. Install desktop entry, icon, and documentation
install -Dm644 data/io.github.duckydeck.Studio.desktop \
    ~/.local/share/applications/io.github.duckydeck.Studio.desktop
install -Dm644 data/icons/io.github.duckydeck.Studio.svg \
    ~/.local/share/icons/hicolor/scalable/apps/io.github.duckydeck.Studio.svg
install -Dm644 docs/user-guide.html \
    ~/.local/share/duckydeck-studio/docs/user-guide.html
```

Run the application:
```bash
~/.local/bin/duckydeck-studio
```

---

### 4. System-Wide Installation (`/usr/local`)

If you prefer to install DuckyDeck Studio system-wide for all users:

```bash
cargo build --release
sudo install -Dm755 target/release/duckydeck-studio /usr/local/bin/duckydeck-studio
sudo install -Dm644 data/io.github.duckydeck.Studio.desktop /usr/share/applications/io.github.duckydeck.Studio.desktop
sudo install -Dm644 data/icons/io.github.duckydeck.Studio.svg /usr/share/icons/hicolor/scalable/apps/io.github.duckydeck.Studio.svg
sudo install -Dm644 data/io.github.duckydeck.Studio.metainfo.xml /usr/share/metainfo/io.github.duckydeck.Studio.metainfo.xml
sudo ./data/udev/install-udev-rules.sh
```

---

### 5. Building & Installing as a Flatpak

1. Install the KDE Plasma 6.8 Flatpak SDK and Rust/LLVM extensions:
   ```bash
   flatpak install --user flathub org.kde.Platform//6.8 org.kde.Sdk//6.8 \
       org.freedesktop.Sdk.Extension.rust-stable//24.08 \
       org.freedesktop.Sdk.Extension.llvm18//24.08
   ```
2. Build and install the Flatpak package (and optionally export a single-file `.flatpak` bundle):
   ```bash
   flatpak-builder --user --install --force-clean flatpak-build-dir io.github.duckydeck.Studio.yml
   flatpak build-export flatpak-repo flatpak-build-dir
   flatpak build-bundle flatpak-repo duckydeck-studio.flatpak io.github.duckydeck.Studio
   ```
3. Run the Flatpak:
   ```bash
   flatpak run io.github.duckydeck.Studio
   ```

---

## 🙏 Special Thanks & Official duckyPad Links

A special thank you to **[dekuNukem](https://github.com/dekuNukem)** for creating the **duckyPad** and **duckyPad Pro**, designing the **DuckyScript 3 / DSVM v2** architecture, and open-sourcing the hardware, firmware, and USB HID specifications for the community!

- **🦆 Official duckyPad Website:** [https://www.duckypad.com](https://www.duckypad.com)
- **🛒 dekuNukem's Official Tindie Store:** [https://www.tindie.com/stores/dekunukem/](https://www.tindie.com/stores/dekunukem/)
- **📦 duckyPad Pro on Tindie:** [https://www.tindie.com/products/dekunukem/duckypad-pro-macro-scripting-beyond-qmkvia/](https://www.tindie.com/products/dekunukem/duckypad-pro-macro-scripting-beyond-qmkvia/)
- **💻 duckyPad Pro GitHub Repository:** [https://github.com/dekuNukem/duckyPad-Pro](https://github.com/dekuNukem/duckyPad-Pro)

---

## ☕ Support the Project

If **DuckyDeck Studio** makes your duckyPad workflow better on Linux, consider buying Russell Milliner a coffee!

- **Ko-fi:** [https://ko-fi.com/russellmilliner](https://ko-fi.com/russellmilliner)
- **GitHub:** [https://github.com/millinr/duckydeck-studio](https://github.com/millinr/duckydeck-studio)

