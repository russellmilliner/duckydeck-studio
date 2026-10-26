# Changelog — duckydeck-studio

All notable changes and bumped dependencies for `russellmilliner/duckydeck-studio` are documented in this file.

## [v0.1.12](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.12) — 2026-10-26

### 🐛 Bug Fixes ([Issue #2](https://github.com/russellmilliner/duckydeck-studio/issues/2))
- **DuckyScript 3 `USE_STDLIB` & `ABS()` Support (`src/vm/compiler.rs`)**:
  - Registered `"USE_STDLIB" -> "DPDSSTDLIB"` in `default_defines()` and updated `preprocess_all()` to expand and recognize both `USE_STDLIB` and `DPDSSTDLIB` directives (including lines with trailing comments/whitespace).
  - Removed obsolete DSVM v1 `IF _UNSIGNED_MATH == 1` branch from `FUN ABS(n)` in `default_stdlib_lines()` so scripts calling `ABS()` compile cleanly on DSVM v2 without `Unknown symbol "_UNSIGNED_MATH"` errors.
  - Isolated negative-line-number stdlib definitions (`lnum < 0`) from user `DEFINE` macro replacement in `preprocess_all()` and `single_pass()` so user macros (e.g. `DEFINE n 10` or `DEFINE a 20`) cannot corrupt stdlib parameter identifiers (`ABS(n)`, `MIN(a, b)`, `MAX(a, b)`, `WAITKEY(key_id)`, `MEMSET(addr, value, length)`).
  - Added `USE_STDLIB` and `MEMSET` to the IDE command reference & autocomplete catalog (`src/qml/Main.qml`) and syntax highlighting (`data/syntax/duckyscript3.xml`).
- **IDE Script Editor & SD Sync Log Scrolling (`src/qml/Main.qml`, `src/qml/SdSyncDialog.qml`)**:
  - Added explicit `implicitWidth` and `implicitHeight` bindings to `scriptEditor` and `syncLogArea` (`T.TextArea` attached via `T.TextArea.flickable` to `QQC2.ScrollView`) so `QQuickTextAreaPrivate::resizeFlickableContent()` updates `scriptScrollView.contentHeight` dynamically as multi-line scripts grow, enabling smooth mouse wheel, touchpad, scrollbar drag, and cursor scrolling across Wayland compositors (including Niri and KWin).
  - Reset `scriptScrollView.contentY = 0` and `scriptScrollView.contentX = 0` when switching slots or click phases (`On Press` / `Double Click` / `Long Press`).
- **Desktop OLED Widget Compact Header & Multi-Device Settings Font Sizing (`src/qml/OledDesktopWidget.qml`, `src/qml/SettingsPage.qml`)**:
  - Compacted `OledDesktopWidget` top header pills and tool buttons when both multi-device and `PAUSED` badges are visible simultaneously so controls never overflow narrow widget widths.
  - Standardized connected-device custom name controls in `SettingsPage.qml` to `10pt` scaled minimum font size.

## [v0.1.11-deps](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.11-deps) — 2026-10-24

### 📦 Bumped Dependencies (4 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `cc` | `1.9.0` | **`1.10.0`** | Lockfile (`Cargo.lock`) |
| `smallvec` | `1.16.3` | **`1.16.4`** | Lockfile (`Cargo.lock`) |
| `uuid` | `1.28.0` | **`1.29.0`** | Lockfile (`Cargo.lock`) |
| `webpki-roots` | `1.0.10` | **`1.0.11`** | Lockfile (`Cargo.lock`) |

## [v0.1.11](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.11) — 2026-10-23

## [v0.1.10](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.10) — 2026-10-22

### ✨ Features & Enhancements
- **Custom Hardware duckyPad Naming**: Assign friendly custom names to individual hardware duckyPads by serial number (persisted in `~/.config/duckydeck-studio/device_names.json` / `%APPDATA%\duckydeck-studio\device_names.json`), accessible via the status bar rename button (`🏷️`), **Settings → Multi-Device Management**, and `POST /api/v1/devices/rename`.
- **Automatic Widget & Status Bar Cleanup on Disconnect**: Status bar per-device widget toggles and floating `OledDesktopWidget` windows now automatically close and hide when a duckyPad is unplugged over USB, re-appearing seamlessly when reconnected.
- **Built-in `duckypad-esp32-ha` Web Server (100% Feature Sync + Auto-Shutdown)**: Automatically detects USB-connected ESP32-HA duckyPads (`0x303A:*` / `DPP_ESP32` / `ESP32-HA`), serves the full-featured ESP32 Home Assistant Web Configurator on `http://127.0.0.1:8099`, and automatically shuts down the server and hides the status bar badge when no ESP32-HA duckyPad is connected.
- **Custom Virtual Keycode Support (`VK_0xNN` / `HID_0xNN` / `KEYCODE_0xNN`)**: Send any arbitrary raw USB HID Usage ID (`0x00..0xFF`, Keyboard/Consumer page `0x07`) in both the DSVM v2 compiler (`src/vm/compiler.rs`) and `duckypad-esp32-ha` (`F13`–`F24` + `VK_0x74..0x8D`).
- **Multi-Device Profile Switching & Reloading Fixes**: Added `#[cxx_name = "isDuckyPad2020"]` QML binding, strict serial filtering in `query_connected_duckypad_info_for_serial`, `EMUK` compiler expansion to `KEYDOWN`/`KEYUP`, and `.sd_synced_snapshot.json` recovery so switching between `duckyPad Pro` and `duckyPad (2020)` always preserves and reloads all profiles cleanly.
- **Major Memory & CPU Optimizations (Linux & Windows)**: Deferred QML dialog instantiation via `Loader`, capped `Image` `sourceSize` decode dimensions, replaced periodic `mount` subprocess polling with zero-spawn `/proc/mounts` hash / `GetLogicalDrives()` bitmask checks, and bounded API/event buffers — reducing Linux RSS from `164.8 MB` to `37.2 MB` (-77%) and Windows Working Set from `148.2 MB` to `61.4 MB` (-59%).

## [v0.1.9](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.9) — 2026-10-22

### Changes
- docs(changelog): add v0.1.8 dependency changelog (5c26a79)

## [v0.1.8](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.8) — 2026-10-22

### 📦 Bumped Dependencies (20 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `tokio` | `1.53.3` | **`1.54.1`** | Direct (`Cargo.toml`) |
| `cc` | `1.8.0` | **`1.9.0`** | Lockfile (`Cargo.lock`) |
| `find-msvc-tools` | `0.1.14` | **`0.1.15`** | Lockfile (`Cargo.lock`) |
| `icu_collections` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_locale_core` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_normalizer` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_normalizer_data` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_properties` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_properties_data` | `2.3.0` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `icu_provider` | `2.3.1` | **`2.4.0`** | Lockfile (`Cargo.lock`) |
| `libc` | `0.2.190` | **`0.2.191`** | Lockfile (`Cargo.lock`) |
| `rustls-webpki` | `0.103.15` | **`0.103.16`** | Lockfile (`Cargo.lock`) |
| `tinystr` | `0.8.4` | **`0.8.5`** | Lockfile (`Cargo.lock`) |
| `webpki-roots` | `1.0.9` | **`1.0.10`** | Lockfile (`Cargo.lock`) |
| `yoke` | `0.8.3` | **`0.8.4`** | Lockfile (`Cargo.lock`) |
| `zbus_names` | `4.3.4` | **`4.3.5`** | Lockfile (`Cargo.lock`) |
| `zerovec` | `0.11.8` | **`0.11.9`** | Lockfile (`Cargo.lock`) |
| `zerovec-derive` | `0.11.6` | **`0.11.7`** | Lockfile (`Cargo.lock`) |
| `zvariant` | `5.15.0` | **`5.15.1`** | Lockfile (`Cargo.lock`) |
| `zvariant_derive` | `5.15.0` | **`5.15.1`** | Lockfile (`Cargo.lock`) |
