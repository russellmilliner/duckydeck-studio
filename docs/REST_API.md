# DuckyDeck Studio HTTP REST API Reference

DuckyDeck Studio includes a built-in, low-latency HTTP/1.1 REST API server (`src/api_server.rs`) that exposes full programmatic control over your **duckyPad Pro** (`0x0483:0xd11d`) and **duckyPad (2020)** (`0x0483:0xd11c`), profiles, mechanical key & rotary encoder slots, duckyScript 3 DSVM compiler, Persistent Global Variables (PGV), live RGB LED overrides, and microSD synchronization.

- **Online Documentation & User Guide**: [https://duckydeckstudio.com](https://duckydeckstudio.com)
- **OpenAPI 3.0.3 YAML Specification**: [`docs/openapi.yaml`](openapi.yaml) (`GET /api/v1/openapi.yaml`)
- **Swagger / OpenAPI 3.0 JSON Specification**: [`docs/swagger.json`](swagger.json) (`GET /api/v1/swagger.json` or `GET /api/v1/openapi.json`)

---

## 1. Enabling & Configuring the Server

1. Open **DuckyDeck Studio** and click **Settings** (`⚙`) in the top toolbar.
2. Scroll to the **HTTP REST API Server (Local & LAN Control)** card:
   - **Enable HTTP REST API Server**: Toggle on to start the background listener immediately.
   - **Bind IP Address**:
     - `127.0.0.1` *(Default)* — Localhost only; accessible only from processes on the same machine.
     - `0.0.0.0` — Listen on all network interfaces (LAN / Home Assistant / Node-RED / Companion).
   - **TCP Port**: Default is `28491` (`1024..65535`).
   - **Require API Token Authentication**: Enabled by default. Uses a 48-character random hexadecimal token stored in `~/.config/duckydeck/switcher_rules.json`.

---

## 2. Authentication & Security

When **Require API Token Authentication** is enabled, all endpoints (except `GET /api/v1/health`, `GET /api/v1/openapi.json`, `GET /api/v1/swagger.json`, and `GET /api/v1/openapi.yaml`) require your API token in the request headers.

You may supply the token via either header:

```http
Authorization: Bearer <YOUR_API_TOKEN>
```

or:

```http
X-API-Token: <YOUR_API_TOKEN>
```

- **Constant-Time Verification**: Token verification uses a constant-time byte comparison (`constant_time_eq`) to prevent timing side-channel attacks.
- **CORS Support**: All responses include permissive CORS headers (`Access-Control-Allow-Origin: *`, `Access-Control-Allow-Methods: GET, POST, PUT, PATCH, DELETE, OPTIONS`, `Access-Control-Allow-Headers: Authorization, Content-Type, X-API-Token`) and handle `OPTIONS` preflight requests with `204 No Content`.

---

## 3. Endpoint Summary

| Method | Endpoint | Auth | Description |
| :--- | :--- | :---: | :--- |
| `GET` | `/api/v1/health` | No | Liveness probe, version, and whether auth is required |
| `GET` | `/api/v1/openapi.json` | No | OpenAPI 3.0 / Swagger JSON specification |
| `GET` | `/api/v1/swagger.json` | No | Alias for `/api/v1/openapi.json` |
| `GET` | `/api/v1/openapi.yaml` | No | OpenAPI 3.0 YAML specification |
| `GET` | `/api/v1/status` *(or `/api/v1`)* | Yes | Full hardware connection, firmware, SD card, active profile, and auto-switcher status |
| `GET` | `/api/v1/profiles` | Yes | List all loaded profiles (1..64) and active profile number |
| `POST` | `/api/v1/profiles` | Yes | Create a new profile or duplicate an existing profile (`duplicateFrom`) |
| `POST` | `/api/v1/profiles/select` | Yes | Select a profile in Studio and optionally switch physical duckyPad (`switchHardware`) |
| `POST` | `/api/v1/profiles/reorder` | Yes | Move `fromProfile` to `toProfile` and renumber profile folders |
| `GET` | `/api/v1/profiles/{number}` | Yes | Get full profile properties and all 26 key/encoder slots |
| `PUT` / `PATCH` | `/api/v1/profiles/{number}` | Yes | Update profile properties (name, colors, OLED & Desktop Widget rotation, webhook) |
| `DELETE` | `/api/v1/profiles/{number}` | Yes | Delete profile `{number}` |
| `GET` | `/api/v1/profiles/{number}/slots` | Yes | List all 26 slots (`1..20` keys, `21..26` rotary encoders) in profile `{number}` |
| `GET` | `/api/v1/profiles/{number}/slots/{slot}` | Yes | Get configuration and duckyScript for slot `{slot}` (`1..26`) |
| `PUT` / `POST` / `PATCH` | `/api/v1/profiles/{number}/slots/{slot}` | Yes | Update slot labels (`l1`, `l2`, `wl1`, `wl2`), RGB color, script, flags, or template |
| `DELETE` | `/api/v1/profiles/{number}/slots/{slot}` | Yes | Clear slot `{slot}` back to unconfigured state |
| `POST` | `/api/v1/profiles/{number}/slots/{slot}/trigger` | Yes | Remotely execute the host action (`@HOST_APP`, `@HA_ACTION`, `@WEBHOOK`) on slot `{slot}` |
| `POST` | `/api/v1/dsvm/compile` | Yes | Compile & validate duckyScript 3 source code into `.dsb` bytecode |
| `GET` | `/api/v1/templates` | Yes | List all built-in and user-saved key templates |
| `GET` | `/api/v1/pgv` | Yes | Read all 16 Persistent Global Variables (`PGV 1..16`) |
| `PUT` / `POST` | `/api/v1/pgv` | Yes | Update one or more Persistent Global Variables (`PGV 1..16`) |
| `POST` | `/api/v1/leds` | Yes | Live-override per-key RGB LEDs over USB HID or restore profile LEDs |
| `POST` | `/api/v1/sd/sync` | Yes | Start async microSD profile & `.dsb` sync (`202 Accepted` + `jobId`) |
| `POST` | `/api/v1/sd/reload` | Yes | Reload profiles from microSD card or local cache |
| `POST` | `/api/v1/sd/mount` | Yes | Start async duckyPad Pro USB Mass Storage mount (`202 Accepted` + `jobId`) |
| `POST` | `/api/v1/sd/unmount` | Yes | Start async microSD flush, unmount, and eject (`202 Accepted` + `jobId`) |
| `POST` | `/api/v1/device/sleep` | Yes | Put connected duckyPad into sleep mode |
| `POST` | `/api/v1/device/wake` | Yes | Wake connected duckyPad from sleep mode |
| `POST` | `/api/v1/device/reboot` | Yes | Start async soft-reboot into normal operating mode (`202 Accepted` + `jobId`) |
| `POST` | `/api/v1/device/sync-rtc` | Yes | Synchronize duckyPad Pro real-time clock (RTC) with host system time |
| `GET` | `/api/v1/jobs/{id}` | Yes | Poll status (`running`, `completed`, `failed`) of an async job |
| `GET` | `/api/v1/logs` | Yes | Retrieve recent system, HID, script, and automation event logs |

---

## 4. Detailed Endpoint Reference & Examples

Assume the following environment variables for the `curl` examples below:

```bash
export DUCKY_URL="http://127.0.0.1:28491"
export DUCKY_TOKEN="your-api-token-here"
```

### 4.1 Health & Status

#### `GET /api/v1/health`
Unauthenticated health probe.

```bash
curl -s "$DUCKY_URL/api/v1/health"
```

```json
{
  "ok": true,
  "service": "duckydeck-studio",
  "version": "0.1.24",
  "authRequired": true
}
```

#### `GET /api/v1/status`
Returns full device and application state.

```bash
curl -s -H "Authorization: Bearer $DUCKY_TOKEN" "$DUCKY_URL/api/v1/status"
```

```json
{
  "ok": true,
  "version": "0.1.24",
  "connected": true,
  "sdCardMode": false,
  "sdMounted": false,
  "sdRootPath": "",
  "sdSpaceSummary": "SD: 14.8 GB free / 15.9 GB (7%)",
  "sdSpaceUsedPercent": 7,
  "sdSyncBusy": false,
  "modelName": "duckyPad Pro",
  "firmwareVersion": "1.0.0-studio",
  "latestFirmwareVersion": "1.0.0",
  "firmwareUpdateAvailable": false,
  "serialNumber": "DP24A1B2",
  "activeProfile": 1,
  "activeProfileName": "Default",
  "activeProfileBgColor": "#38bdf8",
  "profileCount": 6,
  "autoSwitcherEnabled": true,
  "activeWindowClass": "org.kde.konsole",
  "activeWindowTitle": "konsole",
  "statusMessage": "Connected to duckyPad Pro"
}
```

---

### 4.2 Profiles

#### `GET /api/v1/profiles`
Lists all loaded profiles and their orientation/color summaries.

```bash
curl -s -H "Authorization: Bearer $DUCKY_TOKEN" "$DUCKY_URL/api/v1/profiles"
```

#### `POST /api/v1/profiles/select`
Switches the active profile in DuckyDeck Studio and sends a USB HID profile switch command to the hardware (`switchHardware: true` by default).

```bash
curl -s -X POST "$DUCKY_URL/api/v1/profiles/select" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"profile": 2, "switchHardware": true}'
```

#### `POST /api/v1/profiles`
Creates a new profile or duplicates an existing profile (`duplicateFrom`).

```bash
curl -s -X POST "$DUCKY_URL/api/v1/profiles" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "Streaming", "duplicateFrom": 1}'
```

#### `PUT /api/v1/profiles/{number}` (or `PATCH`)
Updates profile properties (colors, hardware OLED rotation `0|90|180|270`, Desktop Widget rotation `-1|0|90|180|270`, and profile switch webhook). Partial JSON payloads are merged with existing profile settings.

```bash
curl -s -X PATCH "$DUCKY_URL/api/v1/profiles/1" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "bgColor": "#0ea5e9",
    "oledRotation": 90,
    "widgetRotation": 90,
    "dimUnusedKeys": true
  }'
```

---

### 4.3 Key & Rotary Encoder Slots (`1..26`)

Slots `1..20` correspond to the 20 mechanical RGB keys. Slots `21..26` correspond to the two rotary encoders:
- `21`: Rotary Encoder 1 Counter-Clockwise (CCW)
- `22`: Rotary Encoder 1 Clockwise (CW)
- `23`: Rotary Encoder 1 Push Button
- `24`: Rotary Encoder 2 Counter-Clockwise (CCW)
- `25`: Rotary Encoder 2 Clockwise (CW)
- `26`: Rotary Encoder 2 Push Button

#### `PUT /api/v1/profiles/{number}/slots/{slot}`
Updates a slot's hardware OLED labels (`labelLine1`/`l1`, `labelLine2`/`l2`), extended Desktop Widget labels (`widgetLabelLine1`/`wl1`, `widgetLabelLine2`/`wl2`), RGB key color (`colorHex` or `r`/`g`/`b`), duckyScript 3 source (`scriptContent`/`script`), or host command (`hostApp`/`hostCmd`).

```bash
curl -s -X PUT "$DUCKY_URL/api/v1/profiles/1/slots/5" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "labelLine1": "Mute",
    "labelLine2": "Mic",
    "widgetLabelLine1": "Toggle Mic",
    "widgetLabelLine2": "PipeWire",
    "colorHex": "#ef4444",
    "scriptContent": "// Host-handled action\n",
    "hostApp": "wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle"
  }'
```

#### `POST /api/v1/profiles/{number}/slots/{slot}/trigger`
Triggers the host-side action (`# @HOST_APP`, `# @HA_ACTION`, or `# @WEBHOOK`) configured on a slot.

```bash
curl -s -X POST "$DUCKY_URL/api/v1/profiles/1/slots/5/trigger" \
  -H "Authorization: Bearer $DUCKY_TOKEN"
```

---

### 4.4 duckyScript 3 DSVM Compiler

#### `POST /api/v1/dsvm/compile`
Compiles duckyScript 3 source code into duckyPad Pro `.dsb` bytecode and returns line-by-line diagnostics, bytecode size, CRC32 checksum, and hex-encoded `.dsb` bytes.

```bash
curl -s -X POST "$DUCKY_URL/api/v1/dsvm/compile" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"script": "VAR $x = 5\nWHILE $x > 0\n  STRINGLN Count: $x\n  $x -= 1\nEND_WHILE"}'
```

---

### 4.5 Live RGB LED Control & Device Operations

#### `POST /api/v1/leds`
Overrides individual or all 20 mechanical key RGB LEDs in real time over USB HID, or restores the profile's default LED colors (`"restore": true`).

```bash
# Set Key 1 to green and Key 2 to red:
curl -s -X POST "$DUCKY_URL/api/v1/leds" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "leds": [
      {"slot": 1, "colorHex": "#22c55e"},
      {"slot": 2, "colorHex": "#ef4444"}
    ]
  }'

# Restore profile LED colors:
curl -s -X POST "$DUCKY_URL/api/v1/leds" \
  -H "Authorization: Bearer $DUCKY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"restore": true}'
```

#### `POST /api/v1/device/sleep`, `/wake`, `/reboot`, `/sync-rtc`
Controls hardware power state and synchronizes the duckyPad Pro internal Real-Time Clock (RTC) with the host computer's local time.

```bash
curl -s -X POST -H "Authorization: Bearer $DUCKY_TOKEN" "$DUCKY_URL/api/v1/device/sync-rtc"
```
