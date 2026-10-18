import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Window {
    id: duckyColorPickerWindow
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property bool isLightTheme: root ? root.isLightTheme : false
    property color highContrastPositiveText: root ? root.highContrastPositiveText : "#4ade80"
    property var hardwareCanvas: null
    property var scriptEditor: null
    property var profilePropertiesWindow: null
    property var keyPropertiesWindow: null
    property alias lightingAnimatorDialog: lightingAnimatorDialog

    component HighContrastScrollBar: QQC2.ScrollBar {
        id: hcBar
        objectName: "highContrastScrollBar"
        readonly property bool isProfileBoxStyleScrollBar: true
        readonly property int styleTrackThickness: 11
        readonly property int styleThumbThickness: 7
        readonly property int styleTrackRadius: 5
        readonly property int styleThumbRadius: 4
        policy: size < 1.0 ? QQC2.ScrollBar.AlwaysOn : QQC2.ScrollBar.AsNeeded
        minimumSize: 0.14
        implicitWidth: orientation === Qt.Vertical ? 11 : (parent ? parent.width : 40)
        implicitHeight: orientation === Qt.Horizontal ? 11 : (parent ? parent.height : 40)
        x: orientation === Qt.Vertical ? (parent ? parent.width - width : 0) : 0
        y: orientation === Qt.Horizontal ? (parent ? parent.height - height : 0) : 0
        width: orientation === Qt.Vertical ? 11 : (parent ? parent.width : implicitWidth)
        height: orientation === Qt.Horizontal ? 11 : (parent ? parent.height : implicitHeight)
        z: 50
        padding: 1
        background: Rectangle {
            visible: hcBar.size < 1.0 || hcBar.policy === QQC2.ScrollBar.AlwaysOn
            implicitWidth: hcBar.orientation === Qt.Vertical ? 11 : 40
            implicitHeight: hcBar.orientation === Qt.Horizontal ? 11 : 40
            radius: 5
            color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.28)
            border.width: 1
        }
        contentItem: Rectangle {
            visible: hcBar.size < 1.0 || hcBar.policy === QQC2.ScrollBar.AlwaysOn
            implicitWidth: hcBar.orientation === Qt.Vertical ? 7 : 28
            implicitHeight: hcBar.orientation === Qt.Horizontal ? 7 : 28
            radius: 4
            color: hcBar.pressed
                ? Kirigami.Theme.highlightColor
                : (hcBar.hovered
                    ? Qt.tint(Kirigami.Theme.highlightColor, Qt.rgba(1, 1, 1, 0.22))
                    : Qt.alpha(Kirigami.Theme.textColor, 0.62))
            border.color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.55)
            border.width: 1
        }
    }

    component HighContrastCheckBox: QQC2.CheckBox {
        id: hcCheck
        spacing: 8
        onCheckedChanged: hcCheckMarkCanvas.requestPaint()
        indicator: Rectangle {
            objectName: "highContrastCheckBoxBox"
            implicitWidth: 18
            implicitHeight: 18
            width: 18
            height: 18
            x: hcCheck.leftPadding
            y: hcCheck.topPadding + Math.round((hcCheck.availableHeight - height) / 2)
            radius: 4
            color: hcCheck.checked
                ? Kirigami.Theme.highlightColor
                : (hcCheck.down
                    ? Qt.alpha(Kirigami.Theme.textColor, 0.22)
                    : Kirigami.Theme.backgroundColor)
            border.width: (hcCheck.checked || hcCheck.hovered || hcCheck.visualFocus) ? 2 : 1.5
            border.color: hcCheck.checked
                ? Kirigami.Theme.highlightColor
                : (hcCheck.hovered || hcCheck.visualFocus
                    ? Kirigami.Theme.highlightColor
                    : Qt.alpha(Kirigami.Theme.textColor, 0.78))

            function pixelMetric(name) {
                return name === "checkboxlabelspacing" ? 8 : 18
            }

            Canvas {
                id: hcCheckMarkCanvas
                anchors.fill: parent
                visible: hcCheck.checked
                onPaint: {
                    const ctx = getContext("2d")
                    ctx.reset()
                    const lum = 0.2126 * Kirigami.Theme.highlightColor.r
                        + 0.7152 * Kirigami.Theme.highlightColor.g
                        + 0.0722 * Kirigami.Theme.highlightColor.b
                    ctx.strokeStyle = lum > 0.52 ? "#020617" : "#ffffff"
                    ctx.lineWidth = 2.2
                    ctx.lineCap = "round"
                    ctx.lineJoin = "round"
                    ctx.beginPath()
                    ctx.moveTo(4.0, 9.5)
                    ctx.lineTo(7.5, 13.0)
                    ctx.lineTo(14.0, 5.0)
                    ctx.stroke()
                }
            }
        }
    }

    property var allProfileAnimationsStore: ({}) // { "P1": { profile: {...}, keys: { "1": {...} } } }

    function lightingStoreFilePath() {
        return (deviceController && deviceController.logFilePath)
            ? String(deviceController.logFilePath).replace(/duckydeck\.log$/, "lighting_animations.json")
            : ""
    }

    function defaultLightingConfig(isProfileScope) {
        return {
            enabled: false,
            trigger: isProfileScope ? "idle" : "keypress",
            preset: isProfileScope ? "wave_h" : "ripple",
            speed: 5,
            color1: "#00f0ff",
            color2: "#a855f7",
            idleTimeoutSec: 5,
            streamToHardware: true,
            embedInKeyScript: !isProfileScope
        }
    }

    function currentProfileAnimBucketKey() {
        const pnum = deviceController ? deviceController.activeProfile : 1
        const pname = (deviceController && deviceController.activeProfileName)
            ? String(deviceController.activeProfileName).trim()
            : ("Profile" + pnum)
        return "P" + pnum + ":" + pname
    }

    function getProfileLightingConfig() {
        const key = duckyColorPickerWindow.currentProfileAnimBucketKey()
        const bucket = (duckyColorPickerWindow.allProfileAnimationsStore || {})[key]
        if (bucket && bucket.profile) {
            return Object.assign(duckyColorPickerWindow.defaultLightingConfig(true), bucket.profile)
        }
        return duckyColorPickerWindow.defaultLightingConfig(true)
    }

    function getKeyLightingConfig(slotNum) {
        const key = duckyColorPickerWindow.currentProfileAnimBucketKey()
        const bucket = (duckyColorPickerWindow.allProfileAnimationsStore || {})[key]
        const sKey = String(slotNum)
        if (bucket && bucket.keys && bucket.keys[sKey]) {
            return Object.assign(duckyColorPickerWindow.defaultLightingConfig(false), bucket.keys[sKey])
        }
        // Also check if current slot script has an embedded // @DUCKYDECK_ANIM {...} comment
        if (slotNum === root.selectedSlotNumber && scriptEditor && scriptEditor.text) {
            const parsed = duckyColorPickerWindow.extractLightingAnimFromScript(scriptEditor.text)
            if (parsed) {
                return Object.assign(duckyColorPickerWindow.defaultLightingConfig(false), parsed)
            }
        }
        return duckyColorPickerWindow.defaultLightingConfig(false)
    }

    function extractLightingAnimFromScript(scriptText) {
        const lines = String(scriptText || "").split("\n")
        for (let i = 0; i < lines.length; ++i) {
            const trimmed = lines[i].trim()
            if (trimmed.indexOf("// @DUCKYDECK_ANIM ") === 0) {
                try {
                    const obj = JSON.parse(trimmed.slice(19).trim())
                    if (obj && typeof obj === "object") return obj
                } catch (e) {}
            }
        }
        return null
    }

    function buildKeypressDuckyScriptAnimationBlock(cfg, slotNum) {
        if (!cfg || !cfg.enabled) return ""
        const c1 = hardwareCanvas.parseHexRgb(cfg.color1 || "#00f0ff", 0, 240, 255)
        const c2 = hardwareCanvas.parseHexRgb(cfg.color2 || "#a855f7", 168, 85, 247)
        const stepDelay = Math.max(20, Math.round(135 - (cfg.speed || 5) * 11))
        const targetKey = (slotNum >= 1 && slotNum <= 20) ? slotNum : 1
        const originCol = (targetKey - 1) % 4
        const originRow = Math.floor((targetKey - 1) / 4)
        const preset = cfg.preset || "ripple"
        const metaJson = JSON.stringify({
            enabled: true,
            trigger: cfg.trigger || "keypress",
            preset: preset,
            speed: cfg.speed || 5,
            color1: cfg.color1 || "#00f0ff",
            color2: cfg.color2 || "#a855f7",
            idleTimeoutSec: cfg.idleTimeoutSec || 5,
            streamToHardware: cfg.streamToHardware !== false,
            embedInKeyScript: cfg.embedInKeyScript !== false
        })

        const out = [
            "// @DUCKYDECK_ANIM " + metaJson,
            "// --- DuckyDeck Studio RGB Keypress Animation (" + preset + ") ---"
        ]

        if (preset === "pulse" || preset === "starlight") {
            const steps = [1.0, 0.75, 0.45, 0.2, 0.65, 1.0, 0.35]
            for (let s = 0; s < steps.length; ++s) {
                const w = steps[s]
                const r = Math.round(c2[0] + (c1[0] - c2[0]) * w)
                const g = Math.round(c2[1] + (c1[1] - c2[1]) * w)
                const b = Math.round(c2[2] + (c1[2] - c2[2]) * w)
                out.push("SWC_SET " + targetKey + " " + r + " " + g + " " + b)
                out.push("DELAY " + stepDelay)
            }
            out.push("SWC_RESET " + targetKey)
        } else if (preset === "ripple") {
            for (let ring = 0; ring <= 3; ++ring) {
                for (let k = 1; k <= 20; ++k) {
                    const kc = (k - 1) % 4
                    const kr = Math.floor((k - 1) / 4)
                    const d = Math.round(Math.hypot(kc - originCol, kr - originRow))
                    if (d === ring) {
                        out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                    }
                }
                out.push("DELAY " + stepDelay)
                if (ring > 0) {
                    for (let k = 1; k <= 20; ++k) {
                        const kc = (k - 1) % 4
                        const kr = Math.floor((k - 1) / 4)
                        const d = Math.round(Math.hypot(kc - originCol, kr - originRow))
                        if (d === ring - 1) {
                            out.push("SWC_RESET " + k)
                        }
                    }
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "wave_h" || preset === "scanner") {
            for (let col = 0; col < 4; ++col) {
                for (let row = 0; row < 5; ++row) {
                    const k = row * 4 + col + 1
                    out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                }
                out.push("DELAY " + stepDelay)
                for (let row = 0; row < 5; ++row) {
                    const k = row * 4 + col + 1
                    out.push("SWC_RESET " + k)
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "wave_v") {
            for (let row = 0; row < 5; ++row) {
                for (let col = 0; col < 4; ++col) {
                    const k = row * 4 + col + 1
                    out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                }
                out.push("DELAY " + stepDelay)
                for (let col = 0; col < 4; ++col) {
                    const k = row * 4 + col + 1
                    out.push("SWC_RESET " + k)
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "spectrum") {
            for (let f = 0; f < 6; ++f) {
                const rgb = hardwareCanvas.hsvToRgb(f / 6.0, 0.95, 1.0)
                out.push("SWC_SET " + targetKey + " " + rgb[0] + " " + rgb[1] + " " + rgb[2])
                out.push("DELAY " + stepDelay)
            }
            out.push("SWC_RESET " + targetKey)
        }
        out.push("// --- End DuckyDeck Studio RGB Animation ---")
        return out.join("\n")
    }

    function applyLightingAnimBlockToScript(existingScript, cfg, slotNum) {
        const lines = String(existingScript || "").split("\n")
        const cleaned = []
        let skippingBlock = false
        for (let i = 0; i < lines.length; ++i) {
            const t = lines[i].trim()
            if (t.indexOf("// @DUCKYDECK_ANIM ") === 0) {
                continue
            }
            if (t.indexOf("// --- DuckyDeck Studio RGB Keypress Animation") === 0) {
                skippingBlock = true
                continue
            }
            if (skippingBlock && t.indexOf("// --- End DuckyDeck Studio RGB Animation ---") === 0) {
                skippingBlock = false
                continue
            }
            if (!skippingBlock) {
                cleaned.push(lines[i])
            }
        }
        while (cleaned.length > 0 && cleaned[0].trim() === "") {
            cleaned.shift()
        }
        const body = cleaned.join("\n")
        if (!cfg || !cfg.enabled || cfg.embedInKeyScript === false) {
            return body
        }
        const block = duckyColorPickerWindow.buildKeypressDuckyScriptAnimationBlock(cfg, slotNum)
        return body.length > 0 ? (block + "\n\n" + body) : block
    }

    function syncLightingAnimationsForActiveProfile() {
        if (!hardwareCanvas) return
        const bKey = duckyColorPickerWindow.currentProfileAnimBucketKey()
        const store = Object.assign({}, duckyColorPickerWindow.allProfileAnimationsStore || {})
        if (!store[bKey]) {
            store[bKey] = { profile: duckyColorPickerWindow.defaultLightingConfig(true), keys: {} }
        }
        // Scan active profile slots for any embedded // @DUCKYDECK_ANIM metadata
        if (deviceController) {
            for (let s = 1; s <= 20; ++s) {
                const script = deviceController.getSlotScript(s, false)
                const embedded = duckyColorPickerWindow.extractLightingAnimFromScript(script)
                if (embedded && !store[bKey].keys[String(s)]) {
                    store[bKey].keys[String(s)] = embedded
                }
            }
        }
        duckyColorPickerWindow.allProfileAnimationsStore = store
        hardwareCanvas.profileAnimationConfig = Object.assign(duckyColorPickerWindow.defaultLightingConfig(true), store[bKey].profile || {})
        hardwareCanvas.keyAnimationConfigs = Object.assign({}, store[bKey].keys || {})
        hardwareCanvas.syncSlotsFromController()
    }

    function saveLightingAnimationConfig(scope, slotNum, cfg) {
        const bKey = duckyColorPickerWindow.currentProfileAnimBucketKey()
        const store = Object.assign({}, duckyColorPickerWindow.allProfileAnimationsStore || {})
        if (!store[bKey]) {
            store[bKey] = { profile: duckyColorPickerWindow.defaultLightingConfig(true), keys: {} }
        }
        if (scope === "profile") {
            store[bKey].profile = Object.assign({}, cfg)
            hardwareCanvas.profileAnimationConfig = Object.assign({}, cfg)
        } else {
            const sNum = (slotNum >= 1 && slotNum <= 20) ? slotNum : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 1)
            const keysMap = Object.assign({}, store[bKey].keys || {})
            keysMap[String(sNum)] = Object.assign({}, cfg)
            store[bKey].keys = keysMap
            hardwareCanvas.keyAnimationConfigs = keysMap

            // If embedInKeyScript is enabled (or disabled to remove previous block), update the slot's press script
            if (deviceController) {
                const existingPress = (root.hasSelectedSlot && root.selectedSlotNumber === sNum && !root.editingReleaseScript)
                    ? scriptEditor.text
                    : deviceController.getSlotScript(sNum, false)
                const updatedPress = duckyColorPickerWindow.applyLightingAnimBlockToScript(existingPress, cfg, sNum)
                if (updatedPress !== existingPress) {
                    if (root.hasSelectedSlot && root.selectedSlotNumber === sNum) {
                        root.pressScriptBuffer = updatedPress
                        if (!root.editingReleaseScript) {
                            scriptEditor.text = updatedPress
                        }
                        root.updateCompilerStatus()
                        root.saveCurrentSlotToController()
                    } else {
                        const meta = JSON.parse(deviceController.getSlotMetaJson(sNum) || "{}")
                        const rel = deviceController.getSlotScript(sNum, true)
                        deviceController.saveSlotConfig(
                            sNum,
                            meta.l1 || "",
                            meta.l2 || "",
                            updatedPress,
                            rel,
                            !!meta.allowAbort,
                            !!meta.disableAutorepeat
                        )
                    }
                }
            }
        }
        duckyColorPickerWindow.allProfileAnimationsStore = store
        hardwareCanvas.syncSlotsFromController()
        hardwareCanvas.notifyUserActivity(scope === "key" ? slotNum : 1)
        if (cfg.enabled && (cfg.trigger === "keypress" || cfg.trigger === "both")) {
            hardwareCanvas.triggerKeypressBurst(
                scope === "key" ? slotNum : 10,
                cfg,
                scope
            )
        } else if (cfg.enabled && cfg.trigger === "idle") {
            hardwareCanvas.idleAnimationActive = true
        } else if (!cfg.enabled) {
            hardwareCanvas.resetAnimatedKeyColors()
        }
    }


    ColorDialog {
        id: nativeSystemColorDialog
        title: qsTr("System Color Picker")
        selectedColor: duckyColorPickerWindow.currentColor
        onAccepted: {
            duckyColorPickerWindow.setColorFromQColor(selectedColor)
        }
    }


    Window {
        id: lightingAnimatorDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: lightingAnimatorDialog.visible
            onActivated: lightingAnimatorDialog.close()
        }
        title: qsTr("RGB Lighting Effects & Animations — DuckyDeck Studio")
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint
        width: Math.min(820, Screen.desktopAvailableWidth - 80)
        height: Math.min(680, Screen.desktopAvailableHeight - 80)
        minimumWidth: 680
        minimumHeight: 500
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property string targetScope: "profile" // "profile" or "key"
        property int targetKeySlot: 1
        property bool animEnabled: false
        property string animPreset: "wave_h"
        property string animTrigger: "both" // "idle" | "keypress" | "both"
        property int animSpeed: 5
        property string animColor1: "#38bdf8"
        property string animColor2: "#a855f7"
        property int animIdleTimeoutSec: 5
        property bool animPushToHardware: true
        property bool animCompileIntoScript: true
        property string statusBannerText: ""

        readonly property var presetCatalog: [
            { id: "pulse", icon: "💓", title: qsTr("Breathing / Pulse"), desc: qsTr("Smooth sinusoidal breathing between Primary & Secondary RGB colors") },
            { id: "wave_h", icon: "🌊", title: qsTr("Horizontal Wave"), desc: qsTr("Sweeping RGB color wave flowing left-to-right across key columns") },
            { id: "wave_v", icon: "💧", title: qsTr("Vertical Cascade"), desc: qsTr("Cascading RGB color wave flowing top-to-bottom down key rows") },
            { id: "ripple", icon: "💥", title: qsTr("Ripple / Shockwave"), desc: qsTr("Radial shockwave ring expanding outward from pressed or center key") },
            { id: "starlight", icon: "✨", title: qsTr("Reactive Starlight"), desc: qsTr("Twinkling multi-speed stardust sparkles across keycaps") },
            { id: "spectrum", icon: "🌈", title: qsTr("Rainbow Spectrum"), desc: qsTr("Continuous 360° HSV gaming rainbow cycle across all keys") },
            { id: "scanner", icon: "🚨", title: qsTr("Knight Rider Scanner"), desc: qsTr("Sweeping Cylon/KITT beam with glowing aftertrail") }
        ]

        readonly property var quickColors: [
            "#38bdf8", "#a855f7", "#ec4899", "#ef4444",
            "#f97316", "#eab308", "#22c55e", "#14b8a6",
            "#3b82f6", "#6366f1", "#ffffff", "#0f172a"
        ]

        function loadTargetConfig() {
            let cfg = (lightingAnimatorDialog.targetScope === "key")
                ? duckyColorPickerWindow.getKeyLightingConfig(lightingAnimatorDialog.targetKeySlot)
                : duckyColorPickerWindow.getProfileLightingConfig()
            if (!cfg) {
                cfg = duckyColorPickerWindow.defaultLightingConfig(lightingAnimatorDialog.targetScope !== "key")
            }
            lightingAnimatorDialog.animEnabled = Boolean(cfg.enabled)
            lightingAnimatorDialog.animPreset = cfg.preset ? String(cfg.preset) : "wave_h"
            lightingAnimatorDialog.animTrigger = cfg.trigger ? String(cfg.trigger) : "both"
            lightingAnimatorDialog.animSpeed = cfg.speed !== undefined ? Number(cfg.speed) : 5
            lightingAnimatorDialog.animColor1 = cfg.color1 ? String(cfg.color1) : "#38bdf8"
            lightingAnimatorDialog.animColor2 = cfg.color2 ? String(cfg.color2) : "#a855f7"
            lightingAnimatorDialog.animIdleTimeoutSec = cfg.idleTimeoutSec !== undefined ? Number(cfg.idleTimeoutSec) : 5
            lightingAnimatorDialog.animPushToHardware = cfg.streamToHardware !== undefined
                ? Boolean(cfg.streamToHardware)
                : (cfg.pushToHardware !== undefined ? Boolean(cfg.pushToHardware) : true)
            lightingAnimatorDialog.animCompileIntoScript = cfg.embedInKeyScript !== undefined
                ? Boolean(cfg.embedInKeyScript)
                : (cfg.compileIntoScript !== undefined ? Boolean(cfg.compileIntoScript) : true)
            lightingAnimatorDialog.statusBannerText = ""
        }

        function currentConfigObject() {
            return {
                enabled: lightingAnimatorDialog.animEnabled,
                preset: lightingAnimatorDialog.animPreset,
                trigger: lightingAnimatorDialog.animTrigger,
                speed: lightingAnimatorDialog.animSpeed,
                color1: lightingAnimatorDialog.animColor1,
                color2: lightingAnimatorDialog.animColor2,
                idleTimeoutSec: lightingAnimatorDialog.animIdleTimeoutSec,
                streamToHardware: lightingAnimatorDialog.animPushToHardware,
                embedInKeyScript: lightingAnimatorDialog.animCompileIntoScript,
                pushToHardware: lightingAnimatorDialog.animPushToHardware,
                compileIntoScript: lightingAnimatorDialog.animCompileIntoScript
            }
        }

        function openFor(scope, slotNumber) {
            lightingAnimatorDialog.targetScope = (scope === "key") ? "key" : "profile"
            if (slotNumber >= 1 && slotNumber <= 20) {
                lightingAnimatorDialog.targetKeySlot = slotNumber
            } else if (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20) {
                lightingAnimatorDialog.targetKeySlot = root.selectedSlotNumber
            } else {
                lightingAnimatorDialog.targetKeySlot = 1
            }
            lightingAnimatorDialog.loadTargetConfig()
            lightingAnimatorDialog.show()
            lightingAnimatorDialog.raise()
            lightingAnimatorDialog.requestActivate()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            // Header
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: Qt.alpha("#a855f7", 0.2)
                    border.color: "#c084fc"
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: "✨"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Gaming RGB Lighting Effects & Key Animations")
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Attach idle or reactive keypress animations (Pulse, Waves, Ripple, Starlight, Spectrum, Scanner) to an entire profile or an individual key.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }
            }

            // Scope Selector Bar (Profile-Wide vs Individual Key)
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: scopeRow.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1

                RowLayout {
                    id: scopeRow
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    QQC2.Label {
                        text: qsTr("Attach To:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("🎛 Profile #%1 (%2)")
                            .arg(deviceController.activeProfile)
                            .arg(deviceController.activeProfileName)
                        checkable: true
                        checked: lightingAnimatorDialog.targetScope === "profile"
                        highlighted: lightingAnimatorDialog.targetScope === "profile"
                        font.bold: lightingAnimatorDialog.targetScope === "profile"
                        onClicked: {
                            lightingAnimatorDialog.targetScope = "profile"
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("⌨ Individual Key")
                        checkable: true
                        checked: lightingAnimatorDialog.targetScope === "key"
                        highlighted: lightingAnimatorDialog.targetScope === "key"
                        font.bold: lightingAnimatorDialog.targetScope === "key"
                        onClicked: {
                            lightingAnimatorDialog.targetScope = "key"
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    QQC2.ComboBox {
                        font.pointSize: 10 * root.fontScale
                        id: animKeySelectorCombo
                        visible: lightingAnimatorDialog.targetScope === "key"
                        Layout.preferredWidth: 150
                        model: {
                            let arr = []
                            for (let k = 1; k <= 20; k++) {
                                const t = hardwareCanvas.slotTitleFor(k)
                                arr.push(qsTr("Key %1 — %2").arg(k).arg(t))
                            }
                            return arr
                        }
                        currentIndex: Math.max(0, Math.min(19, lightingAnimatorDialog.targetKeySlot - 1))
                        onActivated: (idx) => {
                            lightingAnimatorDialog.targetKeySlot = idx + 1
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Switch {
                        font.pointSize: 10 * root.fontScale
                        checked: lightingAnimatorDialog.animEnabled
                        text: lightingAnimatorDialog.animEnabled ? qsTr("Enabled") : qsTr("Disabled")
                        font.bold: true
                        onToggled: lightingAnimatorDialog.animEnabled = checked
                    }
                }
            }

            // Scrollable Settings Body
            Flickable {
                id: lightingFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: lightingScrollCol.implicitHeight + 8
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: lightingFlickable
                    anchors.top: lightingFlickable.top
                    anchors.right: lightingFlickable.right
                    anchors.bottom: lightingFlickable.bottom
                    policy: lightingFlickable.contentHeight > lightingFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: lightingScrollCol
                    width: Math.max(
                        200,
                        lightingFlickable.width - (lightingFlickable.contentHeight > lightingFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // Trigger Condition Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: triggerCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: triggerCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Animation Trigger:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("⚡ On Keypress + When Idle (Both)")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "both"
                                    highlighted: lightingAnimatorDialog.animTrigger === "both"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "both"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("👆 On Keypress Only")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "keypress"
                                    highlighted: lightingAnimatorDialog.animTrigger === "keypress"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "keypress"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("🌙 When Idle Only")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "idle"
                                    highlighted: lightingAnimatorDialog.animTrigger === "idle"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "idle"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }

                            RowLayout {
                                visible: lightingAnimatorDialog.animTrigger === "idle" || lightingAnimatorDialog.animTrigger === "both"
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Start Idle Animation After:")
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.SpinBox {
                                    font.pointSize: 10 * root.fontScale
                                    from: 0
                                    to: 300
                                    value: lightingAnimatorDialog.animIdleTimeoutSec
                                    onValueModified: lightingAnimatorDialog.animIdleTimeoutSec = value
                                }

                                QQC2.Label {
                                    text: qsTr("seconds of inactivity (0 = animate continuously)")
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                }
                            }
                        }
                    }

                    // Preset Cards Grid
                    QQC2.Label {
                        text: qsTr("Select Animation Preset:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                    }

                    GridLayout {
                        Layout.fillWidth: true
                        columns: 2
                        rowSpacing: 8
                        columnSpacing: 8

                        Repeater {
                            model: lightingAnimatorDialog.presetCatalog

                            delegate: Rectangle {
                                required property var modelData
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: presetCardRow.implicitHeight + 16
                                radius: 6
                                readonly property bool isSelected: lightingAnimatorDialog.animPreset === modelData.id
                                color: isSelected
                                    ? Qt.alpha("#a855f7", 0.22)
                                    : (presetMouse.containsMouse ? Qt.lighter(Kirigami.Theme.alternateBackgroundColor, 1.15) : Kirigami.Theme.alternateBackgroundColor)
                                border.color: isSelected ? "#c084fc" : Qt.alpha(Kirigami.Theme.textColor, 0.16)
                                border.width: isSelected ? 2 : 1

                                RowLayout {
                                    id: presetCardRow
                                    anchors.fill: parent
                                    anchors.margins: 8
                                    spacing: 8

                                    QQC2.Label {
                                        text: modelData.icon
                                        font.pointSize: 14 * root.fontScale
                                    }

                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 0
                                        Layout.preferredWidth: 1
                                        spacing: 2
                                        QQC2.Label {
                                            text: modelData.title
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: isSelected
                                                ? (root.isLightTheme ? "#6b21a8" : "#f3e8ff")
                                                : Kirigami.Theme.textColor
                                            Layout.fillWidth: true
                                            elide: Text.ElideRight
                                        }
                                        QQC2.Label {
                                            text: modelData.desc
                                            font.pointSize: 10 * root.fontScale
                                            color: isSelected
                                                ? Qt.alpha(Kirigami.Theme.textColor, 0.82)
                                                : Kirigami.Theme.disabledTextColor
                                            wrapMode: Text.WordWrap
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 0
                                        }
                                    }
                                }

                                MouseArea {
                                    id: presetMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: {
                                        lightingAnimatorDialog.animPreset = modelData.id
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }
                            }
                        }
                    }

                    // Speed & Colors Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: speedColorCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: speedColorCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                QQC2.Label {
                                    text: qsTr("Animation Speed:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                QQC2.Slider {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    from: 1
                                    to: 10
                                    stepSize: 1
                                    snapMode: QQC2.Slider.SnapAlways
                                    value: lightingAnimatorDialog.animSpeed
                                    onMoved: lightingAnimatorDialog.animSpeed = Math.round(value)
                                }

                                QQC2.Label {
                                    text: qsTr("%1 / 10").arg(lightingAnimatorDialog.animSpeed)
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 48
                                }
                            }

                            // Primary Color Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Primary Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: lightingAnimatorDialog.animColor1
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    Layout.preferredWidth: 86
                                    text: lightingAnimatorDialog.animColor1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    maximumLength: 7
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            lightingAnimatorDialog.animColor1 = text.trim()
                                        }
                                    }
                                }

                                Repeater {
                                    model: lightingAnimatorDialog.quickColors
                                    delegate: Rectangle {
                                        required property string modelData
                                        width: 18
                                        height: 18
                                        radius: 3
                                        color: modelData
                                        border.color: lightingAnimatorDialog.animColor1.toLowerCase() === modelData.toLowerCase()
                                            ? "#ffffff"
                                            : "#00000055"
                                        border.width: lightingAnimatorDialog.animColor1.toLowerCase() === modelData.toLowerCase() ? 2 : 1
                                        MouseArea {
                                            anchors.fill: parent
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.animColor1 = modelData
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }

                            // Secondary Color Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Secondary Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: lightingAnimatorDialog.animColor2
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    Layout.preferredWidth: 86
                                    text: lightingAnimatorDialog.animColor2
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    maximumLength: 7
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            lightingAnimatorDialog.animColor2 = text.trim()
                                        }
                                    }
                                }

                                Repeater {
                                    model: lightingAnimatorDialog.quickColors
                                    delegate: Rectangle {
                                        required property string modelData
                                        width: 18
                                        height: 18
                                        radius: 3
                                        color: modelData
                                        border.color: lightingAnimatorDialog.animColor2.toLowerCase() === modelData.toLowerCase()
                                            ? "#ffffff"
                                            : "#00000055"
                                        border.width: lightingAnimatorDialog.animColor2.toLowerCase() === modelData.toLowerCase() ? 2 : 1
                                        MouseArea {
                                            anchors.fill: parent
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.animColor2 = modelData
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }
                        }
                    }

                    // Hardware Streaming & Autonomous DuckyScript Compilation Options
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: hwOptsCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: hwOptsCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            HighContrastCheckBox {
                                id: animPushHwCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                checked: lightingAnimatorDialog.animPushToHardware
                                text: qsTr("💻 [Host App Required — Not Hardware-Portable] Stream live RGB animation frames to physical duckyPad LEDs over USB HID while DuckyDeck Studio is running")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                onToggled: lightingAnimatorDialog.animPushToHardware = checked
                                contentItem: QQC2.Label {
                                    leftPadding: (animPushHwCheck.indicator ? animPushHwCheck.indicator.width : 18) + animPushHwCheck.spacing
                                    text: animPushHwCheck.text
                                    font: animPushHwCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            HighContrastCheckBox {
                                id: animCompileScriptCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                visible: lightingAnimatorDialog.targetScope === "key" && (lightingAnimatorDialog.animTrigger === "keypress" || lightingAnimatorDialog.animTrigger === "both")
                                checked: lightingAnimatorDialog.animCompileIntoScript
                                text: qsTr("✅ [Hardware-Portable — Runs Without PC App] Embed standalone DuckyScript 3 SWC_SET / SWC_RESET animation block into Key %1's script on microSD")
                                    .arg(lightingAnimatorDialog.targetKeySlot)
                                font.pointSize: 10 * root.fontScale
                                onToggled: lightingAnimatorDialog.animCompileIntoScript = checked
                                contentItem: QQC2.Label {
                                    leftPadding: (animCompileScriptCheck.indicator ? animCompileScriptCheck.indicator.width : 18) + animCompileScriptCheck.spacing
                                    text: animCompileScriptCheck.text
                                    font: animCompileScriptCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Status feedback banner
                    Rectangle {
                        visible: lightingAnimatorDialog.statusBannerText.length > 0
                        Layout.fillWidth: true
                        implicitHeight: statusLbl.implicitHeight + 12
                        radius: 5
                        color: Qt.alpha("#22c55e", 0.16)
                        border.color: "#22c55e"
                        border.width: 1

                        QQC2.Label {
                            id: statusLbl
                            anchors.fill: parent
                            anchors.margins: 6
                            text: lightingAnimatorDialog.statusBannerText
                            color: "#86efac"
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            wrapMode: Text.WordWrap
                        }
                    }
                }
            }

            // Bottom Action Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("🗑 Disable / Clear Effect")
                    onClicked: {
                        lightingAnimatorDialog.animEnabled = false
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        duckyColorPickerWindow.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        lightingAnimatorDialog.statusBannerText = qsTr("Cleared animation effect for %1.")
                            .arg(lightingAnimatorDialog.targetScope === "key"
                                ? qsTr("Key %1").arg(lightingAnimatorDialog.targetKeySlot)
                                : qsTr("Profile #%1").arg(deviceController.activeProfile))
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("▶ Preview Burst Now")
                    onClicked: {
                        lightingAnimatorDialog.animEnabled = true
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        duckyColorPickerWindow.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        const previewSlot = lightingAnimatorDialog.targetScope === "key"
                            ? lightingAnimatorDialog.targetKeySlot
                            : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 8)
                        hardwareCanvas.triggerKeypressBurst(previewSlot, cfg, lightingAnimatorDialog.targetScope)
                        lightingAnimatorDialog.statusBannerText = qsTr("Previewing '%1' on Hardware Canvas%2!")
                            .arg(lightingAnimatorDialog.animPreset)
                            .arg(deviceController.connected && lightingAnimatorDialog.animPushToHardware ? qsTr(" and physical duckyPad") : "")
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("Close")
                    onClicked: lightingAnimatorDialog.close()
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("💾 Save & Apply Animation")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        duckyColorPickerWindow.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        const previewSlot = lightingAnimatorDialog.targetScope === "key"
                            ? lightingAnimatorDialog.targetKeySlot
                            : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 8)
                        if (cfg.enabled) {
                            hardwareCanvas.triggerKeypressBurst(previewSlot, cfg, lightingAnimatorDialog.targetScope)
                        }
                        lightingAnimatorDialog.statusBannerText = qsTr("Saved lighting animation for %1.")
                            .arg(lightingAnimatorDialog.targetScope === "key"
                                ? qsTr("Key %1").arg(lightingAnimatorDialog.targetKeySlot)
                                : qsTr("Profile #%1 (%2)").arg(deviceController.activeProfile).arg(deviceController.activeProfileName))
                    }
                }
            }
        }
    }

    readonly property bool closableWithEscape: true
    title: pickerMode === "key"
        ? qsTr("Set Key %1 Color — DuckyDeck Studio").arg(targetSlotNumber)
        : qsTr("Set Default Color (BG_COLOR) — Profile #%1 (%2)").arg(targetProfileNumber).arg(targetProfileName)
    width: Math.min(720, Screen.desktopAvailableWidth > 0 ? Screen.desktopAvailableWidth - 60 : 720)
    height: Math.min(780, Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight - 60 : 780)
    minimumWidth: 640
    minimumHeight: 680
    visible: false
    transientParent: root
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    property string pickerMode: "key" // "key" or "profile_bg"
    property int targetSlotNumber: 1
    property int targetProfileNumber: 1
    property string targetProfileName: "Default"

    property string initialHex: "#00aaff"
    property real hue: 0.55
    property real saturation: 1.0
    property real val: 1.0
    property int redVal: 0
    property int greenVal: 170
    property int blueVal: 255
    property bool updatingInternally: false

    property var savedColorsList: []
    property var usedColorsList: []

    readonly property color currentColor: Qt.rgba(redVal / 255.0, greenVal / 255.0, blueVal / 255.0, 1.0)
    readonly property string currentHex: formatRgbHex(redVal, greenVal, blueVal)

    function clampByte(n) {
        const v = Math.round(Number(n) || 0)
        return Math.max(0, Math.min(255, v))
    }

    function byteToHex(n) {
        const s = clampByte(n).toString(16)
        return s.length < 2 ? ("0" + s) : s
    }

    function formatRgbHex(r, g, b) {
        return "#" + byteToHex(r) + byteToHex(g) + byteToHex(b)
    }

    function refreshPaletteLists() {
        if (!deviceController) return
        deviceController.refreshColorPalettes()
        try {
            const sArr = JSON.parse(deviceController.savedColorsJson || "[]")
            savedColorsList = Array.isArray(sArr) ? sArr : []
        } catch (e) {
            savedColorsList = []
        }
        try {
            const uArr = JSON.parse(deviceController.usedColorsJson || "[]")
            usedColorsList = Array.isArray(uArr) ? uArr : []
        } catch (e) {
            usedColorsList = []
        }
    }

    function syncRgbFromHsv() {
        if (updatingInternally) return
        updatingInternally = true
        const c = Qt.hsva(
            Math.max(0.0, Math.min(0.9999, hue)),
            Math.max(0.0, Math.min(1.0, saturation)),
            Math.max(0.0, Math.min(1.0, val)),
            1.0
        )
        redVal = clampByte(c.r * 255)
        greenVal = clampByte(c.g * 255)
        blueVal = clampByte(c.b * 255)
        hexTextField.text = formatRgbHex(redVal, greenVal, blueVal)
        updatingInternally = false
    }

    function syncHsvFromRgb() {
        if (updatingInternally) return
        updatingInternally = true
        const c = Qt.rgba(redVal / 255.0, greenVal / 255.0, blueVal / 255.0, 1.0)
        if (c.hsvHue >= 0) {
            hue = c.hsvHue
        }
        saturation = c.hsvSaturation
        val = c.hsvValue
        hexTextField.text = formatRgbHex(redVal, greenVal, blueVal)
        updatingInternally = false
    }

    function setColorFromHex(hexStr) {
        if (!hexStr) return
        let clean = String(hexStr).trim()
        if (clean.startsWith("#")) {
            clean = clean.slice(1)
        }
        if (clean.length === 3) {
            clean = clean[0] + clean[0] + clean[1] + clean[1] + clean[2] + clean[2]
        }
        if (clean.length !== 6) return
        const r = parseInt(clean.slice(0, 2), 16)
        const g = parseInt(clean.slice(2, 4), 16)
        const b = parseInt(clean.slice(4, 6), 16)
        if (isNaN(r) || isNaN(g) || isNaN(b)) return
        redVal = clampByte(r)
        greenVal = clampByte(g)
        blueVal = clampByte(b)
        syncHsvFromRgb()
    }

    function setColorFromQColor(c) {
        if (!c) return
        redVal = clampByte(c.r * 255)
        greenVal = clampByte(c.g * 255)
        blueVal = clampByte(c.b * 255)
        syncHsvFromRgb()
    }

    function openForKey(slotNum, startHex) {
        pickerMode = "key"
        targetSlotNumber = slotNum
        initialHex = (startHex && startHex.length > 0) ? startHex : "#00aaff"
        refreshPaletteLists()
        setColorFromHex(initialHex)
        initialHex = currentHex
        show()
        raise()
        requestActivate()
    }

    function openForProfileBg(profNum, profName, startHex) {
        pickerMode = "profile_bg"
        targetProfileNumber = profNum
        targetProfileName = profName || "Default"
        initialHex = (startHex && startHex.length > 0) ? startHex : "#00aaff"
        refreshPaletteLists()
        setColorFromHex(initialHex)
        initialHex = currentHex
        show()
        raise()
        requestActivate()
    }

    function applySelectedColor() {
        if (pickerMode === "key") {
            hardwareCanvas.setKeyRgbAt(targetSlotNumber, redVal, greenVal, blueVal)
            if (root.selectedSlotNumber === targetSlotNumber) {
                root.selectedSlotColor = currentHex
            }
            if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlotNumber) {
                keyPropertiesWindow.slotColor = currentHex
                keyPropertiesWindow.keyPropColorField.text = currentHex
            }
        } else {
            deviceController.setProfileBgColor(targetProfileNumber, redVal, greenVal, blueVal)
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncSlotsFromController()
            if (root.hasSelectedSlot) {
                root.selectedSlotColor = hardwareCanvas.slotColorFor(root.selectedSlotNumber)
            }
            if (profilePropertiesWindow.visible && profilePropertiesWindow.profNumber === targetProfileNumber) {
                profilePropertiesWindow.profBgColor = currentHex
                profilePropertiesWindow.profBgColorField.text = currentHex
            }
        }
        refreshPaletteLists()
        close()
    }

    Connections {
        target: deviceController
        function onSavedColorsJsonChanged() {
            if (duckyColorPickerWindow.visible) {
                try {
                    const sArr = JSON.parse(deviceController.savedColorsJson || "[]")
                    duckyColorPickerWindow.savedColorsList = Array.isArray(sArr) ? sArr : []
                } catch (e) {}
            }
        }
        function onUsedColorsJsonChanged() {
            if (duckyColorPickerWindow.visible) {
                try {
                    const uArr = JSON.parse(deviceController.usedColorsJson || "[]")
                    duckyColorPickerWindow.usedColorsList = Array.isArray(uArr) ? uArr : []
                } catch (e) {}
            }
        }
    }

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: duckyColorPickerWindow.visible
        onActivated: duckyColorPickerWindow.close()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 14
        spacing: 10

        // Header bar with target info & Current vs New preview
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2

                QQC2.Label {
                    text: duckyColorPickerWindow.pickerMode === "key"
                        ? qsTr("Key %1 Color").arg(duckyColorPickerWindow.targetSlotNumber)
                        : qsTr("Profile #%1 (%2) Default Color").arg(duckyColorPickerWindow.targetProfileNumber).arg(duckyColorPickerWindow.targetProfileName)
                    font.bold: true
                    font.pointSize: 12 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.Label {
                    text: duckyColorPickerWindow.pickerMode === "key"
                        ? qsTr("Pick a custom per-key RGB LED color (SWCOLOR) or choose from your Saved & Used palettes.")
                        : qsTr("Pick the default key background LED color (BG_COLOR) for this profile.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    elide: Text.ElideRight
                    Layout.fillWidth: true
                }
            }

            // Current vs Selected Swatch comparison
            Rectangle {
                Layout.preferredWidth: 116
                Layout.preferredHeight: 38
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.25)
                border.width: 1
                clip: true

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 3
                    spacing: 3

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        radius: 4
                        color: duckyColorPickerWindow.initialHex
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                        border.width: 1

                        QQC2.Label {
                            anchors.centerIn: parent
                            text: qsTr("Old")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: Qt.color(duckyColorPickerWindow.initialHex).hslLightness > 0.55 ? "#111111" : "#ffffff"
                        }

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: duckyColorPickerWindow.setColorFromHex(duckyColorPickerWindow.initialHex)
                            QQC2.ToolTip.visible: containsMouse
                            QQC2.ToolTip.text: qsTr("Revert to previous color (%1)").arg(duckyColorPickerWindow.initialHex)
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        radius: 4
                        color: duckyColorPickerWindow.currentColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                        border.width: 1

                        QQC2.Label {
                            anchors.centerIn: parent
                            text: qsTr("New")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: duckyColorPickerWindow.currentColor.hslLightness > 0.55 ? "#111111" : "#ffffff"
                        }
                    }
                }
            }
        }

        // Main Interactive Picker Area: 2D SV Square + Hue Strip + RGB Sliders & Hex
        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 190
            spacing: 12

            // 2D Saturation / Value Square
            Item {
                id: svSquareContainer
                Layout.preferredWidth: 190
                Layout.fillHeight: true

                Rectangle {
                    id: svBaseHueRect
                    anchors.fill: parent
                    radius: 6
                    color: Qt.hsva(Math.max(0.0, Math.min(0.9999, duckyColorPickerWindow.hue)), 1.0, 1.0, 1.0)
                     clip: true

                    // Horizontal Saturation gradient (White -> Transparent)
                    Rectangle {
                        width: parent.height
                        height: parent.width
                        anchors.centerIn: parent
                        rotation: -90
                        gradient: Gradient {
                            GradientStop { position: 0.0; color: "#ffffff" }
                            GradientStop { position: 1.0; color: "#00ffffff" }
                        }
                    }

                    // Vertical Value gradient (Transparent -> Black)
                    Rectangle {
                        anchors.fill: parent
                        radius: 6
                        gradient: Gradient {
                            GradientStop { position: 0.0; color: "#00000000" }
                            GradientStop { position: 1.0; color: "#ff000000" }
                        }
                    }

                    Rectangle {
                        anchors.fill: parent
                        radius: 6
                        color: "transparent"
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
                        border.width: 1
                    }

                    // Crosshair indicator
                    Rectangle {
                        x: Math.round(duckyColorPickerWindow.saturation * parent.width) - width / 2
                        y: Math.round((1.0 - duckyColorPickerWindow.val) * parent.height) - height / 2
                        width: 14
                        height: 14
                        radius: 7
                        color: "transparent"
                        border.color: duckyColorPickerWindow.val > 0.6 && duckyColorPickerWindow.saturation < 0.4 ? "#000000" : "#ffffff"
                        border.width: 2

                        Rectangle {
                            anchors.fill: parent
                            anchors.margins: 2
                            radius: 5
                            color: duckyColorPickerWindow.currentColor
                            border.color: "#000000"
                            border.width: 1
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.CrossCursor
                        function updateSv(mx, my) {
                            duckyColorPickerWindow.saturation = Math.max(0.0, Math.min(1.0, mx / width))
                            duckyColorPickerWindow.val = Math.max(0.0, Math.min(1.0, 1.0 - (my / height)))
                            duckyColorPickerWindow.syncRgbFromHsv()
                        }
                        onPressed: (mouse) => updateSv(mouse.x, mouse.y)
                        onPositionChanged: (mouse) => {
                            if (pressed) updateSv(mouse.x, mouse.y)
                        }
                    }
                }
            }

            // Vertical Hue Slider Bar
            Item {
                id: hueBarContainer
                Layout.preferredWidth: 24
                Layout.fillHeight: true

                Rectangle {
                    anchors.fill: parent
                    radius: 6
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
                    border.width: 1
                    gradient: Gradient {
                        GradientStop { position: 0.00; color: "#ff0000" }
                        GradientStop { position: 0.17; color: "#ffff00" }
                        GradientStop { position: 0.33; color: "#00ff00" }
                        GradientStop { position: 0.50; color: "#00ffff" }
                        GradientStop { position: 0.67; color: "#0000ff" }
                        GradientStop { position: 0.83; color: "#ff00ff" }
                        GradientStop { position: 1.00; color: "#ff0000" }
                    }

                    Rectangle {
                        x: -2
                        y: Math.round(duckyColorPickerWindow.hue * (parent.height - height))
                        width: parent.width + 4
                        height: 8
                        radius: 3
                        color: "transparent"
                        border.color: "#ffffff"
                        border.width: 2

                        Rectangle {
                            anchors.fill: parent
                            anchors.margins: 1
                            radius: 2
                            color: "transparent"
                            border.color: "#000000"
                            border.width: 1
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        function updateHue(my) {
                            duckyColorPickerWindow.hue = Math.max(0.0, Math.min(0.9999, my / height))
                            duckyColorPickerWindow.syncRgbFromHsv()
                        }
                        onPressed: (mouse) => updateHue(mouse.y)
                        onPositionChanged: (mouse) => {
                            if (pressed) updateHue(mouse.y)
                        }
                    }
                }
            }

            // RGB Sliders + SpinBoxes + Hex Input + System Color Picker button
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 6

                // Red Row
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6
                    QQC2.Label {
                        font.pointSize: 10 * root.fontScale
                        text: "R"
                        font.bold: true
                        color: root.isLightTheme ? "#dc2626" : "#f87171"
                        Layout.preferredWidth: 14
                    }
                    QQC2.Slider {
                        Layout.fillWidth: true
                        from: 0
                        to: 255
                        stepSize: 1
                        value: duckyColorPickerWindow.redVal
                        onMoved: {
                            duckyColorPickerWindow.redVal = Math.round(value)
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                    QQC2.SpinBox {
                        font.pointSize: 10 * root.fontScale
                        from: 0
                        to: 255
                        value: duckyColorPickerWindow.redVal
                        editable: true
                        Layout.preferredWidth: 68
                        onValueModified: {
                            duckyColorPickerWindow.redVal = value
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                }

                // Green Row
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6
                    QQC2.Label {
                        font.pointSize: 10 * root.fontScale
                        text: "G"
                        font.bold: true
                        color: root.highContrastPositiveText
                        Layout.preferredWidth: 14
                    }
                    QQC2.Slider {
                        Layout.fillWidth: true
                        from: 0
                        to: 255
                        stepSize: 1
                        value: duckyColorPickerWindow.greenVal
                        onMoved: {
                            duckyColorPickerWindow.greenVal = Math.round(value)
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                    QQC2.SpinBox {
                        font.pointSize: 10 * root.fontScale
                        from: 0
                        to: 255
                        value: duckyColorPickerWindow.greenVal
                        editable: true
                        Layout.preferredWidth: 68
                        onValueModified: {
                            duckyColorPickerWindow.greenVal = value
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                }

                // Blue Row
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6
                    QQC2.Label {
                        font.pointSize: 10 * root.fontScale
                        text: "B"
                        font.bold: true
                        color: root.isLightTheme ? "#2563eb" : "#60a5fa"
                        Layout.preferredWidth: 14
                    }
                    QQC2.Slider {
                        Layout.fillWidth: true
                        from: 0
                        to: 255
                        stepSize: 1
                        value: duckyColorPickerWindow.blueVal
                        onMoved: {
                            duckyColorPickerWindow.blueVal = Math.round(value)
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                    QQC2.SpinBox {
                        font.pointSize: 10 * root.fontScale
                        from: 0
                        to: 255
                        value: duckyColorPickerWindow.blueVal
                        editable: true
                        Layout.preferredWidth: 68
                        onValueModified: {
                            duckyColorPickerWindow.blueVal = value
                            duckyColorPickerWindow.syncHsvFromRgb()
                        }
                    }
                }

                Item { Layout.fillHeight: true }

                // Hex field + System Picker
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Label {
                        text: qsTr("Hex:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.TextField {
                        id: hexTextField
                        Layout.preferredWidth: 96
                        text: duckyColorPickerWindow.currentHex
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        maximumLength: 7
                        selectByMouse: true
                        onEditingFinished: {
                            duckyColorPickerWindow.setColorFromHex(text)
                        }
                        onAccepted: {
                            duckyColorPickerWindow.setColorFromHex(text)
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "color-picker"
                        text: qsTr("System Picker...")
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            nativeSystemColorDialog.selectedColor = duckyColorPickerWindow.currentColor
                            nativeSystemColorDialog.open()
                        }
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Open native desktop color dialog / screen eyedropper")
                    }
                }
            }
        }

        // Divider
        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
        }

        // SAVED COLORS Palette Section
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 6

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                QQC2.Label {
                    text: qsTr("SAVED COLORS (%1)").arg(duckyColorPickerWindow.savedColorsList.length)
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.Label {
                    text: qsTr("• Left-click to select, right-click to remove")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    icon.name: "list-add"
                    text: qsTr("Save Current (%1)").arg(duckyColorPickerWindow.currentHex)
                    implicitHeight: 24
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        deviceController.savePaletteColor(duckyColorPickerWindow.currentHex)
                        duckyColorPickerWindow.refreshPaletteLists()
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Save %1 to your persistent Saved Colors palette").arg(duckyColorPickerWindow.currentHex)
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: Math.min(130, Math.max(84, savedColorsFlow.implicitHeight + 16))
                Layout.maximumHeight: 130
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1
                clip: true

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 6
                    clip: true
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    Flow {
                        id: savedColorsFlow
                        width: parent.width
                        spacing: 6

                        Repeater {
                            model: duckyColorPickerWindow.savedColorsList

                            delegate: Rectangle {
                                id: savedSwatch
                                required property string modelData
                                readonly property bool isCurrentMatch: modelData.toLowerCase() === duckyColorPickerWindow.currentHex.toLowerCase()
                                width: 30
                                height: 30
                                radius: 5
                                color: modelData
                                border.width: isCurrentMatch ? 2 : (savedSwatchMouse.containsMouse ? 2 : 1)
                                border.color: isCurrentMatch
                                    ? Kirigami.Theme.highlightColor
                                    : (savedSwatchMouse.containsMouse ? "#ffffff" : Qt.alpha("#000000", 0.45))

                                // Inner ring when selected
                                Rectangle {
                                    visible: savedSwatch.isCurrentMatch
                                    anchors.fill: parent
                                    anchors.margins: 2
                                    radius: 3
                                    color: "transparent"
                                    border.color: "#ffffff"
                                    border.width: 1
                                }

                                MouseArea {
                                    id: savedSwatchMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: (mouse) => {
                                        if (mouse.button === Qt.RightButton) {
                                            deviceController.removePaletteColor(savedSwatch.modelData)
                                            duckyColorPickerWindow.refreshPaletteLists()
                                        } else {
                                            duckyColorPickerWindow.setColorFromHex(savedSwatch.modelData)
                                        }
                                    }
                                    onDoubleClicked: (mouse) => {
                                        if (mouse.button === Qt.LeftButton) {
                                            duckyColorPickerWindow.setColorFromHex(savedSwatch.modelData)
                                            duckyColorPickerWindow.applySelectedColor()
                                        }
                                    }
                                }

                                QQC2.ToolTip.visible: savedSwatchMouse.containsMouse
                                QQC2.ToolTip.text: qsTr("%1 • Left-click to select • Double-click to apply • Right-click to remove").arg(savedSwatch.modelData)
                            }
                        }
                    }
                }
            }
        }

        // USED COLORS Palette Section
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 6

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                QQC2.Label {
                    text: qsTr("USED COLORS IN PROFILES & KEYS (%1)").arg(duckyColorPickerWindow.usedColorsList.length)
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.Label {
                    text: qsTr("• Left-click to select, right-click to save to palette")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 74
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1
                clip: true

                QQC2.Label {
                    anchors.centerIn: parent
                    visible: duckyColorPickerWindow.usedColorsList.length === 0
                    text: qsTr("No profile or key colors loaded yet.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 6
                    clip: true
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    Flow {
                        width: parent.width
                        spacing: 6

                        Repeater {
                            model: duckyColorPickerWindow.usedColorsList

                            delegate: Rectangle {
                                id: usedSwatch
                                required property string modelData
                                readonly property bool isCurrentMatch: modelData.toLowerCase() === duckyColorPickerWindow.currentHex.toLowerCase()
                                width: 30
                                height: 30
                                radius: 5
                                color: modelData
                                border.width: isCurrentMatch ? 2 : (usedSwatchMouse.containsMouse ? 2 : 1)
                                border.color: isCurrentMatch
                                    ? Kirigami.Theme.highlightColor
                                    : (usedSwatchMouse.containsMouse ? "#ffffff" : Qt.alpha("#000000", 0.45))

                                Rectangle {
                                    visible: usedSwatch.isCurrentMatch
                                    anchors.fill: parent
                                    anchors.margins: 2
                                    radius: 3
                                    color: "transparent"
                                    border.color: "#ffffff"
                                    border.width: 1
                                }

                                MouseArea {
                                    id: usedSwatchMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: (mouse) => {
                                        if (mouse.button === Qt.RightButton) {
                                            deviceController.savePaletteColor(usedSwatch.modelData)
                                            duckyColorPickerWindow.refreshPaletteLists()
                                        } else {
                                            duckyColorPickerWindow.setColorFromHex(usedSwatch.modelData)
                                        }
                                    }
                                    onDoubleClicked: (mouse) => {
                                        if (mouse.button === Qt.LeftButton) {
                                            duckyColorPickerWindow.setColorFromHex(usedSwatch.modelData)
                                            duckyColorPickerWindow.applySelectedColor()
                                        }
                                    }
                                }

                                QQC2.ToolTip.visible: usedSwatchMouse.containsMouse
                                QQC2.ToolTip.text: qsTr("%1 • Left-click to select • Double-click to apply • Right-click to save to Saved Colors").arg(usedSwatch.modelData)
                            }
                        }
                    }
                }
            }
        }

        // Footer Action Row
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                visible: duckyColorPickerWindow.pickerMode === "key"
                icon.name: "edit-clear"
                text: qsTr("Reset to Profile Default")
                onClicked: {
                    hardwareCanvas.clearKeyRgbAt(duckyColorPickerWindow.targetSlotNumber)
                    if (root.selectedSlotNumber === duckyColorPickerWindow.targetSlotNumber) {
                        root.selectedSlotColor = hardwareCanvas.slotColorFor(duckyColorPickerWindow.targetSlotNumber)
                    }
                    duckyColorPickerWindow.close()
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Remove custom SWCOLOR from Key %1 and use profile BG_COLOR").arg(duckyColorPickerWindow.targetSlotNumber)
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "dialog-cancel"
                text: qsTr("Cancel")
                onClicked: duckyColorPickerWindow.close()
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "dialog-ok-apply"
                text: qsTr("Apply Color")
                highlighted: true
                onClicked: duckyColorPickerWindow.applySelectedColor()
            }
        }
    }
}
