import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Item {
    id: canvasRoot
    implicitWidth: 520
    implicitHeight: 450
    clip: true

    property var controller: null
    property int selectedSlot: 0
    property bool isRotated: false
    property bool showExpansion: false
    property bool oledScriptMode: false
    property int activeProfileOledRotation: 0
    readonly property bool isLightTheme: (0.2126 * Kirigami.Theme.backgroundColor.r + 0.7152 * Kirigami.Theme.backgroundColor.g + 0.0722 * Kirigami.Theme.backgroundColor.b) > 0.45
    readonly property color highContrastHighlightText: isLightTheme ? "#0369a1" : Kirigami.Theme.highlightColor

    property string enc1CcwLabel: "Size -"
    property string enc1CwLabel: "Size +"
    property string enc1PushLabel: "Invert"
    property string enc2CcwLabel: "Zoom -"
    property string enc2CwLabel: "Zoom +"
    property string enc2PushLabel: "Frame"

    signal slotSelected(int slotNumber, string slotTitle, color slotColor)
    signal beforeSlotMutationRequested()
    signal slotContentMutated(int newSelectedSlot)
    signal renameProfileRequested(int profileNumber, string currentName)
    signal keyColorPickerRequested(int slotNumber, string currentColorHex)
    signal profileDefaultColorPickerRequested(int profileNumber, string profileName, string currentColorHex)
    signal openLightingAnimatorRequested(string targetScope, int targetSlot)
    signal openKeyPropertiesRequested(int slotNumber)

    // Gaming-Keyboard RGB Animation Engine State (Profile-level + Individual Key-level)
    property var profileAnimationConfig: ({
        enabled: false,
        trigger: "idle",       // "idle", "keypress", "both"
        preset: "wave_h",      // "pulse", "wave_h", "wave_v", "ripple", "starlight", "spectrum", "scanner"
        speed: 5,              // 1..10
        color1: "#00f0ff",
        color2: "#a855f7",
        idleTimeoutSec: 5,
        streamToHardware: true
    })
    property var keyAnimationConfigs: ({}) // map of slot ("1".."20") -> config object
    property real animPhase: 0.0
    property real lastActivityMs: Date.now()
    property bool idleAnimationActive: false
    property var activeKeyBursts: ([]) // [{ slot: 1..20, startMs: ..., preset: ..., color1: ..., color2: ..., speed: ..., scope: "key"|"profile" }]
    property int hwStreamTickCounter: 0

    function keyHasCustomAnimation(slotNum) {
        if (!canvasRoot.keyAnimationConfigs || slotNum < 1 || slotNum > 20) return false
        const cfg = canvasRoot.keyAnimationConfigs[String(slotNum)]
        return !!(cfg && cfg.enabled)
    }

    function visibleOledChars(str) {
        if (!str) return []
        const raw = Array.from(String(str))
        const out = []
        for (let i = 0; i < raw.length; ++i) {
            const ch = raw[i]
            if (ch === "\uFE0E" || ch === "\uFE0F" || ch === "\u200D") {
                if (out.length > 0) out[out.length - 1] += ch
                continue
            }
            out.push(ch)
        }
        return out
    }

    function sliceOledLabel(str, maxChars) {
        const chars = canvasRoot.visibleOledChars(str)
        return chars.slice(0, maxChars || 5).join("")
    }

    function isSingleIconLabel(str) {
        const trimmed = String(str || "").trim()
        if (!trimmed) return false
        if (trimmed === ">_" || trimmed === "{}") return true
        const chars = canvasRoot.visibleOledChars(trimmed)
        if (chars.length !== 1) return false
        const cp = chars[0].codePointAt(0)
        return cp > 127
    }

    function notifyUserActivity(pressedSlot) {
        const now = Date.now()
        const wasIdle = canvasRoot.idleAnimationActive
        canvasRoot.lastActivityMs = now
        canvasRoot.idleAnimationActive = false

        if (pressedSlot >= 1 && pressedSlot <= 20) {
            const keyCfg = canvasRoot.keyAnimationConfigs[String(pressedSlot)]
            const profCfg = canvasRoot.profileAnimationConfig
            if (keyCfg && keyCfg.enabled && (keyCfg.trigger === "keypress" || keyCfg.trigger === "both")) {
                canvasRoot.triggerKeypressBurst(pressedSlot, keyCfg, "key")
            } else if (profCfg && profCfg.enabled && (profCfg.trigger === "keypress" || profCfg.trigger === "both")) {
                canvasRoot.triggerKeypressBurst(pressedSlot, profCfg, "profile")
            }
        }
        if (wasIdle) {
            canvasRoot.resetAnimatedKeyColors()
        }
    }

    function triggerKeypressBurst(slotNum, cfg, scope) {
        if (!cfg) return
        const list = (canvasRoot.activeKeyBursts || []).slice()
        list.push({
            slot: slotNum,
            startMs: Date.now(),
            preset: cfg.preset || "ripple",
            color1: cfg.color1 || "#00f0ff",
            color2: cfg.color2 || "#a855f7",
            speed: cfg.speed || 5,
            scope: scope || "profile",
            streamToHardware: cfg.streamToHardware !== false
        })
        if (list.length > 8) list.shift()
        canvasRoot.activeKeyBursts = list
        if (!rgbAnimTimer.running) rgbAnimTimer.start()
    }

    function parseHexRgb(hexStr, fallbackR, fallbackG, fallbackB) {
        const s = String(hexStr || "").trim().replace(/^#/, "")
        if (s.length === 6) {
            const r = parseInt(s.slice(0, 2), 16)
            const g = parseInt(s.slice(2, 4), 16)
            const b = parseInt(s.slice(4, 6), 16)
            if (!isNaN(r) && !isNaN(g) && !isNaN(b)) return [r, g, b]
        }
        return [fallbackR, fallbackG, fallbackB]
    }

    function hsvToRgb(h, s, v) {
        const hh = ((h % 1.0) + 1.0) % 1.0
        const i = Math.floor(hh * 6)
        const f = hh * 6 - i
        const p = v * (1 - s)
        const q = v * (1 - f * s)
        const t = v * (1 - (1 - f) * s)
        let r = 0, g = 0, b = 0
        switch (i % 6) {
            case 0: r = v; g = t; b = p; break
            case 1: r = q; g = v; b = p; break
            case 2: r = p; g = v; b = t; break
            case 3: r = p; g = q; b = v; break
            case 4: r = t; g = p; b = v; break
            case 5: r = v; g = p; b = q; break
        }
        return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)]
    }

    function evalPresetRgb(preset, col, row, phase, c1, c2, baseR, baseG, baseB, originCol, originRow, progress01) {
        if (preset === "pulse") {
            if (originCol >= 0 && originCol <= 3) {
                // Keypress burst: smooth breathing pulse across entire pad from base to c1 and back
                const blend = Math.sin(Math.min(1.0, Math.max(0.0, progress01)) * Math.PI)
                return [
                    Math.round(baseR * (1.0 - blend) + c1[0] * blend),
                    Math.round(baseG * (1.0 - blend) + c1[1] * blend),
                    Math.round(baseB * (1.0 - blend) + c1[2] * blend)
                ]
            }
            const w = 0.5 + 0.5 * Math.sin(phase * Math.PI * 2.0)
            return [
                Math.round(c2[0] + (c1[0] - c2[0]) * w),
                Math.round(c2[1] + (c1[1] - c2[1]) * w),
                Math.round(c2[2] + (c1[2] - c2[2]) * w)
            ]
        } else if (preset === "wave_h") {
            if (originCol >= 0 && originCol <= 3) {
                // Keypress burst: sweeping wave left to right across all columns
                const waveCol = progress01 * 5.0 - 0.5
                const dist = Math.abs(col - waveCol)
                const band = Math.max(0.0, 1.0 - dist / 1.1)
                return [
                    Math.round(baseR * (1.0 - band) + c1[0] * band),
                    Math.round(baseG * (1.0 - band) + c1[1] * band),
                    Math.round(baseB * (1.0 - band) + c1[2] * band)
                ]
            }
            const w = 0.5 + 0.5 * Math.sin((phase - col * 0.22) * Math.PI * 2.0)
            return [
                Math.round(c2[0] + (c1[0] - c2[0]) * w),
                Math.round(c2[1] + (c1[1] - c2[1]) * w),
                Math.round(c2[2] + (c1[2] - c2[2]) * w)
            ]
        } else if (preset === "wave_v") {
            if (originCol >= 0 && originCol <= 3) {
                // Keypress burst: sweeping cascade top to bottom down all rows
                const waveRow = progress01 * 6.0 - 0.5
                const dist = Math.abs(row - waveRow)
                const band = Math.max(0.0, 1.0 - dist / 1.1)
                return [
                    Math.round(baseR * (1.0 - band) + c1[0] * band),
                    Math.round(baseG * (1.0 - band) + c1[1] * band),
                    Math.round(baseB * (1.0 - band) + c1[2] * band)
                ]
            }
            const w = 0.5 + 0.5 * Math.sin((phase - row * 0.20) * Math.PI * 2.0)
            return [
                Math.round(c2[0] + (c1[0] - c2[0]) * w),
                Math.round(c2[1] + (c1[1] - c2[1]) * w),
                Math.round(c2[2] + (c1[2] - c2[2]) * w)
            ]
        } else if (preset === "spectrum") {
            const hue = (phase + col * 0.08 + row * 0.06) % 1.0
            const rgb = canvasRoot.hsvToRgb(hue, 0.95, 1.0)
            if (originCol >= 0 && originCol <= 3) {
                const blend = Math.sin(Math.min(1.0, Math.max(0.0, progress01)) * Math.PI)
                return [
                    Math.round(baseR * (1.0 - blend) + rgb[0] * blend),
                    Math.round(baseG * (1.0 - blend) + rgb[1] * blend),
                    Math.round(baseB * (1.0 - blend) + rgb[2] * blend)
                ]
            }
            return rgb
        } else if (preset === "scanner") {
            if (originCol >= 0 && originCol <= 3) {
                // Keypress burst: ping-pong sweep across columns (0 -> 3 -> 0)
                const pingPong = progress01 <= 0.5 ? (progress01 * 6.0) : ((1.0 - progress01) * 6.0)
                const dist = Math.abs(col - pingPong)
                const intensity = Math.max(0.0, 1.0 - dist * 0.75) * (1.0 - progress01 * 0.4)
                return [
                    Math.round(baseR * (1.0 - intensity) + c1[0] * intensity),
                    Math.round(baseG * (1.0 - intensity) + c1[1] * intensity),
                    Math.round(baseB * (1.0 - intensity) + c1[2] * intensity)
                ]
            }
            const pingPong = 1.5 + 1.5 * Math.sin(phase * Math.PI * 2.0)
            const dist = Math.abs(col - pingPong)
            const intensity = Math.max(0.0, 1.0 - dist * 0.65)
            return [
                Math.round(baseR * (1.0 - intensity) + c1[0] * intensity),
                Math.round(baseG * (1.0 - intensity) + c1[1] * intensity),
                Math.round(baseB * (1.0 - intensity) + c1[2] * intensity)
            ]
        } else if (preset === "ripple") {
            const dist = Math.hypot(col - originCol, row - originRow)
            const ringRadius = progress01 * 5.2
            const ringDist = Math.abs(dist - ringRadius)
            const band = Math.max(0.0, 1.0 - ringDist / 1.15) * (1.0 - progress01 * 0.35)
            return [
                Math.round(baseR * (1.0 - band) + c1[0] * band),
                Math.round(baseG * (1.0 - band) + c1[1] * band),
                Math.round(baseB * (1.0 - band) + c1[2] * band)
            ]
        } else if (preset === "starlight") {
            const decay = Math.max(0.0, 1.0 - progress01)
            if (originCol >= 0 && originCol <= 3) {
                // Keypress burst across whole duckyPad:
                // Pressed key has an immediate bright flare that smoothly decays
                const isOrigin = (col === originCol && row === originRow)
                // Deterministic pseudo-random twinkle per key based on coordinates & progress
                const keySeed = (col * 7 + row * 13 + originCol * 3 + originRow * 5) % 17
                const twinkle = Math.sin((progress01 * 12.0 + keySeed) * Math.PI)
                const sparkle = isOrigin
                    ? decay
                    : (Math.max(0.0, twinkle) * decay * 0.85)
                return [
                    Math.round(baseR * (1.0 - sparkle) + c1[0] * sparkle),
                    Math.round(baseG * (1.0 - sparkle) + c1[1] * sparkle),
                    Math.round(baseB * (1.0 - sparkle) + c1[2] * sparkle)
                ]
            }
            // Ambient / idle starlight mode
            const seed = Math.sin((col * 7 + row * 13) * 1.7 + phase * 6.28)
            const sparkle = Math.max(0.0, (seed - 0.35) / 0.65)
            return [
                Math.round(baseR * (1.0 - sparkle) + c1[0] * sparkle),
                Math.round(baseG * (1.0 - sparkle) + c1[1] * sparkle),
                Math.round(baseB * (1.0 - sparkle) + c1[2] * sparkle)
            ]
        }
        return [baseR, baseG, baseB]
    }

    function resetAnimatedKeyColors() {
        const hwFrame = []
        for (let i = 0; i < keyModel.count; ++i) {
            const it = keyModel.get(i)
            keyModel.setProperty(i, "animR", it.r)
            keyModel.setProperty(i, "animG", it.g)
            keyModel.setProperty(i, "animB", it.b)
            keyModel.setProperty(i, "animActive", false)
            hwFrame.push({ slot: i + 1, r: it.r, g: it.g, b: it.b })
        }
        if (canvasRoot.controller && canvasRoot.controller.connected) {
            canvasRoot.controller.pushHardwareLedFrameJson(JSON.stringify(hwFrame))
        }
    }

    Timer {
        id: rgbAnimTimer
        interval: 55
        repeat: true
        running: true
        onTriggered: {
            if (keyModel.count < 20) return
            const now = Date.now()
            const profCfg = canvasRoot.profileAnimationConfig || {}
            const keyCfgs = canvasRoot.keyAnimationConfigs || {}

            // Check if any idle animation is configured on the profile or any key
            let anyIdleConfigured = !!(profCfg.enabled && (profCfg.trigger === "idle" || profCfg.trigger === "both"))
            let minIdleTimeoutMs = Math.max(1, (profCfg.idleTimeoutSec !== undefined ? profCfg.idleTimeoutSec : 5)) * 1000
            for (let k in keyCfgs) {
                const kc = keyCfgs[k]
                if (kc && kc.enabled && (kc.trigger === "idle" || kc.trigger === "both")) {
                    anyIdleConfigured = true
                    const kt = Math.max(1, (kc.idleTimeoutSec !== undefined ? kc.idleTimeoutSec : 5)) * 1000
                    if (kt < minIdleTimeoutMs) minIdleTimeoutMs = kt
                }
            }

            if (anyIdleConfigured && (now - canvasRoot.lastActivityMs >= minIdleTimeoutMs)) {
                canvasRoot.idleAnimationActive = true
            } else if (!anyIdleConfigured) {
                canvasRoot.idleAnimationActive = false
            }

            // Prune expired keypress bursts
            const activeBursts = []
            const bursts = canvasRoot.activeKeyBursts || []
            for (let b = 0; b < bursts.length; ++b) {
                const item = bursts[b]
                const durMs = Math.max(350, 1400 - (item.speed || 5) * 95)
                if (now - item.startMs < durMs) {
                    activeBursts.push(item)
                }
            }
            if (activeBursts.length !== bursts.length) {
                canvasRoot.activeKeyBursts = activeBursts
                if (activeBursts.length === 0 && !canvasRoot.idleAnimationActive) {
                    canvasRoot.resetAnimatedKeyColors()
                    return
                }
            }

            if (!canvasRoot.idleAnimationActive && activeBursts.length === 0) {
                return
            }

            const speedMult = (profCfg.speed || 5) * 0.012
            canvasRoot.animPhase = (canvasRoot.animPhase + speedMult) % 100.0

            const profC1 = canvasRoot.parseHexRgb(profCfg.color1, 0, 240, 255)
            const profC2 = canvasRoot.parseHexRgb(profCfg.color2, 168, 85, 247)
            let shouldStreamHw = false
            const hwFrame = []

            for (let i = 0; i < 20; ++i) {
                const slot = i + 1
                const col = i % 4
                const row = Math.floor(i / 4)
                const baseItem = keyModel.get(i)
                let outR = baseItem.r
                let outG = baseItem.g
                let outB = baseItem.b
                let isAnim = false

                // 1. Idle layer (Profile idle, then Key-specific idle override)
                if (canvasRoot.idleAnimationActive) {
                    if (profCfg.enabled && (profCfg.trigger === "idle" || profCfg.trigger === "both")) {
                        const rgb = canvasRoot.evalPresetRgb(
                            profCfg.preset || "wave_h",
                            col, row,
                            canvasRoot.animPhase,
                            profC1, profC2,
                            baseItem.r, baseItem.g, baseItem.b,
                            1.5, 2.0,
                            (canvasRoot.animPhase % 1.0)
                        )
                        outR = rgb[0]; outG = rgb[1]; outB = rgb[2]
                        isAnim = true
                        if (profCfg.streamToHardware !== false) shouldStreamHw = true
                    }
                    const kc = keyCfgs[String(slot)]
                    if (kc && kc.enabled && (kc.trigger === "idle" || kc.trigger === "both")) {
                        const kc1 = canvasRoot.parseHexRgb(kc.color1, 0, 240, 255)
                        const kc2 = canvasRoot.parseHexRgb(kc.color2, baseItem.r, baseItem.g, baseItem.b)
                        const kPhase = (now * 0.001 * (kc.speed || 5) * 0.45) % 100.0
                        const rgb = canvasRoot.evalPresetRgb(
                            kc.preset || "pulse",
                            col, row,
                            kPhase,
                            kc1, kc2,
                            baseItem.r, baseItem.g, baseItem.b,
                            col, row,
                            (kPhase % 1.0)
                        )
                        outR = rgb[0]; outG = rgb[1]; outB = rgb[2]
                        isAnim = true
                        if (kc.streamToHardware !== false) shouldStreamHw = true
                    }
                }

                // 2. Active Keypress Bursts layer (applies across full duckyPad)
                for (let b = 0; b < activeBursts.length; ++b) {
                    const burst = activeBursts[b]
                    const durMs = Math.max(350, 1400 - (burst.speed || 5) * 95)
                    const prog = Math.min(1.0, Math.max(0.0, (now - burst.startMs) / durMs))
                    const oCol = (burst.slot - 1) % 4
                    const oRow = Math.floor((burst.slot - 1) / 4)
                    const bc1 = canvasRoot.parseHexRgb(burst.color1, 0, 240, 255)
                    const bc2 = canvasRoot.parseHexRgb(burst.color2, baseItem.r, baseItem.g, baseItem.b)
                    const rgb = canvasRoot.evalPresetRgb(
                        burst.preset || "ripple",
                        col, row,
                        prog * 2.0,
                        bc1, bc2,
                        outR, outG, outB,
                        oCol, oRow,
                        prog
                    )
                    outR = rgb[0]; outG = rgb[1]; outB = rgb[2]
                    isAnim = true
                    if (burst.streamToHardware !== false) shouldStreamHw = true
                }

                keyModel.setProperty(i, "animR", outR)
                keyModel.setProperty(i, "animG", outG)
                keyModel.setProperty(i, "animB", outB)
                keyModel.setProperty(i, "animActive", isAnim)
                hwFrame.push({ slot: slot, r: outR, g: outG, b: outB })
            }

            if (shouldStreamHw && canvasRoot.controller && canvasRoot.controller.connected) {
                const hasBursts = (canvasRoot.activeKeyBursts && canvasRoot.activeKeyBursts.length > 0)
                if (hasBursts) {
                    // Stream every tick during reactive key bursts for fluid shockwave animation
                    canvasRoot.controller.pushHardwareLedFrameJson(JSON.stringify(hwFrame))
                } else {
                    canvasRoot.hwStreamTickCounter = (canvasRoot.hwStreamTickCounter + 1) % 3
                    if (canvasRoot.hwStreamTickCounter === 0) {
                        canvasRoot.controller.pushHardwareLedFrameJson(JSON.stringify(hwFrame))
                    }
                }
            }
        }
    }

    property int contextMenuSlot: 1
    property int draggingSlot: -1
    property int dropTargetSlot: -1
    property bool dragShiftMode: false

    function clearSelection() {
        if (canvasRoot.selectedSlot !== 0) {
            canvasRoot.selectedSlot = 0
            canvasRoot.slotSelected(0, "", Kirigami.Theme.highlightColor)
            oledSimulator.requestPaint()
        }
    }

    readonly property real canvasScale: 1.0
    readonly property real uiFontScale: (canvasRoot.controller && canvasRoot.controller.uiTextScale > 0
        ? canvasRoot.controller.uiTextScale
        : 100) / 100.0

    readonly property int requiredWidthForFullHeight: Math.max(
        500,
        Math.ceil(topToolbarRow.implicitWidth + 12),
        Math.ceil(((Math.max(320, canvasRoot.height - topToolbarRow.implicitHeight - mainCol.spacing) - 20) / 354.0) * 430.0 + 20)
    )

    property var profileLabels: []
    property var profileNumbers: []
    property int currentProfileIndex: 0
    property var expansionLabels: ({})

    ListModel {
        id: keyModel
        Component.onCompleted: {
            for (let i = 1; i <= 20; ++i) {
                append({
                    slot: i,
                    l1: "",
                    l2: "",
                    r: 0,
                    g: 170,
                    b: 255,
                    animR: 0,
                    animG: 170,
                    animB: 255,
                    animActive: false,
                    hasAnim: false,
                    hasRel: false,
                    hasLoop: false,
                    hasWebApi: false,
                    hasAppLaunch: false
                })
            }
            canvasRoot.syncProfilesFromController()
            canvasRoot.syncSlotsFromController()
        }
    }

    onControllerChanged: {
        syncProfilesFromController()
        syncSlotsFromController()
    }

    Connections {
        target: canvasRoot.controller
        function onActiveProfileSlotsJsonChanged() {
            canvasRoot.syncSlotsFromController()
        }
        function onProfilesJsonChanged() {
            canvasRoot.syncProfilesFromController()
        }
        function onActiveProfileChanged() {
            canvasRoot.syncActiveProfileSelection()
            oledSimulator.requestPaint()
        }
        function onActiveProfileNameChanged() {
            oledSimulator.requestPaint()
        }
    }

    function syncActiveProfileSelection() {
        if (!controller || !profileNumbers || profileNumbers.length === 0) return
        const idx = profileNumbers.indexOf(controller.activeProfile)
        if (idx >= 0 && canvasRoot.currentProfileIndex !== idx) {
            canvasRoot.currentProfileIndex = idx
        }
        canvasRoot.syncProfilesFromController()
    }

    function syncProfilesFromController() {
        if (!controller || !controller.profilesJson) return
        try {
            const arr = JSON.parse(controller.profilesJson)
            if (!Array.isArray(arr)) return
            const labels = []
            const nums = []
            let activeIdx = 0
            let activeRot = 0
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                nums.push(p.number)
                labels.push("P" + p.number + ": " + p.name)
                if (p.number === controller.activeProfile) {
                    activeIdx = i
                    activeRot = p.oledRotation !== undefined ? p.oledRotation : (p.isLandscape ? 90 : 0)
                }
            }
            canvasRoot.profileNumbers = nums
            canvasRoot.profileLabels = labels
            canvasRoot.currentProfileIndex = activeIdx
            canvasRoot.activeProfileOledRotation = activeRot
        } catch (e) {}
    }

    function formatEncoderLabel(item, fallback) {
        const l1 = (item.l1 || "").trim()
        const l2 = (item.l2 || "").trim()
        const combined = (l1 + (l2.length > 0 ? " " + l2 : "")).trim()
        return combined.length > 0 ? combined : fallback
    }

    function syncSlotsFromController() {
        if (!controller || !controller.activeProfileSlotsJson || keyModel.count < 20) return
        try {
            const arr = JSON.parse(controller.activeProfileSlotsJson)
            if (!Array.isArray(arr)) return
            const expMap = {}
            for (let i = 0; i < arr.length; ++i) {
                const item = arr[i]
                const s = item.slot
                if (s >= 1 && s <= 20) {
                    const br = item.r !== undefined ? item.r : 0
                    const bg = item.g !== undefined ? item.g : 170
                    const bb = item.b !== undefined ? item.b : 255
                    const kc = canvasRoot.keyAnimationConfigs ? canvasRoot.keyAnimationConfigs[String(s)] : null
                    keyModel.setProperty(s - 1, "l1", item.l1 || "")
                    keyModel.setProperty(s - 1, "l2", item.l2 || "")
                    keyModel.setProperty(s - 1, "r", br)
                    keyModel.setProperty(s - 1, "g", bg)
                    keyModel.setProperty(s - 1, "b", bb)
                    if (!keyModel.get(s - 1).animActive) {
                        keyModel.setProperty(s - 1, "animR", br)
                        keyModel.setProperty(s - 1, "animG", bg)
                        keyModel.setProperty(s - 1, "animB", bb)
                    }
                    keyModel.setProperty(s - 1, "hasAnim", !!(kc && kc.enabled))
                    keyModel.setProperty(s - 1, "hasRel", !!item.hasRel)
                    keyModel.setProperty(s - 1, "hasLoop", !!item.hasLoop)
                    keyModel.setProperty(s - 1, "hasWebApi", !!item.hasWebApi)
                    keyModel.setProperty(s - 1, "hasAppLaunch", !!item.hasAppLaunch)
                } else if (s === 21) {
                    canvasRoot.enc1CcwLabel = formatEncoderLabel(item, "CCW")
                } else if (s === 22) {
                    canvasRoot.enc1CwLabel = formatEncoderLabel(item, "CW")
                } else if (s === 23) {
                    canvasRoot.enc1PushLabel = formatEncoderLabel(item, "Push")
                } else if (s === 24) {
                    canvasRoot.enc2CcwLabel = formatEncoderLabel(item, "CCW")
                } else if (s === 25) {
                    canvasRoot.enc2CwLabel = formatEncoderLabel(item, "CW")
                } else if (s === 26) {
                    canvasRoot.enc2PushLabel = formatEncoderLabel(item, "Push")
                } else if (s >= 27 && s <= 58) {
                    const lbl = ((item.l1 || "") + (item.l2 ? " " + item.l2 : "")).trim()
                    if (lbl.length > 0 || item.hasScript) {
                        expMap[s] = lbl.length > 0 ? lbl : "Active"
                    }
                }
            }
            canvasRoot.expansionLabels = expMap
            oledSimulator.requestPaint()
        } catch (e) {}
    }

    function slotColorFor(slotNumber) {
        if (slotNumber >= 1 && slotNumber <= 20 && keyModel.count >= slotNumber) {
            const item = keyModel.get(slotNumber - 1)
            return Qt.rgba(item.r / 255.0, item.g / 255.0, item.b / 255.0, 1.0)
        }
        return Kirigami.Theme.highlightColor
    }

    function slotTitleFor(slotNumber) {
        if (slotNumber >= 1 && slotNumber <= 20 && keyModel.count >= slotNumber) {
            const item = keyModel.get(slotNumber - 1)
            const label = ((item.l1 || "") + " " + (item.l2 || "")).trim()
            return label.length > 0
                ? ("Key " + slotNumber + " (" + label + ")")
                : ("Key " + slotNumber + " (Unassigned)")
        } else if (slotNumber === 21) {
            return "Encoder 1 CCW — " + canvasRoot.enc1CcwLabel + " (Slot 21)"
        } else if (slotNumber === 22) {
            return "Encoder 1 CW — " + canvasRoot.enc1CwLabel + " (Slot 22)"
        } else if (slotNumber === 23) {
            return "Encoder 1 Push — " + canvasRoot.enc1PushLabel + " (Slot 23)"
        } else if (slotNumber === 24) {
            return "Encoder 2 CCW — " + canvasRoot.enc2CcwLabel + " (Slot 24)"
        } else if (slotNumber === 25) {
            return "Encoder 2 CW — " + canvasRoot.enc2CwLabel + " (Slot 25)"
        } else if (slotNumber === 26) {
            return "Encoder 2 Push — " + canvasRoot.enc2PushLabel + " (Slot 26)"
        } else {
            const idx = slotNumber - 27
            const modNum = Math.floor(idx / 8) + 1
            const chNum = (idx % 8) + 1
            const expLbl = canvasRoot.expansionLabels[slotNumber] || ""
            return expLbl.length > 0
                ? ("Expansion M" + modNum + "·C" + chNum + " (" + expLbl + " • Slot " + slotNumber + ")")
                : ("Expansion M" + modNum + "·C" + chNum + " (Slot " + slotNumber + ")")
        }
    }

    function setKeyRgbAt(slotNum, r, g, b) {
        if (slotNum >= 1 && slotNum <= 20) {
            keyModel.setProperty(slotNum - 1, "r", r)
            keyModel.setProperty(slotNum - 1, "g", g)
            keyModel.setProperty(slotNum - 1, "b", b)
            keyModel.setProperty(slotNum - 1, "animR", r)
            keyModel.setProperty(slotNum - 1, "animG", g)
            keyModel.setProperty(slotNum - 1, "animB", b)
            if (controller) {
                controller.setKeyRgb(slotNum - 1, r, g, b)
            }
            syncSlotsFromController()
            oledSimulator.requestPaint()
        }
    }

    function clearKeyRgbAt(slotNum) {
        if (slotNum >= 1 && slotNum <= 20) {
            if (controller) {
                controller.clearKeyCustomRgb(slotNum - 1)
            }
            syncSlotsFromController()
            oledSimulator.requestPaint()
        }
    }

    function updateSelectedKeyRgb(r, g, b) {
        if (selectedSlot >= 1 && selectedSlot <= 20) {
            setKeyRgbAt(selectedSlot, r, g, b)
        }
    }

    function copySlotAction(slotNum) {
        if (!controller) return
        canvasRoot.beforeSlotMutationRequested()
        controller.copySlot(slotNum)
    }

    function pasteSlotAction(targetSlotNum) {
        if (!controller || !controller.hasCopiedSlot) return
        canvasRoot.beforeSlotMutationRequested()
        if (controller.pasteSlot(targetSlotNum)) {
            canvasRoot.slotContentMutated(targetSlotNum)
            oledSimulator.requestPaint()
        }
    }

    function clearSlotAction(slotNum) {
        if (!controller) return
        canvasRoot.beforeSlotMutationRequested()
        if (controller.clearSlot(slotNum)) {
            canvasRoot.slotContentMutated(slotNum)
            oledSimulator.requestPaint()
        }
    }

    function moveOrSwapSlotAction(fromSlot, toSlot, shiftBetween) {
        if (!controller || fromSlot === toSlot) return
        canvasRoot.beforeSlotMutationRequested()
        if (controller.moveOrSwapSlot(fromSlot, toSlot, shiftBetween)) {
            if (canvasRoot.selectedSlot === fromSlot) {
                canvasRoot.selectedSlot = toSlot
            }
            canvasRoot.slotContentMutated(toSlot)
            oledSimulator.requestPaint()
        }
    }

    function openSlotContextMenu(slotNum) {
        canvasRoot.contextMenuSlot = slotNum
        slotContextMenu.popup()
    }

    QQC2.Menu {
        id: slotContextMenu
        font.pointSize: 10.5 * canvasRoot.uiFontScale

        QQC2.MenuItem {
            icon.name: "document-properties"
            text: qsTr("Properties...")
            onTriggered: {
                canvasRoot.openKeyPropertiesRequested(canvasRoot.contextMenuSlot)
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "edit-copy"
            text: canvasRoot.contextMenuSlot >= 1 && canvasRoot.contextMenuSlot <= 20
                ? qsTr("Copy Key %1").arg(canvasRoot.contextMenuSlot)
                : qsTr("Copy Slot %1").arg(canvasRoot.contextMenuSlot)
            onTriggered: canvasRoot.copySlotAction(canvasRoot.contextMenuSlot)
        }

        QQC2.MenuItem {
            icon.name: "edit-paste"
            enabled: canvasRoot.controller !== null && canvasRoot.controller.hasCopiedSlot
            text: canvasRoot.controller && canvasRoot.controller.hasCopiedSlot
                ? qsTr("Paste (%1) → Slot %2").arg(canvasRoot.controller.copiedSlotSummary).arg(canvasRoot.contextMenuSlot)
                : qsTr("Paste Button")
            onTriggered: canvasRoot.pasteSlotAction(canvasRoot.contextMenuSlot)
        }

        QQC2.MenuSeparator {
            visible: canvasRoot.contextMenuSlot >= 1 && canvasRoot.contextMenuSlot <= 20
        }

        QQC2.MenuItem {
            visible: canvasRoot.contextMenuSlot >= 1 && canvasRoot.contextMenuSlot <= 20
            icon.name: "color-picker"
            text: qsTr("Set Key %1 Color").arg(canvasRoot.contextMenuSlot)
            onTriggered: {
                canvasRoot.keyColorPickerRequested(
                    canvasRoot.contextMenuSlot,
                    canvasRoot.slotColorFor(canvasRoot.contextMenuSlot).toString()
                )
            }
        }

        QQC2.MenuItem {
            visible: canvasRoot.contextMenuSlot >= 1 && canvasRoot.contextMenuSlot <= 20
            icon.name: "preferences-desktop-color"
            text: qsTr("✨ Key %1 RGB Lighting Animation...").arg(canvasRoot.contextMenuSlot)
            onTriggered: {
                canvasRoot.openLightingAnimatorRequested("key", canvasRoot.contextMenuSlot)
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "edit-clear"
            text: canvasRoot.contextMenuSlot >= 1 && canvasRoot.contextMenuSlot <= 20
                ? qsTr("Clear / Reset Key %1").arg(canvasRoot.contextMenuSlot)
                : qsTr("Clear / Reset Slot %1").arg(canvasRoot.contextMenuSlot)
            onTriggered: canvasRoot.clearSlotAction(canvasRoot.contextMenuSlot)
        }
    }

    ColumnLayout {
        id: mainCol
        anchors.fill: parent
        spacing: 0

        Item {
            id: topToolbarRow
            Layout.fillWidth: true
            Layout.preferredHeight: 34

            RowLayout {
                anchors.fill: parent
                spacing: 4

                // Tab 1: Onboard (20 Keys + 2 Encoders)
                Rectangle {
                    id: onboardTabBtn
                    readonly property bool isActiveTab: !canvasRoot.showExpansion
                    Layout.preferredHeight: 34
                    implicitWidth: onboardTabRow.implicitWidth + 24
                    topLeftRadius: 8
                    topRightRadius: 8
                    bottomLeftRadius: 0
                    bottomRightRadius: 0
                    color: isActiveTab
                        ? Kirigami.Theme.alternateBackgroundColor
                        : (onboardTabMouse.containsMouse
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.12)
                            : Kirigami.Theme.backgroundColor)
                    border.color: isActiveTab
                        ? Kirigami.Theme.highlightColor
                        : Qt.alpha(Kirigami.Theme.textColor, 0.22)
                    border.width: 1

                    Rectangle {
                        visible: parent.isActiveTab
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: 3
                        topLeftRadius: 8
                        topRightRadius: 8
                        color: Kirigami.Theme.highlightColor
                    }

                    Rectangle {
                        visible: parent.isActiveTab
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.leftMargin: 1
                        anchors.rightMargin: 1
                        height: 2
                        color: Kirigami.Theme.alternateBackgroundColor
                    }

                    RowLayout {
                        id: onboardTabRow
                        anchors.centerIn: parent
                        spacing: 6

                        Kirigami.Icon {
                            source: "input-keyboard"
                            Layout.preferredWidth: 16
                            Layout.preferredHeight: 16
                            color: onboardTabBtn.isActiveTab
                                ? canvasRoot.highContrastHighlightText
                                : Kirigami.Theme.textColor
                        }

                        QQC2.Label {
                            text: qsTr("Onboard")
                            font.pointSize: 10.5 * canvasRoot.uiFontScale
                            font.bold: onboardTabBtn.isActiveTab
                            color: onboardTabBtn.isActiveTab
                                ? canvasRoot.highContrastHighlightText
                                : Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: onboardTabMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: canvasRoot.showExpansion = false
                    }

                    QQC2.ToolTip.visible: onboardTabMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Onboard (20 Keys + 2 Encoders)")
                }

                // Tab 2: Expansion (Channels 27–58)
                Rectangle {
                    id: expansionTabBtn
                    readonly property bool isActiveTab: canvasRoot.showExpansion
                    Layout.preferredHeight: 34
                    implicitWidth: expansionTabRow.implicitWidth + 24
                    topLeftRadius: 8
                    topRightRadius: 8
                    bottomLeftRadius: 0
                    bottomRightRadius: 0
                    color: isActiveTab
                        ? Kirigami.Theme.alternateBackgroundColor
                        : (expansionTabMouse.containsMouse
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.12)
                            : Kirigami.Theme.backgroundColor)
                    border.color: isActiveTab
                        ? Kirigami.Theme.highlightColor
                        : Qt.alpha(Kirigami.Theme.textColor, 0.22)
                    border.width: 1

                    Rectangle {
                        visible: parent.isActiveTab
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: 3
                        topLeftRadius: 8
                        topRightRadius: 8
                        color: Kirigami.Theme.highlightColor
                    }

                    Rectangle {
                        visible: parent.isActiveTab
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.leftMargin: 1
                        anchors.rightMargin: 1
                        height: 2
                        color: Kirigami.Theme.alternateBackgroundColor
                    }

                    RowLayout {
                        id: expansionTabRow
                        anchors.centerIn: parent
                        spacing: 6

                        Kirigami.Icon {
                            source: "network-wired"
                            Layout.preferredWidth: 16
                            Layout.preferredHeight: 16
                            color: expansionTabBtn.isActiveTab
                                ? canvasRoot.highContrastHighlightText
                                : Kirigami.Theme.textColor
                        }

                        QQC2.Label {
                            text: qsTr("Expansion")
                            font.pointSize: 10.5 * canvasRoot.uiFontScale
                            font.bold: expansionTabBtn.isActiveTab
                            color: expansionTabBtn.isActiveTab
                                ? canvasRoot.highContrastHighlightText
                                : Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: expansionTabMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: canvasRoot.showExpansion = true
                    }

                    QQC2.ToolTip.visible: expansionTabMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Daisy-Chained Expansion Channels (27–58)")
                }

                Item { Layout.fillWidth: true }

                QQC2.ToolButton {
                    Layout.alignment: Qt.AlignVCenter
                    checkable: true
                    checked: canvasRoot.oledScriptMode
                    icon.name: "video-television"
                    display: QQC2.AbstractButton.TextBesideIcon
                    font.pointSize: 10.5 * canvasRoot.uiFontScale
                    text: canvasRoot.oledScriptMode ? qsTr("OLED VM") : qsTr("OLED Grid")
                    onToggled: {
                        canvasRoot.oledScriptMode = checked
                        oledSimulator.requestPaint()
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: canvasRoot.oledScriptMode ? qsTr("Switch to Profile Grid HUD") : qsTr("Switch to duckStack VM HUD")
                }
            }
        }

        Rectangle {
            id: pcbChassis
            objectName: "pcbChassis"
            Kirigami.Theme.colorSet: Kirigami.Theme.View
            Kirigami.Theme.inherit: false
            readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            readonly property real uiScale: Math.max(
                0.70,
                Math.min(
                    (pcbChassis.width - 20) / 430.0,
                    (pcbChassis.height - 20) / 354.0
                )
            )

            Layout.fillWidth: true
            Layout.fillHeight: true
            topLeftRadius: 0
            topRightRadius: 12
            bottomLeftRadius: 12
            bottomRightRadius: 12
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: pcbChassis.frameBorderColor
            border.width: 1
            clip: true

            MouseArea {
                anchors.fill: parent
                onClicked: canvasRoot.clearSelection()
            }

            Rectangle {
                anchors.fill: parent
                anchors.margins: 5
                radius: 10
                color: "transparent"
                border.color: pcbChassis.frameBorderColor
                opacity: 0.5
                border.width: 1
            }

            Row {
                anchors.centerIn: parent
                spacing: 12
                scale: pcbChassis.uiScale
                transformOrigin: Item.Center
                visible: !canvasRoot.showExpansion

                Grid {
                    id: switchMatrix
                    columns: 4
                    rowSpacing: 6
                    columnSpacing: 6

                    Repeater {
                        model: keyModel

                        delegate: Item {
                            id: keycapItem
                            width: 66
                            height: 66
                            required property int index
                            required property int slot
                            required property string l1
                            required property string l2
                            required property int r
                            required property int g
                            required property int b
                            required property int animR
                            required property int animG
                            required property int animB
                            required property bool animActive
                            required property bool hasAnim
                            required property bool hasRel
                            required property bool hasLoop
                            required property bool hasWebApi
                            required property bool hasAppLaunch

                            readonly property bool isSelected: canvasRoot.selectedSlot === slot
                            readonly property bool isDraggingThis: canvasRoot.draggingSlot === slot
                            readonly property bool isDropTarget: canvasRoot.dropTargetSlot === slot
                                && canvasRoot.draggingSlot >= 1
                                && canvasRoot.draggingSlot <= 20
                                && canvasRoot.draggingSlot !== slot
                            readonly property color baseLedColor: Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 1.0)
                            readonly property color ledColor: animActive
                                ? Qt.rgba(animR / 255.0, animG / 255.0, animB / 255.0, 1.0)
                                : baseLedColor
                            readonly property bool hasFooterBadges: keycapItem.hasAppLaunch
                                || keycapItem.hasWebApi
                                || keycapItem.hasAnim
                                || keycapItem.hasRel
                                || keycapItem.hasLoop
                            property real dragOffsetX: 0
                            property real dragOffsetY: 0
                            objectName: "keycapItem_" + slot

                            z: isDraggingThis ? 100 : (isDropTarget ? 10 : 1)

                            // Underlying slot socket shown while keycap is lifted for drag
                            Rectangle {
                                visible: keycapItem.isDraggingThis
                                anchors.fill: parent
                                radius: 8
                                color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.45)
                                border.width: 1
                                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                            }

                            Rectangle {
                                id: keyVisual
                                objectName: "keyVisual"
                                width: parent.width
                                height: parent.height
                                x: keycapItem.isDraggingThis ? keycapItem.dragOffsetX : 0
                                y: keycapItem.isDraggingThis ? keycapItem.dragOffsetY : 0
                                scale: keycapItem.isDraggingThis ? 1.06 : 1.0
                                opacity: keycapItem.isDraggingThis ? 0.92 : 1.0
                                radius: 8
                                color: Kirigami.Theme.backgroundColor
                                border.width: (keycapItem.isDropTarget || shortcutDropArea.containsDrag) ? 2.5 : (keycapItem.isSelected ? 2 : 1)
                                border.color: (keycapItem.isDropTarget || shortcutDropArea.containsDrag)
                                    ? Kirigami.Theme.positiveTextColor
                                    : (keycapItem.isSelected
                                        ? Kirigami.Theme.highlightColor
                                        : pcbChassis.frameBorderColor)

                                QQC2.Label {
                                    id: keySlotNumberLabel
                                    objectName: "keySlotNumberLabel"
                                    x: 6
                                    y: 1
                                    width: 14
                                    height: 10
                                    horizontalAlignment: Text.AlignLeft
                                    verticalAlignment: Text.AlignVCenter
                                    text: keycapItem.slot
                                    font.pixelSize: 8
                                    font.bold: true
                                    font.family: "monospace"
                                    elide: Text.ElideRight
                                    color: keycapItem.isSelected
                                        ? Kirigami.Theme.textColor
                                        : Kirigami.Theme.disabledTextColor
                                }

                                Rectangle {
                                    id: keyLedBar
                                    objectName: "keyLedBar"
                                    x: 22
                                    y: 4
                                    width: keycapItem.animActive ? 36 : 34
                                    height: 4
                                    radius: 2
                                    color: keycapItem.ledColor
                                }

                                Rectangle {
                                    id: keyInnerWell
                                    objectName: "keyInnerWell"
                                    anchors.fill: parent
                                    anchors.margins: 5
                                    anchors.topMargin: 12
                                    radius: 6
                                    clip: true
                                    color: keycapItem.isDropTarget
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.positiveTextColor.r, Kirigami.Theme.positiveTextColor.g, Kirigami.Theme.positiveTextColor.b, 0.28))
                                        : (keyMouse.pressed || keycapItem.animActive
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(keycapItem.ledColor.r, keycapItem.ledColor.g, keycapItem.ledColor.b, keyMouse.pressed ? 0.38 : 0.22))
                                            : (keycapItem.isSelected
                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.18))
                                                : Kirigami.Theme.alternateBackgroundColor))
                                    border.width: keycapItem.animActive ? 1.5 : 1
                                    border.color: keycapItem.isDropTarget
                                        ? Kirigami.Theme.positiveTextColor
                                        : (keycapItem.isSelected
                                            ? Kirigami.Theme.highlightColor
                                            : (keycapItem.animActive
                                                ? Qt.rgba(keycapItem.ledColor.r, keycapItem.ledColor.g, keycapItem.ledColor.b, 0.85)
                                                : Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 0.5)))

                                    Column {
                                        id: keyLabelColumn
                                        objectName: "keyLabelColumn"
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        anchors.verticalCenter: parent.verticalCenter
                                        anchors.verticalCenterOffset: (keycapItem.hasFooterBadges && !keycapItem.isDropTarget) ? -6 : 0
                                        width: parent.width - 6
                                        spacing: 0

                                        QQC2.Label {
                                            id: keyLine1Label
                                            objectName: "keyLine1Label"
                                            readonly property bool isIcon1: canvasRoot.isSingleIconLabel(keycapItem.l1)
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                            text: keycapItem.l1
                                            font.pixelSize: isIcon1
                                                ? (keycapItem.l2.length > 0 ? 13 : 16)
                                                : (keycapItem.l2.length > 0 ? 10 : 11)
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 7
                                            elide: Text.ElideRight
                                            font.bold: true
                                            color: Kirigami.Theme.textColor
                                        }
                                        QQC2.Label {
                                            id: keyLine2Label
                                            objectName: "keyLine2Label"
                                            readonly property bool isIcon2: canvasRoot.isSingleIconLabel(keycapItem.l2)
                                            visible: keycapItem.l2.length > 0
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                            text: keycapItem.l2
                                            font.pixelSize: isIcon2
                                                ? (keycapItem.l1.length > 0 ? 12 : 15)
                                                : 9
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 7
                                            elide: Text.ElideRight
                                            font.weight: Font.Medium
                                            color: Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    Rectangle {
                                        id: keyFooterDivider
                                        objectName: "keyFooterDivider"
                                        visible: keycapItem.hasFooterBadges && !keycapItem.isDropTarget
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 4
                                        anchors.rightMargin: 4
                                        anchors.bottom: parent.bottom
                                        anchors.bottomMargin: 13
                                        height: 1
                                        color: Qt.alpha(Kirigami.Theme.textColor, 0.12)
                                    }

                                    Row {
                                        id: keyFooterRow
                                        objectName: "keyFooterRow"
                                        visible: keycapItem.hasFooterBadges && !keycapItem.isDropTarget
                                        anchors.bottom: parent.bottom
                                        anchors.bottomMargin: 2
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        spacing: 2

                                        Rectangle {
                                            visible: keycapItem.hasAppLaunch
                                            radius: 2
                                            color: Qt.alpha("#f59e0b", canvasRoot.isLightTheme ? 0.16 : 0.22)
                                            border.color: Qt.alpha(canvasRoot.isLightTheme ? "#d97706" : "#f59e0b", 0.65)
                                            border.width: 1
                                            width: appBadgeLbl.implicitWidth + 3
                                            height: 10

                                            QQC2.Label {
                                                id: appBadgeLbl
                                                anchors.centerIn: parent
                                                text: "APP"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: canvasRoot.isLightTheme ? "#92400e" : "#fbbf24"
                                            }
                                        }
                                        Rectangle {
                                            visible: keycapItem.hasWebApi && !keycapItem.hasAppLaunch
                                            radius: 2
                                            color: Qt.alpha("#f59e0b", canvasRoot.isLightTheme ? 0.16 : 0.22)
                                            border.color: Qt.alpha(canvasRoot.isLightTheme ? "#d97706" : "#f59e0b", 0.65)
                                            border.width: 1
                                            width: apiBadgeLbl.implicitWidth + 3
                                            height: 10

                                            QQC2.Label {
                                                id: apiBadgeLbl
                                                anchors.centerIn: parent
                                                text: "API"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: canvasRoot.isLightTheme ? "#92400e" : "#fbbf24"
                                            }
                                        }
                                        Rectangle {
                                            visible: keycapItem.hasAnim
                                            radius: 2
                                            color: Qt.alpha("#a855f7", canvasRoot.isLightTheme ? 0.15 : 0.24)
                                            border.color: Qt.alpha(canvasRoot.isLightTheme ? "#7e22ce" : "#a855f7", 0.65)
                                            border.width: 1
                                            width: fxBadgeLbl.implicitWidth + 3
                                            height: 10

                                            QQC2.Label {
                                                id: fxBadgeLbl
                                                anchors.centerIn: parent
                                                text: "FX"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: canvasRoot.isLightTheme ? "#6b21a8" : "#c084fc"
                                            }
                                        }
                                        Rectangle {
                                            visible: keycapItem.hasRel
                                            radius: 2
                                            color: Qt.alpha("#0284c7", canvasRoot.isLightTheme ? 0.14 : 0.20)
                                            border.color: Qt.alpha(canvasRoot.isLightTheme ? "#0284c7" : "#38bdf8", 0.60)
                                            border.width: 1
                                            width: relBadgeLbl.implicitWidth + 3
                                            height: 10

                                            QQC2.Label {
                                                id: relBadgeLbl
                                                anchors.centerIn: parent
                                                text: "↑"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: canvasRoot.isLightTheme ? "#0369a1" : "#38bdf8"
                                            }
                                        }
                                        Rectangle {
                                            visible: keycapItem.hasLoop
                                            radius: 2
                                            color: Qt.alpha("#10b981", canvasRoot.isLightTheme ? 0.14 : 0.20)
                                            border.color: Qt.alpha(canvasRoot.isLightTheme ? "#059669" : "#34d399", 0.60)
                                            border.width: 1
                                            width: loopBadgeLbl.implicitWidth + 3
                                            height: 10

                                            QQC2.Label {
                                                id: loopBadgeLbl
                                                anchors.centerIn: parent
                                                text: "∞"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: canvasRoot.isLightTheme ? "#047857" : "#34d399"
                                            }
                                        }
                                    }

                                    Rectangle {
                                        visible: keycapItem.isDropTarget
                                        anchors.bottom: parent.bottom
                                        anchors.bottomMargin: 2
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        radius: 3
                                        width: dropModeLbl.implicitWidth + 6
                                        height: 13
                                        color: Kirigami.Theme.positiveTextColor

                                        QQC2.Label {
                                            id: dropModeLbl
                                            anchors.centerIn: parent
                                            text: canvasRoot.dragShiftMode ? qsTr("→ Shift") : qsTr("⇄ Swap")
                                            font.pixelSize: 9
                                            font.bold: true
                                            color: Kirigami.Theme.backgroundColor
                                        }
                                    }
                                }
                            }

                            MouseArea {
                                id: keyMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                acceptedButtons: Qt.LeftButton | Qt.RightButton
                                preventStealing: true
                                property real pressX: 0
                                property real pressY: 0
                                property bool didDrag: false

                                onPressed: (mouse) => {
                                    canvasRoot.notifyUserActivity(keycapItem.slot)
                                    if (mouse.button === Qt.LeftButton) {
                                        pressX = mouse.x
                                        pressY = mouse.y
                                        didDrag = false
                                        keycapItem.dragOffsetX = 0
                                        keycapItem.dragOffsetY = 0
                                    }
                                }

                                onPositionChanged: (mouse) => {
                                    if (!(mouse.buttons & Qt.LeftButton)) return
                                    const dx = mouse.x - pressX
                                    const dy = mouse.y - pressY
                                    if (!didDrag && Math.hypot(dx, dy) > 6) {
                                        didDrag = true
                                        canvasRoot.draggingSlot = keycapItem.slot
                                    }
                                    if (didDrag) {
                                        keycapItem.dragOffsetX = dx
                                        keycapItem.dragOffsetY = dy
                                        canvasRoot.dragShiftMode = (mouse.modifiers & Qt.ShiftModifier) !== 0
                                        const pt = keyMouse.mapToItem(switchMatrix, mouse.x, mouse.y)
                                        const stride = 72
                                        if (pt.x >= 0 && pt.x < switchMatrix.width && pt.y >= 0 && pt.y < switchMatrix.height) {
                                            const col = Math.min(3, Math.max(0, Math.floor(pt.x / stride)))
                                            const row = Math.min(4, Math.max(0, Math.floor(pt.y / stride)))
                                            const target = row * 4 + col + 1
                                            canvasRoot.dropTargetSlot = (target !== keycapItem.slot) ? target : -1
                                        } else {
                                            canvasRoot.dropTargetSlot = -1
                                        }
                                    }
                                }

                                onReleased: (mouse) => {
                                    if (mouse.button === Qt.LeftButton && didDrag) {
                                        const target = canvasRoot.dropTargetSlot
                                        const shift = (mouse.modifiers & Qt.ShiftModifier) !== 0
                                        didDrag = false
                                        keycapItem.dragOffsetX = 0
                                        keycapItem.dragOffsetY = 0
                                        canvasRoot.draggingSlot = -1
                                        canvasRoot.dropTargetSlot = -1
                                        if (target >= 1 && target <= 20 && target !== keycapItem.slot) {
                                            canvasRoot.moveOrSwapSlotAction(keycapItem.slot, target, shift)
                                        }
                                        return
                                    }
                                    if (mouse.button === Qt.LeftButton) {
                                        if (canvasRoot.selectedSlot === keycapItem.slot) {
                                            canvasRoot.clearSelection()
                                        } else {
                                            canvasRoot.selectedSlot = keycapItem.slot
                                            canvasRoot.slotSelected(
                                                keycapItem.slot,
                                                canvasRoot.slotTitleFor(keycapItem.slot),
                                                keycapItem.ledColor
                                            )
                                            oledSimulator.requestPaint()
                                        }
                                    } else if (mouse.button === Qt.RightButton) {
                                        canvasRoot.openSlotContextMenu(keycapItem.slot)
                                    }
                                }

                                onCanceled: {
                                    didDrag = false
                                    keycapItem.dragOffsetX = 0
                                    keycapItem.dragOffsetY = 0
                                    if (canvasRoot.draggingSlot === keycapItem.slot) {
                                        canvasRoot.draggingSlot = -1
                                        canvasRoot.dropTargetSlot = -1
                                    }
                                }

                                QQC2.ToolTip.visible: keyMouse.containsMouse && !didDrag && !slotContextMenu.visible && !(deviceController && deviceController.isDemoShowcaseMode())
                                QQC2.ToolTip.delay: 650
                                QQC2.ToolTip.text: (keycapItem.hasAppLaunch || keycapItem.hasWebApi)
                                    ? qsTr("Key %1 • 💻 Uses Host Action (App Launch / Web API) — requires DuckyDeck Studio running on host PC (not portable standalone) • Click to edit • Drag to swap • Right-click for Copy/Paste").arg(keycapItem.slot)
                                    : qsTr("Key %1 • ✅ Standalone Hardware Macro • Click to edit (click again to close editor) • Drag to swap (Shift+Drag to shift) • Right-click for Copy/Paste").arg(keycapItem.slot)
                            }

                            DropArea {
                                id: shortcutDropArea
                                anchors.fill: parent
                                keys: ["application/x-ducky-shortcut"]
                                onEntered: (drag) => {
                                    drag.acceptProposedAction()
                                }
                                onDropped: (drop) => {
                                    if (drop.hasFormat("application/x-ducky-shortcut")) {
                                        try {
                                            const sc = JSON.parse(drop.getDataAsString("application/x-ducky-shortcut"))
                                            if (sc && canvasRoot.controller) {
                                                canvasRoot.controller.assignShortcutToKeySlot(
                                                    0,
                                                    keycapItem.slot,
                                                    sc.action || "",
                                                    sc.shortcut || "",
                                                    sc.duckyscript || "",
                                                    sc.category || ""
                                                )
                                                canvasRoot.slotContentMutated(keycapItem.slot)
                                                drop.acceptProposedAction()
                                            }
                                        } catch (e) {
                                            console.warn("Failed dropping shortcut:", e)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Column {
                    width: 136
                    spacing: 6
                    anchors.verticalCenter: parent.verticalCenter

                    RotaryEncoderWidget {
                        anchors.horizontalCenter: parent.horizontalCenter
                        encoderTitle: qsTr("Encoder 1 (Top)")
                        ccwSlot: 21
                        cwSlot: 22
                        pushSlot: 23
                        ccwLabel: canvasRoot.enc1CcwLabel
                        cwLabel: canvasRoot.enc1CwLabel
                        pushLabel: canvasRoot.enc1PushLabel
                        selectedSlot: canvasRoot.selectedSlot
                        textScale: canvasRoot.canvasScale
                        onActionTriggered: (slot, desc) => {
                            if (canvasRoot.selectedSlot === slot) {
                                canvasRoot.clearSelection()
                            } else {
                                canvasRoot.selectedSlot = slot
                                canvasRoot.slotSelected(slot, canvasRoot.slotTitleFor(slot), Kirigami.Theme.highlightColor)
                                oledSimulator.requestPaint()
                            }
                        }
                        onContextMenuRequested: (slot) => {
                            canvasRoot.openSlotContextMenu(slot)
                        }
                    }

                    RotaryEncoderWidget {
                        anchors.horizontalCenter: parent.horizontalCenter
                        encoderTitle: qsTr("Encoder 2 (Mid)")
                        ccwSlot: 24
                        cwSlot: 25
                        pushSlot: 26
                        ccwLabel: canvasRoot.enc2CcwLabel
                        cwLabel: canvasRoot.enc2CwLabel
                        pushLabel: canvasRoot.enc2PushLabel
                        selectedSlot: canvasRoot.selectedSlot
                        textScale: canvasRoot.canvasScale
                        onActionTriggered: (slot, desc) => {
                            if (canvasRoot.selectedSlot === slot) {
                                canvasRoot.clearSelection()
                            } else {
                                canvasRoot.selectedSlot = slot
                                canvasRoot.slotSelected(slot, canvasRoot.slotTitleFor(slot), Kirigami.Theme.highlightColor)
                                oledSimulator.requestPaint()
                            }
                        }
                        onContextMenuRequested: (slot) => {
                            canvasRoot.openSlotContextMenu(slot)
                        }
                    }

                    Rectangle {
                        anchors.horizontalCenter: parent.horizontalCenter
                        width: 134
                        height: 134
                        radius: 8
                        color: "#000000" // Keep OLED simulator true black
                        border.color: pcbChassis.frameBorderColor
                        border.width: 2

                        Canvas {
                            id: oledSimulator
                            width: 128
                            height: 128
                            anchors.centerIn: parent
                            rotation: canvasRoot.activeProfileOledRotation
                            transformOrigin: Item.Center
                            scale: 1.0
                            smooth: false

                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.save()
                                ctx.fillStyle = "#000000"
                                ctx.fillRect(0, 0, 128, 128)

                                if (!canvasRoot.oledScriptMode) {
                                    ctx.fillStyle = "#38bdf8"
                                    ctx.fillRect(0, 0, 128, 13)
                                    ctx.fillStyle = "#020617"
                                    ctx.font = "bold 9px sans-serif"
                                    ctx.textAlign = "center"
                                    const profNum = canvasRoot.controller ? canvasRoot.controller.activeProfile : 1
                                    const profName = (canvasRoot.controller && canvasRoot.controller.activeProfileName)
                                        ? canvasRoot.controller.activeProfileName
                                        : "Default"
                                    ctx.fillText("P" + profNum + ": " + profName.slice(0, 16), 64, 10)

                                    const cellW = 32
                                    const cellH = 23

                                    for (let i = 0; i < keyModel.count; ++i) {
                                        const item = keyModel.get(i)
                                        const col = i % 4
                                        const row = Math.floor(i / 4)
                                        const x = col * cellW
                                        const y = 13 + row * cellH
                                        const isSel = (canvasRoot.selectedSlot === item.slot)

                                        if (isSel) {
                                            ctx.fillStyle = "#38bdf8"
                                            ctx.fillRect(x + 1, y + 1, cellW - 2, cellH - 2)
                                        } else {
                                            ctx.strokeStyle = "#334155"
                                            ctx.strokeRect(x, y, cellW, cellH)
                                        }
                                    }
                                    ctx.restore()
                                } else {
                                    ctx.restore()
                                    ctx.strokeStyle = "#38bdf8"
                                    ctx.lineWidth = 1
                                    ctx.strokeRect(2, 2, 124, 124)

                                    ctx.font = "bold 10px sans-serif"
                                    ctx.fillStyle = "#38bdf8"
                                    ctx.fillText("duckStack VM HUD", 14, 18)

                                    ctx.beginPath()
                                    ctx.moveTo(8, 24)
                                    ctx.lineTo(120, 24)
                                    ctx.stroke()

                                    ctx.beginPath()
                                    ctx.arc(64, 68, 20, 0, Math.PI * 2)
                                    ctx.stroke()

                                    ctx.font = "bold 10px monospace"
                                    ctx.fillStyle = "#ffffff"
                                    ctx.fillText(
                                        canvasRoot.selectedSlot >= 1 ? ("Slot #" + canvasRoot.selectedSlot) : "Ready",
                                        canvasRoot.selectedSlot >= 1 ? 36 : 44,
                                        72
                                    )
                                    ctx.fillStyle = "#94a3b8"
                                    ctx.fillText("128x128 OLED", 28, 114)
                                }
                            }

                            Item {
                                anchors.fill: parent
                                visible: !canvasRoot.oledScriptMode

                                Repeater {
                                    model: keyModel
                                    delegate: Item {
                                        required property int index
                                        required property int slot
                                        required property string l1
                                        required property string l2

                                        readonly property int col: index % 4
                                        readonly property int row: Math.floor(index / 4)
                                        readonly property bool isSel: canvasRoot.selectedSlot === slot
                                        readonly property string s1: canvasRoot.sliceOledLabel(l1 || "", 5)
                                        readonly property string s2: canvasRoot.sliceOledLabel(l2 || "", 5)
                                        readonly property bool icon1: canvasRoot.isSingleIconLabel(s1)
                                        readonly property bool icon2: canvasRoot.isSingleIconLabel(s2)

                                        x: col * 32
                                        y: 13 + row * 23
                                        width: 32
                                        height: 23
                                        clip: true

                                        Column {
                                            anchors.centerIn: parent
                                            width: parent.width - 2
                                            spacing: 0

                                            Text {
                                                visible: s1.length > 0 || s2.length === 0
                                                width: parent.width
                                                horizontalAlignment: Text.AlignHCenter
                                                verticalAlignment: Text.AlignVCenter
                                                text: s1
                                                color: isSel ? "#020617" : "#f8fafc"
                                                font.family: icon1 ? "sans-serif" : "monospace"
                                                font.bold: true
                                                font.pixelSize: icon1 ? (s2.length > 0 ? 10 : 12) : (s2.length > 0 ? 8 : 9)
                                                fontSizeMode: Text.HorizontalFit
                                                minimumPixelSize: 6
                                                elide: Text.ElideRight
                                            }

                                            Text {
                                                visible: s2.length > 0
                                                width: parent.width
                                                horizontalAlignment: Text.AlignHCenter
                                                verticalAlignment: Text.AlignVCenter
                                                text: s2
                                                color: isSel ? "#020617" : "#f8fafc"
                                                font.family: icon2 ? "sans-serif" : "monospace"
                                                font.bold: true
                                                font.pixelSize: icon2 ? (s1.length > 0 ? 10 : 12) : 8
                                                fontSizeMode: Text.HorizontalFit
                                                minimumPixelSize: 6
                                                elide: Text.ElideRight
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Row {
                        anchors.horizontalCenter: parent.horizontalCenter
                        spacing: 6

                        QQC2.Button {
                            id: prevProfileBtn
                            width: 64
                            implicitHeight: 26
                            leftPadding: 2
                            rightPadding: 2
                            topPadding: 2
                            bottomPadding: 2
                            background: Rectangle {
                                radius: 5
                                color: prevProfileBtn.down
                                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : (prevProfileBtn.hovered
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.12))
                                        : Kirigami.Theme.backgroundColor)
                                border.width: 1
                                border.color: prevProfileBtn.hovered || prevProfileBtn.down
                                    ? Kirigami.Theme.highlightColor
                                    : pcbChassis.frameBorderColor
                            }
                            contentItem: Text {
                                text: "− Profile"
                                font.pixelSize: 10
                                fontSizeMode: Text.HorizontalFit
                                minimumPixelSize: 8
                                font.bold: true
                                color: Kirigami.Theme.textColor
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideRight
                            }
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && canvasRoot.currentProfileIndex > 0) {
                                        canvasRoot.currentProfileIndex -= 1
                                        const profNum = canvasRoot.profileNumbers[canvasRoot.currentProfileIndex]
                                        canvasRoot.controller.selectProfile(profNum)
                                    } else {
                                        const next = Math.max(1, canvasRoot.controller.activeProfile - 1)
                                        canvasRoot.controller.selectProfile(next)
                                    }
                                    if (canvasRoot.selectedSlot >= 1 && canvasRoot.selectedSlot <= 58) {
                                        canvasRoot.slotSelected(
                                            canvasRoot.selectedSlot,
                                            canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                                            canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                                        )
                                    }
                                }
                            }
                        }

                        QQC2.Button {
                            id: nextProfileBtn
                            width: 64
                            implicitHeight: 26
                            leftPadding: 2
                            rightPadding: 2
                            topPadding: 2
                            bottomPadding: 2
                            background: Rectangle {
                                radius: 5
                                color: nextProfileBtn.down
                                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : (nextProfileBtn.hovered
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.12))
                                        : Kirigami.Theme.backgroundColor)
                                border.width: 1
                                border.color: nextProfileBtn.hovered || nextProfileBtn.down
                                    ? Kirigami.Theme.highlightColor
                                    : pcbChassis.frameBorderColor
                            }
                            contentItem: Text {
                                text: "+ Profile"
                                font.pixelSize: 10
                                fontSizeMode: Text.HorizontalFit
                                minimumPixelSize: 8
                                font.bold: true
                                color: Kirigami.Theme.textColor
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideRight
                            }
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && canvasRoot.currentProfileIndex + 1 < canvasRoot.profileNumbers.length) {
                                        canvasRoot.currentProfileIndex += 1
                                        const profNum = canvasRoot.profileNumbers[canvasRoot.currentProfileIndex]
                                        canvasRoot.controller.selectProfile(profNum)
                                    } else {
                                        const next = Math.min(64, canvasRoot.controller.activeProfile + 1)
                                        canvasRoot.controller.selectProfile(next)
                                    }
                                    if (canvasRoot.selectedSlot >= 1 && canvasRoot.selectedSlot <= 58) {
                                        canvasRoot.slotSelected(
                                            canvasRoot.selectedSlot,
                                            canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                                            canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 12
                spacing: 6
                visible: canvasRoot.showExpansion

                QQC2.Label {
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                    text: qsTr("Expansion Modules (4 Modules × 8 Channels) • Drag to swap, Right-click for Copy/Paste")
                    font.bold: true
                    font.pointSize: 10.5 * canvasRoot.uiFontScale
                    color: Kirigami.Theme.textColor
                }

                GridLayout {
                    id: expansionGrid
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    columns: 4
                    rowSpacing: 4
                    columnSpacing: 6

                    Repeater {
                        model: 32
                        delegate: Rectangle {
                            id: expSlotItem
                            required property int index
                            readonly property int slotNum: 27 + index
                            readonly property int moduleNum: Math.floor(index / 8) + 1
                            readonly property int chanNum: (index % 8) + 1
                            readonly property bool isSel: canvasRoot.selectedSlot === slotNum
                            readonly property bool isDraggingThis: canvasRoot.draggingSlot === slotNum
                            readonly property bool isDropTarget: canvasRoot.dropTargetSlot === slotNum
                                && canvasRoot.draggingSlot >= 27
                                && canvasRoot.draggingSlot <= 58
                                && canvasRoot.draggingSlot !== slotNum
                            readonly property string expLabel: canvasRoot.expansionLabels[slotNum] || ""

                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            Layout.preferredWidth: 1
                            Layout.preferredHeight: 1
                            Layout.minimumWidth: 48
                            Layout.minimumHeight: 24
                            radius: 6
                            clip: true
                            opacity: isDraggingThis ? 0.55 : 1.0
                            color: isDropTarget
                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.positiveTextColor.r, Kirigami.Theme.positiveTextColor.g, Kirigami.Theme.positiveTextColor.b, 0.25))
                                : (isSel
                                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                                    : Kirigami.Theme.backgroundColor)
                            border.width: isDropTarget ? 2.5 : (isSel || expLabel.length > 0 ? 2 : 1)
                            border.color: isDropTarget
                                ? Kirigami.Theme.positiveTextColor
                                : (isSel
                                    ? Kirigami.Theme.highlightColor
                                    : (expLabel.length > 0 ? Qt.alpha(Kirigami.Theme.highlightColor, 0.55) : pcbChassis.frameBorderColor))

                            Column {
                                anchors.centerIn: parent
                                width: Math.max(16, parent.width - 8)
                                spacing: 0

                                Text {
                                    width: parent.width
                                    height: Math.max(10, Math.floor((expSlotItem.height - 4) * 0.56))
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    text: expLabel.length > 0
                                        ? ("M" + moduleNum + "·C" + chanNum + ": " + expLabel)
                                        : ("M" + moduleNum + "·C" + chanNum)
                                    font.bold: true
                                    font.pixelSize: Math.max(10, Math.min(14, Math.round(expSlotItem.height * 0.34)))
                                    fontSizeMode: Text.Fit
                                    minimumPixelSize: 8
                                    elide: Text.ElideRight
                                    color: Kirigami.Theme.textColor
                                }
                                Text {
                                    width: parent.width
                                    height: Math.max(8, Math.floor((expSlotItem.height - 4) * 0.44))
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    text: isDropTarget
                                        ? (canvasRoot.dragShiftMode ? qsTr("→ Shift Here") : qsTr("⇄ Swap Here"))
                                        : ("Slot " + slotNum)
                                    font.pixelSize: Math.max(9, Math.min(12, Math.round(expSlotItem.height * 0.28)))
                                    fontSizeMode: Text.Fit
                                    minimumPixelSize: 7
                                    elide: Text.ElideRight
                                    font.bold: isDropTarget
                                    color: isDropTarget ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.disabledTextColor
                                }
                            }

                            MouseArea {
                                id: expMouse
                                anchors.fill: parent
                                acceptedButtons: Qt.LeftButton | Qt.RightButton
                                preventStealing: true
                                property real pressX: 0
                                property real pressY: 0
                                property bool didDrag: false

                                onPressed: (mouse) => {
                                    if (mouse.button === Qt.LeftButton) {
                                        pressX = mouse.x
                                        pressY = mouse.y
                                        didDrag = false
                                    }
                                }

                                onPositionChanged: (mouse) => {
                                    if (!(mouse.buttons & Qt.LeftButton)) return
                                    const dx = mouse.x - pressX
                                    const dy = mouse.y - pressY
                                    if (!didDrag && Math.hypot(dx, dy) > 6) {
                                        didDrag = true
                                        canvasRoot.draggingSlot = expSlotItem.slotNum
                                    }
                                    if (didDrag) {
                                        canvasRoot.dragShiftMode = (mouse.modifiers & Qt.ShiftModifier) !== 0
                                        const pt = expMouse.mapToItem(expansionGrid, mouse.x, mouse.y)
                                        if (pt.x >= 0 && pt.x < expansionGrid.width && pt.y >= 0 && pt.y < expansionGrid.height) {
                                            const col = Math.min(3, Math.max(0, Math.floor((pt.x / expansionGrid.width) * 4)))
                                            const row = Math.min(7, Math.max(0, Math.floor((pt.y / expansionGrid.height) * 8)))
                                            const target = 27 + row * 4 + col
                                            canvasRoot.dropTargetSlot = (target !== expSlotItem.slotNum) ? target : -1
                                        } else {
                                            canvasRoot.dropTargetSlot = -1
                                        }
                                    }
                                }

                                onReleased: (mouse) => {
                                    if (mouse.button === Qt.LeftButton && didDrag) {
                                        const target = canvasRoot.dropTargetSlot
                                        const shift = (mouse.modifiers & Qt.ShiftModifier) !== 0
                                        didDrag = false
                                        canvasRoot.draggingSlot = -1
                                        canvasRoot.dropTargetSlot = -1
                                        if (target >= 27 && target <= 58 && target !== expSlotItem.slotNum) {
                                            canvasRoot.moveOrSwapSlotAction(expSlotItem.slotNum, target, shift)
                                        }
                                        return
                                    }
                                    if (mouse.button === Qt.LeftButton) {
                                        if (canvasRoot.selectedSlot === expSlotItem.slotNum) {
                                            canvasRoot.clearSelection()
                                        } else {
                                            canvasRoot.selectedSlot = expSlotItem.slotNum
                                            canvasRoot.slotSelected(
                                                expSlotItem.slotNum,
                                                canvasRoot.slotTitleFor(expSlotItem.slotNum),
                                                Kirigami.Theme.highlightColor
                                            )
                                        }
                                    } else if (mouse.button === Qt.RightButton) {
                                        canvasRoot.openSlotContextMenu(expSlotItem.slotNum)
                                    }
                                }

                                onCanceled: {
                                    didDrag = false
                                    if (canvasRoot.draggingSlot === expSlotItem.slotNum) {
                                        canvasRoot.draggingSlot = -1
                                        canvasRoot.dropTargetSlot = -1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    component RotaryEncoderWidget: Item {
        id: encRoot
        implicitWidth: 136
        implicitHeight: 68

        property string encoderTitle: "Encoder"
        property int ccwSlot: 21
        property int cwSlot: 22
        property int pushSlot: 23
        property string ccwLabel: "CCW"
        property string cwLabel: "CW"
        property string pushLabel: "Push"
        property int selectedSlot: 1
        property real visualAngle: 0
        property real textScale: 1.0
        readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

        signal actionTriggered(int slot, string description)
        signal contextMenuRequested(int slot)

        Column {
            anchors.centerIn: parent
            spacing: 3

            QQC2.Label {
                width: 132
                horizontalAlignment: Text.AlignHCenter
                text: encRoot.encoderTitle
                font.pixelSize: 11
                fontSizeMode: Text.HorizontalFit
                minimumPixelSize: 9
                elide: Text.ElideRight
                font.bold: true
                color: Kirigami.Theme.textColor
            }

            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 5

                Rectangle {
                    width: 35
                    height: 48
                    radius: 6
                    color: encRoot.selectedSlot === encRoot.ccwSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.ccwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.ccwSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Column {
                        anchors.centerIn: parent
                        width: parent.width - 2
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟲"
                            font.pixelSize: 13
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.ccwLabel
                            font.pixelSize: 10
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 7
                            elide: Text.ElideRight
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        onClicked: (mouse) => {
                            if (mouse.button === Qt.RightButton) {
                                encRoot.contextMenuRequested(encRoot.ccwSlot)
                                return
                            }
                            encRoot.visualAngle -= 15
                            encRoot.actionTriggered(
                                encRoot.ccwSlot,
                                encRoot.encoderTitle + " Counter-Clockwise (Slot " + encRoot.ccwSlot + ")"
                            )
                        }
                    }
                }

                Rectangle {
                    id: knobBody
                    width: 48
                    height: 48
                    radius: 24
                    color: encRoot.selectedSlot === encRoot.pushSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.pushSlot ? 2 : 1.5
                    border.color: encRoot.selectedSlot === encRoot.pushSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Item {
                        anchors.fill: parent
                        rotation: encRoot.visualAngle
                        Behavior on rotation { NumberAnimation { duration: 90 } }

                        Rectangle {
                            anchors.top: parent.top
                            anchors.topMargin: 4
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: 3
                            height: 7
                            radius: 1.5
                            color: Kirigami.Theme.highlightColor
                        }
                    }

                    Column {
                        anchors.centerIn: parent
                        anchors.verticalCenterOffset: 2
                        width: parent.width - 6
                        spacing: 0
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: "PUSH"
                            font.pixelSize: 8
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 7
                            elide: Text.ElideRight
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.pushLabel
                            font.pixelSize: 10
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 7
                            elide: Text.ElideRight
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        onClicked: (mouse) => {
                            if (mouse.button === Qt.RightButton) {
                                encRoot.contextMenuRequested(encRoot.pushSlot)
                                return
                            }
                            encRoot.actionTriggered(
                                encRoot.pushSlot,
                                encRoot.encoderTitle + " Center Push (Slot " + encRoot.pushSlot + ")"
                            )
                        }
                        onWheel: (wheel) => {
                            if (wheel.angleDelta.y > 0) {
                                encRoot.visualAngle += 15
                                encRoot.actionTriggered(
                                    encRoot.cwSlot,
                                    encRoot.encoderTitle + " Clockwise (Slot " + encRoot.cwSlot + ")"
                                )
                            } else {
                                encRoot.visualAngle -= 15
                                encRoot.actionTriggered(
                                    encRoot.ccwSlot,
                                    encRoot.encoderTitle + " Counter-Clockwise (Slot " + encRoot.ccwSlot + ")"
                                )
                            }
                        }
                    }
                }

                Rectangle {
                    width: 35
                    height: 48
                    radius: 6
                    color: encRoot.selectedSlot === encRoot.cwSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.cwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.cwSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Column {
                        anchors.centerIn: parent
                        width: parent.width - 2
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟳"
                            font.pixelSize: 13
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.cwLabel
                            font.pixelSize: 10
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 7
                            elide: Text.ElideRight
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        onClicked: (mouse) => {
                            if (mouse.button === Qt.RightButton) {
                                encRoot.contextMenuRequested(encRoot.cwSlot)
                                return
                            }
                            encRoot.visualAngle += 15
                            encRoot.actionTriggered(
                                encRoot.cwSlot,
                                encRoot.encoderTitle + " Clockwise (Slot " + encRoot.cwSlot + ")"
                            )
                        }
                    }
                }
            }
        }
    }
}
