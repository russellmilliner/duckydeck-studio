# <img src="data/icons/io.github.duckydeck.Studio.svg" width="36" height="36" align="top" alt="DuckyDeck Studio Icon" /> DuckyDeck Studio

[![Version v0.1.7](https://img.shields.io/badge/Version-v0.1.7-38bdf8?style=for-the-badge)](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7)
[![Windows 10/11 Setup v0.1.7](https://img.shields.io/badge/Windows%2010%2F11%20Setup-v0.1.7%20(.exe)-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe)
[![Windows Portable v0.1.7](https://img.shields.io/badge/Windows%20Portable-v0.1.7%20(.zip)-0284c7?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/duckydeck-studio-v0.1.7-windows-x86_64-portable.zip)
[![Flatpak v0.1.7](https://img.shields.io/badge/Linux%20Flatpak-v0.1.7-4a90d9?style=for-the-badge&logo=flatpak&logoColor=white)](https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-x86_64.flatpak)
[![Firmware v1.1.2](https://img.shields.io/badge/Custom%20Firmware-v1.1.2-10b981?style=for-the-badge)](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7)
[![Website & Documentation](https://img.shields.io/badge/Docs%20%26%20Website-duckydeckstudio.com-0ea5e9?style=for-the-badge&logo=googlechrome&logoColor=white)](https://duckydeckstudio.com)
[![Architecture Guide](https://img.shields.io/badge/Architecture-architecture.html-8b5cf6?style=for-the-badge)](https://duckydeckstudio.com/architecture.html)
[![GitHub Repository](https://img.shields.io/badge/GitHub-russellmilliner%2Fduckydeck--studio-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/russellmilliner/duckydeck-studio)
[![Ko-fi](https://img.shields.io/badge/Ko--fi-Buy%20Me%20a%20Coffee-ff5e5b?logo=ko-fi&logoColor=white&style=for-the-badge)](https://ko-fi.com/russellmilliner)
[![duckyPad Official Site](https://img.shields.io/badge/duckyPad-Official%20Site-f59e0b?style=for-the-badge)](https://www.duckypad.com)
[![Tindie Store](https://img.shields.io/badge/Tindie-dekuNukem%20Store-10b981?style=for-the-badge)](https://www.tindie.com/stores/dekunukem/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Linux & Windows 10/11](https://img.shields.io/badge/Platform-Linux%20(KDE%20Plasma%206)%20%7C%20Windows%2010%2F11-3daee9?logo=kde&logoColor=white&style=for-the-badge)](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7)

**DuckyDeck Studio (`v0.1.7`)** is a native **Linux (KDE Plasma 6 / Wayland & X11)** and **Windows 10/11 (x86_64)** desktop configurator, **DSVM v2 bytecode compiler**, **Window-Aware Auto-Switcher**, and **Macro Template Generator (experimental)** for the **duckyPad Pro (`0x0483:0xd11d`)** and **duckyPad 2020 (`0x0483:0xd11c`)**, built with **Rust**, **CXX-Qt 0.7**, **Qt 6**, and **KDE Frameworks 6 (Kirigami + KSyntaxHighlighting)** (with a self-contained Kirigami QML compatibility layer on Windows).

> 🏷️ **Current Version:** **DuckyDeck Studio `v0.1.7`** · **Custom Firmware `v1.1.2` (`DDS_FW_1.1.2_0f85dab1.bin` for duckyPad Pro & `DDS20_FW_1.1.2_3bd4beef.dfu` for duckyPad 2020)** · **KDE Plasma 6 Applet `v1.0.2`**
>
> 🌐 **Official Website & Online Documentation:** **[https://duckydeckstudio.com](https://duckydeckstudio.com)** · **[Architecture Reference (`architecture.html`)](https://duckydeckstudio.com/architecture.html)**
>
> 💻 **GitHub Repository & Releases:** **[https://github.com/russellmilliner/duckydeck-studio](https://github.com/russellmilliner/duckydeck-studio)** · **[Windows 10/11 Setup Installer (`v0.1.7` `.exe`)](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe)** · **[Windows Portable ZIP (`v0.1.7` `.zip`)](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/duckydeck-studio-v0.1.7-windows-x86_64-portable.zip)** · **[Linux Flatpak Bundle (`v0.1.7`)](https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-x86_64.flatpak)** · **[Linux x86_64 Tarball (`v0.1.7`)](https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-linux-x86_64.tar.gz)**
>
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
  - **Header-Footer Split Keycap Layout:** Each keycap features a dedicated top header with its compact key number (`1..20`) next to a pill-shaped RGB LED color bar, a centered 2-line OLED label preview with automatic icon scaling, and a bottom footer strip displaying `APP`/`API` host badges and `↻`/`⏹` execution indicators with zero text overlap up to `115%+` UI zoom.
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
  - Search installed desktop applications on **Linux** (`.desktop` entries from system, user, and Flatpak directories) and **Windows 10/11** (Start Menu `.lnk` shortcuts from `%ProgramData%\Microsoft\Windows\Start Menu\Programs` and `%APPDATA%\Microsoft\Windows\Start Menu\Programs` resolved via COM `WScript.Shell`), or configure custom CLI commands, `.exe` paths, arguments, working directories, and terminal execution.
  - Triggers instantaneously over USB HID via Persistent Global Variable `_GV31` (`encode_pgv31_trigger`, no keyboard shortcut conflicts!) or via `F13–F24` global hotkeys (Linux `evdev` or Windows Win32 `RegisterHotKey`).
  - **204 Built-in OLED Key Icons (Color UI + 1-Bit Monochrome Bitmaps) Across 14 Categories:** Includes curated color UI symbols and matching `16×16` / `10×10` 1-bit monochrome pixel-art OLED bitmaps across `Web Browsers`, `Word & Docs`, `Excel & Sheets`, `PowerPoint & Slides`, `Photoshop & Creative`, `FreeCAD & Fusion 360`, `Editing & Clipboard`, `Arrows & Nav`, `Media & Audio`, `Coding & Terminal`, `Home Automation`, `Window & System`, `Streaming & Apps`, and `Symbols & Shapes` (see **[`docs/OLED_ICONS.md`](docs/OLED_ICONS.md)**).
- **🌐 Web API / Webhook Builder & 🏠 Home Assistant Entity Search:**
  - Trigger HTTP `GET`, `POST`, `PUT`, `PATCH`, and `DELETE` requests (Home Assistant, Miniflux, OBS, custom webhooks) with custom headers and JSON bodies directly from any duckyPad key via USB HID (`_GV31`) or `F13–F24`.
  - **Built-in Home Assistant Web API Browser:** Connect to your Home Assistant instance (`base_url` + Long-Lived Access Token) to search devices and entities across domains (`light`, `switch`, `scene`, `script`, `automation`, `button`, `input_boolean`, `media_player`, `cover`, `fan`, `climate`, `lock`) and bind `toggle` / `turn_on` / `turn_off` / `trigger` service calls to any key in one click.
  - **🔔 Profile Change Webhooks:** Automatically fire a per-profile HTTP webhook (configured in **Profile Properties**) and/or a global profile-change webhook (configured in **Settings**) whenever the active profile changes via UI, hardware buttons, Window Auto-Switcher, or the REST API, with template placeholders (`{{profile_number}}`, `{{profile_name}}`, `{{previous_profile_number}}`, `{{previous_profile_name}}`, `{{source}}`, `{{device_serial}}`, `{{timestamp}}`).
- **🔌 Built-In HTTP REST API Server (`/api/v1/*`) for Remote Control & Programming:**
  - Optional embedded HTTP/1.1 REST API server configurable in **Settings → HTTP REST API Server** (defaults to `127.0.0.1:28491`, disabled by default, with 160-bit `ddk_...` Bearer token authentication) for controlling and programming every function of the duckyPad remotely (switching/creating/editing/deleting profiles, programming key labels/colors/icons/scripts, reading/writing `_GV0.._GV31`, setting LEDs/OLED, triggering slots, and syncing to microSD).
- **🪟 Cross-Platform Window Auto-Switcher (KDE Plasma 6 Wayland & Windows 10/11 Win32) & Protected Fallback Profile:**
  - **Linux (`zbus` + KWin Wayland):** Pure-Rust D-Bus listener integrated with KDE Plasma 6 KWin Wayland to automatically switch duckyPad hardware profiles when you focus different applications or browser tabs.
  - **Windows 10/11 (Win32 Foreground Tracker):** Native Win32 listener (`GetForegroundWindow`, `GetWindowTextW`, `GetWindowThreadProcessId`, `QueryFullProcessImageNameW`) that tracks active `.exe` process names and window titles (including regex title matching) with zero CPU overhead.
  - **Automatic Rule & Fallback Synchronization:** New profile groups always create a Default Fallback Profile (`★ FALLBACK`) that is protected from deletion. Reordering profiles automatically updates all Auto-Switcher rule bindings, and deleting a profile automatically reassigns any rules referencing it to the Fallback Profile.
  - Includes a **🎯 Crosshair Window Picker** on both Linux (KWin interactive picker) and Windows (`WindowFromPoint` + `GetAsyncKeyState(VK_LBUTTON)` hover-and-click picker) to bind any open window or web app tab in one click, inline `✕` filter clear buttons, hover tooltips for drag-and-drop reordering, and automatic pausing while the Key Editor is open.
- **💾 Offline Profile Cache, Diff Summary & Validated microSD Sync:**
  - Edits are saved immediately to a local per-device cache (`~/.local/share/duckydeck-studio/profile_cache/` on Linux or `%LOCALAPPDATA%\duckydeck-studio\profile_cache\` on Windows).
  - Inspect a detailed **Unsynced Changes Summary** before syncing, then write, flush, and verify `.dsb` bytecode and `config.txt` files on the duckyPad microSD card (supporting Linux `udisksctl` / `/run/media` and Windows `D:\..Z:\` removable volume detection with `FlushFileBuffers` and safe PowerShell drive eject).
  - **Portable `duckydeck_rules.json` microSD Persistence:** Syncs your window auto-switcher rules directly onto the microSD card root alongside hardware profiles so your window bindings travel with your duckyPad across computers.
  - Export and import individual profiles (`.duckyprofile.json` or folder) or complete **Profile Groups (`.duckygroup.json`)**.
- **🧙‍♂️ First-Run Deck Setup & Legacy dekuNukem Migration:**
  - Automatically detects unconfigured or fresh decks and discovers legacy profiles and auto-switcher rules from existing dekuNukem installations (`~/.local/share/duckypad_config/profile_backups/` on Linux or `%APPDATA%\dekuNukem\duckypad_config\profile_backups\` on Windows).
  - Detects running legacy background daemons (`duckypad_autoprofile.py`, `duckypad_autoswitcher`, `duckypad_configurator`) and provides 1-click graceful termination to prevent USB HID bus contention.
- **🔢 Live PGV Inspector (`_GV0`..`_GV31`) & ✨ Macro Template Generator (experimental):**
  - Read and write all 32 non-volatile 32-bit Persistent Global Variables (`_GV0.._GV31`) over USB HID (`0x18`/`0x19`) in Signed, Unsigned, or Hex format.
  - Generate duckyScript 3 macro templates from your local KDE shortcuts (`~/.config/kglobalshortcutsrc`), converting KDE key sequences directly into duckyScript 3 key combos.
- **📋 System Event Log, Global UI Zoom (`Ctrl++`/`Ctrl+-`), High-Contrast Controls & System Tray:**
  - Built-in **System Event Log** (`DEVICE`, `PROFILE`, `AUTOSWITCH`, `SD_SYNC`, `SCRIPT`, `WEB_API`, `APP_LAUNCH`, `SYSTEM`) with search, category/severity filters, and file export.
  - Global UI text zoom (`75%–160%`, minimum `10pt` base font) with standard <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>=</kbd>, <kbd>Ctrl</kbd>+<kbd>-</kbd>, and <kbd>Ctrl</kbd>+<kbd>0</kbd> shortcuts persisted across restarts, high-contrast checkboxes and scrollbars in Dark and Light modes, and a dynamic System Tray icon (KDE `StatusNotifierItem` on Linux, Notification Area Tray on Windows) with quick access to **Show / Hide**, **Toggle Desktop OLED Widget**, **Auto-Switcher**, **Autostart** (XDG `.desktop` on Linux, Registry `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` on Windows), and **About DuckyDeck Studio**.
- **🗂️ Reusable Profile Templates:**
  - Create new profiles from built-in templates (*Blank*, *Media & Streaming*, *IDE & Developer*, *Desktop App & Web API Launcher*) or copy any existing profile into a reusable custom template (`~/.config/duckydeck-studio/profile_templates.json` on Linux or `%APPDATA%\duckydeck-studio\profile_templates.json` on Windows).
- **🖥️ Desktop OLED Companion Widget, Multi-Device Simultaneous Support & 4-Way Rotated Key Labels (`0° / 90° / 180° / 270°`):**
  - Floating, multi-monitor-aware desktop replica of the duckyPad OLED screen (using KWin Wayland placement rules on Linux and Win32 `HWND_TOPMOST` on Windows, plus a companion KDE Plasma 6 Applet in `plasmoid/io.github.duckydeck.oled`) with persistent screen/position/transparency settings, optional 24-character extended labels (`wzN`/`wxN`), and independent **Desktop Widget Orientation** (`Follow OLED`, `0°`, `90°`, `180°`, `270°`) configurable per profile in **Profile Properties** that rotates both the key grid positions and the text inside each key immediately without requiring a microSD card sync.
  - **Simultaneous Multi-Device Management (`duckyPad Pro` + `duckyPad 2020`):** Attach multiple duckyPad Pro (`0x0483:0xd11d`) and/or duckyPad 2020 (`0x0483:0xd11c`) units simultaneously with isolated per-device profile caches (`~/.local/share/duckydeck-studio/device_profiles/<SERIAL>/`), independent per-device Auto-Switcher rules, and dedicated per-device floating Desktop OLED Widgets.
  - **High-Resolution Wheel & Ploopy Knob (`v120`) Smooth Scrolling:** Global C++ `QWheelEvent` filter with fractional sub-notch accumulation and 180 ms gesture target lock so high-resolution rotary encoders (such as the Ploopy Knob) and standard mouse wheels scroll every list and pane smoothly without pausing when child controls pass under the cursor.
- **🔧 DuckyDeck Studio Firmware `v1.1.2` (`duckyPad Pro` & `duckyPad 2020`) & Official Firmware Catalog Downloader:**
  - Custom firmware builds for both **`duckyPad Pro` (`DDS_FW_1.1.2_0f85dab1.bin`)** and **`duckyPad 2020` (`DDS20_FW_1.1.2_3bd4beef.dfu`)**, badged with the DuckyDeck Studio icon in the status bar, supporting 204 1-bit monochrome pixel-art OLED icons, 4 customizable OLED layout styles (on Pro), smooth `SWC_FADE_TO` LED transitions, dirty-page display tracking, and zero-SD-wear profile caching (NVS on Pro, RTC backup registers on 2020).
  - **Official Firmware Catalog Downloader:** Fetch the list of official dekuNukem firmwares directly inside the **Firmware Manager** window, download `.bin`/`.dfu` images with IEEE CRC32 verification, and install them in one click.

---

## ⚠️ Hardware Portability vs. Host-Dependent Capabilities

DuckyDeck Studio clearly badges capabilities in the UI as either **✅ Hardware-Portable** (compiled to the microSD card and executed autonomously by the duckyPad on any computer) or **💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable)**:

| Capability / Feature | Portability | How It Works & What Happens Without DuckyDeck Studio |
| :--- | :--- | :--- |
| **Standard DuckyScript 3 Macros** (`STRING`, `KEYDOWN`, `DELAY`, `MOUSE_MOVE`, `LMOUSE`, loops, `IF`/`WHILE`) | ✅ **Hardware-Portable** | Compiles to `.dsb` bytecode on the microSD card. Works on **any PC/OS** with zero host software installed. |
| **Static Profile & Key RGB Colors** (`BG_COLOR`, `SWCOLOR`, `KEYDOWN_COLOR`, `DIM_UNUSED_KEYS`) | ✅ **Hardware-Portable** | Stored in `config.txt` on the microSD card and rendered directly by duckyPad hardware. |
| **Embedded Key Script RGB Animations** (`SWC_SET`, `SWC_FILL`, `SWC_RESET`) | ✅ **Hardware-Portable** | Compiled directly into `keyN.dsb` bytecode on the microSD card; runs standalone on hardware. |
| **5-Char Physical OLED Key Labels (`zN` / `xN`) & `OLED_*` Script Graphics** | ✅ **Hardware-Portable** | Stored on microSD and rendered by the physical 128×128 OLED display. |
| **Physical OLED Orientation (`0° / 90° / 180° / 270°`) & Monochrome Icons** | ✅ **Hardware-Portable** *(180°/270° & icons require DuckyDeck Studio Firmware v1.1.2)* | Stored in `config.txt` (`IS_LANDSCAPE` / `OLED_ROTATION`) and rendered on the hardware OLED screen. |
| **Manual Profile Switching** (`GOTO_PROFILE`, `NEXT_PROFILE`, `PREV_PROFILE`, `+`/`-` button hold) | ✅ **Hardware-Portable** | Handled natively on the duckyPad hardware. |
| **🚀 Launch Desktop Application (`APP` badge)** | 💻 **Requires DuckyDeck Studio Running** | Triggered over USB HID (`_GV31`) or `F13–F24` (Linux `evdev` / Windows Win32 `RegisterHotKey`) by the DuckyDeck Studio host process. **Will not launch apps if DuckyDeck Studio is closed or on another PC.** |
| **🌐 Web API / HTTP Webhooks & Home Assistant Triggers (`API` badge)** | 💻 **Requires DuckyDeck Studio Running** | Outbound HTTP requests are dispatched by DuckyDeck Studio on the host PC. **The duckyPad hardware has no network stack and cannot fire webhooks standalone.** |
| **🔔 Profile Change Webhooks (Per-Profile & Global)** | 💻 **Requires DuckyDeck Studio Running** | Fired by DuckyDeck Studio when a profile switch is initiated or detected on the host PC. Does not require a microSD card sync. |
| **🔌 HTTP REST API Server (`/api/v1/*`)** | 💻 **Requires DuckyDeck Studio Running** | Hosted by the DuckyDeck Studio process (`127.0.0.1:28491` by default). |
| **🪟 Active-Window Auto-Switcher (KWin Wayland on Linux / Win32 on Windows)** | 💻 **Requires DuckyDeck Studio Running** | Uses KDE Plasma 6 KWin D-Bus tracking on Linux or Win32 `GetForegroundWindow` tracking on Windows to send USB HID profile-switch commands. Without the host app, switch profiles manually on the keypad. |
| **🖥️ Desktop OLED Widget, 24-Char Extended Widget Labels (`wzN`/`wxN`) & Widget Orientation (`DW_ROTATION`)** | 💻 **Requires DuckyDeck Studio Running** | Displayed on your computer monitor by DuckyDeck Studio; orientation changes apply immediately without requiring a microSD card sync. |
| **✨ Live USB HID RGB Animation Streaming** | 💻 **Requires DuckyDeck Studio Running** | Streams RGB frames over USB HID (`0x14`) from DuckyDeck Studio. Use the *"Embed standalone DuckyScript 3 SWC_SET"* option if you want portable on-device key lighting. |

---

## 🔧 DuckyDeck Studio Firmware (`v1.1.2`) vs. Standard Firmware (`3.3.5` / `3.0.4`)

**DuckyDeck Studio Firmware `v1.1.2`** is an optional, 100% backwards-compatible custom firmware suite for both **`duckyPad Pro` (`DDS_FW_1.1.2_0f85dab1.bin`)** and **`duckyPad (2020)` (`DDS20_FW_1.1.2_3bd4beef.dfu`)**. All existing `.dsb` bytecode scripts, profiles, and official features work identically to upstream firmware `3.3.5` (Pro) and `3.0.4` (2020 Evo), while unlocking hardware enhancements tailored to each platform:

- **📦 Download from GitHub Releases:**
  - **duckyPad Pro (`ESP32-S3`):** [`DDS_FW_1.1.2_0f85dab1.bin` (`v1.1.2`)](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7) · IEEE CRC32 `0f85dab1` · Size `891,200 bytes` *(automatically staged as `DPP_FW_1.1.2_0f85dab1.bin` onto microSD for universal bootloader compatibility)*
  - **duckyPad 2020 (`STM32F072`):** [`DDS20_FW_1.1.2_3bd4beef.dfu` (`v1.1.2`)](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7) · IEEE CRC32 `3bd4beef` · Size `63,057 bytes` (`62,748 bytes` Flash used)
- **🌐 Direct Downloads:** [`https://duckydeckstudio.com/firmware/DDS_FW_1.1.2_0f85dab1.bin`](https://duckydeckstudio.com/firmware/DDS_FW_1.1.2_0f85dab1.bin) · [`https://duckydeckstudio.com/firmware/DDS20_FW_1.1.2_3bd4beef.dfu`](https://duckydeckstudio.com/firmware/DDS20_FW_1.1.2_3bd4beef.dfu)

| Feature / Capability | Standard Firmware (`3.3.5` Pro / `3.0.4` 2020) | DuckyDeck Studio Firmware (`1.1.2` Pro & 2020) |
| :--- | :--- | :--- |
| **Firmware Prefix & Identity** (`shared.c` / `hid_task.c`) | Uses `DPP_FW_*.bin` / `duckypad_v*.dfu`; standard `GET_INFO` (`0x00`) response. | ✅ Distinct **`DDS_FW_*.bin`** (Pro) and **`DDS20_FW_*.dfu`** (2020) builds; reports `'D','D','S'` capability signature at HID bytes `20..22` so DuckyDeck Studio automatically identifies and optimizes for it. |
| **Monochrome OLED Key Icons** (`oled_icons.c` / `ui_task.c`) | ❌ ASCII text only (`0x20..0x7E`); Unicode/emoji symbols are stripped or ignored on hardware. | ✅ **204 Native Pixel-Art Icons** across 14 categories:<br>• **duckyPad Pro**: Full-Cell `16×16`, Top-Line `10×10` + Caption, and Inline `7×10` / `6×8` glyphs.<br>• **duckyPad (2020)**: All 204 inline `6×10` / `7×10` monochrome icons packed into 2.4 KB of STM32F072 Flash. |
| **Customizable OLED Layout Styles (Pro)** (`ui_task.c`) | Single fixed header bar and grid style. | ✅ **4 Selectable OLED Layout Styles** (`OLED_LAYOUT_STYLE 0..3`): Top Bezel Header (`0`), Micro-Page Dots (`1`), Header/Footer Split (`2`), and 3px Recessed Divider Strip (`3`). |
| **OLED Display Refresh Speed** (`ssd1306.c`) | Full framebuffer transmitted over SPI (Pro) or 400 kHz I²C (2020) on every screen refresh. | ✅ **Dirty-Page Bitmask Tracking (`ssd1306_dirty_pages`)**: Only modified 128-byte display pages are transmitted over SPI DMA (Pro) or I²C (2020), cutting `OLED_PRINT` / key-update bus time by up to 8×. |
| **Smooth LED Color Fade** (`ds_vm.c` / `neopixel.c`) | ❌ Stepwise jumps only (`SWC_SET` / `SWCOLOR`); 2020 uses software `double` math in 50 Hz ISR. | ✅ **Hardware `SWC_FADE_TO` Opcode (`89`)**: Smooth color transitions (`SWC_FADE_TO <key> <r> <g> <b> <ms>`), plus Q8 fixed-point integer math on STM32F072 (10× faster ISR, saves 7.3 KB Flash). |
| **Active Profile Switching Speed & SD Longevity** (`profiles.c`) | ⚠️ Rewrites `/dpp_config.txt` on the FAT32 microSD card on every profile change (20–200 ms stall + SD wear). | ✅ **Zero-SD-Wear Profile Cache**: Uses internal Flash NVS (`dpp_cfg`) on **duckyPad Pro** and battery/bus-backed `RTC_BKP_DR2` on **duckyPad (2020)** for instantaneous profile switching. |
| **Software Jump-to-DFU Bootloader (2020)** (`main.c` / `hid_task.c`) | ❌ Requires physically flipping DIP switch #1 on the back of the case to enter DFU mode. | ✅ **Software Jump-to-DFU (`RTC_BKP_DR3` magic)**: Enters the STM32F072 ROM DFU bootloader (`0483:df11`) directly over USB HID (`0x14`, mode `2`) with zero switch flipping. |
| **UTF-8 Key Label Decoding** (`profiles.h` / `profiles.c`) | ❌ `KEYNAME_SIZE = 8` bytes with raw byte truncation (`str[5] = 0`), slicing multi-byte UTF-8 characters. | ✅ **Expanded `KEYNAME_SIZE = 24` bytes** with glyph-aware UTF-8 truncation (`oled_utf8_truncate_glyphs`). Reads `ui_z<N>` / `ui_x<N>` directives for icons while keeping `z<N>` / `x<N>` ASCII fallbacks. |
| **Physical OLED Screen Orientations (Pro)** (`ssd1306.c` / `ui_task.c`) | ⚠️ 2 Orientations: `0°` Portrait and `90°` Clockwise. | ✅ **Full 4-Way Hardware Rotation (`0°`, `90°`, `180°`, `270°`)**: Adds `180°` Inverted Portrait and `270°` Counter-Clockwise Landscape with proper key grid remapping. |

---

## 🖼️ OLED Icon Library (Color UI & 1-Bit Monochrome Hardware Bitmaps)

**DuckyDeck Studio (`v0.1.7`)** and **DuckyDeck Studio Firmware (`DDS_FW v1.1.2` & `DDS20_FW v1.1.2`)** provide **204 unique icons** (199 application & macro icons across 14 categories in the in-app **OLED Button Icon Picker**, plus 5 keyboard modifier glyphs) with **zero duplicate icons**.

Each icon is engineered with **two synchronized visual representations**:
1. **Color Desktop UI Version**: Full-color Unicode/Emoji symbol displayed on the interactive Hardware Canvas, Floating OLED Desktop Widget, and Key Inspector.
2. **1-Bit Monochrome Hardware OLED Version (`16×16` & `10×10`)**: Hand-crafted pixel-art bitmaps compiled into `oled_icons.c` (`firmware/generate_oled_icons.py`) for the physical `128×128` SSD1306 monochrome OLED screen:
   - **`16×16` Full-Cell Bitmap**: Rendered centered inside the key cell when **Line 1** is a single icon and **Line 2** is empty (*Big Icon Only* mode).
   - **`10×10` Top-Line Bitmap**: Rendered at the top of the key cell above a 4-character text label on **Line 2** (*Icon + Short Label* mode), or scaled to `7×10` / `6×8` when mixed inline with text.
   - **Stock Firmware (`DPP_FW` / `duckypad_v3.0.4`) Compatibility**: Writes a 4-character ASCII fallback (`z<N>`/`x<N>`) for stock `dekuNukem` firmware while storing the full UTF-8 icon in `ui_z<N>`/`ui_x<N>` for `DDS_FW v1.1.2` and `DDS20_FW v1.1.2`.

- 📄 **[Complete Standalone OLED Icon Reference (`docs/OLED_ICONS.md`)](docs/OLED_ICONS.md)**

![Complete OLED Icon Chart — Color & 1-Bit Monochrome](docs/assets/oled-icons-chart.svg)

<details>
<summary><strong>📋 Click to Expand Complete 204-Icon Table (Color UI vs. 16×16 &amp; 10×10 Monochrome OLED Bitmaps)</strong></summary>

### Web Browsers (16 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 🌐 | <img src="docs/assets/oled-icons/U+1F310.svg" width="32" height="32" alt="GLOBE_WEB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F310.svg" width="24" height="24" alt="GLOBE_WEB 10x10"> | **`WEB`** | `U+1F310` | `GLOBE_WEB` | `WEB` | Browser / Open Website / URL |
| 🧭 | <img src="docs/assets/oled-icons/U+1F9ED.svg" width="32" height="32" alt="COMPASS_ADDR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9ED.svg" width="24" height="24" alt="COMPASS_ADDR 10x10"> | **`ADDR`** | `U+1F9ED` | `COMPASS_ADDR` | `ADDR` | Address Bar / Navigate / Compass |
| 📑 | <img src="docs/assets/oled-icons/U+1F4D1.svg" width="32" height="32" alt="TAB_PLUS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4D1.svg" width="24" height="24" alt="TAB_PLUS 10x10"> | **`TAB+`** | `U+1F4D1` | `TAB_PLUS` | `TAB+` | New Tab / Duplicate Tab / Bookmark Tabs |
| 🗂 | <img src="docs/assets/oled-icons/U+1F5C2.svg" width="32" height="32" alt="TAB_GROUPS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5C2.svg" width="24" height="24" alt="TAB_GROUPS 10x10"> | **`TABS`** | `U+1F5C2` | `TAB_GROUPS` | `TABS` | Tab Groups / Tab Overview / Workspaces |
| 🔄 | <img src="docs/assets/oled-icons/U+1F504.svg" width="32" height="32" alt="RELOAD_ARROWS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F504.svg" width="24" height="24" alt="RELOAD_ARROWS 10x10"> | **`RELD`** | `U+1F504` | `RELOAD_ARROWS` | `RELD` | Reload / Hard Refresh Page |
| 🔖 | <img src="docs/assets/oled-icons/U+1F516.svg" width="32" height="32" alt="BOOKMARK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F516.svg" width="24" height="24" alt="BOOKMARK 10x10"> | **`BMRK`** | `U+1F516` | `BOOKMARK` | `BMRK` | Bookmark Page / Favorites |
| 🕵 | <img src="docs/assets/oled-icons/U+1F575.svg" width="32" height="32" alt="INCOGNITO 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F575.svg" width="24" height="24" alt="INCOGNITO 10x10"> | **`PRIV`** | `U+1F575` | `INCOGNITO` | `PRIV` | Incognito / Private Browsing Window |
| 📥 | <img src="docs/assets/oled-icons/U+1F4E5.svg" width="32" height="32" alt="DOWNLOAD_TRAY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4E5.svg" width="24" height="24" alt="DOWNLOAD_TRAY 10x10"> | **`DL`** | `U+1F4E5` | `DOWNLOAD_TRAY` | `DL` | Downloads / Save Page As |
| 📤 | <img src="docs/assets/oled-icons/U+1F4E4.svg" width="32" height="32" alt="SHARE_TRAY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4E4.svg" width="24" height="24" alt="SHARE_TRAY 10x10"> | **`SHAR`** | `U+1F4E4` | `SHARE_TRAY` | `SHAR` | Share / Upload / Send Link |
| 📜 | <img src="docs/assets/oled-icons/U+1F4DC.svg" width="32" height="32" alt="HISTORY_SCROLL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4DC.svg" width="24" height="24" alt="HISTORY_SCROLL 10x10"> | **`HIST`** | `U+1F4DC` | `HISTORY_SCROLL` | `HIST` | Browsing History / Recently Closed |
| 🧩 | <img src="docs/assets/oled-icons/U+1F9E9.svg" width="32" height="32" alt="EXTENSION_PUZZLE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9E9.svg" width="24" height="24" alt="EXTENSION_PUZZLE 10x10"> | **`EXT`** | `U+1F9E9` | `EXTENSION_PUZZLE` | `EXT` | Browser Extensions / Add-ons / Plugins |
| 🔎 | <img src="docs/assets/oled-icons/U+1F50E.svg" width="32" height="32" alt="INSPECT_MAG 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F50E.svg" width="24" height="24" alt="INSPECT_MAG 10x10"> | **`INSP`** | `U+1F50E` | `INSPECT_MAG` | `INSP` | DevTools Inspect / Find in Page |
| ⛶ | <img src="docs/assets/oled-icons/U+26F6.svg" width="32" height="32" alt="FULLSCREEN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+26F6.svg" width="24" height="24" alt="FULLSCREEN 10x10"> | **`FULL`** | `U+26F6` | `FULLSCREEN` | `FULL` | Fullscreen Mode (F11) |
| 🖨 | <img src="docs/assets/oled-icons/U+1F5A8.svg" width="32" height="32" alt="PRINTER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5A8.svg" width="24" height="24" alt="PRINTER 10x10"> | **`PRNT`** | `U+1F5A8` | `PRINTER` | `PRNT` | Print Page / Save as PDF |
| 🍪 | <img src="docs/assets/oled-icons/U+1F36A.svg" width="32" height="32" alt="COOKIE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F36A.svg" width="24" height="24" alt="COOKIE 10x10"> | **`COOK`** | `U+1F36A` | `COOKIE` | `COOK` | Clear Cookies / Site Data |
| 📰 | <img src="docs/assets/oled-icons/U+1F4F0.svg" width="32" height="32" alt="READER_NEWS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4F0.svg" width="24" height="24" alt="READER_NEWS 10x10"> | **`READ`** | `U+1F4F0` | `READER_NEWS` | `READ` | Reader View / Simplified Article |

### Word & Docs (18 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 📄 | <img src="docs/assets/oled-icons/U+1F4C4.svg" width="32" height="32" alt="NEW_DOC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4C4.svg" width="24" height="24" alt="NEW_DOC 10x10"> | **`NEW`** | `U+1F4C4` | `NEW_DOC` | `NEW` | New Document / Blank Page |
| 📝 | <img src="docs/assets/oled-icons/U+1F4DD.svg" width="32" height="32" alt="EDIT_DOC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4DD.svg" width="24" height="24" alt="EDIT_DOC 10x10"> | **`DOC`** | `U+1F4DD` | `EDIT_DOC` | `DOC` | Edit Document / Rich Text Note |
| 🔤 | <img src="docs/assets/oled-icons/U+1F524.svg" width="32" height="32" alt="FONT_ABC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F524.svg" width="24" height="24" alt="FONT_ABC 10x10"> | **`FONT`** | `U+1F524` | `FONT_ABC` | `FONT` | Font Family & Size Formatting |
| 🅱 | <img src="docs/assets/oled-icons/U+1F171.svg" width="32" height="32" alt="BOLD_B 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F171.svg" width="24" height="24" alt="BOLD_B 10x10"> | **`BOLD`** | `U+1F171` | `BOLD_B` | `BOLD` | Bold Text (Ctrl+B) |
| ℹ | <img src="docs/assets/oled-icons/U+2139.svg" width="32" height="32" alt="ITALIC_I 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2139.svg" width="24" height="24" alt="ITALIC_I 10x10"> | **`ITAL`** | `U+2139` | `ITALIC_I` | `ITAL` | Italic / Info Style (Ctrl+I) |
| ¶ | <img src="docs/assets/oled-icons/U+00B6.svg" width="32" height="32" alt="PARAGRAPH_PILCROW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+00B6.svg" width="24" height="24" alt="PARAGRAPH_PILCROW 10x10"> | **`PARA`** | `U+00B6` | `PARAGRAPH_PILCROW` | `PARA` | Paragraph / Show Formatting Marks |
| § | <img src="docs/assets/oled-icons/U+00A7.svg" width="32" height="32" alt="SECTION_SIGN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+00A7.svg" width="24" height="24" alt="SECTION_SIGN 10x10"> | **`SECT`** | `U+00A7` | `SECTION_SIGN` | `SECT` | Section Break / Page Layout |
| ☰ | <img src="docs/assets/oled-icons/U+2630.svg" width="32" height="32" alt="ALIGN_LINES 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2630.svg" width="24" height="24" alt="ALIGN_LINES 10x10"> | **`ALGN`** | `U+2630` | `ALIGN_LINES` | `ALGN` | Align Text / Justify Paragraph |
| • | <img src="docs/assets/oled-icons/U+2022.svg" width="32" height="32" alt="BULLET_LIST 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2022.svg" width="24" height="24" alt="BULLET_LIST 10x10"> | **`BULT`** | `U+2022` | `BULLET_LIST` | `BULT` | Bulleted List |
| 🔢 | <img src="docs/assets/oled-icons/U+1F522.svg" width="32" height="32" alt="NUMBER_LIST 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F522.svg" width="24" height="24" alt="NUMBER_LIST 10x10"> | **`NUMS`** | `U+1F522` | `NUMBER_LIST` | `NUMS` | Numbered / Multilevel List |
| 🔠 | <img src="docs/assets/oled-icons/U+1F520.svg" width="32" height="32" alt="UPPERCASE_AB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F520.svg" width="24" height="24" alt="UPPERCASE_AB 10x10"> | **`UPPR`** | `U+1F520` | `UPPERCASE_AB` | `UPPR` | Uppercase / Change Case |
| 🔡 | <img src="docs/assets/oled-icons/U+1F521.svg" width="32" height="32" alt="LOWERCASE_AB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F521.svg" width="24" height="24" alt="LOWERCASE_AB 10x10"> | **`LOWR`** | `U+1F521` | `LOWERCASE_AB` | `LOWR` | Lowercase / Small Caps |
| 💬 | <img src="docs/assets/oled-icons/U+1F4AC.svg" width="32" height="32" alt="CHAT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4AC.svg" width="24" height="24" alt="CHAT 10x10"> | **`CMNT`** | `U+1F4AC` | `CHAT` | `CMNT` | Insert Comment / Review Track Changes |
| 👁 | <img src="docs/assets/oled-icons/U+1F441.svg" width="32" height="32" alt="VIEW_EYE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F441.svg" width="24" height="24" alt="VIEW_EYE 10x10"> | **`VIEW`** | `U+1F441` | `VIEW_EYE` | `VIEW` | Read Mode / Print Preview |
| 📖 | <img src="docs/assets/oled-icons/U+1F4D6.svg" width="32" height="32" alt="DICTIONARY_BOOK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4D6.svg" width="24" height="24" alt="DICTIONARY_BOOK 10x10"> | **`DICT`** | `U+1F4D6` | `DICTIONARY_BOOK` | `DICT` | Spell Check / Thesaurus / Word Count |
| 🏷 | <img src="docs/assets/oled-icons/U+1F3F7.svg" width="32" height="32" alt="STYLE_TAG 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3F7.svg" width="24" height="24" alt="STYLE_TAG 10x10"> | **`STYL`** | `U+1F3F7` | `STYLE_TAG` | `STYL` | Heading / Quick Style Tag |
| 📎 | <img src="docs/assets/oled-icons/U+1F4CE.svg" width="32" height="32" alt="ATTACHMENT_CLIP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4CE.svg" width="24" height="24" alt="ATTACHMENT_CLIP 10x10"> | **`ATCH`** | `U+1F4CE` | `ATTACHMENT_CLIP` | `ATCH` | Insert Attachment / Embedded Object |
| ✒ | <img src="docs/assets/oled-icons/U+2712.svg" width="32" height="32" alt="SIGN_NIB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2712.svg" width="24" height="24" alt="SIGN_NIB 10x10"> | **`SIGN`** | `U+2712` | `SIGN_NIB` | `SIGN` | Insert Footnote / Signature / Nib |

### Excel & Sheets (15 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 📊 | <img src="docs/assets/oled-icons/U+1F4CA.svg" width="32" height="32" alt="CHART_BARS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4CA.svg" width="24" height="24" alt="CHART_BARS 10x10"> | **`CHRT`** | `U+1F4CA` | `CHART_BARS` | `CHRT` | Insert Bar / Column / Pivot Chart |
| 📈 | <img src="docs/assets/oled-icons/U+1F4C8.svg" width="32" height="32" alt="TREND_UP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4C8.svg" width="24" height="24" alt="TREND_UP 10x10"> | **`TRND`** | `U+1F4C8` | `TREND_UP` | `TRND` | Line Chart / Trendline / Sparkline |
| 📉 | <img src="docs/assets/oled-icons/U+1F4C9.svg" width="32" height="32" alt="TREND_DOWN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4C9.svg" width="24" height="24" alt="TREND_DOWN 10x10"> | **`VARI`** | `U+1F4C9` | `TREND_DOWN` | `VARI` | Variance / Drawdown Chart |
| ∑ | <img src="docs/assets/oled-icons/U+2211.svg" width="32" height="32" alt="SIGMA_SUM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2211.svg" width="24" height="24" alt="SIGMA_SUM 10x10"> | **`SUM`** | `U+2211` | `SIGMA_SUM` | `SUM` | AutoSum / Sigma Formula (Alt+=) |
| ƒ | <img src="docs/assets/oled-icons/U+0192.svg" width="32" height="32" alt="FUNCTION_FX 16x16"> | <img src="docs/assets/oled-icons-10x10/U+0192.svg" width="24" height="24" alt="FUNCTION_FX 10x10"> | **`FX`** | `U+0192` | `FUNCTION_FX` | `FX` | Insert Function / Formula Bar (fx) |
| 🧮 | <img src="docs/assets/oled-icons/U+1F9EE.svg" width="32" height="32" alt="CALC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9EE.svg" width="24" height="24" alt="CALC 10x10"> | **`CALC`** | `U+1F9EE` | `CALC` | `CALC` | Calculate Sheet / Pivot Table |
| ⊞ | <img src="docs/assets/oled-icons/U+229E.svg" width="32" height="32" alt="GRID_BORDERS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+229E.svg" width="24" height="24" alt="GRID_BORDERS 10x10"> | **`GRID`** | `U+229E` | `GRID_BORDERS` | `GRID` | Table Grid / All Cell Borders |
| ⊟ | <img src="docs/assets/oled-icons/U+229F.svg" width="32" height="32" alt="UNGROUP_MINUS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+229F.svg" width="24" height="24" alt="UNGROUP_MINUS 10x10"> | **`UNGR`** | `U+229F` | `UNGROUP_MINUS` | `UNGR` | Ungroup / Collapse Rows & Columns |
| ∇ | <img src="docs/assets/oled-icons/U+2207.svg" width="32" height="32" alt="FILTER_FUNNEL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2207.svg" width="24" height="24" alt="FILTER_FUNNEL 10x10"> | **`FILT`** | `U+2207` | `FILTER_FUNNEL` | `FILT` | Data Filter / AutoFilter Funnel |
| ⇅ | <img src="docs/assets/oled-icons/U+21C5.svg" width="32" height="32" alt="SWAP_UD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21C5.svg" width="24" height="24" alt="SWAP_UD 10x10"> | **`SORT`** | `U+21C5` | `SWAP_UD` | `SORT` | Sort Ascending / Descending |
| 💲 | <img src="docs/assets/oled-icons/U+1F4B2.svg" width="32" height="32" alt="CURRENCY_DOLLAR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4B2.svg" width="24" height="24" alt="CURRENCY_DOLLAR 10x10"> | **`CURR`** | `U+1F4B2` | `CURRENCY_DOLLAR` | `CURR` | Currency / Accounting Number Format |
| ％ | <img src="docs/assets/oled-icons/U+FF05.svg" width="32" height="32" alt="PERCENT_FMT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+FF05.svg" width="24" height="24" alt="PERCENT_FMT 10x10"> | **`PCT`** | `U+FF05` | `PERCENT_FMT` | `PCT` | Percentage Number Format |
| 🧊 | <img src="docs/assets/oled-icons/U+1F9CA.svg" width="32" height="32" alt="FREEZE_PANES 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9CA.svg" width="24" height="24" alt="FREEZE_PANES 10x10"> | **`FRZE`** | `U+1F9CA` | `FREEZE_PANES` | `FRZE` | Freeze Panes / Top Row |
| ⇔ | <img src="docs/assets/oled-icons/U+21D4.svg" width="32" height="32" alt="AUTOFIT_COL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21D4.svg" width="24" height="24" alt="AUTOFIT_COL 10x10"> | **`CFIT`** | `U+21D4` | `AUTOFIT_COL` | `CFIT` | AutoFit Column Width / Merge Across |
| ⇕ | <img src="docs/assets/oled-icons/U+21D5.svg" width="32" height="32" alt="AUTOFIT_ROW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21D5.svg" width="24" height="24" alt="AUTOFIT_ROW 10x10"> | **`RFIT`** | `U+21D5` | `AUTOFIT_ROW` | `RFIT` | AutoFit Row Height / Insert Row |

### PowerPoint & Slides (12 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 📽 | <img src="docs/assets/oled-icons/U+1F4FD.svg" width="32" height="32" alt="SLIDESHOW_PROJ 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4FD.svg" width="24" height="24" alt="SLIDESHOW_PROJ 10x10"> | **`SHOW`** | `U+1F4FD` | `SLIDESHOW_PROJ` | `SHOW` | Start Slide Show Presentation (F5) |
| 🖼 | <img src="docs/assets/oled-icons/U+1F5BC.svg" width="32" height="32" alt="NEW_SLIDE_PIC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5BC.svg" width="24" height="24" alt="NEW_SLIDE_PIC 10x10"> | **`SLID`** | `U+1F5BC` | `NEW_SLIDE_PIC` | `SLID` | New Slide / Insert Picture |
| 🎭 | <img src="docs/assets/oled-icons/U+1F3AD.svg" width="32" height="32" alt="ANIMATION_STARS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3AD.svg" width="24" height="24" alt="ANIMATION_STARS 10x10"> | **`ANIM`** | `U+1F3AD` | `ANIMATION_STARS` | `ANIM` | Animations & Slide Transitions |
| 🪄 | <img src="docs/assets/oled-icons/U+1FA84.svg" width="32" height="32" alt="DESIGN_WAND 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1FA84.svg" width="24" height="24" alt="DESIGN_WAND 10x10"> | **`DSGN`** | `U+1FA84` | `DESIGN_WAND` | `DSGN` | Design Ideas / Auto-Layout Magic Wand |
| 📐 | <img src="docs/assets/oled-icons/U+1F4D0.svg" width="32" height="32" alt="SLIDE_MASTER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4D0.svg" width="24" height="24" alt="SLIDE_MASTER 10x10"> | **`MSTR`** | `U+1F4D0` | `SLIDE_MASTER` | `MSTR` | Slide Master / Guides & Set Square |
| 🎙 | <img src="docs/assets/oled-icons/U+1F399.svg" width="32" height="32" alt="RECORD_NARRATION 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F399.svg" width="24" height="24" alt="RECORD_NARRATION 10x10"> | **`NARR`** | `U+1F399` | `RECORD_NARRATION` | `NARR` | Record Slide Narration / Voiceover |
| 👉 | <img src="docs/assets/oled-icons/U+1F449.svg" width="32" height="32" alt="LASER_POINTER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F449.svg" width="24" height="24" alt="LASER_POINTER 10x10"> | **`LASR`** | `U+1F449` | `LASER_POINTER` | `LASR` | Laser Pointer / Presenter Highlight |
| ▦ | <img src="docs/assets/oled-icons/U+25A6.svg" width="32" height="32" alt="SLIDE_SORTER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25A6.svg" width="24" height="24" alt="SLIDE_SORTER 10x10"> | **`SRTR`** | `U+25A6` | `SLIDE_SORTER` | `SRTR` | Slide Sorter Grid View |
| 🗒 | <img src="docs/assets/oled-icons/U+1F5D2.svg" width="32" height="32" alt="SPEAKER_NOTES 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5D2.svg" width="24" height="24" alt="SPEAKER_NOTES 10x10"> | **`NOTE`** | `U+1F5D2` | `SPEAKER_NOTES` | `NOTE` | Speaker Notes / Presenter View |
| ⧉ | <img src="docs/assets/oled-icons/U+29C9.svg" width="32" height="32" alt="DUPLICATE_SLIDE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+29C9.svg" width="24" height="24" alt="DUPLICATE_SLIDE 10x10"> | **`DUPL`** | `U+29C9` | `DUPLICATE_SLIDE` | `DUPL` | Duplicate Slide / Group Objects (Ctrl+D) |
| ⬛ | <img src="docs/assets/oled-icons/U+2B1B.svg" width="32" height="32" alt="BLACKOUT_SCREEN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2B1B.svg" width="24" height="24" alt="BLACKOUT_SCREEN 10x10"> | **`BLCK`** | `U+2B1B` | `BLACKOUT_SCREEN` | `BLCK` | Blackout Presentation Screen (B) |
| ⬜ | <img src="docs/assets/oled-icons/U+2B1C.svg" width="32" height="32" alt="WHITEOUT_SCREEN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2B1C.svg" width="24" height="24" alt="WHITEOUT_SCREEN 10x10"> | **`WHTE`** | `U+2B1C` | `WHITEOUT_SCREEN` | `WHTE` | Whiteout Presentation Screen (W) |

### Photoshop & Creative (16 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 🎨 | <img src="docs/assets/oled-icons/U+1F3A8.svg" width="32" height="32" alt="PALETTE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3A8.svg" width="24" height="24" alt="PALETTE 10x10"> | **`CLR`** | `U+1F3A8` | `PALETTE` | `CLR` | Color Picker / Swatches Palette |
| 🖌 | <img src="docs/assets/oled-icons/U+1F58C.svg" width="32" height="32" alt="BRUSH_TOOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F58C.svg" width="24" height="24" alt="BRUSH_TOOL 10x10"> | **`BRSH`** | `U+1F58C` | `BRUSH_TOOL` | `BRSH` | Brush Tool / Paint Stroke (B) |
| ✏ | <img src="docs/assets/oled-icons/U+270F.svg" width="32" height="32" alt="EDIT_PENCIL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+270F.svg" width="24" height="24" alt="EDIT_PENCIL 10x10"> | **`PENC`** | `U+270F` | `EDIT_PENCIL` | `PENC` | Pencil / Freehand Draw Tool |
| 🖍 | <img src="docs/assets/oled-icons/U+1F58D.svg" width="32" height="32" alt="FILL_BUCKET 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F58D.svg" width="24" height="24" alt="FILL_BUCKET 10x10"> | **`FILL`** | `U+1F58D` | `FILL_BUCKET` | `FILL` | Paint Bucket / Gradient Fill (G) |
| 🧽 | <img src="docs/assets/oled-icons/U+1F9FD.svg" width="32" height="32" alt="ERASER_TOOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9FD.svg" width="24" height="24" alt="ERASER_TOOL 10x10"> | **`ERAS`** | `U+1F9FD` | `ERASER_TOOL` | `ERAS` | Eraser Tool / Clear Pixels (E) |
| ◩ | <img src="docs/assets/oled-icons/U+25E9.svg" width="32" height="32" alt="CROP_TOOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25E9.svg" width="24" height="24" alt="CROP_TOOL 10x10"> | **`CROP`** | `U+25E9` | `CROP_TOOL` | `CROP` | Crop / Trim Canvas Tool (C) |
| ⬚ | <img src="docs/assets/oled-icons/U+2B1A.svg" width="32" height="32" alt="MARQUEE_SELECT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2B1A.svg" width="24" height="24" alt="MARQUEE_SELECT 10x10"> | **`MARQ`** | `U+2B1A` | `MARQUEE_SELECT` | `MARQ` | Rectangular Marquee Selection (M) |
| ➰ | <img src="docs/assets/oled-icons/U+27B0.svg" width="32" height="32" alt="LASSO_SELECT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+27B0.svg" width="24" height="24" alt="LASSO_SELECT 10x10"> | **`LASO`** | `U+27B0` | `LASSO_SELECT` | `LASO` | Lasso / Freeform Selection Tool (L) |
| 🧪 | <img src="docs/assets/oled-icons/U+1F9EA.svg" width="32" height="32" alt="FLASK_TEST 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9EA.svg" width="24" height="24" alt="FLASK_TEST 10x10"> | **`DROP`** | `U+1F9EA` | `FLASK_TEST` | `DROP` | Eyedropper / Color Sampler (I) |
| 🩹 | <img src="docs/assets/oled-icons/U+1FA79.svg" width="32" height="32" alt="HEALING_PATCH 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1FA79.svg" width="24" height="24" alt="HEALING_PATCH 10x10"> | **`HEAL`** | `U+1FA79` | `HEALING_PATCH` | `HEAL` | Spot Healing / Patch Brush (J) |
| 💠 | <img src="docs/assets/oled-icons/U+1F4A0.svg" width="32" height="32" alt="CLONE_STAMP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4A0.svg" width="24" height="24" alt="CLONE_STAMP 10x10"> | **`CLON`** | `U+1F4A0` | `CLONE_STAMP` | `CLON` | Clone Stamp / Pattern Stamp (S) |
| 📚 | <img src="docs/assets/oled-icons/U+1F4DA.svg" width="32" height="32" alt="LAYERS_STACK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4DA.svg" width="24" height="24" alt="LAYERS_STACK 10x10"> | **`LAYR`** | `U+1F4DA` | `LAYERS_STACK` | `LAYR` | Layers Panel / Merge Layers (F7) |
| ◐ | <img src="docs/assets/oled-icons/U+25D0.svg" width="32" height="32" alt="LAYER_MASK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25D0.svg" width="24" height="24" alt="LAYER_MASK 10x10"> | **`MASK`** | `U+25D0` | `LAYER_MASK` | `MASK` | Layer Mask / Contrast / Invert |
| 🔆 | <img src="docs/assets/oled-icons/U+1F506.svg" width="32" height="32" alt="EXPOSURE_LEVELS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F506.svg" width="24" height="24" alt="EXPOSURE_LEVELS 10x10"> | **`EXPO`** | `U+1F506` | `EXPOSURE_LEVELS` | `EXPO` | Exposure / Levels / Curves Adjustment |
| ✋ | <img src="docs/assets/oled-icons/U+270B.svg" width="32" height="32" alt="HAND_PAN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+270B.svg" width="24" height="24" alt="HAND_PAN 10x10"> | **`HAND`** | `U+270B` | `HAND_PAN` | `HAND` | Hand Tool / Pan Canvas (H) |
| 🅰 | <img src="docs/assets/oled-icons/U+1F170.svg" width="32" height="32" alt="TYPE_TEXT_T 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F170.svg" width="24" height="24" alt="TYPE_TEXT_T 10x10"> | **`TEXT`** | `U+1F170` | `TYPE_TEXT_T` | `TEXT` | Type / Artistic Text Tool (T) |

### FreeCAD & Fusion 360 (18 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ⬡ | <img src="docs/assets/oled-icons/U+2B21.svg" width="32" height="32" alt="SOLID_BODY_HEX 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2B21.svg" width="24" height="24" alt="SOLID_BODY_HEX 10x10"> | **`BODY`** | `U+2B21` | `SOLID_BODY_HEX` | `BODY` | 3D Solid Body / Part Design Workbench |
| ⬢ | <img src="docs/assets/oled-icons/U+2B22.svg" width="32" height="32" alt="MESH_BODY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2B22.svg" width="24" height="24" alt="MESH_BODY 10x10"> | **`MESH`** | `U+2B22` | `MESH_BODY` | `MESH` | STL Mesh / Surface Body / Tessellate |
| ✎ | <img src="docs/assets/oled-icons/U+270E.svg" width="32" height="32" alt="SKETCH_2D 16x16"> | <img src="docs/assets/oled-icons-10x10/U+270E.svg" width="24" height="24" alt="SKETCH_2D 10x10"> | **`SKCH`** | `U+270E` | `SKETCH_2D` | `SKCH` | Create 2D Sketch / Edit Sketch |
| 📏 | <img src="docs/assets/oled-icons/U+1F4CF.svg" width="32" height="32" alt="DIMENSION_RULER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4CF.svg" width="24" height="24" alt="DIMENSION_RULER 10x10"> | **`DIM`** | `U+1F4CF` | `DIMENSION_RULER` | `DIM` | Sketch Dimension / Measure Distance (D) |
| ⌖ | <img src="docs/assets/oled-icons/U+2316.svg" width="32" height="32" alt="ORIGIN_DATUM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2316.svg" width="24" height="24" alt="ORIGIN_DATUM 10x10"> | **`ORIG`** | `U+2316` | `ORIGIN_DATUM` | `ORIG` | Origin / Center Point / Datum Crosshair |
| ⏫ | <img src="docs/assets/oled-icons/U+23EB.svg" width="32" height="32" alt="EXTRUDE_PAD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23EB.svg" width="24" height="24" alt="EXTRUDE_PAD 10x10"> | **`EXTR`** | `U+23EB` | `EXTRUDE_PAD` | `EXTR` | Extrude / Pad Sketch Profile (E) |
| ⏬ | <img src="docs/assets/oled-icons/U+23EC.svg" width="32" height="32" alt="POCKET_CUT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23EC.svg" width="24" height="24" alt="POCKET_CUT 10x10"> | **`PKT`** | `U+23EC` | `POCKET_CUT` | `PKT` | Pocket / Cut / Hole Feature |
| ↻ | <img src="docs/assets/oled-icons/U+21BB.svg" width="32" height="32" alt="SYNC_CW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21BB.svg" width="24" height="24" alt="SYNC_CW 10x10"> | **`REVL`** | `U+21BB` | `SYNC_CW` | `REVL` | Revolve Feature / Orbit 3D View |
| ◡ | <img src="docs/assets/oled-icons/U+25E1.svg" width="32" height="32" alt="FILLET_ARC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25E1.svg" width="24" height="24" alt="FILLET_ARC 10x10"> | **`FLLT`** | `U+25E1` | `FILLET_ARC` | `FLLT` | Fillet / Round Edge (F) |
| ⟋ | <img src="docs/assets/oled-icons/U+27CB.svg" width="32" height="32" alt="CHAMFER_BEVEL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+27CB.svg" width="24" height="24" alt="CHAMFER_BEVEL 10x10"> | **`CHMF`** | `U+27CB` | `CHAMFER_BEVEL` | `/` | Chamfer / Bevel Edge |
| ∥ | <img src="docs/assets/oled-icons/U+2225.svg" width="32" height="32" alt="PARALLEL_CONST 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2225.svg" width="24" height="24" alt="PARALLEL_CONST 10x10"> | **`PRLL`** | `U+2225` | `PARALLEL_CONST` | `||` | Parallel Sketch Constraint |
| ⊥ | <img src="docs/assets/oled-icons/U+22A5.svg" width="32" height="32" alt="PERPENDICULAR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+22A5.svg" width="24" height="24" alt="PERPENDICULAR 10x10"> | **`PERP`** | `U+22A5` | `PERPENDICULAR` | `_|_` | Perpendicular / Normal To Sketch Plane |
| ⊙ | <img src="docs/assets/oled-icons/U+2299.svg" width="32" height="32" alt="CONCENTRIC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2299.svg" width="24" height="24" alt="CONCENTRIC 10x10"> | **`CONC`** | `U+2299` | `CONCENTRIC` | `CONC` | Concentric Constraint / Center Circle (C) |
| ◰ | <img src="docs/assets/oled-icons/U+25F0.svg" width="32" height="32" alt="ISOMETRIC_VIEW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25F0.svg" width="24" height="24" alt="ISOMETRIC_VIEW 10x10"> | **`ISO`** | `U+25F0` | `ISOMETRIC_VIEW` | `ISO` | Isometric View / Fit All (F6) |
| ∷ | <img src="docs/assets/oled-icons/U+2237.svg" width="32" height="32" alt="PATTERN_ARRAY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2237.svg" width="24" height="24" alt="PATTERN_ARRAY 10x10"> | **`PATN`** | `U+2237` | `PATTERN_ARRAY` | `::` | Rectangular / Circular Pattern Array |
| 🐚 | <img src="docs/assets/oled-icons/U+1F41A.svg" width="32" height="32" alt="SHELL_HOLLOW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F41A.svg" width="24" height="24" alt="SHELL_HOLLOW 10x10"> | **`SHLL`** | `U+1F41A` | `SHELL_HOLLOW` | `SHLL` | Shell / Hollow Wall Thickness |
| ⛓ | <img src="docs/assets/oled-icons/U+26D3.svg" width="32" height="32" alt="ASSEMBLY_JOINT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+26D3.svg" width="24" height="24" alt="ASSEMBLY_JOINT 10x10"> | **`JNT`** | `U+26D3` | `ASSEMBLY_JOINT` | `JNT` | Assembly Joint / Mate Constraint (J) |
| 🧱 | <img src="docs/assets/oled-icons/U+1F9F1.svg" width="32" height="32" alt="SLICE_LAYERS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9F1.svg" width="24" height="24" alt="SLICE_LAYERS 10x10"> | **`SLCE`** | `U+1F9F1` | `SLICE_LAYERS` | `SLCE` | Section Analysis / Slicer Layer Preview |

### Editing & Clipboard (12 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ✂ | <img src="docs/assets/oled-icons/U+2702.svg" width="32" height="32" alt="CUT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2702.svg" width="24" height="24" alt="CUT 10x10"> | **`CUT`** | `U+2702` | `CUT` | `8<` | Cut / Scissors (Ctrl+X) |
| 📋 | <img src="docs/assets/oled-icons/U+1F4CB.svg" width="32" height="32" alt="COPY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4CB.svg" width="24" height="24" alt="COPY 10x10"> | **`COPY`** | `U+1F4CB` | `COPY` | `COPY` | Copy / Clipboard (Ctrl+C) |
| 📌 | <img src="docs/assets/oled-icons/U+1F4CC.svg" width="32" height="32" alt="PASTE_PIN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4CC.svg" width="24" height="24" alt="PASTE_PIN 10x10"> | **`PASTE`** | `U+1F4CC` | `PASTE_PIN` | `PAST` | Paste / Pin Clipboard (Ctrl+V) |
| 💾 | <img src="docs/assets/oled-icons/U+1F4BE.svg" width="32" height="32" alt="SAVE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4BE.svg" width="24" height="24" alt="SAVE 10x10"> | **`SAVE`** | `U+1F4BE` | `SAVE` | `SAVE` | Save File / Floppy (Ctrl+S) |
| 📁 | <img src="docs/assets/oled-icons/U+1F4C1.svg" width="32" height="32" alt="FOLDER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4C1.svg" width="24" height="24" alt="FOLDER 10x10"> | **`OPEN`** | `U+1F4C1` | `FOLDER` | `OPEN` | Open Folder / Browse File (Ctrl+O) |
| 🔍 | <img src="docs/assets/oled-icons/U+1F50D.svg" width="32" height="32" alt="SEARCH 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F50D.svg" width="24" height="24" alt="SEARCH 10x10"> | **`FIND`** | `U+1F50D` | `SEARCH` | `FIND` | Search / Zoom / Find & Replace (Ctrl+F) |
| 🗑 | <img src="docs/assets/oled-icons/U+1F5D1.svg" width="32" height="32" alt="DELETE_TRASH 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5D1.svg" width="24" height="24" alt="DELETE_TRASH 10x10"> | **`DEL`** | `U+1F5D1` | `DELETE_TRASH` | `DEL` | Delete / Trash |
| 📷 | <img src="docs/assets/oled-icons/U+1F4F7.svg" width="32" height="32" alt="CAMERA 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4F7.svg" width="24" height="24" alt="CAMERA 10x10"> | **`SHOT`** | `U+1F4F7` | `CAMERA` | `SHOT` | Screenshot / Capture Region |
| 🔗 | <img src="docs/assets/oled-icons/U+1F517.svg" width="32" height="32" alt="LINK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F517.svg" width="24" height="24" alt="LINK 10x10"> | **`LINK`** | `U+1F517` | `LINK` | `LINK` | Insert Hyperlink / Copy URL (Ctrl+K) |
| ↩ | <img src="docs/assets/oled-icons/U+21A9.svg" width="32" height="32" alt="UNDO 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21A9.svg" width="24" height="24" alt="UNDO 10x10"> | **`UNDO`** | `U+21A9` | `UNDO` | `UNDO` | Undo Action (Ctrl+Z) |
| ↪ | <img src="docs/assets/oled-icons/U+21AA.svg" width="32" height="32" alt="REDO 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21AA.svg" width="24" height="24" alt="REDO 10x10"> | **`REDO`** | `U+21AA` | `REDO` | `REDO` | Redo Action (Ctrl+Y) |
| ↺ | <img src="docs/assets/oled-icons/U+21BA.svg" width="32" height="32" alt="RESET_CCW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21BA.svg" width="24" height="24" alt="RESET_CCW 10x10"> | **`RESET`** | `U+21BA` | `RESET_CCW` | `RESE` | Revert / Reset Changes |

### Arrows & Nav (12 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ↑ | <img src="docs/assets/oled-icons/U+2191.svg" width="32" height="32" alt="ARROW_UP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2191.svg" width="24" height="24" alt="ARROW_UP 10x10"> | **`UP`** | `U+2191` | `ARROW_UP` | `UP` | Arrow Up |
| ↓ | <img src="docs/assets/oled-icons/U+2193.svg" width="32" height="32" alt="ARROW_DOWN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2193.svg" width="24" height="24" alt="ARROW_DOWN 10x10"> | **`DOWN`** | `U+2193` | `ARROW_DOWN` | `DOWN` | Arrow Down |
| ← | <img src="docs/assets/oled-icons/U+2190.svg" width="32" height="32" alt="ARROW_LEFT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2190.svg" width="24" height="24" alt="ARROW_LEFT 10x10"> | **`LEFT`** | `U+2190` | `ARROW_LEFT` | `LEFT` | Arrow Left |
| → | <img src="docs/assets/oled-icons/U+2192.svg" width="32" height="32" alt="ARROW_RIGHT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2192.svg" width="24" height="24" alt="ARROW_RIGHT 10x10"> | **`RIGHT`** | `U+2192` | `ARROW_RIGHT` | `RIGH` | Arrow Right |
| ↖ | <img src="docs/assets/oled-icons/U+2196.svg" width="32" height="32" alt="ARROW_NW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2196.svg" width="24" height="24" alt="ARROW_NW 10x10"> | **`HOME`** | `U+2196` | `ARROW_NW` | `<^` | Home / Top-Left |
| ↘ | <img src="docs/assets/oled-icons/U+2198.svg" width="32" height="32" alt="ARROW_SE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2198.svg" width="24" height="24" alt="ARROW_SE 10x10"> | **`END`** | `U+2198` | `ARROW_SE` | `v>` | End / Bottom-Right |
| ▲ | <img src="docs/assets/oled-icons/U+25B2.svg" width="32" height="32" alt="TRI_UP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25B2.svg" width="24" height="24" alt="TRI_UP 10x10"> | **`PGUP`** | `U+25B2` | `TRI_UP` | `PGUP` | Triangle Up / Page Up |
| ▼ | <img src="docs/assets/oled-icons/U+25BC.svg" width="32" height="32" alt="TRI_DOWN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25BC.svg" width="24" height="24" alt="TRI_DOWN 10x10"> | **`PGDN`** | `U+25BC` | `TRI_DOWN` | `PGDN` | Triangle Down / Page Down |
| ◀ | <img src="docs/assets/oled-icons/U+25C0.svg" width="32" height="32" alt="TRI_LEFT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25C0.svg" width="24" height="24" alt="TRI_LEFT 10x10"> | **`BACK`** | `U+25C0` | `TRI_LEFT` | `BACK` | Navigate Back / Previous |
| ▷ | <img src="docs/assets/oled-icons/U+25B7.svg" width="32" height="32" alt="STEP_FWD_OUTLINE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25B7.svg" width="24" height="24" alt="STEP_FWD_OUTLINE 10x10"> | **`FWD`** | `U+25B7` | `STEP_FWD_OUTLINE` | `FWD` | Navigate Forward / Step Next |
| ⇄ | <img src="docs/assets/oled-icons/U+21C4.svg" width="32" height="32" alt="SWAP_LR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21C4.svg" width="24" height="24" alt="SWAP_LR 10x10"> | **`SWAP`** | `U+21C4` | `SWAP_LR` | `SWAP` | Swap Left / Right |
| ⤢ | <img src="docs/assets/oled-icons/U+2922.svg" width="32" height="32" alt="EXPAND_DIAG 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2922.svg" width="24" height="24" alt="EXPAND_DIAG 10x10"> | **`MAX`** | `U+2922` | `EXPAND_DIAG` | `MAX` | Expand / Diagonal Resize |

### Media & Audio (16 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ▶ | <img src="docs/assets/oled-icons/U+25B6.svg" width="32" height="32" alt="PLAY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25B6.svg" width="24" height="24" alt="PLAY 10x10"> | **`PLAY`** | `U+25B6` | `PLAY` | `PLAY` | Play Media |
| ⏸ | <img src="docs/assets/oled-icons/U+23F8.svg" width="32" height="32" alt="PAUSE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23F8.svg" width="24" height="24" alt="PAUSE 10x10"> | **`PAUSE`** | `U+23F8` | `PAUSE` | `||` | Pause Media |
| ⏯ | <img src="docs/assets/oled-icons/U+23EF.svg" width="32" height="32" alt="PLAY_PAUSE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23EF.svg" width="24" height="24" alt="PLAY_PAUSE 10x10"> | **`PLPA`** | `U+23EF` | `PLAY_PAUSE` | `PLPA` | Play / Pause Toggle |
| ⏹ | <img src="docs/assets/oled-icons/U+23F9.svg" width="32" height="32" alt="STOP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23F9.svg" width="24" height="24" alt="STOP 10x10"> | **`STOP`** | `U+23F9` | `STOP` | `STOP` | Stop Playback |
| ⏭ | <img src="docs/assets/oled-icons/U+23ED.svg" width="32" height="32" alt="NEXT_TRACK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23ED.svg" width="24" height="24" alt="NEXT_TRACK 10x10"> | **`NEXT`** | `U+23ED` | `NEXT_TRACK` | `NEXT` | Next Track |
| ⏮ | <img src="docs/assets/oled-icons/U+23EE.svg" width="32" height="32" alt="PREV_TRACK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23EE.svg" width="24" height="24" alt="PREV_TRACK 10x10"> | **`PREV`** | `U+23EE` | `PREV_TRACK` | `PREV` | Previous Track |
| ⏩ | <img src="docs/assets/oled-icons/U+23E9.svg" width="32" height="32" alt="FAST_FORWARD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23E9.svg" width="24" height="24" alt="FAST_FORWARD 10x10"> | **`FFWD`** | `U+23E9` | `FAST_FORWARD` | `>>` | Fast Forward |
| ⏪ | <img src="docs/assets/oled-icons/U+23EA.svg" width="32" height="32" alt="REWIND 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23EA.svg" width="24" height="24" alt="REWIND 10x10"> | **`RWND`** | `U+23EA` | `REWIND` | `<<` | Rewind |
| 🔊 | <img src="docs/assets/oled-icons/U+1F50A.svg" width="32" height="32" alt="VOL_UP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F50A.svg" width="24" height="24" alt="VOL_UP 10x10"> | **`VOL+`** | `U+1F50A` | `VOL_UP` | `VOL+` | Volume Up |
| 🔉 | <img src="docs/assets/oled-icons/U+1F509.svg" width="32" height="32" alt="VOL_DOWN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F509.svg" width="24" height="24" alt="VOL_DOWN 10x10"> | **`VOL-`** | `U+1F509` | `VOL_DOWN` | `VOL-` | Volume Down |
| 🔇 | <img src="docs/assets/oled-icons/U+1F507.svg" width="32" height="32" alt="MUTE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F507.svg" width="24" height="24" alt="MUTE 10x10"> | **`MUTE`** | `U+1F507` | `MUTE` | `MUTE` | Mute Audio |
| 🎤 | <img src="docs/assets/oled-icons/U+1F3A4.svg" width="32" height="32" alt="MIC 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3A4.svg" width="24" height="24" alt="MIC 10x10"> | **`MIC`** | `U+1F3A4` | `MIC` | `MIC` | Microphone Mute / Push-to-Talk |
| 🎧 | <img src="docs/assets/oled-icons/U+1F3A7.svg" width="32" height="32" alt="HEADPHONES 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3A7.svg" width="24" height="24" alt="HEADPHONES 10x10"> | **`HEAD`** | `U+1F3A7` | `HEADPHONES` | `HEAD` | Headphones / Audio Output |
| 🎵 | <img src="docs/assets/oled-icons/U+1F3B5.svg" width="32" height="32" alt="MUSIC_NOTE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3B5.svg" width="24" height="24" alt="MUSIC_NOTE 10x10"> | **`MUSIC`** | `U+1F3B5` | `MUSIC_NOTE` | `MUSI` | Music Note |
| 🎶 | <img src="docs/assets/oled-icons/U+1F3B6.svg" width="32" height="32" alt="MUSIC_NOTES 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3B6.svg" width="24" height="24" alt="MUSIC_NOTES 10x10"> | **`SONG`** | `U+1F3B6` | `MUSIC_NOTES` | `SONG` | Music Notes / Playlist |
| ⏺ | <img src="docs/assets/oled-icons/U+23FA.svg" width="32" height="32" alt="RECORD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23FA.svg" width="24" height="24" alt="RECORD 10x10"> | **`REC`** | `U+23FA` | `RECORD` | `REC` | Record Audio / Video |

### Coding & Terminal (14 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ⌨ | <img src="docs/assets/oled-icons/U+2328.svg" width="32" height="32" alt="TERMINAL_KB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2328.svg" width="24" height="24" alt="TERMINAL_KB 10x10"> | **`TERM`** | `U+2328` | `TERMINAL_KB` | `>_` | Terminal / Console / Shell |
| 💻 | <img src="docs/assets/oled-icons/U+1F4BB.svg" width="32" height="32" alt="LAPTOP_IDE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4BB.svg" width="24" height="24" alt="LAPTOP_IDE 10x10"> | **`IDE`** | `U+1F4BB` | `LAPTOP_IDE` | `IDE` | Code Editor / VS Code / IDE |
| ⚡ | <img src="docs/assets/oled-icons/U+26A1.svg" width="32" height="32" alt="BOLT_BUILD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+26A1.svg" width="24" height="24" alt="BOLT_BUILD 10x10"> | **`BUILD`** | `U+26A1` | `BOLT_BUILD` | `BUIL` | Build / Compile / Flash |
| 🐞 | <img src="docs/assets/oled-icons/U+1F41E.svg" width="32" height="32" alt="BUG_DEBUG 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F41E.svg" width="24" height="24" alt="BUG_DEBUG 10x10"> | **`DEBUG`** | `U+1F41E` | `BUG_DEBUG` | `DEBU` | Debug / Breakpoint / Bug |
| 🚀 | <img src="docs/assets/oled-icons/U+1F680.svg" width="32" height="32" alt="ROCKET_RUN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F680.svg" width="24" height="24" alt="ROCKET_RUN 10x10"> | **`RUN`** | `U+1F680` | `ROCKET_RUN` | `RUN` | Run / Deploy / Launch |
| 🔧 | <img src="docs/assets/oled-icons/U+1F527.svg" width="32" height="32" alt="WRENCH_TOOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F527.svg" width="24" height="24" alt="WRENCH_TOOL 10x10"> | **`TOOL`** | `U+1F527` | `WRENCH_TOOL` | `TOOL` | Wrench / Refactor / Lint |
| ⚙ | <img src="docs/assets/oled-icons/U+2699.svg" width="32" height="32" alt="GEAR_SETTINGS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2699.svg" width="24" height="24" alt="GEAR_SETTINGS 10x10"> | **`PREF`** | `U+2699` | `GEAR_SETTINGS` | `PREF` | Settings / Configuration Gear |
| 📦 | <img src="docs/assets/oled-icons/U+1F4E6.svg" width="32" height="32" alt="PACKAGE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4E6.svg" width="24" height="24" alt="PACKAGE 10x10"> | **`PKG`** | `U+1F4E6` | `PACKAGE` | `PKG` | Package / Cargo / NPM Build |
| 🌿 | <img src="docs/assets/oled-icons/U+1F33F.svg" width="32" height="32" alt="GIT_BRANCH 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F33F.svg" width="24" height="24" alt="GIT_BRANCH 10x10"> | **`GIT`** | `U+1F33F` | `GIT_BRANCH` | `Y-` | Git Branch / Commit / Merge |
| ☁ | <img src="docs/assets/oled-icons/U+2601.svg" width="32" height="32" alt="CLOUD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2601.svg" width="24" height="24" alt="CLOUD 10x10"> | **`PUSH`** | `U+2601` | `CLOUD` | `PUSH` | Git Push / Cloud Sync |
| 🐳 | <img src="docs/assets/oled-icons/U+1F433.svg" width="32" height="32" alt="DOCKER_WHALE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F433.svg" width="24" height="24" alt="DOCKER_WHALE 10x10"> | **`DCKR`** | `U+1F433` | `DOCKER_WHALE` | `DCKR` | Docker Container / Compose |
| 🔑 | <img src="docs/assets/oled-icons/U+1F511.svg" width="32" height="32" alt="SSH_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F511.svg" width="24" height="24" alt="SSH_KEY 10x10"> | **`SSH`** | `U+1F511` | `SSH_KEY` | `SSH` | SSH Key / Auth Token / Secret |
| 🤖 | <img src="docs/assets/oled-icons/U+1F916.svg" width="32" height="32" alt="ROBOT_AI 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F916.svg" width="24" height="24" alt="ROBOT_AI 10x10"> | **`AI`** | `U+1F916` | `ROBOT_AI` | `AI` | AI Assistant / Copilot / Agent |
| ⌘ | <img src="docs/assets/oled-icons/U+2318.svg" width="32" height="32" alt="COMMAND_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2318.svg" width="24" height="24" alt="COMMAND_KEY 10x10"> | **`CMD`** | `U+2318` | `COMMAND_KEY` | `CMD` | Command Palette / Super Key |

### Home Automation (19 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 💡 | <img src="docs/assets/oled-icons/U+1F4A1.svg" width="32" height="32" alt="LIGHT_BULB 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4A1.svg" width="24" height="24" alt="LIGHT_BULB 10x10"> | **`BULB`** | `U+1F4A1` | `LIGHT_BULB` | `BULB` | Light / Lamp / Bulb |
| 🔌 | <img src="docs/assets/oled-icons/U+1F50C.svg" width="32" height="32" alt="SMART_PLUG 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F50C.svg" width="24" height="24" alt="SMART_PLUG 10x10"> | **`PLUG`** | `U+1F50C` | `SMART_PLUG` | `PLUG` | Smart Plug / Socket |
| 🌡 | <img src="docs/assets/oled-icons/U+1F321.svg" width="32" height="32" alt="THERMOSTAT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F321.svg" width="24" height="24" alt="THERMOSTAT 10x10"> | **`TEMP`** | `U+1F321` | `THERMOSTAT` | `TEMP` | Thermostat / Temperature |
| 🔥 | <img src="docs/assets/oled-icons/U+1F525.svg" width="32" height="32" alt="FIRE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F525.svg" width="24" height="24" alt="FIRE 10x10"> | **`HEAT`** | `U+1F525` | `FIRE` | `HEAT` | Heater / Warmth |
| ❄ | <img src="docs/assets/oled-icons/U+2744.svg" width="32" height="32" alt="SNOWFLAKE_COOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2744.svg" width="24" height="24" alt="SNOWFLAKE_COOL 10x10"> | **`COOL`** | `U+2744` | `SNOWFLAKE_COOL` | `COOL` | AC / Air Conditioner / Cooling |
| 🌀 | <img src="docs/assets/oled-icons/U+1F300.svg" width="32" height="32" alt="CEILING_FAN 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F300.svg" width="24" height="24" alt="CEILING_FAN 10x10"> | **`FAN`** | `U+1F300` | `CEILING_FAN` | `FAN` | Ceiling Fan / Ventilation |
| 🔒 | <img src="docs/assets/oled-icons/U+1F512.svg" width="32" height="32" alt="LOCK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F512.svg" width="24" height="24" alt="LOCK 10x10"> | **`LOCK`** | `U+1F512` | `LOCK` | `LOCK` | Lock / Armed |
| 🔓 | <img src="docs/assets/oled-icons/U+1F513.svg" width="32" height="32" alt="UNLOCK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F513.svg" width="24" height="24" alt="UNLOCK 10x10"> | **`UNLCK`** | `U+1F513` | `UNLOCK` | `UNLC` | Unlock / Disarmed |
| 🛡 | <img src="docs/assets/oled-icons/U+1F6E1.svg" width="32" height="32" alt="SECURITY_SHIELD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F6E1.svg" width="24" height="24" alt="SECURITY_SHIELD 10x10"> | **`ALRM`** | `U+1F6E1` | `SECURITY_SHIELD` | `ALRM` | Security Alarm / Shield |
| 📹 | <img src="docs/assets/oled-icons/U+1F4F9.svg" width="32" height="32" alt="SECURITY_CAM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4F9.svg" width="24" height="24" alt="SECURITY_CAM 10x10"> | **`CAM`** | `U+1F4F9` | `SECURITY_CAM` | `CAM` | Security Camera / CCTV |
| 🚪 | <img src="docs/assets/oled-icons/U+1F6AA.svg" width="32" height="32" alt="DOOR_SENSOR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F6AA.svg" width="24" height="24" alt="DOOR_SENSOR 10x10"> | **`DOOR`** | `U+1F6AA` | `DOOR_SENSOR` | `DOOR` | Door / Contact Sensor |
| 🪟 | <img src="docs/assets/oled-icons/U+1FA9F.svg" width="32" height="32" alt="WINDOW_TILE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1FA9F.svg" width="24" height="24" alt="WINDOW_TILE 10x10"> | **`WNDW`** | `U+1FA9F` | `WINDOW_TILE` | `WNDW` | Window / Blinds / Shade |
| 🧹 | <img src="docs/assets/oled-icons/U+1F9F9.svg" width="32" height="32" alt="ROBOT_VACUUM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F9F9.svg" width="24" height="24" alt="ROBOT_VACUUM 10x10"> | **`VACC`** | `U+1F9F9` | `ROBOT_VACUUM` | `VACC` | Robot Vacuum / Cleaner |
| 💧 | <img src="docs/assets/oled-icons/U+1F4A7.svg" width="32" height="32" alt="WATER_DROP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4A7.svg" width="24" height="24" alt="WATER_DROP 10x10"> | **`LEAK`** | `U+1F4A7` | `WATER_DROP` | `LEAK` | Water / Flood / Leak Sensor |
| 🚨 | <img src="docs/assets/oled-icons/U+1F6A8.svg" width="32" height="32" alt="SIREN_STROBE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F6A8.svg" width="24" height="24" alt="SIREN_STROBE 10x10"> | **`SIREN`** | `U+1F6A8` | `SIREN_STROBE` | `SIRE` | Siren / Emergency Strobe |
| 🏠 | <img src="docs/assets/oled-icons/U+1F3E0.svg" width="32" height="32" alt="HOME 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3E0.svg" width="24" height="24" alt="HOME 10x10"> | **`HOME`** | `U+1F3E0` | `HOME` | `HOME` | Home / Occupied Scene |
| 🌙 | <img src="docs/assets/oled-icons/U+1F319.svg" width="32" height="32" alt="MOON_SLEEP 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F319.svg" width="24" height="24" alt="MOON_SLEEP 10x10"> | **`NIGHT`** | `U+1F319` | `MOON_SLEEP` | `((` | Night / Goodnight Scene |
| ☀ | <img src="docs/assets/oled-icons/U+2600.svg" width="32" height="32" alt="SUN_BRIGHT 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2600.svg" width="24" height="24" alt="SUN_BRIGHT 10x10"> | **`DAY`** | `U+2600` | `SUN_BRIGHT` | `DAY` | Morning / Daytime Scene |
| 🚗 | <img src="docs/assets/oled-icons/U+1F697.svg" width="32" height="32" alt="EV_CAR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F697.svg" width="24" height="24" alt="EV_CAR 10x10"> | **`EV`** | `U+1F697` | `EV_CAR` | `EV` | EV Charger / Garage Door |

### Window & System (10 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 🖥 | <img src="docs/assets/oled-icons/U+1F5A5.svg" width="32" height="32" alt="DESKTOP_MON 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5A5.svg" width="24" height="24" alt="DESKTOP_MON 10x10"> | **`DESK`** | `U+1F5A5` | `DESKTOP_MON` | `DESK` | Desktop / Monitor Switch |
| ▣ | <img src="docs/assets/oled-icons/U+25A3.svg" width="32" height="32" alt="TILE_WINDOW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+25A3.svg" width="24" height="24" alt="TILE_WINDOW 10x10"> | **`TILE`** | `U+25A3` | `TILE_WINDOW` | `TILE` | Tile Window / Snap Layout |
| ⏻ | <img src="docs/assets/oled-icons/U+23FB.svg" width="32" height="32" alt="POWER_BUTTON 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23FB.svg" width="24" height="24" alt="POWER_BUTTON 10x10"> | **`PWR`** | `U+23FB` | `POWER_BUTTON` | `PWR` | System Power / Shutdown / Reboot |
| 💤 | <img src="docs/assets/oled-icons/U+1F4A4.svg" width="32" height="32" alt="SLEEP_ZZZ 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4A4.svg" width="24" height="24" alt="SLEEP_ZZZ 10x10"> | **`SLEEP`** | `U+1F4A4` | `SLEEP_ZZZ` | `SLEE` | Sleep / Suspend System |
| 🔅 | <img src="docs/assets/oled-icons/U+1F505.svg" width="32" height="32" alt="BRIGHTNESS_LOW 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F505.svg" width="24" height="24" alt="BRIGHTNESS_LOW 10x10"> | **`BRITE`** | `U+1F505` | `BRIGHTNESS_LOW` | `BRIT` | Screen Brightness / Night Light |
| 🔋 | <img src="docs/assets/oled-icons/U+1F50B.svg" width="32" height="32" alt="BATTERY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F50B.svg" width="24" height="24" alt="BATTERY 10x10"> | **`BATT`** | `U+1F50B` | `BATTERY` | `BATT` | Battery / Power Profile |
| 📡 | <img src="docs/assets/oled-icons/U+1F4E1.svg" width="32" height="32" alt="WIFI_NET 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4E1.svg" width="24" height="24" alt="WIFI_NET 10x10"> | **`WIFI`** | `U+1F4E1` | `WIFI_NET` | `WIFI` | Network / Wi-Fi Toggle |
| ⏱ | <img src="docs/assets/oled-icons/U+23F1.svg" width="32" height="32" alt="STOPWATCH 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23F1.svg" width="24" height="24" alt="STOPWATCH 10x10"> | **`TIMER`** | `U+23F1` | `STOPWATCH` | `TIME` | Stopwatch / Macro Delay |
| 🔔 | <img src="docs/assets/oled-icons/U+1F514.svg" width="32" height="32" alt="BELL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F514.svg" width="24" height="24" alt="BELL 10x10"> | **`BELL`** | `U+1F514` | `BELL` | `BELL` | Notifications / Do Not Disturb |
| ⏏ | <img src="docs/assets/oled-icons/U+23CF.svg" width="32" height="32" alt="EJECT_SYMBOL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23CF.svg" width="24" height="24" alt="EJECT_SYMBOL 10x10"> | **`EJECT`** | `U+23CF` | `EJECT_SYMBOL` | `EJEC` | Eject Drive / Safe Unmount |

### Streaming & Apps (9 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| 🎥 | <img src="docs/assets/oled-icons/U+1F3A5.svg" width="32" height="32" alt="VIDEO_CAM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3A5.svg" width="24" height="24" alt="VIDEO_CAM 10x10"> | **`SCENE`** | `U+1F3A5` | `VIDEO_CAM` | `SCEN` | OBS Scene / Camera Switch |
| 📺 | <img src="docs/assets/oled-icons/U+1F4FA.svg" width="32" height="32" alt="TV_STREAM 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4FA.svg" width="24" height="24" alt="TV_STREAM 10x10"> | **`LIVE`** | `U+1F4FA` | `TV_STREAM` | `LIVE` | Go Live / Stream Monitor |
| 🎬 | <img src="docs/assets/oled-icons/U+1F3AC.svg" width="32" height="32" alt="CLAPPERBOARD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3AC.svg" width="24" height="24" alt="CLAPPERBOARD 10x10"> | **`CLIP`** | `U+1F3AC` | `CLAPPERBOARD` | `CLIP` | Save Replay Clip / Marker |
| 🗨 | <img src="docs/assets/oled-icons/U+1F5E8.svg" width="32" height="32" alt="CHAT_BUBBLE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F5E8.svg" width="24" height="24" alt="CHAT_BUBBLE 10x10"> | **`CHAT`** | `U+1F5E8` | `CHAT_BUBBLE` | `CHAT` | Stream Chat / Discord Message |
| 🎮 | <img src="docs/assets/oled-icons/U+1F3AE.svg" width="32" height="32" alt="GAMEPAD 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3AE.svg" width="24" height="24" alt="GAMEPAD 10x10"> | **`GAME`** | `U+1F3AE` | `GAMEPAD` | `GAME` | Game Capture / Controller |
| 📻 | <img src="docs/assets/oled-icons/U+1F4FB.svg" width="32" height="32" alt="RADIO_MIXER 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4FB.svg" width="24" height="24" alt="RADIO_MIXER 10x10"> | **`AUDIO`** | `U+1F4FB` | `RADIO_MIXER` | `AUDI` | Audio Mixer / Radio Stream |
| 📧 | <img src="docs/assets/oled-icons/U+1F4E7.svg" width="32" height="32" alt="MAIL 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4E7.svg" width="24" height="24" alt="MAIL 10x10"> | **`MAIL`** | `U+1F4E7` | `MAIL` | `MAIL` | Email Client / Compose Mail |
| 📅 | <img src="docs/assets/oled-icons/U+1F4C5.svg" width="32" height="32" alt="CALENDAR 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F4C5.svg" width="24" height="24" alt="CALENDAR 10x10"> | **`CAL`** | `U+1F4C5` | `CALENDAR` | `CAL` | Calendar / New Meeting |
| 🛒 | <img src="docs/assets/oled-icons/U+1F6D2.svg" width="32" height="32" alt="SHOPPING_CART 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F6D2.svg" width="24" height="24" alt="SHOPPING_CART 10x10"> | **`SHOP`** | `U+1F6D2` | `SHOPPING_CART` | `SHOP` | Shopping / Quick Order |

### Symbols & Shapes (12 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ★ | <img src="docs/assets/oled-icons/U+2605.svg" width="32" height="32" alt="STAR_SOLID 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2605.svg" width="24" height="24" alt="STAR_SOLID 10x10"> | **`STAR`** | `U+2605` | `STAR_SOLID` | `STAR` | Solid Star / Favorite |
| ☆ | <img src="docs/assets/oled-icons/U+2606.svg" width="32" height="32" alt="STAR_OUTLINE 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2606.svg" width="24" height="24" alt="STAR_OUTLINE 10x10"> | **`FAV`** | `U+2606` | `STAR_OUTLINE` | `FAV` | Outline Star / Bookmark |
| ♥ | <img src="docs/assets/oled-icons/U+2665.svg" width="32" height="32" alt="HEART 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2665.svg" width="24" height="24" alt="HEART 10x10"> | **`LIKE`** | `U+2665` | `HEART` | `<3` | Heart / Like |
| ✔ | <img src="docs/assets/oled-icons/U+2714.svg" width="32" height="32" alt="CHECK 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2714.svg" width="24" height="24" alt="CHECK 10x10"> | **`OK`** | `U+2714` | `CHECK` | `OK` | Checkmark / Confirm / Apply |
| ✖ | <img src="docs/assets/oled-icons/U+2716.svg" width="32" height="32" alt="CROSS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2716.svg" width="24" height="24" alt="CROSS 10x10"> | **`CLOSE`** | `U+2716` | `CROSS` | `CLOS` | Cross / Close / Cancel |
| ➕ | <img src="docs/assets/oled-icons/U+2795.svg" width="32" height="32" alt="PLUS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2795.svg" width="24" height="24" alt="PLUS 10x10"> | **`ADD`** | `U+2795` | `PLUS` | `ADD` | Plus / Zoom In / New Item |
| ➖ | <img src="docs/assets/oled-icons/U+2796.svg" width="32" height="32" alt="MINUS 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2796.svg" width="24" height="24" alt="MINUS 10x10"> | **`SUB`** | `U+2796` | `MINUS` | `SUB` | Minus / Zoom Out / Remove |
| ⚠ | <img src="docs/assets/oled-icons/U+26A0.svg" width="32" height="32" alt="WARNING 16x16"> | <img src="docs/assets/oled-icons-10x10/U+26A0.svg" width="24" height="24" alt="WARNING 10x10"> | **`WARN`** | `U+26A0` | `WARNING` | `WARN` | Warning / Alert |
| 💎 | <img src="docs/assets/oled-icons/U+1F48E.svg" width="32" height="32" alt="DIAMOND 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F48E.svg" width="24" height="24" alt="DIAMOND 10x10"> | **`PRO`** | `U+1F48E` | `DIAMOND` | `<>` | Diamond / Macro Preset |
| 🎯 | <img src="docs/assets/oled-icons/U+1F3AF.svg" width="32" height="32" alt="TARGET 16x16"> | <img src="docs/assets/oled-icons-10x10/U+1F3AF.svg" width="24" height="24" alt="TARGET 10x10"> | **`AIM`** | `U+1F3AF` | `TARGET` | `AIM` | Target / Focus Mode |
| ∞ | <img src="docs/assets/oled-icons/U+221E.svg" width="32" height="32" alt="INFINITY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+221E.svg" width="24" height="24" alt="INFINITY 10x10"> | **`LOOP`** | `U+221E` | `INFINITY` | `oo` | Infinity / Repeat Loop |
| ❓ | <img src="docs/assets/oled-icons/U+2753.svg" width="32" height="32" alt="HELP_QUESTION 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2753.svg" width="24" height="24" alt="HELP_QUESTION 10x10"> | **`HELP`** | `U+2753` | `HELP_QUESTION` | `HELP` | Help / Shortcut Reference (F1) |

### Keyboard Modifiers (5 Icons)

| Color (UI) | Monochrome 16×16 (OLED) | Monochrome 10×10 (2-Line) | Short Label | Codepoint | Firmware Bitmap ID | Stock ASCII | Function / Menu Action |
| :---: | :---: | :---: | :---: | :---: | :--- | :---: | :--- |
| ⇧ | <img src="docs/assets/oled-icons/U+21E7.svg" width="32" height="32" alt="SHIFT_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+21E7.svg" width="24" height="24" alt="SHIFT_KEY 10x10"> | **`SHFT`** | `U+21E7` | `SHIFT_KEY` | `SHFT` | Shift Modifier Key |
| ⌥ | <img src="docs/assets/oled-icons/U+2325.svg" width="32" height="32" alt="OPTION_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2325.svg" width="24" height="24" alt="OPTION_KEY 10x10"> | **`OPT`** | `U+2325` | `OPTION_KEY` | `ALT` | Option / Alt Modifier Key |
| ⌃ | <img src="docs/assets/oled-icons/U+2303.svg" width="32" height="32" alt="CTRL_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+2303.svg" width="24" height="24" alt="CTRL_KEY 10x10"> | **`CTRL`** | `U+2303` | `CTRL_KEY` | `CTRL` | Control Modifier Key |
| ⏎ | <img src="docs/assets/oled-icons/U+23CE.svg" width="32" height="32" alt="ENTER_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+23CE.svg" width="24" height="24" alt="ENTER_KEY 10x10"> | **`ENTR`** | `U+23CE` | `ENTER_KEY` | `ENT` | Return / Enter Key |
| ⌫ | <img src="docs/assets/oled-icons/U+232B.svg" width="32" height="32" alt="BACKSPACE_KEY 16x16"> | <img src="docs/assets/oled-icons-10x10/U+232B.svg" width="24" height="24" alt="BACKSPACE_KEY 10x10"> | **`BKSP`** | `U+232B` | `BACKSPACE_KEY` | `BKSP` | Backspace / Delete Key |

</details>

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
Full documentation and request/response schemas are available at **[https://duckydeckstudio.com](https://duckydeckstudio.com)** as well as in **[`docs/REST_API.md`](docs/REST_API.md)**, **[`docs/openapi.yaml`](docs/openapi.yaml)**, and **[`docs/swagger.json`](docs/swagger.json)**.

| Method & Path | Description |
| :--- | :--- |
| `GET /api/v1/health` | Unauthenticated liveness probe (`{"ok":true,"service":"duckydeck-studio","version":"0.1.7","authRequired":true}`). |
| `GET /api/v1/openapi.json` \| `/swagger.json` \| `/openapi.yaml` | Unauthenticated OpenAPI 3.0.3 / Swagger specification in JSON or YAML format. |
| `GET /api/v1/status` *(or `/api/v1`)* | Full device connection, firmware, active profile, SD sync, and auto-switcher state. |
| `GET /api/v1/profiles` | List all profiles and their properties. |
| `POST /api/v1/profiles` | Create a new profile (`{"name":"Streaming"}`) or duplicate an existing profile (`{"name":"Copy","duplicateFrom":1}`). |
| `POST /api/v1/profiles/select` | Select active profile and optionally switch hardware (`{"profile":2,"switchHardware":true}`). |
| `POST /api/v1/profiles/reorder` | Move profile (`{"fromProfile":3,"toProfile":1}`) and automatically sync Auto-Switcher rule bindings. |
| `GET /api/v1/profiles/{num}` | Get full profile details, webhook config, and all 26 slots. |
| `PUT` \| `PATCH /api/v1/profiles/{num}` | Update profile properties (`name`, `bgColor`, `oledRotation`, `widgetRotation`, `webhook`, etc.). |
| `DELETE /api/v1/profiles/{num}` | Delete profile `{num}` (Fallback Profile is protected; deleted profile rules reassign to Fallback). |
| `GET /api/v1/profiles/{num}/slots` | List all 26 slots (`1..20` keys, `21..26` rotary encoders) in profile `{num}`. |
| `GET /api/v1/profiles/{num}/slots/{slot}` | Get configuration and scripts for a specific slot (`1..26`). |
| `PUT` \| `POST` \| `PATCH /api/v1/profiles/{num}/slots/{slot}` | Program a slot (`labelLine1`, `labelLine2`, `widgetLabelLine1`, `widgetLabelLine2`, `colorHex`, `scriptContent`, `allowAbort`, `dontRepeat`, `hostApp`, `applyTemplateId`). |
| `DELETE /api/v1/profiles/{num}/slots/{slot}` | Clear/reset slot `{slot}` in profile `{num}`. |
| `POST /api/v1/profiles/{num}/slots/{slot}/trigger` | Remotely execute the slot's configured host action (`@HOST_APP`, `@HA_ACTION`, `@WEBHOOK`). |
| `POST /api/v1/dsvm/compile` | Validate/compile duckyScript 3 source (`{"script":"STRINGLN Hello"}`) into `.dsb` bytecode + CRC32. |
| `GET /api/v1/templates` | List all built-in and user-saved key templates. |
| `GET /api/v1/pgv` \| `PUT` \| `POST /api/v1/pgv` | Read or write all 32 Persistent Global Variables (`_GV0.._GV31`). |
| `POST /api/v1/leds` | Live-override per-key RGB LEDs (`{"colorHex":"#38bdf8"}` or `{"leds":[{"slot":1,"colorHex":"#ff0055"}]}`) or restore profile LEDs (`{"restore":true}`). |
| `POST /api/v1/sd/sync` \| `/reload` \| `/mount` \| `/unmount` | Sync `.dsb` bytecode & profiles to microSD, reload from SD/cache, mount SD mode, or unmount SD. |
| `GET /api/v1/jobs/{id}` | Poll background job status (`running`, `done`, `error`) for async SD/reboot operations. |
| `POST /api/v1/device/sleep` \| `/wake` \| `/reboot` \| `/sync-rtc` | Put duckyPad to sleep, wake it up, soft-reboot, or synchronize hardware RTC with host clock. |
| `GET /api/v1/logs` | Retrieve recent system, HID, script, and automation event logs. |

---

## 📚 Documentation & Links

- **🌐 [Official Website & User Documentation (`https://duckydeckstudio.com`)](https://duckydeckstudio.com)** — Official online documentation, user manual, release downloads, and firmware downloads (accessible directly inside the app via **☰ → User Documentation** or **About → User Guide**; offline copy at [`docs/user-guide.html`](docs/user-guide.html)).
- **🏗️ [Architecture & Design Developer Documentation (`https://duckydeckstudio.com/architecture.html`)](https://duckydeckstudio.com/architecture.html)** — Interactive HTML architecture reference ([`docs/architecture.html`](docs/architecture.html)) and Markdown documentation ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)) detailing the CXX-Qt bridge, USB HID/MSC protocol, DSVM v2 bytecode compiler, HTTP REST API server, Flatpak sandbox, and Cross-Platform Auto-Switcher (KWin Wayland & Windows Win32).
- **💻 [GitHub Repository & Issue Tracker (`https://github.com/russellmilliner/duckydeck-studio`)](https://github.com/russellmilliner/duckydeck-studio)** — Source code, issue tracker, and **[GitHub Releases](https://github.com/russellmilliner/duckydeck-studio/releases)**.
- **🔌 [HTTP REST API Reference (`https://duckydeckstudio.com/REST_API.md` / `docs/REST_API.md`)](https://duckydeckstudio.com/REST_API.md)** — Complete endpoint documentation, authentication guide, and `curl` examples, accompanied by **[`docs/REST_API.md`](docs/REST_API.md)**, **[`docs/openapi.yaml`](docs/openapi.yaml)**, and **[`docs/swagger.json`](docs/swagger.json)**.

---

## 📂 Project Layout

```text
duckydeck-studio/
├── Cargo.toml                             # Rust dependencies (cxx-qt, hidapi, zbus, windows-sys, tokio, reqwest, keyring)
├── build.rs                               # CXX-Qt QML module & QObject bridge compiler (Linux Kirigami + Windows compat)
├── io.github.duckydeck.Studio.yml         # Flatpak manifest (org.kde.Platform 6.11 + Rust SDK)
├── packaging/
│   └── windows/
│       ├── DuckyDeckStudio.iss            # Inno Setup 6 installer script for Windows 10/11 (x86_64)
│       └── duckydeck-studio.ico           # Multi-resolution Windows application icon
├── scripts/
│   ├── check.sh                           # Quality gate (rustfmt, clippy, unit tests, architecture checks)
│   ├── build-flatpak.sh                   # Builds & bundles target/release/duckydeck-studio-v0.1.7-x86_64.flatpak
│   ├── build-windows.sh                   # Cross-compiles Windows x86_64 Portable ZIP & Inno Setup .exe installer
│   └── test-flatpak-ui.sh                 # Runs the automated QML UI test suite inside the Flatpak sandbox
├── docs/
│   ├── user-guide.html                    # Interactive Web User Documentation & Manual
│   ├── architecture.html                  # Interactive Web Architecture & Developer Reference
│   ├── REST_API.md                        # HTTP REST API Reference & curl examples
│   ├── openapi.yaml                       # OpenAPI 3.0.3 YAML specification
│   ├── swagger.json                       # OpenAPI 3.0.3 / Swagger JSON specification
│   └── ARCHITECTURE.md                    # Developer Architecture & Design Documentation (Markdown)
├── data/
│   ├── io.github.duckydeck.Studio.desktop      # XDG Desktop entry
│   ├── io.github.duckydeck.Studio.metainfo.xml # AppStream metadata with USB modalias
│   ├── icons/io.github.duckydeck.Studio.svg    # Scalable application icon
│   ├── syntax/duckyscript3.xml                 # KDE KSyntaxHighlighting grammar for duckyScript 3
│   └── udev/
│       ├── 70-duckypad.rules                   # uaccess udev rules for 0x0483:0xd11d & 0xd11c (Linux only)
│       └── install-udev-rules.sh               # Helper script to install & reload udev rules
└── src/
    ├── main.rs                            # Qt 6 entrypoint, D-Bus/NamedPipe IPC & syntax registration
    ├── paths.rs                           # Cross-platform Linux XDG / Flatpak & Windows %APPDATA% path resolver
    ├── api_server.rs                      # Embedded HTTP/1.1 REST API Server (/api/v1/*) with Bearer auth
    ├── actions/                           # Host actions (web_api.rs, app_launch.rs, desktop_apps.rs)
    ├── bridge/                            # #[cxx_qt::bridge] DeviceController & submodules
    ├── cpp/                               # C++ Qt helpers (app_helper.h, syntax_highlighter.h, kirigami_compat.h)
    ├── hardware/                          # USB HID protocol, profile parser, MSC & firmware manager
    ├── vm/compiler.rs                     # Pure-Rust duckyScript 3 -> DSVM v2 (.dsb) bytecode compiler
    ├── kwin/auto_switcher.rs              # Cross-platform active-window listener (KWin Wayland D-Bus & Windows Win32)
    ├── ai/shortcut_scraper.rs             # Local ~/.config/kglobalshortcutsrc parser & DuckyScript converter
    └── qml/                               # Kirigami QML components (Main, HardwareCanvas, Builders, Dialogs)
        ├── kirigami/                      # Self-contained Kirigami QML compatibility layer for Windows builds
        └── AiArchitectModal.qml           # Macro Template Generator (experimental) modal dialog
```

---

## 🛠️ Build & Installation Instructions

### 1. Windows 10 & 11 Installation (`x86_64`)

DuckyDeck Studio runs natively on **Windows 10 and Windows 11 (`x86_64`)** with **zero extra driver installation required**—Windows automatically binds the duckyPad's vendor-defined USB HID interface (`0x0483:0xd11d` and `0x0483:0xd11c`) and Mass Storage microSD drive upon plug-in.

#### Option A: Windows Setup Installer (Recommended)
1. Download **[`DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe`](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe)** from **[GitHub Releases](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7)** or **[duckydeckstudio.com](https://duckydeckstudio.com/releases/DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe)**.
2. Run the installer (`DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe`) — no Administrator privileges are required (installs per-user to `%LOCALAPPDATA%\Programs\DuckyDeck Studio` by default, creates Start Menu and optional Desktop shortcuts, and optionally enables **Autostart at Windows Login**).
3. Launch **DuckyDeck Studio** from the Start Menu.

#### Option B: Windows Portable ZIP (No Installer Required)
1. Download **[`duckydeck-studio-v0.1.7-windows-x86_64-portable.zip`](https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/duckydeck-studio-v0.1.7-windows-x86_64-portable.zip)**.
2. Extract the archive to any folder and run `duckydeck-studio.exe`:
   ```powershell
   Invoke-WebRequest -Uri "https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.7/duckydeck-studio-v0.1.7-windows-x86_64-portable.zip" -OutFile "duckydeck-studio-portable.zip"
   Expand-Archive -Path "duckydeck-studio-portable.zip" -DestinationPath ".\DuckyDeckStudio"
   .\DuckyDeckStudio\duckydeck-studio.exe
   ```

---

### 2. Quick Install on Linux: Prebuilt Flatpak Bundle (Recommended for Any Linux Distribution)

Download and install the self-contained `io.github.duckydeck.Studio` Flatpak bundle directly from **[duckydeckstudio.com](https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-x86_64.flatpak)** or **[GitHub Releases](https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.7)**:

```bash
# 1. Ensure Flathub is configured (for org.kde.Platform//6.11 runtime):
flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo

# 2. Download & install the DuckyDeck Studio v0.1.7 Flatpak bundle:
curl -LO https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-x86_64.flatpak
flatpak install --user -y duckydeck-studio-v0.1.7-x86_64.flatpak

# 3. Launch DuckyDeck Studio (or open from your KDE Application Launcher):
flatpak run io.github.duckydeck.Studio

# 4. Run CLI status or automation commands via Flatpak:
flatpak run io.github.duckydeck.Studio --status
```

---

### 3. Download Prebuilt Linux Tarball or Clone from GitHub

#### Option A: Clone with Git
```bash
git clone https://github.com/russellmilliner/duckydeck-studio.git
cd duckydeck-studio
```

#### Option B: Download Prebuilt Linux x86_64 Tarball or Source ZIP
Visit **[https://github.com/russellmilliner/duckydeck-studio/releases](https://github.com/russellmilliner/duckydeck-studio/releases)** or **[https://duckydeckstudio.com](https://duckydeckstudio.com)**, or download via terminal:
```bash
curl -LO https://duckydeckstudio.com/releases/duckydeck-studio-v0.1.7-linux-x86_64.tar.gz
tar -xzf duckydeck-studio-v0.1.7-linux-x86_64.tar.gz
cd duckydeck-studio-v0.1.7-linux-x86_64
./duckydeck-studio
```

---

### 4. Install Linux Build Prerequisites (When Building from Source)

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

### 5. Configure Linux USB HID Permissions (`udev` Rules)

*(Linux only — not needed on Windows)* To allow non-root access to `/dev/hidraw*` for the duckyPad Pro (`0x0483:0xd11d`) and duckyPad 2020 (`0x0483:0xd11c`), install the included `udev` rules:

```bash
sudo ./data/udev/install-udev-rules.sh
```
*(Alternatively, click the **Install Rules** button inside the app's top Polkit banner on first launch — works in both Native and Flatpak builds.)*

---

### 6. Local Linux Build & User Installation (`~/.local/bin`)

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
install -Dm644 docs/architecture.html \
    ~/.local/share/duckydeck-studio/docs/architecture.html
```

Run the application:
```bash
~/.local/bin/duckydeck-studio
```

---

### 7. Building & Testing the Linux Flatpak & Windows Installer from Source

#### A. Linux Flatpak Bundle (`scripts/build-flatpak.sh`)
1. Install the KDE Plasma 6.11 Flatpak SDK and Rust/LLVM extensions:
   ```bash
   flatpak install --user flathub org.kde.Platform//6.11 org.kde.Sdk//6.11 \
       org.freedesktop.Sdk.Extension.rust-stable//25.08 \
       org.freedesktop.Sdk.Extension.llvm22//25.08
   ```
2. Build and export the single-file `.flatpak` bundle (`target/release/duckydeck-studio-v0.1.7-x86_64.flatpak`) and install it:
   ```bash
   bash scripts/build-flatpak.sh
   flatpak install --user -y --reinstall target/release/duckydeck-studio-v0.1.7-x86_64.flatpak
   ```
3. Run the automated Flatpak QML UI Overflow, Contrast & Behavioral Test Suite:
   ```bash
   bash scripts/test-flatpak-ui.sh
   ```

#### B. Cross-Compiling the Windows `x86_64` Portable ZIP & Inno Setup Installer (`scripts/build-windows.sh`)
DuckyDeck Studio can be cross-compiled for Windows `x86_64` (`x86_64-pc-windows-gnullvm`) directly from Linux using `llvm-mingw` + Qt 6 MinGW64 + Wine Inno Setup:
```bash
# Cross-compile duckydeck-studio.exe, bundle Qt6 runtime DLLs/QML plugins,
# and generate both the Portable ZIP and Inno Setup .exe installer in target/release/:
bash scripts/build-windows.sh
```
Outputs produced in `target/release/`:
- `target/release/DuckyDeck-Studio-Setup-v0.1.7-x86_64.exe` (Inno Setup installer)
- `target/release/duckydeck-studio-v0.1.7-windows-x86_64-portable.zip` (Self-contained portable archive)

---

## 🙏 Special Thanks & Official duckyPad Links

A special thank you to **[dekuNukem](https://github.com/dekuNukem)** for creating the **duckyPad** and **duckyPad Pro**, designing the **DuckyScript 3 / DSVM v2** architecture, and open-sourcing the hardware, firmware, and USB HID specifications for the community!

- **🦆 Official duckyPad Website:** [https://www.duckypad.com](https://www.duckypad.com)
- **🛒 dekuNukem's Official Tindie Store:** [https://www.tindie.com/stores/dekunukem/](https://www.tindie.com/stores/dekunukem/)
- **📦 duckyPad Pro on Tindie:** [https://www.tindie.com/products/dekunukem/duckypad-pro-macro-scripting-beyond-qmkvia/](https://www.tindie.com/products/dekunukem/duckypad-pro-macro-scripting-beyond-qmkvia/)
- **💻 duckyPad Pro GitHub Repository:** [https://github.com/dekuNukem/duckyPad-Pro](https://github.com/dekuNukem/duckyPad-Pro)

---

## ☕ Support the Project

If **DuckyDeck Studio** makes your duckyPad workflow better on Linux or Windows, consider buying Russell Milliner a coffee!

- **Website & User Manual:** [https://duckydeckstudio.com](https://duckydeckstudio.com)
- **GitHub:** [https://github.com/russellmilliner/duckydeck-studio](https://github.com/russellmilliner/duckydeck-studio)
- **Ko-fi:** [https://ko-fi.com/russellmilliner](https://ko-fi.com/russellmilliner)

