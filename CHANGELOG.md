# Changelog — duckydeck-studio

All notable changes and bumped dependencies for `russellmilliner/duckydeck-studio` are documented in this file.

## [v0.1.21-deps](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.21-deps) — 2026-11-12

### 📦 Bumped Dependencies (3 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `errno` | `0.3.14` | **`0.3.15`** | Lockfile (`Cargo.lock`) |
| `rustls-pki-types` | `1.15.1` | **`1.16.0`** | Lockfile (`Cargo.lock`) |
| `webpki-roots` | `1.0.11` | **`1.0.12`** | Lockfile (`Cargo.lock`) |

## [v0.1.21](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.21) — 2026-11-10

### Highlights
- Opt-in KWin unfocused hover polling, 30s uncached watchdog with Script.stop hygiene, and 2.73 GB heap leak + 57-thread fix

## [v0.1.20](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.20) — 2026-11-10

### Highlights
- Security &amp; architecture hardening: loopback-bound ESP32-HA server with token auth, HMAC-SHA256 host command signing for App Launch, SHA-256 update verification, REST API DNS rebinding/CORS defense, and 0600 atomic secret writes.

### Commits
- chore(deps): update dependencies (itoa 1.0.19, syn 3.0.10, uuid 1.29.2) (a2d29ab)
- chore(deps): update dependencies (smallvec 1.16.5, uuid 1.29.1) (bf26b66)

### 📦 Bumped Dependencies (4 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `itoa` | `1.0.18` | **`1.0.19`** | Lockfile (`Cargo.lock`) |
| `smallvec` | `1.16.4` | **`1.16.5`** | Lockfile (`Cargo.lock`) |
| `syn` | `3.0.9` | **`3.0.10`** | Lockfile (`Cargo.lock`) |
| `uuid` | `1.29.0` | **`1.29.2`** | Lockfile (`Cargo.lock`) |

## [v0.1.19-deps](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.19-deps) — 2026-11-09

### 📦 Bumped Dependencies (3 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `itoa` | `1.0.18` | **`1.0.19`** | Lockfile (`Cargo.lock`) |
| `syn` | `3.0.9` | **`3.0.10`** | Lockfile (`Cargo.lock`) |
| `uuid` | `1.29.1` | **`1.29.2`** | Lockfile (`Cargo.lock`) |

## [v0.1.19](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.19) — 2026-11-06

### Highlights
- Release v0.1.19 of duckydeck-studio.

### Commits
- fix(updater,flatpak): stage Flatpak self-updates and user guide in host-shared data dir instead of sandbox-private /tmp (cbca0a9)

## [v0.1.18](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.18) — 2026-11-06

### Highlights
- Release v0.1.18 of duckydeck-studio.

### Commits
- fix(flatpak,docs): remove unused Flatpak permissions and document sandbox permission justifications (ab3a33b)
- chore(deps): update dependencies (cc 1.11.1, toml_datetime 1.1.3+spec-1.1.0, toml_edit 0.25.19+spec-1.1.0) (37a7f67)
- chore(deps): update dependencies (winnow 1.0.5) (32edec1)
- chore(deps): update dependencies (toml_edit 0.25.18+spec-1.1.0) (0f74772)

### 📦 Bumped Dependencies (4 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `cc` | `1.11.0` | **`1.11.1`** | Lockfile (`Cargo.lock`) |
| `toml_datetime` | `1.1.2+spec-1.1.0` | **`1.1.3+spec-1.1.0`** | Lockfile (`Cargo.lock`) |
| `toml_edit` | `0.25.17+spec-1.1.0` | **`0.25.19+spec-1.1.0`** | Lockfile (`Cargo.lock`) |
| `winnow` | `1.0.4` | **`1.0.5`** | Lockfile (`Cargo.lock`) |

## [v0.1.17-deps](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.17-deps) — 2026-11-06

### 📦 Bumped Dependencies (3 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `cc` | `1.11.0` | **`1.11.1`** | Lockfile (`Cargo.lock`) |
| `toml_datetime` | `1.1.2+spec-1.1.0` | **`1.1.3+spec-1.1.0`** | Lockfile (`Cargo.lock`) |
| `toml_edit` | `0.25.18+spec-1.1.0` | **`0.25.19+spec-1.1.0`** | Lockfile (`Cargo.lock`) |

## [v0.1.17](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.17) — 2026-11-03

### Highlights
- Update dependencies: cc 1.11.0

### 📦 Bumped Dependencies (1 package)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `cc` | `1.10.0` | **`1.11.0`** | Lockfile (`Cargo.lock`) |

## [v0.1.16](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.16) — 2026-11-01

### Highlights
- Self-healing KWin 6 Wayland auto-switcher watchdog, app/window-title-scoped Main-Keyboard Modifier Layer rules, in-app interactive Tutorials & Guides, Desktop OLED Widget sync stability, and main window screen/position persistence.

### Commits
- fix(autoswitch,modifier,ui,docs): self-healing KWin 6 auto-switcher watchdog, app/title-scoped modifier rules, interactive tutorials, and OLED widget sync stability (781109f)

## [v0.1.15](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.15) — 2026-10-31

### Highlights
- Release v0.1.15 of duckydeck-studio.

### Commits
- feat(autoswitch,modifier): add multi-desktop Linux auto-switcher, window picker, and held-modifier bleed suppression (Omarchy/Hyprland, COSMIC, GNOME, Cinnamon/X11, Windows) (2275a51)

## [v0.1.14](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.14) — 2026-10-31

### Highlights
- Add Main-Keyboard Modifier Layer Shift (Hold, One-Shot, Double-Tap + Hold, Toggle) with 200 ms Chord Guard, Desktop OLED Widget Full App button, and Auto-Switcher/SD Sync UI enhancements

### Commits
- feat(autoswitch,ui,docs): add Main-Keyboard Modifier Layer Shift, widget Full App button, and profile/sync UI fixes (bad3db8)
- chore(deps): update dependencies (js-sys 0.3.107, log 0.4.35, rustix 1.1.6, unicode-width 0.2.3, utf8_iter 1.1.0, wasm-bindgen 0.2.130 (+6 more)) (b57143d)

### 📦 Bumped Dependencies (12 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `js-sys` | `0.3.106` | **`0.3.107`** | Lockfile (`Cargo.lock`) |
| `log` | `0.4.34` | **`0.4.35`** | Lockfile (`Cargo.lock`) |
| `rustix` | `1.1.5` | **`1.1.6`** | Lockfile (`Cargo.lock`) |
| `unicode-width` | `0.2.2` | **`0.2.3`** | Lockfile (`Cargo.lock`) |
| `utf8_iter` | `1.0.4` | **`1.1.0`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-futures` | `0.4.79` | **`0.4.80`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-macro` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-macro-support` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-shared` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `web-sys` | `0.3.106` | **`0.3.107`** | Lockfile (`Cargo.lock`) |
| `zcheapstr` | `1.1.0` | **`1.2.0`** | Lockfile (`Cargo.lock`) |

## [v0.1.13-deps](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.13-deps) — 2026-10-31

### 📦 Bumped Dependencies (12 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `js-sys` | `0.3.106` | **`0.3.107`** | Lockfile (`Cargo.lock`) |
| `log` | `0.4.34` | **`0.4.35`** | Lockfile (`Cargo.lock`) |
| `rustix` | `1.1.5` | **`1.1.6`** | Lockfile (`Cargo.lock`) |
| `unicode-width` | `0.2.2` | **`0.2.3`** | Lockfile (`Cargo.lock`) |
| `utf8_iter` | `1.0.4` | **`1.1.0`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-futures` | `0.4.79` | **`0.4.80`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-macro` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-macro-support` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `wasm-bindgen-shared` | `0.2.129` | **`0.2.130`** | Lockfile (`Cargo.lock`) |
| `web-sys` | `0.3.106` | **`0.3.107`** | Lockfile (`Cargo.lock`) |
| `zcheapstr` | `1.1.0` | **`1.2.0`** | Lockfile (`Cargo.lock`) |

## [v0.1.13](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.13) — 2026-10-30

### Highlights
- Fix 2020 duckyPad SD card auto-mounting & profile folder naming, release DDS20_FW v1.1.3 firmware, and auto-close Key Editor on sync

### Commits
- fix(sd,firmware,ui): auto-mount 2020 duckyPad SD cards, release DDS20_FW v1.1.3, and close Key Editor on sync (f2fbe92)

## [v0.1.12](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.12) — 2026-10-26

### Changes
- fix(compiler,ide): support USE_STDLIB / ABS() in DSVM v2 and fix IDE script scrolling (#2) (ec495d0)
- chore(deps): update dependencies (cc 1.10.0, smallvec 1.16.4, uuid 1.29.0, webpki-roots 1.0.11) (a9cca5c)

### 📦 Bumped Dependencies (4 packages)

| Package | Previous | Updated | Scope |
| :--- | :--- | :--- | :--- |
| `cc` | `1.9.0` | **`1.10.0`** | Lockfile (`Cargo.lock`) |
| `smallvec` | `1.16.3` | **`1.16.4`** | Lockfile (`Cargo.lock`) |
| `uuid` | `1.28.0` | **`1.29.0`** | Lockfile (`Cargo.lock`) |
| `webpki-roots` | `1.0.10` | **`1.0.11`** | Lockfile (`Cargo.lock`) |

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
