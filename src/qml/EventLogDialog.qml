import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Window {
    id: eventLogDialog
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property bool isLightTheme: root ? root.isLightTheme : false
    property color highContrastHighlightText: root ? root.highContrastHighlightText : "#38bdf8"
    property var duckyColorPickerWindow: null
    property var webApiKeyConfigWindow: null
    property var appLaunchConfigWindow: null
    property var sdSyncDialog: null
    property var settingsDialog: null
    property var pgvDrawer: null
    property var autoSwitchDrawer: null
    property var aiModal: null
    property var oledDesktopWidget: null
    property var hardwareCanvas: null
    property var profilesBoxScrollBar: null
    property var scriptScrollView: null
    property var scriptEditorVerticalScrollBar: null
    property var scriptEditor: null
    property var label1Input: null
    property var label2Input: null
    readonly property var lightingAnimatorDialog: duckyColorPickerWindow ? duckyColorPickerWindow.lightingAnimatorDialog : null
    readonly property var webApiDestinationsDialog: webApiKeyConfigWindow ? webApiKeyConfigWindow.webApiDestinationsDialog : null
    readonly property var installedAppPickerDialog: appLaunchConfigWindow ? appLaunchConfigWindow.installedAppPickerDialog : null
    readonly property var macroRecorderDialog: appLaunchConfigWindow ? appLaunchConfigWindow.macroRecorderDialog : null
    readonly property var oledIconPickerDialog: appLaunchConfigWindow ? appLaunchConfigWindow.oledIconPickerDialog : null
    readonly property var unsyncedChangesDialog: sdSyncDialog ? sdSyncDialog.unsyncedChangesDialog : null
    readonly property var firmwareUpdateDialog: sdSyncDialog ? sdSyncDialog.firmwareUpdateDialog : null
    readonly property var newProfileGroupDialog: settingsDialog ? settingsDialog.newProfileGroupDialog : null
    readonly property var profileTemplatesDialog: settingsDialog ? settingsDialog.profileTemplatesDialog : null
    readonly property var renameProfileDialog: settingsDialog ? settingsDialog.renameProfileDialog : null
    readonly property var aboutDialog: settingsDialog ? settingsDialog.aboutDialog : null
    readonly property var profilePropertiesWindow: settingsDialog ? settingsDialog.profilePropertiesWindow : null
    readonly property var keyPropertiesWindow: settingsDialog ? settingsDialog.keyPropertiesWindow : null
    property alias showcaseRunnerTimer: showcaseRunnerTimer
    property alias uiTestSuiteRunner: uiTestSuiteRunner

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

    ListModel {
        id: eventLogModel
    }
    property string eventLogSearchText: ""
    property string eventLogCategoryFilter: "ALL"
    property string eventLogLevelFilter: "ALL"
    readonly property var eventLogCategories: [
        "ALL",
        "DEVICE",
        "PROFILE",
        "AUTOSWITCH",
        "SD_SYNC",
        "SCRIPT",
        "WEB_API",
        "APP_LAUNCH",
        "SYSTEM"
    ]
    readonly property var eventLogLevels: ["ALL", "INFO", "WARN", "ERROR"]

    function levelColorForLog(lvl) {
        if (lvl === "ERROR") return root.isLightTheme ? "#b91c1c" : Kirigami.Theme.negativeTextColor
        if (lvl === "WARN") return root.isLightTheme ? "#b45309" : Kirigami.Theme.neutralTextColor
        return root.highContrastHighlightText
    }

    function categoryColorForLog(cat) {
        if (root.isLightTheme) {
            if (cat === "DEVICE") return "#0369a1"
            if (cat === "PROFILE") return "#6d28d9"
            if (cat === "AUTOSWITCH") return "#15803d"
            if (cat === "SD_SYNC") return "#b45309"
            if (cat === "SCRIPT") return "#0f766e"
            if (cat === "WEB_API") return "#be185d"
            if (cat === "APP_LAUNCH") return "#c2410c"
            return "#334155"
        }
        if (cat === "DEVICE") return "#38bdf8"
        if (cat === "PROFILE") return "#a78bfa"
        if (cat === "AUTOSWITCH") return "#4ade80"
        if (cat === "SD_SYNC") return "#f59e0b"
        if (cat === "SCRIPT") return "#2dd4bf"
        if (cat === "WEB_API") return "#f472b6"
        if (cat === "APP_LAUNCH") return "#fb923c"
        return "#94a3b8"
    }

    function refreshEventLogModel() {
        if (!deviceController) return
        eventLogModel.clear()
        const raw = deviceController.eventLogJson || "[]"
        const catFilter = (eventLogDialog.eventLogCategoryFilter || "ALL").toUpperCase()
        const lvlFilter = (eventLogDialog.eventLogLevelFilter || "ALL").toUpperCase()
        const query = (eventLogDialog.eventLogSearchText || "").trim().toLowerCase()
        try {
            const arr = JSON.parse(raw)
            if (!Array.isArray(arr)) return
            // Show newest events at the bottom or top; newest first or chronological? Chronological with auto-scroll to bottom (or newest at bottom)
            for (let i = 0; i < arr.length; ++i) {
                const it = arr[i]
                const lvl = (it.level || "INFO").toUpperCase()
                const cat = (it.category || "SYSTEM").toUpperCase()
                const msg = it.message || ""
                const ts = it.timestamp || ""
                if (catFilter !== "ALL" && cat !== catFilter) continue
                if (lvlFilter !== "ALL" && lvl !== lvlFilter) continue
                if (query.length > 0) {
                    const hay = (ts + " " + lvl + " " + cat + " " + msg).toLowerCase()
                    if (hay.indexOf(query) === -1) continue
                }
                eventLogModel.append({
                    timestamp: ts,
                    level: lvl,
                    category: cat,
                    message: msg
                })
            }
            if (eventLogListView.count > 0) {
                eventLogListView.positionViewAtEnd()
            }
        } catch (e) {}
    }


    FileDialog {
        id: exportEventLogFileDialog
        title: qsTr("Export System Event Log (.log)")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "log"
        nameFilters: [
            qsTr("Log Files (*.log *.txt)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            deviceController.exportEventLog(selectedFile.toString())
        }
    }


    Timer {
        id: showcaseRunnerTimer
        interval: 350
        repeat: true
        running: deviceController.isDemoShowcaseMode()
        onTriggered: {
            const baseCaptureDir = deviceController.demoCaptureDir()
            const screenshotsDir = baseCaptureDir + "/screenshots/"
            const framesDir = baseCaptureDir + "/video_frames/"
            const step = root.showcaseStep
            root.showcaseStep = step + 1

            if (step === 0) {
                deviceController.selectProfile(1)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 1) {
                deviceController.captureShowcaseFrame(screenshotsDir + "01-dolphin-overview.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_01.png",
                    "DuckyDeck Studio — KDE Plasma 6 Configurator for duckyPad Pro & 2020",
                    "Profile #1: Dolphin File Manager • 20 RGB Mechanical Keys, Dual Rotary Encoders & Live DSVM v2 Compiler"
                )
                deviceController.selectProfile(2)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 2) {
                deviceController.captureShowcaseFrame(screenshotsDir + "02-google-chrome.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_02.png",
                    "Profile #2: Google Chrome — Browser Navigation, Tab Management & DevTools",
                    "Instant KWin Wayland Auto-Switching when Google Chrome receives window focus"
                )
                root.selectShowcaseSlot(5)
            } else if (step === 3) {
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_03.png",
                    "Interactive 26-Slot Hardware Canvas & Real-Time DuckyScript 3 Editor",
                    "Click any mechanical key or rotary encoder action to inspect and compile DuckyScript 3 macros"
                )
                deviceController.selectProfile(3)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 4) {
                deviceController.captureShowcaseFrame(screenshotsDir + "03-vscode-ide.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_04.png",
                    "Profile #3: Visual Studio Code — Build, Debug, Git & Editor Macros",
                    "Syntax highlighting, line numbers, Ctrl+Space autocomplete & live .dsb bytecode verification"
                )
                duckyColorPickerWindow.openForKey(4, "#f97316")
            } else if (step === 5) {
                deviceController.captureShowcaseFrame(screenshotsDir + "04-color-picker-palette.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_05.png",
                    "Custom Key & Profile Background Color Picker",
                    "Interactive HSV color wheel, RGB/Hex sliders, Saved Colors palette & Used Colors across all profiles"
                )
                duckyColorPickerWindow.close()
                deviceController.selectProfile(4)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(17)
                root.showWebApiBuilder = true
            } else if (step === 6) {
                deviceController.captureShowcaseFrame(screenshotsDir + "05-obs-studio-web-api.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_06.png",
                    "Profile #4: OBS Studio — Scene Switching & Host-Side Web API Triggers",
                    "Trigger HTTP/REST webhooks (Home Assistant, OBS WebSocket, local APIs) directly from duckyPad keys"
                )
                root.showWebApiBuilder = false
                deviceController.selectProfile(5)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(7)
            } else if (step === 7) {
                deviceController.captureShowcaseFrame(screenshotsDir + "06-konsole-terminal.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_07.png",
                    "Profile #5: KDE Konsole — One-Key Terminal Commands, Git & Cargo Workflows",
                    "Automate frequent terminal commands, split panes, and system monitoring with zero typing lag"
                )
                deviceController.selectProfile(6)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(19)
            } else if (step === 8) {
                deviceController.captureShowcaseFrame(screenshotsDir + "07-blender-3d.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_08.png",
                    "Profile #6: Blender 3D — Viewport Navigation, Sculpting Brushes & Loops",
                    "Full DuckyScript 3 loop & conditional support with Dual Rotary Encoder zoom and brush sizing"
                )
                deviceController.selectProfile(7)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 9) {
                deviceController.captureShowcaseFrame(screenshotsDir + "08-spotify-media.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_09.png",
                    "Profile #7: Spotify & Media Controls — Hardware Playback & Volume Encoders",
                    "Dedicated media transport keys and rotary encoder volume/scrubbing controls"
                )
                deviceController.selectProfile(8)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(2)
                root.showAppLaunchBuilder = true
            } else if (step === 10) {
                deviceController.captureShowcaseFrame(screenshotsDir + "09-app-launcher.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_10.png",
                    "Profile #8: Desktop Application Launcher — Start Any App from a Key Press",
                    "Browse installed XDG desktop applications or custom scripts and launch them instantaneously"
                )
                root.showAppLaunchBuilder = false
                deviceController.selectProfile(1)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
                autoSwitchDrawer.open()
            } else if (step === 11) {
                deviceController.captureShowcaseFrame(screenshotsDir + "10-wayland-auto-switcher.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_11.png",
                    "KWin Wayland D-Bus Auto-Switcher — Automatic Profile Switching",
                    "Bind window classes and titles (Dolphin, Chrome, VS Code, OBS, Konsole, Blender, Spotify) to profiles"
                )
                autoSwitchDrawer.close()
                eventLogDialog.openLogWindow()
            } else if (step === 12) {
                deviceController.captureShowcaseFrame(screenshotsDir + "11-system-event-log.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_12.png",
                    "Live System Event Log — Real-Time Diagnostics & Audit Trail",
                    "Filter USB HID, SD Sync, KWin Auto-Switcher, Web API, and Application Launch events by category"
                )
                eventLogDialog.close()
                aboutDialog.openAboutWindow()
            } else if (step === 13) {
                deviceController.captureShowcaseFrame(screenshotsDir + "12-about-duckydeck.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_13.png",
                    "Open Source for the duckyPad Community — Special Thanks to dekuNukem",
                    "Explore official duckyPad hardware at duckypad.com and dekuNukem's Tindie store"
                )
                aboutDialog.close()
                showcaseRunnerTimer.stop()
                deviceController.quitApplication()
            }
        }
    }


    Item {
        id: uiTestSuiteRunner
        visible: false
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false

        property int stepIndex: 0
        property int passIndex: 0
        property string currentTheme: "dark"
        property int currentZoom: 100
        property var auditPassConfigs: [
            { theme: "dark", zoom: 100 },
            { theme: "dark", zoom: 105 },
            { theme: "dark", zoom: 110 },
            { theme: "dark", zoom: 115 },
            { theme: "light", zoom: 100 },
            { theme: "light", zoom: 105 },
            { theme: "light", zoom: 110 },
            { theme: "light", zoom: 115 }
        ]
        property var behavioralResults: []
        property var overflowViolations: []
        property var contrastViolations: []
        property var scrollBarStyleViolations: []
        property var minFontPointSizeViolations: []
        property var auditedWindows: []
        property int totalElementsInspected: 0
        property int totalTextNodesInspected: 0
        property int totalControlsInspected: 0
        property int totalInputFieldsInspected: 0
        property int totalScrollBarsInspected: 0
        property int totalKeycapsAudited: 0
        property int keycapLayoutViolationsCount: 0

        function findChildByObjectName(rootNode, targetName) {
            if (!rootNode) return null
            var stack = [rootNode]
            var guard = 0
            while (stack.length > 0 && guard < 256) {
                var n = stack.pop()
                guard++
                if (!n) continue
                if (n.objectName === targetName) return n
                if (n.children) {
                    for (var i = n.children.length - 1; i >= 0; i--) {
                        stack.push(n.children[i])
                    }
                }
            }
            return null
        }

        function auditKeycapOptionCLayout(windowName, keycapNode) {
            if (!keycapNode) return
            uiTestSuiteRunner.totalKeycapsAudited++
            var slotLbl = findChildByObjectName(keycapNode, "keySlotNumberLabel")
            var ledBar = findChildByObjectName(keycapNode, "keyLedBar")
            var innerWell = findChildByObjectName(keycapNode, "keyInnerWell")
            var labelCol = findChildByObjectName(keycapNode, "keyLabelColumn")
            var footerDiv = findChildByObjectName(keycapNode, "keyFooterDivider")
            var footerRow = findChildByObjectName(keycapNode, "keyFooterRow")

            var reasons = []
            if (!slotLbl || !ledBar || !innerWell || !labelCol || !footerDiv || !footerRow) {
                reasons.push("missing_option_c_child_nodes")
            } else {
                if (slotLbl.y + slotLbl.height > innerWell.y + 1) {
                    reasons.push("slot_number_intrudes_into_inner_well(slotBottom=" + (slotLbl.y + slotLbl.height) + ",wellTop=" + innerWell.y + ")")
                }
                if (ledBar.y + ledBar.height > innerWell.y + 1) {
                    reasons.push("led_bar_intrudes_into_inner_well(ledBottom=" + (ledBar.y + ledBar.height) + ",wellTop=" + innerWell.y + ")")
                }
                if (slotLbl.x + slotLbl.implicitWidth > ledBar.x + 1) {
                    reasons.push("slot_number_overlaps_led_bar(slotRight=" + (slotLbl.x + slotLbl.implicitWidth) + ",ledX=" + ledBar.x + ")")
                }
                if (labelCol.x < -0.5 || (labelCol.x + labelCol.width) > innerWell.width + 0.5) {
                    reasons.push("label_column_horizontal_overflow(x=" + labelCol.x + ",w=" + labelCol.width + ",wellW=" + innerWell.width + ")")
                }
                if (labelCol.y < -0.5 || (labelCol.y + labelCol.height) > innerWell.height + 0.5) {
                    reasons.push("label_column_vertical_overflow(y=" + labelCol.y + ",h=" + labelCol.height + ",wellH=" + innerWell.height + ")")
                }
                if (footerRow.visible) {
                    if (labelCol.y + labelCol.height > footerDiv.y + 1.5) {
                        reasons.push("label_column_overlaps_footer_divider(labelBottom=" + Math.round(labelCol.y + labelCol.height) + ",divY=" + Math.round(footerDiv.y) + ")")
                    }
                    if (footerRow.width > innerWell.width - 2) {
                        reasons.push("footer_row_horizontal_overflow(rowW=" + Math.round(footerRow.width) + ",wellW=" + Math.round(innerWell.width) + ")")
                    }
                    if (footerRow.y < footerDiv.y - 0.5 || (footerRow.y + footerRow.height) > innerWell.height + 0.5) {
                        reasons.push("footer_row_vertical_overflow(rowY=" + Math.round(footerRow.y) + ",rowH=" + Math.round(footerRow.height) + ",wellH=" + Math.round(innerWell.height) + ")")
                    }
                }
            }

            if (reasons.length > 0) {
                uiTestSuiteRunner.keycapLayoutViolationsCount++
                uiTestSuiteRunner.overflowViolations.push({
                    theme: uiTestSuiteRunner.currentTheme + "@" + uiTestSuiteRunner.currentZoom + "%",
                    window: windowName,
                    element: String(keycapNode.objectName),
                    kind: "keycap_option_c_layout_violation",
                    reasons: reasons
                })
            }
        }

        function srgbChannelToLinear(c) {
            return (c <= 0.03928) ? (c / 12.92) : Math.pow((c + 0.055) / 1.055, 2.4)
        }

        function relativeLuminance(r, g, b) {
            return 0.2126 * srgbChannelToLinear(r)
                 + 0.7152 * srgbChannelToLinear(g)
                 + 0.0722 * srgbChannelToLinear(b)
        }

        function contrastRatio(c1, c2) {
            var l1 = relativeLuminance(c1.r, c1.g, c1.b)
            var l2 = relativeLuminance(c2.r, c2.g, c2.b)
            var lighter = Math.max(l1, l2)
            var darker = Math.min(l1, l2)
            return (lighter + 0.05) / (darker + 0.05)
        }

        function blendColors(fg, bg, extraOpacity) {
            var alpha = Math.max(0, Math.min(1, fg.a * (extraOpacity !== undefined ? extraOpacity : 1.0)))
            return {
                r: fg.r * alpha + bg.r * (1.0 - alpha),
                g: fg.g * alpha + bg.g * (1.0 - alpha),
                b: fg.b * alpha + bg.b * (1.0 - alpha),
                a: 1.0
            }
        }

        function isRectangleNode(node) {
            if (!node) return false
            var s = node.toString()
            return s.indexOf("QQuickRectangle") !== -1 || s.indexOf("ShadowedRectangle") !== -1
        }

        function resolveEffectiveBackground(item, fallbackWindowColor) {
            var layers = []
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && guard < 64) {
                guard++
                // 1. Check if cur itself is a Rectangle with a visible fill
                if (isRectangleNode(cur) && cur.color !== undefined && cur.color.a !== undefined && cur.color.a > 0.01) {
                    var rectAlpha = cur.color.a * (cur.opacity !== undefined ? cur.opacity : 1.0)
                    layers.push({
                        r: cur.color.r,
                        g: cur.color.g,
                        b: cur.color.b,
                        a: rectAlpha
                    })
                    if (rectAlpha >= 0.98) break
                }
                // 2. Check if cur is a Control with a background item
                if (cur.background && cur.background.visible !== false && (cur.background.opacity === undefined || cur.background.opacity > 0.05)) {
                    if (isRectangleNode(cur.background) && cur.background.color !== undefined && cur.background.color.a !== undefined && cur.background.color.a > 0.01) {
                        var bgAlpha = cur.background.color.a * (cur.background.opacity !== undefined ? cur.background.opacity : 1.0)
                        layers.push({
                            r: cur.background.color.r,
                            g: cur.background.color.g,
                            b: cur.background.color.b,
                            a: bgAlpha
                        })
                        if (bgAlpha >= 0.98) break
                    } else {
                        // Native Breeze QQC2 style background (Button, ComboBox, TextField, SpinBox, etc.)
                        var curType = cur.toString()
                        var isFlatUnpressed = (cur.flat === true && !cur.down && !cur.checked && !cur.highlighted)
                        if (!isFlatUnpressed && (curType.indexOf("Button") !== -1
                                || curType.indexOf("TextField") !== -1
                                || curType.indexOf("TextArea") !== -1
                                || curType.indexOf("ComboBox") !== -1
                                || curType.indexOf("SpinBox") !== -1)) {
                            var themeBg = (cur.Kirigami && cur.Kirigami.Theme && cur.Kirigami.Theme.backgroundColor && cur.Kirigami.Theme.backgroundColor.a > 0.01)
                                ? cur.Kirigami.Theme.backgroundColor
                                : root.color
                            if (themeBg && themeBg.a > 0.01) {
                                layers.push({
                                    r: themeBg.r,
                                    g: themeBg.g,
                                    b: themeBg.b,
                                    a: themeBg.a
                                })
                                if (themeBg.a >= 0.98) break
                            }
                        }
                    }
                }
                cur = cur.parent
            }

            var hasValidFallback = (fallbackWindowColor
                && fallbackWindowColor.a !== undefined
                && fallbackWindowColor.a > 0.01
                && !(fallbackWindowColor.r === 0 && fallbackWindowColor.g === 0 && fallbackWindowColor.b === 0 && uiTestSuiteRunner.currentTheme === "light"))

            var base = hasValidFallback
                ? {
                    r: fallbackWindowColor.r,
                    g: fallbackWindowColor.g,
                    b: fallbackWindowColor.b,
                    a: 1.0
                }
                : (uiTestSuiteRunner.currentTheme === "light"
                    ? { r: 0.937, g: 0.941, b: 0.945, a: 1.0 }
                    : { r: 0.125, g: 0.137, b: 0.149, a: 1.0 })

            for (var i = layers.length - 1; i >= 0; i--) {
                base = blendColors(layers[i], base, 1.0)
            }
            return base
        }

        function isEffectivelyEnabled(item) {
            var cur = item
            var guard = 0
            while (cur && guard < 64) {
                guard++
                if (cur.enabled === false) return false
                if (cur.visible === false) return false
                if (cur.opacity !== undefined && cur.opacity < 0.45) return false
                cur = cur.parent
            }
            return true
        }

        function isPlaceholderTextNode(item) {
            if (!item || !item.parent) return false
            if (item.parent.placeholderText !== undefined && String(item.text) === String(item.parent.placeholderText)) {
                return true
            }
            return false
        }

        function hasAlphanumericText(s) {
            if (!s) return false
            return /[A-Za-z0-9]/.test(String(s))
        }

        function isInsideScrollableHorizontal(item, rootItem) {
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && cur !== rootItem && guard < 64) {
                guard++
                var tName = cur.toString()
                if (cur.contentWidth !== undefined && cur.contentItem !== undefined) {
                    if (cur.contentWidth > cur.width + 2) return true
                }
                if (tName.indexOf("Flickable") !== -1 || tName.indexOf("ListView") !== -1 || tName.indexOf("GridView") !== -1) {
                    return true
                }
                cur = cur.parent
            }
            return false
        }

        function findEnclosingFlickable(item, rootItem) {
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && cur !== rootItem && guard < 64) {
                guard++
                if (cur.contentWidth !== undefined && cur.contentHeight !== undefined && cur.contentItem !== undefined) {
                    return cur
                }
                cur = cur.parent
            }
            return null
        }

        function describeItem(item) {
            if (!item) return "<null>"
            var rawType = item.toString().split("(")[0]
            var txt = ""
            if (item.text !== undefined && String(item.text).length > 0) {
                txt = String(item.text).replace(/\s+/g, " ").substring(0, 48)
            } else if (item.title !== undefined && String(item.title).length > 0) {
                txt = String(item.title).replace(/\s+/g, " ").substring(0, 48)
            }
            return rawType + (txt.length > 0 ? "[\"" + txt + "\"]" : "")
        }

        function validateScrollBarStyle(windowName, sb) {
            if (!sb) return false
            uiTestSuiteRunner.totalScrollBarsInspected++

            var refBar = profilesBoxScrollBar
            var isVert = (sb.orientation === undefined || sb.orientation === Qt.Vertical)
            var bg = sb.background
            var thumb = sb.contentItem

            var expectedTrackThickness = refBar ? refBar.styleTrackThickness : 11
            var expectedThumbThickness = refBar ? refBar.styleThumbThickness : 7
            var expectedTrackRadius = refBar ? refBar.styleTrackRadius : 5
            var expectedThumbRadius = refBar ? refBar.styleThumbRadius : 4
            var expectedMinSize = refBar ? refBar.minimumSize : 0.14
            var expectedPadding = refBar ? refBar.padding : 1
            var expectedZ = refBar ? refBar.z : 50

            var actualTrackThickness = isVert ? Math.round(sb.implicitWidth) : Math.round(sb.implicitHeight)
            var actualThumbThickness = thumb
                ? (isVert ? Math.round(thumb.implicitWidth) : Math.round(thumb.implicitHeight))
                : -1
            var actualTrackRadius = (bg && bg.radius !== undefined) ? Math.round(bg.radius) : -1
            var actualThumbRadius = (thumb && thumb.radius !== undefined) ? Math.round(thumb.radius) : -1
            var actualTrackBorderWidth = (bg && bg.border) ? Math.round(bg.border.width) : -1
            var actualThumbBorderWidth = (thumb && thumb.border) ? Math.round(thumb.border.width) : -1
            var actualTrackAlpha = (bg && bg.color && bg.color.a !== undefined) ? bg.color.a : -1
            var actualTrackBorderAlpha = (bg && bg.border && bg.border.color && bg.border.color.a !== undefined) ? bg.border.color.a : -1
            var actualThumbBorderAlpha = (thumb && thumb.border && thumb.border.color && thumb.border.color.a !== undefined) ? thumb.border.color.a : -1

            var reasons = []
            if (sb.isProfileBoxStyleScrollBar !== true || sb.objectName !== "highContrastScrollBar") {
                reasons.push("not HighContrastScrollBar (objectName=" + sb.objectName + ")")
            }
            if (Math.abs((sb.minimumSize || 0) - expectedMinSize) > 0.005) {
                reasons.push("minimumSize=" + sb.minimumSize + " expected=" + expectedMinSize)
            }
            if (Math.round(sb.padding || 0) !== expectedPadding) {
                reasons.push("padding=" + sb.padding + " expected=" + expectedPadding)
            }
            if (Math.round(sb.z || 0) !== expectedZ) {
                reasons.push("z=" + sb.z + " expected=" + expectedZ)
            }
            if (actualTrackThickness !== expectedTrackThickness) {
                reasons.push("trackThickness=" + actualTrackThickness + " expected=" + expectedTrackThickness)
            }
            if (actualThumbThickness !== expectedThumbThickness) {
                reasons.push("thumbThickness=" + actualThumbThickness + " expected=" + expectedThumbThickness)
            }
            if (actualTrackRadius !== expectedTrackRadius) {
                reasons.push("trackRadius=" + actualTrackRadius + " expected=" + expectedTrackRadius)
            }
            if (actualThumbRadius !== expectedThumbRadius) {
                reasons.push("thumbRadius=" + actualThumbRadius + " expected=" + expectedThumbRadius)
            }
            if (actualTrackBorderWidth !== 1 || actualThumbBorderWidth !== 1) {
                reasons.push("borderWidth track=" + actualTrackBorderWidth + " thumb=" + actualThumbBorderWidth)
            }
            if (Math.abs(actualTrackAlpha - 0.14) > 0.025
                || Math.abs(actualTrackBorderAlpha - 0.28) > 0.025
                || Math.abs(actualThumbBorderAlpha - 0.55) > 0.025) {
                reasons.push("alpha mismatch track=" + actualTrackAlpha.toFixed(2)
                    + " trackBorder=" + actualTrackBorderAlpha.toFixed(2)
                    + " thumbBorder=" + actualThumbBorderAlpha.toFixed(2))
            }

            if (reasons.length > 0) {
                uiTestSuiteRunner.scrollBarStyleViolations.push({
                    theme: uiTestSuiteRunner.currentTheme + "@" + uiTestSuiteRunner.currentZoom + "%",
                    window: windowName,
                    element: describeItem(sb),
                    parentElement: sb.parent ? describeItem(sb.parent) : "<none>",
                    reasons: reasons
                })
                return false
            }
            return true
        }

        function auditWindowScrollBars(windowName, rootItem) {
            if (!rootItem) return 0
            // OledDesktopWidget is a compact HUD overlay without scrollbars
            if (windowName.indexOf("OledDesktopWidget") === 0) return 0

            var stack = [{ node: rootItem, depth: 0 }]
            var visited = []
            var count = 0
            var guard = 0

            while (stack.length > 0 && guard < 10000) {
                var entry = stack.pop()
                var node = entry.node
                var depth = entry.depth
                if (!node || visited.indexOf(node) !== -1) continue
                visited.push(node)
                guard++

                var tStr = node.toString()
                if (tStr.indexOf("ScrollBar") !== -1 || node.isProfileBoxStyleScrollBar === true) {
                    count++
                    validateScrollBarStyle(windowName, node)
                }

                if (node.children && node.children.length > 0 && depth < 42) {
                    for (var i = node.children.length - 1; i >= 0; i--) {
                        stack.push({ node: node.children[i], depth: depth + 1 })
                    }
                }
                if (node.contentItem && node.contentItem !== node && depth < 42) {
                    stack.push({ node: node.contentItem, depth: depth + 1 })
                }
            }
            return count
        }

        function auditItemTree(windowName, rootItem, winWidth, winHeight, fallbackBgColor) {
            if (!rootItem || winWidth <= 0 || winHeight <= 0) return
            if (windowName.indexOf("MainWindow") !== 0) {
                if (Math.abs(rootItem.width - winWidth) > 1) {
                    rootItem.width = winWidth
                }
                if (Math.abs(rootItem.height - winHeight) > 1) {
                    rootItem.height = winHeight
                }
            }

            var windowScrollBarsCount = auditWindowScrollBars(windowName, rootItem)
            var passThemeLabel = uiTestSuiteRunner.currentTheme + "@" + uiTestSuiteRunner.currentZoom + "%"

            var stack = [{ node: rootItem, depth: 0 }]
            var visitedCount = 0

            while (stack.length > 0 && visitedCount < 8000) {
                var entry = stack.pop()
                var item = entry.node
                var depth = entry.depth
                if (!item) continue
                visitedCount++
                uiTestSuiteRunner.totalElementsInspected++

                if (item.visible === false || (item.opacity !== undefined && item.opacity <= 0.05)) {
                    continue
                }
                if (item.width !== undefined && item.height !== undefined && (item.width <= 0.5 || item.height <= 0.5)) {
                    continue
                }

                if (item.objectName && String(item.objectName).indexOf("keycapItem_") === 0) {
                    auditKeycapOptionCLayout(windowName, item)
                }

                var typeStr = item.toString()
                var isButton = typeStr.indexOf("Button") !== -1 && typeStr.indexOf("ButtonGroup") === -1
                var isControl = isButton
                    || typeStr.indexOf("CheckBox") !== -1
                    || typeStr.indexOf("Switch") !== -1
                    || typeStr.indexOf("ComboBox") !== -1
                    || typeStr.indexOf("TextField") !== -1
                    || typeStr.indexOf("SpinBox") !== -1
                var isTextNode = (typeStr.indexOf("QQuickText") !== -1
                    || typeStr.indexOf("Label") !== -1
                    || (item.text !== undefined && item.font !== undefined && item.elide !== undefined))
                    && typeStr.indexOf("TextField") === -1
                    && typeStr.indexOf("TextArea") === -1
                    && typeStr.indexOf("TextEdit") === -1

                // 1. Check Control / Button Overflow
                if (isControl && item.width > 1 && item.height > 1) {
                    uiTestSuiteRunner.totalControlsInspected++
                    var flick = findEnclosingFlickable(item, rootItem)
                    if (!flick) {
                        var tl = item.mapToItem(rootItem, 0, 0)
                        var br = item.mapToItem(rootItem, item.width, item.height)
                        if (tl && br) {
                            if (br.x > winWidth + 4 || tl.x < -4 || br.y > winHeight + 4 || tl.y < -4) {
                                uiTestSuiteRunner.overflowViolations.push({
                                    theme: passThemeLabel,
                                    window: windowName,
                                    element: describeItem(item),
                                    kind: "control_outside_window_bounds",
                                    bounds: {
                                        left: Math.round(tl.x),
                                        top: Math.round(tl.y),
                                        right: Math.round(br.x),
                                        bottom: Math.round(br.y),
                                        windowWidth: Math.round(winWidth),
                                        windowHeight: Math.round(winHeight)
                                    }
                                })
                            }
                        }
                    } else if (flick.width > 10 && (!flick.contentWidth || flick.contentWidth <= flick.width + 4)) {
                        // Inside a vertical-only Flickable/ScrollView: verify horizontal bounds inside flickable
                        var ftl = item.mapToItem(flick, 0, 0)
                        var fbr = item.mapToItem(flick, item.width, item.height)
                        if (ftl && fbr && (fbr.x > flick.width + 6 || ftl.x < -6)) {
                            uiTestSuiteRunner.overflowViolations.push({
                                theme: passThemeLabel,
                                window: windowName,
                                element: describeItem(item),
                                kind: "control_overflows_scrollview_width",
                                bounds: {
                                    left: Math.round(ftl.x),
                                    right: Math.round(fbr.x),
                                    containerWidth: Math.round(flick.width)
                                }
                            })
                        }
                    }
                }

                // 1B. Check Input Field (TextField) Internal Clipping & Content Capacity across Zoom Levels
                var isTextField = (typeStr.indexOf("TextField") !== -1 || typeStr.indexOf("TextArea") !== -1)
                if (isTextField && item.width > 1 && item.height > 1) {
                    uiTestSuiteRunner.totalInputFieldsInspected = (uiTestSuiteRunner.totalInputFieldsInspected || 0) + 1
                    var topPad = item.topPadding !== undefined ? item.topPadding : 0
                    var botPad = item.bottomPadding !== undefined ? item.bottomPadding : 0
                    var leftPad = item.leftPadding !== undefined ? item.leftPadding : 0
                    var rightPad = item.rightPadding !== undefined ? item.rightPadding : 0
                    var availH = item.height - topPad - botPad
                    var availW = item.width - leftPad - rightPad
                    var contentH = item.contentHeight || (item.font ? (item.font.pixelSize || Math.round(item.font.pointSize * 1.33)) : 14)

                    // Vertical clipping: TextField must have enough height for its text line height plus padding
                    var effectiveVertPad = Math.min(topPad + botPad, 8)
                    if (item.height < contentH + effectiveVertPad - 1.5) {
                        uiTestSuiteRunner.overflowViolations.push({
                            theme: passThemeLabel,
                            window: windowName,
                            element: describeItem(item),
                            kind: "input_field_vertical_clipping",
                            bounds: {
                                height: Math.round(item.height),
                                contentHeight: Math.round(contentH),
                                padding: Math.round(effectiveVertPad),
                                minRequired: Math.round(contentH + effectiveVertPad)
                            }
                        })
                    }

                    // Horizontal clipping on 5-character OLED label fields
                    var isOledLabelField = (item.id === "label1Input" || item.id === "label2Input"
                        || item.id === "keyPropLabel1Field" || item.id === "keyPropLabel2Field")
                    if (isOledLabelField) {
                        var charWidth = item.font ? (item.font.pixelSize || Math.round(item.font.pointSize * 1.33)) * 0.65 : 8
                        var required5ChWidth = Math.ceil(5 * charWidth)
                        if (availW < required5ChWidth) {
                            uiTestSuiteRunner.overflowViolations.push({
                                theme: passThemeLabel,
                                window: windowName,
                                element: describeItem(item),
                                kind: "oled_label_input_field_horizontal_clipping",
                                bounds: {
                                    width: Math.round(item.width),
                                    availableWidth: Math.round(availW),
                                    requiredFor5Chars: Math.round(required5ChWidth),
                                    zoom: uiTestSuiteRunner.currentZoom
                                }
                            })
                        }
                    }
                }

                // 2. Check Text / Label Overflow, Minimum 10pt UI Font (or >=6px on PCB/OLED canvas), & WCAG Color Contrast
                if (isTextNode && hasAlphanumericText(item.text)) {
                    uiTestSuiteRunner.totalTextNodesInspected++

                    var isFixedGeometryCanvas = (windowName.indexOf("OledDesktopWidget") === 0)
                    if (!isFixedGeometryCanvas) {
                        var pWalk = item.parent
                        var pGuard = 0
                        while (pWalk && pGuard < 32) {
                            if (pWalk.objectName === "pcbChassis") {
                                isFixedGeometryCanvas = true
                                break
                            }
                            pWalk = pWalk.parent
                            pGuard++
                        }
                    }

                    // Check minimum 10pt font size (scaled by root.fontScale) on standard UI controls/labels
                    if (!isFixedGeometryCanvas && item.font && item.font.pointSize !== undefined && item.font.pointSize > 0) {
                        var minExpectedPt = 9.95 * root.fontScale
                        if (item.font.pointSize < minExpectedPt) {
                            uiTestSuiteRunner.minFontPointSizeViolations.push({
                                theme: passThemeLabel,
                                window: windowName,
                                element: describeItem(item),
                                pointSize: Number(item.font.pointSize.toFixed(2)),
                                minimumRequiredPointSize: Number((10 * root.fontScale).toFixed(2))
                            })
                        }
                    }

                    // Check if un-elided single-line text overflows its parent container or window
                    var textWrap = item.wrapMode !== undefined ? item.wrapMode : Text.NoWrap
                    var textElide = item.elide !== undefined ? item.elide : Text.ElideNone
                    if (textWrap === Text.NoWrap && textElide === Text.ElideNone && item.implicitWidth > item.width + 8) {
                        var parentClip = item.parent && item.parent.clip === true
                        var ttl = item.mapToItem(rootItem, 0, 0)
                        var tbr = item.mapToItem(rootItem, item.implicitWidth, item.height)
                        if (parentClip || (tbr && tbr.x > winWidth + 6 && !isInsideScrollableHorizontal(item, rootItem))) {
                            uiTestSuiteRunner.overflowViolations.push({
                                theme: passThemeLabel,
                                window: windowName,
                                element: describeItem(item),
                                kind: "text_overflow_unelided",
                                details: {
                                    implicitWidth: Math.round(item.implicitWidth),
                                    allocatedWidth: Math.round(item.width),
                                    mappedRight: tbr ? Math.round(tbr.x) : -1,
                                    windowWidth: Math.round(winWidth)
                                }
                            })
                        }
                    }

                    // Check WCAG Color Contrast for enabled text nodes (excluding empty-input placeholder hints)
                    if (isEffectivelyEnabled(item) && !isPlaceholderTextNode(item) && item.color !== undefined) {
                        var bg = resolveEffectiveBackground(item, fallbackBgColor)
                        var fg = blendColors(item.color, bg, item.opacity !== undefined ? item.opacity : 1.0)
                        var ratio = contrastRatio(fg, bg)
                        var fontPt = (item.font && item.font.pointSize > 0) ? item.font.pointSize : (10 * root.fontScale)
                        var isBold = (item.font && item.font.bold) ? true : false
                        // Minimum contrast threshold: 3.0:1 across all UI controls, badges, headers, captions, and labels
                        var minRatio = 3.0
                        if (ratio < minRatio) {
                            uiTestSuiteRunner.contrastViolations.push({
                                theme: passThemeLabel,
                                window: windowName,
                                element: describeItem(item),
                                contrastRatio: Number(ratio.toFixed(2)),
                                requiredRatio: minRatio,
                                fgColor: Qt.rgba(fg.r, fg.g, fg.b, 1.0).toString(),
                                bgColor: Qt.rgba(bg.r, bg.g, bg.b, 1.0).toString(),
                                fontSize: fontPt,
                                bold: isBold
                            })
                        }
                    }
                }

                // Traverse visible children
                if (item.children && item.children.length > 0 && depth < 42) {
                    for (var c = item.children.length - 1; c >= 0; c--) {
                        stack.push({ node: item.children[c], depth: depth + 1 })
                    }
                }
                // Also traverse contentItem if present and not already in children
                if (item.contentItem && item.contentItem !== item && depth < 42) {
                    var alreadyChild = false
                    if (item.children) {
                        for (var k = 0; k < item.children.length; k++) {
                            if (item.children[k] === item.contentItem) {
                                alreadyChild = true
                                break
                            }
                        }
                    }
                    if (!alreadyChild) {
                        stack.push({ node: item.contentItem, depth: depth + 1 })
                    }
                }
            }

            uiTestSuiteRunner.auditedWindows.push({
                theme: passThemeLabel,
                window: windowName,
                width: Math.round(winWidth),
                height: Math.round(winHeight),
                elementsVisited: visitedCount,
                scrollBarsAudited: windowScrollBarsCount
            })
        }

        function recordBehavior(name, passed, details) {
            uiTestSuiteRunner.behavioralResults.push({
                name: name,
                passed: Boolean(passed),
                details: details || ""
            })
        }

        function ensureTestFixtures() {
            deviceController.resetAllTextScales()
            firmwareUpdateDialog.refreshCatalog()
        }

        function runBehavioralSuite() {
            ensureTestFixtures()

            // 1. Desktop Widget Profile Rotation & Cell Text Rotation (0°, 90°, 180°, 270°)
            deviceController.setActiveProfileWidgetRotation(0)
            oledDesktopWidget.syncFromController()
            var expectedTextSlot1 = oledDesktopWidget.sliceOledLabel(oledDesktopWidget.slotDataFor(1).l1, 5)
            var expectedTextSlot4 = oledDesktopWidget.sliceOledLabel(oledDesktopWidget.slotDataFor(4).l1, 4)
            var expectedTextSlot20 = oledDesktopWidget.sliceOledLabel(oledDesktopWidget.slotDataFor(20).l1, 5)
            var expectedTextSlot17 = oledDesktopWidget.sliceOledLabel(oledDesktopWidget.slotDataFor(17).l1, 4)

            var rot0Ok = (oledDesktopWidget.effectiveRotation === 0
                && oledDesktopWidget.gridCols === 4
                && oledDesktopWidget.gridRows === 5
                && oledDesktopWidget.slotForCell(0, 0) === 1
                && oledDesktopWidget.slotForCell(3, 0) === 4
                && oledDesktopWidget.slotForCell(0, 4) === 17
                && oledDesktopWidget.slotForCell(3, 4) === 20
                && oledDesktopWidget.cellSlotForIndex(0) === 1
                && oledDesktopWidget.cellDisp1ForIndex(0) === expectedTextSlot1
                && oledDesktopWidget.cellTextRotationForIndex(0) === 0)
            recordBehavior("desktop_widget_rotation_0_deg", rot0Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows + " cell0=" + oledDesktopWidget.cellDisp1ForIndex(0) + " textRot=" + oledDesktopWidget.cellTextRotationForIndex(0))

            deviceController.setActiveProfileWidgetRotation(90)
            oledDesktopWidget.syncFromController()
            var rot90Ok = (oledDesktopWidget.effectiveRotation === 90
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4
                && oledDesktopWidget.slotForCell(0, 0) === 4
                && oledDesktopWidget.slotForCell(4, 0) === 20
                && oledDesktopWidget.slotForCell(0, 3) === 1
                && oledDesktopWidget.slotForCell(4, 3) === 17
                && oledDesktopWidget.cellSlotForIndex(0) === 4
                && oledDesktopWidget.cellDisp1ForIndex(0) === expectedTextSlot4
                && oledDesktopWidget.cellTextRotationForIndex(0) === 90)
            recordBehavior("desktop_widget_rotation_90_deg", rot90Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows + " cell0=" + oledDesktopWidget.cellDisp1ForIndex(0) + " textRot=" + oledDesktopWidget.cellTextRotationForIndex(0))

            deviceController.setActiveProfileWidgetRotation(180)
            oledDesktopWidget.syncFromController()
            var rot180Ok = (oledDesktopWidget.effectiveRotation === 180
                && oledDesktopWidget.gridCols === 4
                && oledDesktopWidget.gridRows === 5
                && oledDesktopWidget.slotForCell(0, 0) === 20
                && oledDesktopWidget.slotForCell(3, 0) === 17
                && oledDesktopWidget.slotForCell(0, 4) === 4
                && oledDesktopWidget.slotForCell(3, 4) === 1
                && oledDesktopWidget.cellSlotForIndex(0) === 20
                && oledDesktopWidget.cellDisp1ForIndex(0) === expectedTextSlot20
                && oledDesktopWidget.cellTextRotationForIndex(0) === 180)
            recordBehavior("desktop_widget_rotation_180_deg", rot180Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows + " cell0=" + oledDesktopWidget.cellDisp1ForIndex(0) + " textRot=" + oledDesktopWidget.cellTextRotationForIndex(0))

            deviceController.setActiveProfileWidgetRotation(270)
            oledDesktopWidget.syncFromController()
            var rot270Ok = (oledDesktopWidget.effectiveRotation === 270
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4
                && oledDesktopWidget.slotForCell(0, 0) === 17
                && oledDesktopWidget.slotForCell(4, 0) === 1
                && oledDesktopWidget.slotForCell(0, 3) === 20
                && oledDesktopWidget.slotForCell(4, 3) === 4
                && oledDesktopWidget.cellSlotForIndex(0) === 17
                && oledDesktopWidget.cellDisp1ForIndex(0) === expectedTextSlot17
                && oledDesktopWidget.cellTextRotationForIndex(0) === 270)
            recordBehavior("desktop_widget_rotation_270_deg", rot270Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows + " cell0=" + oledDesktopWidget.cellDisp1ForIndex(0) + " textRot=" + oledDesktopWidget.cellTextRotationForIndex(0))

            // 2. Profile Properties Window hardware rotation + Follow Hardware (-1) propagation
            var activeProfNum = deviceController.activeProfile || 1
            profilePropertiesWindow.openForProfile(activeProfNum)
            profilePropertiesWindow.profOledRotation = 90
            profilePropertiesWindow.profWidgetRotation = -1
            profilePropertiesWindow.saveAllProfileProperties()
            var savedProps = {}
            try {
                savedProps = JSON.parse(deviceController.getProfilePropertiesJson(activeProfNum))
            } catch (e) {}
            var followHwOk = (savedProps.oledRotation === 90
                && savedProps.widgetRotation === -1
                && savedProps.effectiveWidgetRotation === 90
                && oledDesktopWidget.effectiveRotation === 90
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4
                && oledDesktopWidget.cellSlotForIndex(0) === 4
                && oledDesktopWidget.cellDisp1ForIndex(0) === expectedTextSlot4
                && oledDesktopWidget.cellTextRotationForIndex(0) === 90)
            recordBehavior("profile_properties_follow_hardware_rotation", followHwOk,
                "hwRot=" + savedProps.oledRotation
                + " widgetOverride=" + savedProps.widgetRotation
                + " effective=" + oledDesktopWidget.effectiveRotation
                + " cell0=" + oledDesktopWidget.cellDisp1ForIndex(0)
                + " textRot=" + oledDesktopWidget.cellTextRotationForIndex(0))

            // Restore 0° rotation after rotation tests
            profilePropertiesWindow.profOledRotation = 0
            profilePropertiesWindow.profWidgetRotation = -1
            profilePropertiesWindow.saveAllProfileProperties()
            profilePropertiesWindow.close()

            // 3. Firmware Manager filtering & catalog behavior
            firmwareUpdateDialog.refreshCatalog()
            firmwareUpdateDialog.vendorFilter = "all"
            var allCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "studio"
            var studioCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "official"
            var officialCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "all"
            recordBehavior("firmware_manager_vendor_filtering",
                allCount >= 2 && studioCount >= 1 && officialCount >= 1 && (studioCount + officialCount === allCount),
                "all=" + allCount + " studio=" + studioCount + " official=" + officialCount)

            // 4. Resizable windows scrollbar style matches Profiles Box scrollbar (profilesBoxScrollBar)
            var resizableWindowsWithScrollBars = [
                { name: "MainWindow", root: root.contentItem },
                { name: "FirmwareUpdateDialog", root: firmwareUpdateDialog.contentItem },
                { name: "ProfilePropertiesWindow", root: profilePropertiesWindow.contentItem },
                { name: "KeyPropertiesWindow", root: keyPropertiesWindow.contentItem },
                { name: "DuckyColorPickerWindow", root: duckyColorPickerWindow.contentItem },
                { name: "ProfileTemplatesDialog", root: profileTemplatesDialog.contentItem },
                { name: "SettingsDialog", root: settingsDialog.contentItem },
                { name: "WebApiDestinationsDialog", root: webApiDestinationsDialog.contentItem },
                { name: "EventLogDialog", root: eventLogDialog.contentItem },
                { name: "AboutDialog", root: aboutDialog.contentItem },
                { name: "UnsyncedChangesDialog", root: unsyncedChangesDialog.contentItem },
                { name: "SdSyncDialog", root: sdSyncDialog.contentItem },
                { name: "AppLaunchConfigWindow", root: appLaunchConfigWindow.contentItem },
                { name: "InstalledAppPickerDialog", root: installedAppPickerDialog.contentItem },
                { name: "WebApiKeyConfigWindow", root: webApiKeyConfigWindow.contentItem },
                { name: "LightingAnimatorDialog", root: lightingAnimatorDialog.contentItem },
                { name: "MacroRecorderDialog", root: macroRecorderDialog.contentItem },
                { name: "OledIconPickerDialog", root: oledIconPickerDialog.contentItem },
                { name: "PgvDrawer", root: pgvDrawer.contentItem },
                { name: "AutoSwitchDrawer", root: autoSwitchDrawer.contentItem },
                { name: "AiModal", root: aiModal.contentItem }
            ]
            var missingScrollBarWindows = []
            var totalResizableBarsChecked = 0
            for (var wIdx = 0; wIdx < resizableWindowsWithScrollBars.length; wIdx++) {
                var wEntry = resizableWindowsWithScrollBars[wIdx]
                var c = auditWindowScrollBars(wEntry.name + "_BehavioralCheck", wEntry.root)
                totalResizableBarsChecked += c
                if (c === 0) {
                    missingScrollBarWindows.push(wEntry.name)
                }
            }
            var refBarOk = validateScrollBarStyle("ProfilesBox_Reference", profilesBoxScrollBar)
            recordBehavior(
                "resizable_windows_scrollbar_style_matches_profiles_box",
                refBarOk
                    && missingScrollBarWindows.length === 0
                    && uiTestSuiteRunner.scrollBarStyleViolations.length === 0
                    && totalResizableBarsChecked >= 20,
                "windowsVerified=" + (resizableWindowsWithScrollBars.length - missingScrollBarWindows.length)
                    + "/" + resizableWindowsWithScrollBars.length
                    + " scrollBarsVerified=" + totalResizableBarsChecked
                    + " violations=" + uiTestSuiteRunner.scrollBarStyleViolations.length
            )

            // 5. Script Editor Box Scrollbar (always-on vertical HighContrastScrollBar + multi-line scrolling)
            root.selectedSlotNumber = 1
            root.inspectorMode = "code"
            root.showScriptPreviewInBuilder = true
            root.syncWindowWidthToEditorState()
            var savedScriptText = scriptEditor.text
            root.loadingSlotData = true
            var longLines = []
            for (var ln = 1; ln <= 60; ln++) {
                longLines.push("STRING Line " + ln + " — DuckyScript 3 Scrollbar Test")
            }
            scriptEditor.text = longLines.join("\n")
            root.loadingSlotData = false
            var scriptSbStyleOk = validateScrollBarStyle("ScriptEditor_VerticalScrollBar", scriptEditorVerticalScrollBar)
            var scriptSbVisibleOk = Boolean(
                scriptEditorVerticalScrollBar
                && scriptEditorVerticalScrollBar.visible
                && scriptEditorVerticalScrollBar.policy === QQC2.ScrollBar.AlwaysOn
                && scriptEditorVerticalScrollBar.background
                && scriptEditorVerticalScrollBar.background.visible
                && scriptEditorVerticalScrollBar.contentItem
                && scriptEditorVerticalScrollBar.contentItem.visible
            )
            var scriptScrollableOk = scriptScrollView.contentHeight > scriptScrollView.height && scriptEditorVerticalScrollBar.size < 1.0
            scriptScrollView.contentY = Math.max(20, scriptScrollView.contentHeight - scriptScrollView.height)
            var scrolledPos = scriptEditorVerticalScrollBar.position
            scriptScrollView.contentY = 0
            root.loadingSlotData = true
            scriptEditor.text = savedScriptText
            root.loadingSlotData = false
            recordBehavior(
                "script_editor_has_scrollbar",
                scriptSbStyleOk && scriptSbVisibleOk && scriptScrollableOk && scrolledPos > 0.01,
                "styleOk=" + scriptSbStyleOk
                    + " visibleOk=" + scriptSbVisibleOk
                    + " scrollableOk=" + scriptScrollableOk
                    + " scrolledPos=" + scrolledPos.toFixed(3)
            )

            // 6. Settings Dialog removed per-area text sizes & minimum 10pt base font
            settingsDialog.open()
            var forbiddenAreaStrings = [
                "TEXT SIZES BY APPLICATION AREA",
                "Profiles & Auto-Switcher Sidebar",
                "Hardware Pad & Key Labels",
                "Key Inspector & Action Controls",
                "duckyScript Code Editor",
                "Bottom Status Bar"
            ]
            var foundForbiddenSettingsTexts = []
            var sStack = [settingsDialog.contentItem]
            var sVisited = []
            while (sStack.length > 0 && sVisited.length < 4000) {
                var sNode = sStack.pop()
                if (!sNode || sVisited.indexOf(sNode) !== -1) continue
                sVisited.push(sNode)
                if (sNode.text !== undefined) {
                    var sTxt = String(sNode.text)
                    for (var fIdx = 0; fIdx < forbiddenAreaStrings.length; fIdx++) {
                        if (sTxt.indexOf(forbiddenAreaStrings[fIdx]) !== -1) {
                            foundForbiddenSettingsTexts.push(forbiddenAreaStrings[fIdx])
                        }
                    }
                }
                if (sNode.children) {
                    for (var sc = 0; sc < sNode.children.length; sc++) {
                        sStack.push(sNode.children[sc])
                    }
                }
                if (sNode.contentItem && sNode.contentItem !== sNode) {
                    sStack.push(sNode.contentItem)
                }
            }
            settingsDialog.close()
            var baseEditorPt = scriptEditor.font.pointSize / root.fontScale
            recordBehavior(
                "settings_removed_area_text_sizes_and_min_10pt_font",
                foundForbiddenSettingsTexts.length === 0 && baseEditorPt >= 10.0,
                "forbiddenSettingsFound=" + foundForbiddenSettingsTexts.length
                    + " baseEditorPt=" + baseEditorPt.toFixed(1)
            )

            // 7. Ctrl++, Ctrl+=, Ctrl+- Zoom Adjustment & Disk Persistence on Restart
            deviceController.resetAllTextScales()
            var initialZoom = deviceController.uiTextScale
            // Dispatch Ctrl++
            deviceController.sendKeyToActiveWindow(Qt.Key_Plus, Qt.ControlModifier, "+")
            var afterCtrlPlus = deviceController.uiTextScale
            // Dispatch Ctrl+=
            deviceController.sendKeyToActiveWindow(Qt.Key_Equal, Qt.ControlModifier, "=")
            var afterCtrlEqual = deviceController.uiTextScale
            // Verify disk persistence by mutating in-memory scale and reloading from disk (simulating app restart)
            var reloadedAfterZoomIn = deviceController.reloadZoomLevelFromDisk()
            // Dispatch Ctrl+-
            deviceController.sendKeyToActiveWindow(Qt.Key_Minus, Qt.ControlModifier, "-")
            var afterCtrlMinus = deviceController.uiTextScale
            var reloadedAfterZoomOut = deviceController.reloadZoomLevelFromDisk()
            // Dispatch Ctrl+0 to reset to 100%
            deviceController.sendKeyToActiveWindow(Qt.Key_0, Qt.ControlModifier, "0")
            var afterCtrlZero = deviceController.uiTextScale
            var reloadedAfterReset = deviceController.reloadZoomLevelFromDisk()

            var zoomAndPersistenceOk = (
                initialZoom === 100
                && afterCtrlPlus === 105
                && afterCtrlEqual === 110
                && reloadedAfterZoomIn === 110
                && afterCtrlMinus === 105
                && reloadedAfterZoomOut === 105
                && afterCtrlZero === 100
                && reloadedAfterReset === 100
            )
            recordBehavior(
                "ctrl_plus_equal_minus_zoom_and_restart_persistence",
                zoomAndPersistenceOk,
                "initial=" + initialZoom
                    + " ctrlPlus=" + afterCtrlPlus
                    + " ctrlEqual=" + afterCtrlEqual
                    + " reloaded110=" + reloadedAfterZoomIn
                    + " ctrlMinus=" + afterCtrlMinus
                    + " reloaded105=" + reloadedAfterZoomOut
                    + " ctrl0=" + afterCtrlZero
            )

            // 8. Every Popup Window Closable with ESC
            var popupEntries = root.allPopupWindows()
            var escFailedPopups = []
            var escVerifiedCount = 0
            // Ensure clean state before testing popups
            for (var pClean = 0; pClean < popupEntries.length; pClean++) {
                if (popupEntries[pClean].win && popupEntries[pClean].win.visible) {
                    popupEntries[pClean].win.close()
                }
            }
            for (var pIdx = 0; pIdx < popupEntries.length; pIdx++) {
                var pEntry = popupEntries[pIdx]
                var pWin = pEntry.win
                pEntry.openFn()
                var openedOk = Boolean(pWin && pWin.visible === true && pWin.closableWithEscape === true)
                deviceController.sendKeyToActiveWindow(Qt.Key_Escape, 0, "")
                var closedByEscOk = Boolean(pWin && pWin.visible === false)
                if (!openedOk || !closedByEscOk) {
                    escFailedPopups.push(pEntry.name + "(opened=" + openedOk + ",closed=" + closedByEscOk + ")")
                    if (pWin && pWin.visible) {
                        pWin.close()
                    }
                } else {
                    escVerifiedCount++
                }
            }
            root.selectedSlotNumber = 0
            root.syncWindowWidthToEditorState()
            recordBehavior(
                "all_popup_windows_closable_with_esc",
                escFailedPopups.length === 0 && escVerifiedCount === popupEntries.length && escVerifiedCount >= 24,
                "verified=" + escVerifiedCount + "/" + popupEntries.length
                    + (escFailedPopups.length > 0 ? " failed=" + escFailedPopups.join(",") : "")
            )

            // 9–12. Profile Group Default Fallback, Fallback Deletion Protection, Reorder/Add Rule Updates, and Deleted Profile Rule Reassignment to Fallback
            var backupGroupPath = "/tmp/duckydeck-ui-test-backup-group.duckygroup.json"
            deviceController.saveProfileGroup(backupGroupPath, "TestBackup")

            // 9. New profile group always creates 'Default' profile and marks it as fallback (#1)
            var createdGroupOk = deviceController.createNewProfileGroup("StudioTestGroup", "Work")
            var groupProfs = []
            try { groupProfs = JSON.parse(deviceController.profilesJson || "[]") } catch (e) {}
            var newGroupDefaultFallbackOk = (
                createdGroupOk
                && groupProfs.length === 2
                && groupProfs[0].number === 1
                && groupProfs[0].name === "Default"
                && groupProfs[1].number === 2
                && groupProfs[1].name === "Work"
                && deviceController.defaultFallbackProfile === 1
            )
            recordBehavior(
                "new_profile_group_has_default_fallback",
                newGroupDefaultFallbackOk,
                "profs=" + groupProfs.map(function(p) { return p.number + ":" + p.name }).join(",")
                    + " fallback=" + deviceController.defaultFallbackProfile
            )

            // 10. Fallback profile cannot be deleted
            var deleteFallbackAttempt = deviceController.deleteProfile(deviceController.defaultFallbackProfile)
            var profsAfterFbDelete = []
            try { profsAfterFbDelete = JSON.parse(deviceController.profilesJson || "[]") } catch (e) {}
            var fallbackCannotBeDeletedOk = (
                deleteFallbackAttempt === false
                && profsAfterFbDelete.length === 2
                && profsAfterFbDelete[0].name === "Default"
                && deviceController.defaultFallbackProfile === 1
            )
            recordBehavior(
                "fallback_profile_cannot_be_deleted",
                fallbackCannotBeDeletedOk,
                "deleteReturned=" + deleteFallbackAttempt + " count=" + profsAfterFbDelete.length
            )

            // 11. Adding & reordering profiles updates auto-switcher rules and fallback profile
            var addThirdOk = deviceController.createNewProfile("Gaming")
            deviceController.addOrUpdateAutoSwitchRule(-1, true, "code", "", 2)
            deviceController.addOrUpdateAutoSwitchRule(-1, true, "steam", "", 3)
            // Move P3 ('Gaming', index 2) to index 0 -> new order: 1:Gaming, 2:Default(fallback), 3:Work
            var reorderOk = deviceController.reorderProfile(2, 0)
            var rulesAfterReorder = []
            try { rulesAfterReorder = JSON.parse(deviceController.autoSwitchRulesJson || "[]") } catch (e) {}
            var codeRuleAfterReorder = rulesAfterReorder.find(function(r) { return r.resourceClass === "code" })
            var steamRuleAfterReorder = rulesAfterReorder.find(function(r) { return r.resourceClass === "steam" })
            var deleteMovedFallbackAttempt = deviceController.deleteProfile(2)
            var reorderAndAddUpdatesRulesOk = Boolean(
                addThirdOk
                && reorderOk
                && deviceController.defaultFallbackProfile === 2
                && deleteMovedFallbackAttempt === false
                && codeRuleAfterReorder && codeRuleAfterReorder.targetProfile === 3 && codeRuleAfterReorder.targetProfileName === "Work"
                && steamRuleAfterReorder && steamRuleAfterReorder.targetProfile === 1 && steamRuleAfterReorder.targetProfileName === "Gaming"
            )
            recordBehavior(
                "profile_reorder_and_add_updates_autoswitch_rules",
                reorderAndAddUpdatesRulesOk,
                "fbAfterReorder=" + deviceController.defaultFallbackProfile
                    + " codeTarget=" + (codeRuleAfterReorder ? codeRuleAfterReorder.targetProfile : -1)
                    + " steamTarget=" + (steamRuleAfterReorder ? steamRuleAfterReorder.targetProfile : -1)
            )

            // 12. Deleting a profile referenced by an auto-switcher rule updates that rule to reference the fallback profile
            var deleteWorkOk = deviceController.deleteProfile(3)
            var rulesAfterDelete = []
            try { rulesAfterDelete = JSON.parse(deviceController.autoSwitchRulesJson || "[]") } catch (e) {}
            var codeRuleAfterDelete = rulesAfterDelete.find(function(r) { return r.resourceClass === "code" })
            var steamRuleAfterDelete = rulesAfterDelete.find(function(r) { return r.resourceClass === "steam" })
            var deletedRuleReassignedToFallbackOk = Boolean(
                deleteWorkOk
                && rulesAfterDelete.length === 2
                && deviceController.defaultFallbackProfile === 2
                && codeRuleAfterDelete
                && codeRuleAfterDelete.targetProfile === deviceController.defaultFallbackProfile
                && codeRuleAfterDelete.targetProfileName === "Default"
                && steamRuleAfterDelete
                && steamRuleAfterDelete.targetProfile === 1
            )
            recordBehavior(
                "deleted_profile_rule_reassigned_to_fallback",
                deletedRuleReassignedToFallbackOk,
                "rulesLen=" + rulesAfterDelete.length
                    + " codeTargetAfterDelete=" + (codeRuleAfterDelete ? codeRuleAfterDelete.targetProfile + ":" + codeRuleAfterDelete.targetProfileName : "missing")
                    + " fallback=" + deviceController.defaultFallbackProfile
            )

            // 13. DuckyDeck Studio custom firmware (DDS_FW_1.1.2_0f85dab1.bin) is available in the firmware catalog
            var fwList = []
            try { fwList = JSON.parse(deviceController.listAvailableFirmwaresJson() || "[]") } catch (e) {}
            var studioFw = fwList.find(function(fw) {
                return fw.is_official === false
                    && fw.supports_monochrome_icons === true
                    && fw.is_downloaded === true
                    && (fw.filename === "DDS_FW_1.1.2_0f85dab1.bin"
                        || fw.filename.indexOf("DDS_FW_1.1.2_") === 0
                        || fw.filename.indexOf("DDS_FW_1.1.1_") === 0)
            })
            recordBehavior(
                "duckydeck_studio_firmware_available_in_catalog",
                Boolean(studioFw && studioFw.path && studioFw.path.length > 0),
                "fwCount=" + fwList.length + " studioFw=" + (studioFw ? studioFw.filename + "@" + studioFw.path : "missing")
            )

            // 14. Auto-Switcher Rules window twist minimize, disk persistence/restore, & auto-expand on rule double-click
            autoSwitchDrawer.enableSectionExpanded = false
            autoSwitchDrawer.ruleEditorSectionExpanded = false
            var enableMinimizedOk = autoSwitchDrawer.enableSectionExpanded === false
            var ruleMinimizedOk = autoSwitchDrawer.ruleEditorSectionExpanded === false
            deviceController.reloadAutoSwitchSectionsFromDisk()
            var persistedMinimizedOk = (
                deviceController.autoswitchEnableSectionExpanded === false
                && deviceController.autoswitchRuleEditorExpanded === false
                && autoSwitchDrawer.enableSectionExpanded === false
                && autoSwitchDrawer.ruleEditorSectionExpanded === false
            )
            autoSwitchDrawer.loadRuleIntoEditor(0, "code", "", "", 1, false)
            var ruleAutoExpandedOnDoubleClickOk = autoSwitchDrawer.ruleEditorSectionExpanded === true
            deviceController.reloadAutoSwitchSectionsFromDisk()
            var persistedAfterAutoExpandOk = (
                deviceController.autoswitchEnableSectionExpanded === false
                && deviceController.autoswitchRuleEditorExpanded === true
            )
            autoSwitchDrawer.setSectionsExpanded(true, true)
            deviceController.reloadAutoSwitchSectionsFromDisk()
            var enableRestoredOk = (
                autoSwitchDrawer.enableSectionExpanded === true
                && autoSwitchDrawer.ruleEditorSectionExpanded === true
                && deviceController.autoswitchEnableSectionExpanded === true
                && deviceController.autoswitchRuleEditorExpanded === true
            )
            autoSwitchDrawer.editingRuleIndex = -1
            recordBehavior(
                "autoswitch_sections_twist_minimize_and_auto_expand_on_double_click",
                enableMinimizedOk
                    && ruleMinimizedOk
                    && persistedMinimizedOk
                    && ruleAutoExpandedOnDoubleClickOk
                    && persistedAfterAutoExpandOk
                    && enableRestoredOk,
                "enableMin=" + enableMinimizedOk
                    + " ruleMin=" + ruleMinimizedOk
                    + " persistedMin=" + persistedMinimizedOk
                    + " autoExpand=" + ruleAutoExpandedOnDoubleClickOk
                    + " persistedAutoExpand=" + persistedAfterAutoExpandOk
                    + " restored=" + enableRestoredOk
            )

            // 15. Auto-Switcher regex title matching & validation support
            var validRegexErr = deviceController.validateTitleRegexPattern("YouTube|Twitch|^\\[PROD\\]")
            var invalidRegexErr = deviceController.validateTitleRegexPattern("[unclosed(")
            var saveRegexRuleOk = deviceController.saveAutoSwitchRuleFull(-1, true, "*browser*", "YouTube|Twitch", true, "", 1)
            var regexRulesAfterSave = []
            try { regexRulesAfterSave = JSON.parse(deviceController.autoSwitchRulesJson || "[]") } catch (e) {}
            var savedRegexRule = regexRulesAfterSave.find(function(r) {
                return r.resourceClass === "*browser*" && r.titleContains === "YouTube|Twitch"
            })
            autoSwitchDrawer.loadRuleIntoEditor(0, "*browser*", "YouTube|Twitch", "", 1, true)
            var editorRegexErr = autoSwitchDrawer.titleRegexError
            autoSwitchDrawer.editingRuleIndex = -1
            recordBehavior(
                "autoswitch_rule_title_regex_support",
                validRegexErr === ""
                    && invalidRegexErr.length > 0
                    && saveRegexRuleOk
                    && Boolean(savedRegexRule && savedRegexRule.titleIsRegex === true)
                    && editorRegexErr === "",
                "validErrLen=" + validRegexErr.length
                    + " invalidErrLen=" + invalidRegexErr.length
                    + " savedTitleIsRegex=" + (savedRegexRule ? savedRegexRule.titleIsRegex : false)
            )

            // 16. OLED Label input fields fit 5 uppercase characters and emojis without horizontal or vertical clipping
            root.selectedSlotNumber = 1
            root.refreshSelectedSlotFromController()
            var prevL1 = label1Input.text
            var prevL2 = label2Input.text
            label1Input.text = "WWWWW"
            label2Input.text = "MMMMM"
            var l1AvailW = label1Input.width - label1Input.leftPadding - label1Input.rightPadding
            var l2AvailW = label2Input.width - label2Input.leftPadding - label2Input.rightPadding
            var l1FitOk = label1Input.contentWidth <= l1AvailW + 1
            var l2FitOk = label2Input.contentWidth <= l2AvailW + 1
            var l1HeightOk = label1Input.height >= label1Input.contentHeight + label1Input.topPadding + label1Input.bottomPadding - 1.5
            var l2HeightOk = label2Input.height >= label2Input.contentHeight + label2Input.topPadding + label2Input.bottomPadding - 1.5
            label1Input.text = "💡"
            var l1EmojiFitOk = label1Input.contentWidth <= l1AvailW + 1
            label1Input.text = prevL1
            label2Input.text = prevL2
            recordBehavior(
                "oled_label_input_fields_no_clipping",
                l1FitOk && l2FitOk && l1HeightOk && l2HeightOk && l1EmojiFitOk,
                "l1Fit=" + l1FitOk + " l2Fit=" + l2FitOk + " l1H=" + l1HeightOk + " l2H=" + l2HeightOk + " emoji=" + l1EmojiFitOk
            )

            // 17. duckyPad (2020) 15-key (3x5) & duckyPad Pro 20-key (4x5) model switching & multi-device discovery
            deviceController.setSimulatedDeviceModel(20)
            var dp20ModelOk = (
                deviceController.isDuckyPad2020 === true
                && deviceController.activeHwModel === 20
                && deviceController.onboardKeyCount === 15
                && hardwareCanvas.isDuckyPad2020 === true
                && hardwareCanvas.onboardGridColumns === 3
                && hardwareCanvas.onboardKeyCount === 15
                && oledDesktopWidget.isDuckyPad2020 === true
                && oledDesktopWidget.onboardKeyCount === 15
                && oledDesktopWidget.basePortraitCols === 3
            )
            deviceController.setSimulatedDeviceModel(24)
            var dp24ModelOk = (
                deviceController.isDuckyPad2020 === false
                && deviceController.activeHwModel === 24
                && deviceController.onboardKeyCount === 20
                && hardwareCanvas.isDuckyPad2020 === false
                && hardwareCanvas.onboardGridColumns === 4
                && hardwareCanvas.onboardKeyCount === 20
                && oledDesktopWidget.isDuckyPad2020 === false
                && oledDesktopWidget.onboardKeyCount === 20
                && oledDesktopWidget.basePortraitCols === 4
            )
            var scannedJson = deviceController.scanAvailableDevices()
            var scannedParsedOk = false
            try {
                var scannedArr = JSON.parse(scannedJson || "[]")
                scannedParsedOk = Array.isArray(scannedArr)
            } catch (e) {}
            recordBehavior(
                "duckypad_2020_and_multi_device_selector_support",
                dp20ModelOk && dp24ModelOk && scannedParsedOk,
                "dp20=" + dp20ModelOk + " dp24=" + dp24ModelOk + " scanJsonOk=" + scannedParsedOk
            )

            // Restore initial profile group for visual window steps
            deviceController.loadProfileGroup(backupGroupPath)
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
            root.refreshSelectedSlotFromController()
        }

        property var windowSteps: [
            {
                name: "MainWindow_Default",
                open: function() {
                    root.selectedSlotNumber = 0
                    root.syncWindowWidthToEditorState()
                },
                audit: function() {
                    auditItemTree("MainWindow_Default", root.contentItem, root.width, root.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {}
            },
            {
                name: "MainWindow_SlotInspector",
                open: function() {
                    root.selectedSlotNumber = 1
                    root.syncWindowWidthToEditorState()
                },
                audit: function() {
                    auditItemTree("MainWindow_SlotInspector", root.contentItem, root.width, root.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    root.selectedSlotNumber = 0
                    root.syncWindowWidthToEditorState()
                }
            },
            {
                name: "OledDesktopWidget_0deg",
                open: function() {
                    deviceController.setActiveProfileWidgetRotation(0)
                    oledDesktopWidget.show()
                    oledDesktopWidget.syncFromController()
                },
                audit: function() {
                    auditItemTree("OledDesktopWidget_0deg", oledDesktopWidget.contentItem, oledDesktopWidget.width, oledDesktopWidget.height, { r: 0.02, g: 0.03, b: 0.06, a: 1.0 })
                },
                close: function() {}
            },
            {
                name: "OledDesktopWidget_90deg",
                open: function() {
                    deviceController.setActiveProfileWidgetRotation(90)
                    oledDesktopWidget.show()
                    oledDesktopWidget.syncFromController()
                },
                audit: function() {
                    auditItemTree("OledDesktopWidget_90deg", oledDesktopWidget.contentItem, oledDesktopWidget.width, oledDesktopWidget.height, { r: 0.02, g: 0.03, b: 0.06, a: 1.0 })
                },
                close: function() {
                    deviceController.setActiveProfileWidgetRotation(0)
                    oledDesktopWidget.syncFromController()
                    oledDesktopWidget.hide()
                }
            },
            {
                name: "FirmwareUpdateDialog",
                open: function() {
                    ensureTestFixtures()
                    firmwareUpdateDialog.open()
                },
                audit: function() {
                    auditItemTree("FirmwareUpdateDialog", firmwareUpdateDialog.contentItem, firmwareUpdateDialog.width, firmwareUpdateDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    firmwareUpdateDialog.close()
                }
            },
            {
                name: "ProfilePropertiesWindow",
                open: function() {
                    profilePropertiesWindow.openForProfile(deviceController.activeProfile || 1)
                },
                audit: function() {
                    auditItemTree("ProfilePropertiesWindow", profilePropertiesWindow.contentItem, profilePropertiesWindow.width, profilePropertiesWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    profilePropertiesWindow.close()
                }
            },
            {
                name: "KeyPropertiesWindow",
                open: function() {
                    keyPropertiesWindow.openForSlot(1)
                },
                audit: function() {
                    auditItemTree("KeyPropertiesWindow", keyPropertiesWindow.contentItem, keyPropertiesWindow.width, keyPropertiesWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    keyPropertiesWindow.close()
                }
            },
            {
                name: "DuckyColorPickerWindow",
                open: function() {
                    duckyColorPickerWindow.openForKey(1, "#38BDF8")
                },
                audit: function() {
                    auditItemTree("DuckyColorPickerWindow", duckyColorPickerWindow.contentItem, duckyColorPickerWindow.width, duckyColorPickerWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    duckyColorPickerWindow.close()
                }
            },
            {
                name: "ProfileTemplatesDialog",
                open: function() {
                    profileTemplatesDialog.openGeneral()
                },
                audit: function() {
                    auditItemTree("ProfileTemplatesDialog", profileTemplatesDialog.contentItem, profileTemplatesDialog.width, profileTemplatesDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    profileTemplatesDialog.close()
                }
            },
            {
                name: "SettingsDialog",
                open: function() {
                    settingsDialog.open()
                },
                audit: function() {
                    auditItemTree("SettingsDialog", settingsDialog.contentItem, settingsDialog.width, settingsDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    settingsDialog.close()
                }
            },
            {
                name: "WebApiDestinationsDialog",
                open: function() {
                    webApiDestinationsDialog.openForDestination("", "POST", "", "")
                },
                audit: function() {
                    auditItemTree("WebApiDestinationsDialog", webApiDestinationsDialog.contentItem, webApiDestinationsDialog.width, webApiDestinationsDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    webApiDestinationsDialog.close()
                }
            },
            {
                name: "EventLogDialog",
                open: function() {
                    eventLogDialog.openLogWindow()
                },
                audit: function() {
                    auditItemTree("EventLogDialog", eventLogDialog.contentItem, eventLogDialog.width, eventLogDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    eventLogDialog.close()
                }
            },
            {
                name: "AboutDialog",
                open: function() {
                    aboutDialog.openAboutWindow()
                },
                audit: function() {
                    auditItemTree("AboutDialog", aboutDialog.contentItem, aboutDialog.width, aboutDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    aboutDialog.close()
                }
            },
            {
                name: "UnsyncedChangesDialog",
                open: function() {
                    unsyncedChangesDialog.open()
                },
                audit: function() {
                    auditItemTree("UnsyncedChangesDialog", unsyncedChangesDialog.contentItem, unsyncedChangesDialog.width, unsyncedChangesDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    unsyncedChangesDialog.close()
                }
            },
            {
                name: "SdSyncDialog",
                open: function() {
                    sdSyncDialog.open()
                },
                audit: function() {
                    auditItemTree("SdSyncDialog", sdSyncDialog.contentItem, sdSyncDialog.width, sdSyncDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    sdSyncDialog.close()
                }
            },
            {
                name: "AppLaunchConfigWindow",
                open: function() {
                    appLaunchConfigWindow.openWindow(1)
                },
                audit: function() {
                    auditItemTree("AppLaunchConfigWindow", appLaunchConfigWindow.contentItem, appLaunchConfigWindow.width, appLaunchConfigWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    appLaunchConfigWindow.close()
                }
            },
            {
                name: "InstalledAppPickerDialog",
                open: function() {
                    installedAppPickerDialog.show()
                },
                audit: function() {
                    auditItemTree("InstalledAppPickerDialog", installedAppPickerDialog.contentItem, installedAppPickerDialog.width, installedAppPickerDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    installedAppPickerDialog.close()
                }
            },
            {
                name: "WebApiKeyConfigWindow",
                open: function() {
                    webApiKeyConfigWindow.openWindow(1)
                },
                audit: function() {
                    auditItemTree("WebApiKeyConfigWindow", webApiKeyConfigWindow.contentItem, webApiKeyConfigWindow.width, webApiKeyConfigWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    webApiKeyConfigWindow.close()
                }
            },
            {
                name: "LightingAnimatorDialog",
                open: function() {
                    lightingAnimatorDialog.openFor("profile", 1)
                },
                audit: function() {
                    auditItemTree("LightingAnimatorDialog", lightingAnimatorDialog.contentItem, lightingAnimatorDialog.width, lightingAnimatorDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    lightingAnimatorDialog.close()
                }
            },
            {
                name: "MacroRecorderDialog",
                open: function() {
                    macroRecorderDialog.openRecorder()
                },
                audit: function() {
                    auditItemTree("MacroRecorderDialog", macroRecorderDialog.contentItem, macroRecorderDialog.width, macroRecorderDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    macroRecorderDialog.close()
                }
            },
            {
                name: "OledIconPickerDialog",
                open: function() {
                    oledIconPickerDialog.openForInspector(0)
                },
                audit: function() {
                    auditItemTree("OledIconPickerDialog", oledIconPickerDialog.contentItem, oledIconPickerDialog.width, oledIconPickerDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    oledIconPickerDialog.close()
                }
            },
            {
                name: "NewProfileGroupDialog",
                open: function() {
                    newProfileGroupDialog.openWithDefaults()
                },
                audit: function() {
                    auditItemTree("NewProfileGroupDialog", newProfileGroupDialog.contentItem, newProfileGroupDialog.width, newProfileGroupDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    newProfileGroupDialog.close()
                }
            },
            {
                name: "RenameProfileDialog",
                open: function() {
                    renameProfileDialog.openWithProfile(1, "Default")
                },
                audit: function() {
                    auditItemTree("RenameProfileDialog", renameProfileDialog.contentItem, renameProfileDialog.width, renameProfileDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    renameProfileDialog.close()
                }
            },
            {
                name: "PgvDrawer",
                open: function() {
                    pgvDrawer.open()
                },
                audit: function() {
                    auditItemTree("PgvDrawer", pgvDrawer.contentItem, pgvDrawer.width, pgvDrawer.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    pgvDrawer.close()
                }
            },
            {
                name: "AutoSwitchDrawer",
                open: function() {
                    autoSwitchDrawer.open()
                },
                audit: function() {
                    auditItemTree("AutoSwitchDrawer", autoSwitchDrawer.contentItem, autoSwitchDrawer.width, autoSwitchDrawer.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    autoSwitchDrawer.close()
                }
            },
            {
                name: "AiModal",
                open: function() {
                    aiModal.open()
                },
                audit: function() {
                    auditItemTree("AiModal", aiModal.contentItem, aiModal.width, aiModal.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    aiModal.close()
                }
            }
        ]

        property int phase: 0 // 0=init, 1=open window, 2=audit+close window

        Timer {
            id: uiTestStepTimer
            interval: 25
            repeat: true
            running: deviceController.isUiTestSuiteMode()
            onTriggered: {
                try {
                    var totalStepsPerPass = uiTestSuiteRunner.windowSteps.length
                    if (uiTestSuiteRunner.phase === 0) {
                        uiTestSuiteRunner.phase = 1
                        uiTestSuiteRunner.passIndex = 0
                        uiTestSuiteRunner.stepIndex = 0
                        try {
                            uiTestSuiteRunner.runBehavioralSuite()
                        } catch (errBehavior) {
                            uiTestSuiteRunner.recordBehavior("behavioral_suite_exception", false, String(errBehavior))
                        }
                        var firstCfg = uiTestSuiteRunner.auditPassConfigs[0]
                        uiTestSuiteRunner.currentTheme = firstCfg.theme
                        uiTestSuiteRunner.currentZoom = firstCfg.zoom
                        deviceController.applyAppThemeMode(firstCfg.theme)
                        deviceController.applyUiTextScale(firstCfg.zoom)
                        root.syncWindowWidthToEditorState()
                        return
                    }

                    if (uiTestSuiteRunner.stepIndex >= totalStepsPerPass) {
                        if (uiTestSuiteRunner.passIndex + 1 < uiTestSuiteRunner.auditPassConfigs.length) {
                            uiTestSuiteRunner.passIndex++
                            var nextCfg = uiTestSuiteRunner.auditPassConfigs[uiTestSuiteRunner.passIndex]
                            uiTestSuiteRunner.currentTheme = nextCfg.theme
                            uiTestSuiteRunner.currentZoom = nextCfg.zoom
                            deviceController.applyAppThemeMode(nextCfg.theme)
                            deviceController.applyUiTextScale(nextCfg.zoom)
                            root.syncWindowWidthToEditorState()
                            uiTestSuiteRunner.stepIndex = 0
                            uiTestSuiteRunner.phase = 1
                            return
                        } else {
                            // Finished all Dark/Light × [100%, 105%, 110%, 115%] audits!
                            uiTestStepTimer.stop()
                            deviceController.resetAllTextScales()
                            root.syncWindowWidthToEditorState()

                            uiTestSuiteRunner.recordBehavior(
                                "option_c_keycap_layout_no_overlap",
                                uiTestSuiteRunner.totalKeycapsAudited >= 160
                                    && uiTestSuiteRunner.keycapLayoutViolationsCount === 0,
                                "keycapsAudited=" + uiTestSuiteRunner.totalKeycapsAudited
                                    + " violations=" + uiTestSuiteRunner.keycapLayoutViolationsCount
                            )
                            uiTestSuiteRunner.recordBehavior(
                                "no_overflows_at_zoom_levels_up_to_115_percent",
                                uiTestSuiteRunner.overflowViolations.length === 0
                                    && uiTestSuiteRunner.passIndex === uiTestSuiteRunner.auditPassConfigs.length - 1,
                                "passesCompleted=" + (uiTestSuiteRunner.passIndex + 1)
                                    + "/" + uiTestSuiteRunner.auditPassConfigs.length
                                    + " windowsAudited=" + uiTestSuiteRunner.auditedWindows.length
                                    + " overflowViolations=" + uiTestSuiteRunner.overflowViolations.length
                            )

                            var failedBehaviors = []
                            for (var b = 0; b < uiTestSuiteRunner.behavioralResults.length; b++) {
                                if (!uiTestSuiteRunner.behavioralResults[b].passed) {
                                    failedBehaviors.push(uiTestSuiteRunner.behavioralResults[b])
                                }
                            }
                            var totalFailures = failedBehaviors.length
                                + uiTestSuiteRunner.overflowViolations.length
                                + uiTestSuiteRunner.contrastViolations.length
                                + uiTestSuiteRunner.scrollBarStyleViolations.length
                                + uiTestSuiteRunner.minFontPointSizeViolations.length

                            var report = {
                                passed: totalFailures === 0,
                                summary: {
                                    themesTested: ["dark", "light"],
                                    zoomLevelsTested: [100, 105, 110, 115],
                                    windowsAuditedCount: uiTestSuiteRunner.auditedWindows.length,
                                    totalElementsInspected: uiTestSuiteRunner.totalElementsInspected,
                                    totalControlsInspected: uiTestSuiteRunner.totalControlsInspected,
                                    totalTextNodesInspected: uiTestSuiteRunner.totalTextNodesInspected,
                                    totalScrollBarsInspected: uiTestSuiteRunner.totalScrollBarsInspected,
                                    totalKeycapsAudited: uiTestSuiteRunner.totalKeycapsAudited,
                                    behavioralChecksCount: uiTestSuiteRunner.behavioralResults.length,
                                    behavioralFailuresCount: failedBehaviors.length,
                                    overflowViolationsCount: uiTestSuiteRunner.overflowViolations.length,
                                    contrastViolationsCount: uiTestSuiteRunner.contrastViolations.length,
                                    scrollBarStyleViolationsCount: uiTestSuiteRunner.scrollBarStyleViolations.length,
                                    minFontPointSizeViolationsCount: uiTestSuiteRunner.minFontPointSizeViolations.length
                                },
                                behavioralResults: uiTestSuiteRunner.behavioralResults,
                                overflowViolations: uiTestSuiteRunner.overflowViolations,
                                contrastViolations: uiTestSuiteRunner.contrastViolations,
                                scrollBarStyleViolations: uiTestSuiteRunner.scrollBarStyleViolations,
                                minFontPointSizeViolations: uiTestSuiteRunner.minFontPointSizeViolations,
                                auditedWindows: uiTestSuiteRunner.auditedWindows
                            }

                            deviceController.writeUiTestReport(JSON.stringify(report, null, 2), totalFailures === 0 ? 0 : 1)
                            Qt.quit()
                            return
                        }
                    }

                    var currentStep = uiTestSuiteRunner.windowSteps[uiTestSuiteRunner.stepIndex]
                    var passTag = uiTestSuiteRunner.currentTheme + "@" + uiTestSuiteRunner.currentZoom + "%"
                    if (uiTestSuiteRunner.phase === 1) {
                        try {
                            currentStep.open()
                        } catch (e) {
                            uiTestSuiteRunner.recordBehavior("open_" + currentStep.name + "_" + passTag, false, String(e))
                        }
                        uiTestSuiteRunner.phase = 2
                    } else if (uiTestSuiteRunner.phase === 2) {
                        try {
                            currentStep.audit()
                        } catch (e2) {
                            uiTestSuiteRunner.recordBehavior("audit_" + currentStep.name + "_" + passTag, false, String(e2))
                        }
                        try {
                            currentStep.close()
                        } catch (e3) {}
                        uiTestSuiteRunner.stepIndex++
                        uiTestSuiteRunner.phase = 1
                    }
                } catch (fatalErr) {
                    uiTestStepTimer.stop()
                    var fatalReport = {
                        passed: false,
                        summary: {
                            fatalError: String(fatalErr),
                            behavioralFailuresCount: 1,
                            overflowViolationsCount: uiTestSuiteRunner.overflowViolations.length,
                            contrastViolationsCount: uiTestSuiteRunner.contrastViolations.length,
                            scrollBarStyleViolationsCount: uiTestSuiteRunner.scrollBarStyleViolations.length,
                            minFontPointSizeViolationsCount: uiTestSuiteRunner.minFontPointSizeViolations.length,
                            totalScrollBarsInspected: uiTestSuiteRunner.totalScrollBarsInspected
                        },
                        behavioralResults: uiTestSuiteRunner.behavioralResults,
                        overflowViolations: uiTestSuiteRunner.overflowViolations,
                        contrastViolations: uiTestSuiteRunner.contrastViolations,
                        scrollBarStyleViolations: uiTestSuiteRunner.scrollBarStyleViolations,
                        minFontPointSizeViolations: uiTestSuiteRunner.minFontPointSizeViolations,
                        auditedWindows: uiTestSuiteRunner.auditedWindows
                    }
                    deviceController.writeUiTestReport(JSON.stringify(fatalReport, null, 2), 1)
                    Qt.quit()
                }
            }
        }
    }

    readonly property bool closableWithEscape: true
    title: qsTr("System Event Log — DuckyDeck Studio v%1").arg(deviceController.appVersion)
    width: 960
    height: 560
    minimumWidth: 780
    minimumHeight: 420
    visible: false
    transientParent: root
    modality: Qt.NonModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    function openLogWindow() {
        deviceController.refreshEventLog()
        eventLogDialog.refreshEventLogModel()
        eventLogDialog.show()
        eventLogDialog.raise()
        eventLogDialog.requestActivate()
    }

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: eventLogDialog.visible
        onActivated: eventLogDialog.close()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Kirigami.Units.largeSpacing
        spacing: Kirigami.Units.mediumSpacing

        // Header Banner
        Rectangle {
            Layout.fillWidth: true
            implicitHeight: logHeaderCol.implicitHeight + 20
            radius: Kirigami.Units.smallSpacing
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
            border.width: 1

            ColumnLayout {
                id: logHeaderCol
                anchors.fill: parent
                anchors.margins: 10
                spacing: 4

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.Label {
                        text: qsTr("📋 System Event Log")
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }

                    Rectangle {
                        radius: 4
                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                        border.width: 1
                        implicitWidth: logCountBadgeLbl.implicitWidth + 12
                        implicitHeight: Math.round(22 * root.eventLogScale)

                        QQC2.Label {
                            id: logCountBadgeLbl
                            anchors.centerIn: parent
                            text: qsTr("%1 shown / %2 total")
                                .arg(eventLogListView.count)
                                .arg(deviceController.eventLogCount)
                            font.family: "monospace"
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: root.highContrastHighlightText
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "view-refresh"
                        text: qsTr("Refresh")
                        implicitHeight: Math.round(26 * root.eventLogScale)
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            deviceController.refreshEventLog()
                            eventLogDialog.refreshEventLogModel()
                        }
                    }
                }

                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    text: qsTr("Log file: %1").arg(deviceController.logFilePath)
                    font.family: "monospace"
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    elide: Text.ElideMiddle
                }
            }
        }

        // Search & Level Filter Row
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            QQC2.TextField {
                id: eventLogSearchField
                Layout.fillWidth: true
                Layout.minimumWidth: 140
                Layout.preferredWidth: 1
                implicitHeight: Math.round(30 * root.eventLogScale)
                font.pointSize: 10 * root.fontScale
                selectByMouse: true
                placeholderText: qsTr("🔍 Filter events by keyword, timestamp, profile, window class, or error...")
                text: eventLogDialog.eventLogSearchText
                onTextEdited: {
                    eventLogDialog.eventLogSearchText = text
                    eventLogDialog.refreshEventLogModel()
                }
            }

            QQC2.ToolButton {
                visible: eventLogSearchField.text.length > 0
                text: "✕"
                implicitWidth: Math.round(28 * root.eventLogScale)
                implicitHeight: Math.round(28 * root.eventLogScale)
                onClicked: {
                    eventLogSearchField.text = ""
                    eventLogDialog.eventLogSearchText = ""
                    eventLogDialog.refreshEventLogModel()
                }
            }

            QQC2.Label {
                text: qsTr("Level:")
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.disabledTextColor
            }

            Repeater {
                model: eventLogDialog.eventLogLevels

                delegate: QQC2.Button {
                    required property string modelData
                    readonly property bool isSel: eventLogDialog.eventLogLevelFilter === modelData
                    text: modelData
                    checkable: true
                    checked: isSel
                    highlighted: isSel
                    implicitHeight: Math.round(26 * root.eventLogScale)
                    font.pointSize: 10 * root.fontScale
                    font.bold: isSel
                    onClicked: {
                        eventLogDialog.eventLogLevelFilter = modelData
                        eventLogDialog.refreshEventLogModel()
                    }
                }
            }
        }

        // Category Filter Pills Row (Flow wraps cleanly at any font scale)
        Flow {
            Layout.fillWidth: true
            spacing: 4

            QQC2.Label {
                text: qsTr("Category:")
                height: Math.round(24 * root.eventLogScale)
                verticalAlignment: Text.AlignVCenter
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.disabledTextColor
            }

            Repeater {
                model: eventLogDialog.eventLogCategories

                delegate: QQC2.Button {
                    required property string modelData
                    readonly property bool isSelCat: eventLogDialog.eventLogCategoryFilter === modelData
                    text: modelData
                    checkable: true
                    checked: isSelCat
                    highlighted: isSelCat
                    implicitHeight: Math.round(24 * root.eventLogScale)
                    font.pointSize: 10 * root.fontScale
                    font.bold: isSelCat
                    onClicked: {
                        eventLogDialog.eventLogCategoryFilter = modelData
                        eventLogDialog.refreshEventLogModel()
                    }
                }
            }
        }

        // Event Log List Container
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: Kirigami.Units.smallSpacing
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            border.width: 1
            clip: true

            QQC2.ScrollView {
                anchors.fill: parent
                anchors.margins: 4
                clip: true
                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    policy: eventLogListView.contentHeight > eventLogListView.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }
                QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                    orientation: Qt.Horizontal
                }

                ListView {
                    id: eventLogListView
                    model: eventLogModel
                    spacing: 3
                    boundsBehavior: Flickable.StopAtBounds

                    delegate: Rectangle {
                        id: logRow
                        required property int index
                        required property string timestamp
                        required property string level
                        required property string category
                        required property string message

                        width: eventLogListView.width - 8
                        implicitHeight: Math.max(Math.round(28 * root.eventLogScale), logRowLayout.implicitHeight + 8)
                        radius: 4
                        color: logRowMouse.containsMouse
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                            : (index % 2 === 0
                                ? Qt.alpha(Kirigami.Theme.backgroundColor, 0.55)
                                : Qt.alpha(Kirigami.Theme.backgroundColor, 0.25))
                        border.color: logRow.level === "ERROR"
                            ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.45)
                            : (logRow.level === "WARN"
                                ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.35)
                                : "transparent")
                        border.width: 1

                        RowLayout {
                            id: logRowLayout
                            anchors.fill: parent
                            anchors.leftMargin: 8
                            anchors.rightMargin: 8
                            spacing: 8

                            QQC2.Label {
                                text: logRow.timestamp
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                Layout.preferredWidth: Math.round(165 * root.eventLogScale)
                            }

                            Rectangle {
                                radius: 3
                                color: Qt.alpha(eventLogDialog.levelColorForLog(logRow.level), 0.18)
                                border.color: Qt.alpha(eventLogDialog.levelColorForLog(logRow.level), 0.55)
                                border.width: 1
                                Layout.preferredWidth: Math.round(48 * root.eventLogScale)
                                implicitHeight: Math.round(18 * root.eventLogScale)

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: logRow.level
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: eventLogDialog.levelColorForLog(logRow.level)
                                }
                            }

                            Rectangle {
                                radius: 3
                                color: Qt.alpha(eventLogDialog.categoryColorForLog(logRow.category), 0.18)
                                border.color: Qt.alpha(eventLogDialog.categoryColorForLog(logRow.category), 0.55)
                                border.width: 1
                                Layout.preferredWidth: Math.round(78 * root.eventLogScale)
                                implicitHeight: Math.round(18 * root.eventLogScale)

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: logRow.category
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: eventLogDialog.categoryColorForLog(logRow.category)
                                }
                            }

                            QQC2.Label {
                                text: logRow.message
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                color: logRow.level === "ERROR"
                                    ? Kirigami.Theme.negativeTextColor
                                    : Kirigami.Theme.textColor
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                wrapMode: Text.WrapAnywhere
                            }
                        }

                        MouseArea {
                            id: logRowMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                const lineStr = "[" + logRow.timestamp + "] [" + logRow.level + "] [" + logRow.category + "] " + logRow.message
                                root.copyTextToClipboard(lineStr)
                            }
                        }

                        QQC2.ToolTip.visible: logRowMouse.containsMouse
                        QQC2.ToolTip.delay: 400
                        QQC2.ToolTip.text: qsTr("Click to copy this log entry to clipboard")
                    }
                }
            }
        }

        // Footer Actions Row
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QQC2.Button {
                icon.name: "edit-copy"
                text: qsTr("Copy Log to Clipboard")
                font.pointSize: 10 * root.fontScale
                onClicked: {
                    const txt = deviceController.formatEventLogText(
                        eventLogDialog.eventLogCategoryFilter,
                        eventLogDialog.eventLogLevelFilter,
                        eventLogDialog.eventLogSearchText
                    )
                    root.copyTextToClipboard(txt)
                }
            }

            QQC2.Button {
                icon.name: "document-export"
                text: qsTr("Export Log (.log)...")
                font.pointSize: 10 * root.fontScale
                onClicked: exportEventLogFileDialog.open()
            }

            QQC2.Button {
                icon.name: "edit-clear-history"
                text: qsTr("Clear Log")
                font.pointSize: 10 * root.fontScale
                onClicked: {
                    deviceController.clearEventLog()
                    eventLogDialog.refreshEventLogModel()
                }
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                text: qsTr("Close")
                font.pointSize: 10 * root.fontScale
                onClicked: eventLogDialog.close()
            }
        }
    }
}
