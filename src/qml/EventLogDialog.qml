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
    property var tutorialsDialog: null
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
    readonly property var uiTestSuiteRunner: uiTestSuiteLoader.item

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


    Loader {
        id: uiTestSuiteLoader
        active: deviceController ? deviceController.isUiTestSuiteMode() : false
        source: Qt.resolvedUrl("UiTestHarness.qml")
        onLoaded: {
            if (item) {
                // qmllint disable missing-property
                item.host = eventLogDialog
                // qmllint enable missing-property
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
