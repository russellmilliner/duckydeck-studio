import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Window {
    id: appLaunchConfigWindow
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property color highContrastPositiveText: root ? root.highContrastPositiveText : "#4ade80"
    property color highContrastWarningText: root ? root.highContrastWarningText : "#fbbf24"
    property var hardwareCanvas: null
    property var liveSlotAutoSaveTimer: null
    property var label1Input: null
    property var label2Input: null
    property var scriptEditor: null
    property var keyPropertiesWindow: null
    property alias installedAppPickerDialog: installedAppPickerDialog
    property alias macroRecorderDialog: macroRecorderDialog
    property alias oledIconPickerDialog: oledIconPickerDialog
    property alias appLaunchCommandField: appLaunchCommandField

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

    ListModel {
        id: installedAppsModel
    }


    function buildAppLaunchConfigJson() {
        const modeIdx = Math.max(0, appLaunchTriggerModeCombo.currentIndex)
        const fkeyIdx = Math.max(0, appLaunchFkeyCombo.currentIndex)
        const cfg = {
            enabled: appLaunchEnabledCheck.checked,
            appName: root.appLaunchAppName || "",
            desktopId: root.appLaunchDesktopId || "",
            command: appLaunchCommandField.text.trim(),
            args: appLaunchArgsField.text.trim(),
            workingDir: appLaunchWorkDirField.text.trim(),
            runInTerminal: appLaunchTerminalCheck.checked,
            oledText: appLaunchOledField.text.trim(),
            ledFeedback: appLaunchLedCheck.checked,
            triggerMode: root.appLaunchTriggerModes[modeIdx] || "pgv",
            fkey: root.webApiFkeys[fkeyIdx] || "F13"
        }
        return JSON.stringify(cfg)
    }

    function loadAppLaunchBuilderFromScript(scriptText) {
        if (!deviceController) return
        root.syncingAppLaunchScript = true
        try {
            const rawJson = deviceController.extractScriptAppLaunchJson(scriptText, root.selectedSlotNumber)
            const cfg = JSON.parse(rawJson)
            const isEnabled = !!cfg.enabled
            root.appLaunchEnabled = isEnabled
            appLaunchEnabledCheck.checked = isEnabled
            root.appLaunchAppName = cfg.appName || ""
            root.appLaunchDesktopId = cfg.desktopId || ""
            appLaunchCommandField.text = cfg.command || ""
            appLaunchArgsField.text = cfg.args || ""
            appLaunchWorkDirField.text = cfg.workingDir || ""
            appLaunchTerminalCheck.checked = !!cfg.runInTerminal
            appLaunchOledField.text = cfg.oledText || ""
            appLaunchLedCheck.checked = cfg.ledFeedback !== undefined ? !!cfg.ledFeedback : true
            const tMode = (cfg.triggerMode || "pgv").toLowerCase()
            const mIdx = root.appLaunchTriggerModes.indexOf(tMode)
            appLaunchTriggerModeCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            const fIdx = root.webApiFkeys.indexOf((cfg.fkey || "F13").toUpperCase())
            appLaunchFkeyCombo.currentIndex = fIdx >= 0 ? fIdx : ((root.selectedSlotNumber - 1) % 12)
            if (isEnabled) {
                root.showAppLaunchBuilder = true
            }
        } catch (e) {
            root.appLaunchEnabled = false
            appLaunchEnabledCheck.checked = false
            root.appLaunchAppName = ""
            root.appLaunchDesktopId = ""
        }
        root.syncingAppLaunchScript = false
    }

    function syncAppLaunchBuilderToScript() {
        if (root.loadingSlotData || root.syncingAppLaunchScript || !deviceController) return
        root.syncingAppLaunchScript = true
        root.appLaunchEnabled = appLaunchEnabledCheck.checked
        const cfgJson = appLaunchConfigWindow.buildAppLaunchConfigJson()
        const updated = deviceController.applyAppLaunchToScript(
            scriptEditor.text,
            cfgJson,
            root.selectedSlotNumber,
            root.editingReleaseScript
        )
        scriptEditor.text = updated
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = updated
        } else {
            root.pressScriptBuffer = updated
        }
        root.syncingAppLaunchScript = false
        root.updateCompilerStatus()
        if (root.compileOk) {
            root.saveCurrentSlotToController()
            hardwareCanvas.syncSlotsFromController()
        }
    }

    function refreshInstalledAppsModel() {
        if (!deviceController) return
        installedAppsModel.clear()
        const raw = deviceController.installedAppsJson || "[]"
        const query = (root.installedAppSearchText || "").trim().toLowerCase()
        try {
            const arr = JSON.parse(raw)
            if (!Array.isArray(arr)) return
            for (let i = 0; i < arr.length; ++i) {
                const app = arr[i]
                const name = app.name || ""
                const desktopId = app.desktopId || ""
                const execCmd = app.execCommand || app.execCmd || ""
                const execArgs = app.execArgs || ""
                const comment = app.comment || ""
                const categories = app.categories || ""
                if (query.length > 0) {
                    const hay = (name + " " + desktopId + " " + execCmd + " " + execArgs + " " + comment + " " + categories).toLowerCase()
                    if (!hay.includes(query)) continue
                }
                installedAppsModel.append({
                    desktopId: desktopId,
                    name: name,
                    execCmd: execCmd,
                    execArgs: execArgs,
                    icon: app.icon || "application-x-executable",
                    comment: comment,
                    terminal: !!app.terminal,
                    categories: categories
                })
            }
        } catch (e) {}
    }

    FileDialog {
        id: appExecutableFileDialog
        title: qsTr("Select Application, Script, or Executable")
        nameFilters: [
            qsTr("All Files (*)"),
            qsTr("Scripts & AppImages (*.sh *.py *.AppImage *.desktop)")
        ]
        onAccepted: {
            let filePath = selectedFile.toString()
            if (filePath.indexOf("file://") === 0) {
                filePath = decodeURIComponent(filePath.substring(7))
                if (/^\/[A-Za-z]:/.test(filePath)) {
                    filePath = filePath.substring(1)
                }
            }
            if (filePath.length > 0) {
                const parts = filePath.split(/[\/\\]/)
                const baseName = parts.length > 0 ? parts[parts.length - 1] : filePath
                root.appLaunchDesktopId = ""
                root.appLaunchAppName = baseName
                appLaunchCommandField.text = filePath
                appLaunchEnabledCheck.checked = true
                if (appLaunchOledField.text.trim().length === 0) {
                    appLaunchOledField.text = "Run: " + baseName.substring(0, 14)
                }
                if (label1Input.text.trim().length === 0 && label2Input.text.trim().length === 0) {
                    label1Input.text = baseName.substring(0, 5)
                }
                appLaunchConfigWindow.syncAppLaunchBuilderToScript()
            }
        }
    }

    Window {
        id: installedAppPickerDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: installedAppPickerDialog.visible
            onActivated: installedAppPickerDialog.close()
        }
        title: qsTr("Select Installed Application — DuckyDeck Studio")
        width: 720
        height: 540
        minimumWidth: 560
        minimumHeight: 400
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function selectDesktopApp(name, desktopId, execCmd, execArgs, terminal) {
            root.appLaunchAppName = name || ""
            root.appLaunchDesktopId = desktopId || ""
            appLaunchCommandField.text = execCmd || ""
            appLaunchArgsField.text = execArgs || ""
            appLaunchTerminalCheck.checked = !!terminal
            appLaunchEnabledCheck.checked = true
            appLaunchOledField.text = "Run: " + (name || desktopId || "App").substring(0, 14)
            if (label1Input.text.trim().length === 0 && label2Input.text.trim().length === 0 && name && name.length > 0) {
                label1Input.text = name.substring(0, 5)
            }
            appLaunchConfigWindow.syncAppLaunchBuilderToScript()
            installedAppPickerDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("🚀 Choose an Installed Desktop Application")
                    font.bold: true
                    font.pointSize: 11 * root.fontScale
                }

                Item { Layout.fillWidth: true }

                QQC2.Label {
                    text: qsTr("%1 applications").arg(installedAppsModel.count)
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "view-refresh"
                    text: qsTr("Rescan")
                    onClicked: {
                        deviceController.refreshInstalledApps()
                        appLaunchConfigWindow.refreshInstalledAppsModel()
                    }
                }
            }

            QQC2.TextField {
                id: installedAppSearchInput
                Layout.fillWidth: true
                placeholderText: qsTr("Search applications by name, command, category, or description...")
                selectByMouse: true
                font.pointSize: 10 * root.fontScale
                onTextChanged: {
                    root.installedAppSearchText = text
                    appLaunchConfigWindow.refreshInstalledAppsModel()
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 2
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        policy: installedAppsListView.contentHeight > installedAppsListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    ListView {
                        id: installedAppsListView
                        model: installedAppsModel
                        clip: true
                        spacing: 2

                        delegate: Rectangle {
                            required property int index
                            required property string desktopId
                            required property string name
                            required property string execCmd
                            required property string execArgs
                            required property string icon
                            required property string comment
                            required property bool terminal
                            required property string categories

                            width: installedAppsListView.width
                            height: Math.round(52 * root.fontScale)
                            radius: 4
                            color: appRowMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                                : (index % 2 === 0 ? "transparent" : Qt.alpha(Kirigami.Theme.textColor, 0.03))

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 10
                                anchors.rightMargin: 10
                                spacing: 10

                                Kirigami.Icon {
                                    source: icon && icon.length > 0 ? icon : "application-x-executable"
                                    fallback: "application-x-executable"
                                    Layout.preferredWidth: Math.round(28 * root.fontScale)
                                    Layout.preferredHeight: Math.round(28 * root.fontScale)
                                }

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 2

                                    RowLayout {
                                        Layout.fillWidth: true
                                        spacing: 6

                                        QQC2.Label {
                                            text: name
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            elide: Text.ElideRight
                                        }

                                        Rectangle {
                                            visible: terminal
                                            radius: 3
                                            color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.2)
                                            implicitWidth: termBadgeLbl.implicitWidth + 8
                                            implicitHeight: termBadgeLbl.implicitHeight + 2

                                            QQC2.Label {
                                                id: termBadgeLbl
                                                anchors.centerIn: parent
                                                text: qsTr("Terminal")
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.neutralTextColor
                                            }
                                        }

                                        QQC2.Label {
                                            Layout.fillWidth: true
                                            text: desktopId
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                        }
                                    }

                                    QQC2.Label {
                                        Layout.fillWidth: true
                                        readonly property string fullExecText: execArgs.length > 0 ? (execCmd + " " + execArgs) : execCmd
                                        text: comment.length > 0 ? (comment + "  •  " + fullExecText) : fullExecText
                                        font.family: comment.length > 0 ? Kirigami.Theme.defaultFont.family : "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                        elide: Text.ElideRight
                                    }
                                }

                                QQC2.Button {
                                    text: qsTr("Select")
                                    implicitHeight: Math.round(26 * root.fontScale)
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                                }
                            }

                            MouseArea {
                                id: appRowMouse
                                anchors.fill: parent
                                anchors.rightMargin: 80
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onDoubleClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                                onClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("📁 Browse File System...")
                    onClicked: {
                        installedAppPickerDialog.close()
                        appExecutableFileDialog.open()
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "window-close"
                    text: qsTr("Cancel")
                    onClicked: installedAppPickerDialog.close()
                }
            }
        }
    }

    Window {
        id: macroRecorderDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: macroRecorderDialog.visible
            onActivated: { if (macroRecorderDialog.isRecording) macroRecorderDialog.stopRecording(); macroRecorderDialog.close() }
        }
        title: qsTr("Live Macro Recorder (DuckyScript 3) — DuckyDeck Studio")
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)
        width: Math.max(840, minimumWidth)
        height: 680
        minimumWidth: 780
        minimumHeight: 520
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property bool isRecording: false
        property bool globalEvdevActive: false
        property string timingMode: "exact" // "exact" | "fast" | "fixed"
        property int fixedDelayMs: 35
        property int minDelayThresholdMs: 12
        property bool captureKeyboard: true
        property bool captureMouseClicks: true
        property bool captureMouseMoves: false
        property var rawEvents: []
        property double recordingStartMs: 0
        property double lastInputMs: 0
        property real lastPadMouseX: -1
        property real lastPadMouseY: -1
        property double lastEvdevEventMs: 0

        function openRecorder() {
            macroRecorderDialog.show()
            macroRecorderDialog.raise()
            macroRecorderDialog.requestActivate()
            captureFocusPad.forceActiveFocus()
        }

        function startRecordingSession() {
            macroRecorderDialog.rawEvents = []
            macroRecorderDialog.recordingStartMs = Date.now()
            macroRecorderDialog.lastInputMs = macroRecorderDialog.recordingStartMs
            macroRecorderDialog.lastPadMouseX = -1
            macroRecorderDialog.lastPadMouseY = -1
            macroRecorderDialog.isRecording = true
            macroRecorderDialog.globalEvdevActive = deviceController.startGlobalMacroCapture(
                macroRecorderDialog.captureMouseMoves,
                macroRecorderDialog.captureMouseClicks
            )
            evdevPollTimer.start()
            macroRecorderDialog.rebuildGeneratedDuckyScript()
            captureFocusPad.forceActiveFocus()
        }

        function stopRecordingSession() {
            if (!macroRecorderDialog.isRecording) return
            evdevPollTimer.stop()
            macroRecorderDialog.pollEvdevOnce()
            deviceController.stopGlobalMacroCapture()
            macroRecorderDialog.isRecording = false
            macroRecorderDialog.globalEvdevActive = false
            macroRecorderDialog.rebuildGeneratedDuckyScript()
        }

        onClosing: {
            if (macroRecorderDialog.isRecording) {
                macroRecorderDialog.stopRecordingSession()
            }
        }

        function qtKeyToDuckyName(key, text) {
            switch (key) {
                case Qt.Key_Return:
                case Qt.Key_Enter: return "ENTER"
                case Qt.Key_Escape: return "ESC"
                case Qt.Key_Backspace: return "BACKSPACE"
                case Qt.Key_Tab: return "TAB"
                case Qt.Key_Space: return "SPACE"
                case Qt.Key_CapsLock: return "CAPSLOCK"
                case Qt.Key_Print: return "PRINTSCREEN"
                case Qt.Key_ScrollLock: return "SCROLLLOCK"
                case Qt.Key_Pause: return "PAUSE"
                case Qt.Key_Insert: return "INSERT"
                case Qt.Key_Home: return "HOME"
                case Qt.Key_PageUp: return "PAGEUP"
                case Qt.Key_Delete: return "DELETE"
                case Qt.Key_End: return "END"
                case Qt.Key_PageDown: return "PAGEDOWN"
                case Qt.Key_Right: return "RIGHT"
                case Qt.Key_Left: return "LEFT"
                case Qt.Key_Down: return "DOWN"
                case Qt.Key_Up: return "UP"
                case Qt.Key_NumLock: return "NUMLOCK"
                case Qt.Key_Menu: return "MENU"
                case Qt.Key_Shift: return "SHIFT"
                case Qt.Key_Control: return "CTRL"
                case Qt.Key_Alt: return "ALT"
                case Qt.Key_AltGr: return "RALT"
                case Qt.Key_Meta:
                case Qt.Key_Super_L:
                case Qt.Key_Super_R: return "WIN"
                case Qt.Key_F1: return "F1"
                case Qt.Key_F2: return "F2"
                case Qt.Key_F3: return "F3"
                case Qt.Key_F4: return "F4"
                case Qt.Key_F5: return "F5"
                case Qt.Key_F6: return "F6"
                case Qt.Key_F7: return "F7"
                case Qt.Key_F8: return "F8"
                case Qt.Key_F9: return "F9"
                case Qt.Key_F10: return "F10"
                case Qt.Key_F11: return "F11"
                case Qt.Key_F12: return "F12"
                case Qt.Key_F13: return "F13"
                case Qt.Key_F14: return "F14"
                case Qt.Key_F15: return "F15"
                case Qt.Key_F16: return "F16"
                case Qt.Key_F17: return "F17"
                case Qt.Key_F18: return "F18"
                case Qt.Key_F19: return "F19"
                case Qt.Key_F20: return "F20"
                case Qt.Key_F21: return "F21"
                case Qt.Key_F22: return "F22"
                case Qt.Key_F23: return "F23"
                case Qt.Key_F24: return "F24"
                case Qt.Key_Minus: return "MINUS"
                case Qt.Key_Equal: return "EQUAL"
                case Qt.Key_BracketLeft: return "LEFTBRACE"
                case Qt.Key_BracketRight: return "RIGHTBRACE"
                case Qt.Key_Backslash: return "BACKSLASH"
                case Qt.Key_Semicolon: return "SEMICOLON"
                case Qt.Key_Apostrophe: return "APOSTROPHE"
                case Qt.Key_QuoteLeft: return "GRAVE"
                case Qt.Key_Comma: return "COMMA"
                case Qt.Key_Period: return "DOT"
                case Qt.Key_Slash: return "SLASH"
                default: break
            }
            if (key >= Qt.Key_A && key <= Qt.Key_Z) {
                return String.fromCharCode(97 + (key - Qt.Key_A))
            }
            if (key >= Qt.Key_0 && key <= Qt.Key_9) {
                return String.fromCharCode(48 + (key - Qt.Key_0))
            }
            if (text && text.length === 1) {
                const c = text.toLowerCase()
                if (/[a-z0-9]/.test(c)) return c
            }
            return ""
        }

        function appendRecordedEvent(evt) {
            let list = macroRecorderDialog.rawEvents.slice()
            // Coalesce consecutive mouse_move events within 25ms
            if (evt.kind === "mouse_move" && list.length > 0) {
                let prev = list[list.length - 1]
                if (prev.kind === "mouse_move" && (evt.elapsed_ms - prev.elapsed_ms) <= 25) {
                    prev.dx = (prev.dx || 0) + (evt.dx || 0)
                    prev.dy = (prev.dy || 0) + (evt.dy || 0)
                    prev.elapsed_ms = evt.elapsed_ms
                    list[list.length - 1] = prev
                    macroRecorderDialog.rawEvents = list
                    macroRecorderDialog.rebuildGeneratedDuckyScript()
                    return
                }
            }
            list.push(evt)
            macroRecorderDialog.rawEvents = list
            macroRecorderDialog.rebuildGeneratedDuckyScript()
        }

        function pollEvdevOnce() {
            if (!macroRecorderDialog.isRecording) return
            const jsonStr = deviceController.pollGlobalMacroEventsJson()
            if (!jsonStr || jsonStr === "[]") return
            try {
                const arr = JSON.parse(jsonStr)
                if (!Array.isArray(arr) || arr.length === 0) return
                let list = macroRecorderDialog.rawEvents.slice()
                for (let i = 0; i < arr.length; i++) {
                    const e = arr[i]
                    if ((e.kind === "key_down" || e.kind === "key_up") && !macroRecorderDialog.captureKeyboard) continue
                    if ((e.kind === "mouse_click" || e.kind === "mouse_wheel") && !macroRecorderDialog.captureMouseClicks) continue
                    if (e.kind === "mouse_move" && !macroRecorderDialog.captureMouseMoves) continue
                    macroRecorderDialog.lastEvdevEventMs = Date.now()
                    if (e.kind === "mouse_move" && list.length > 0 && list[list.length - 1].kind === "mouse_move" && (e.elapsed_ms - list[list.length - 1].elapsed_ms) <= 25) {
                        list[list.length - 1].dx += e.dx
                        list[list.length - 1].dy += e.dy
                        list[list.length - 1].elapsed_ms = e.elapsed_ms
                    } else {
                        list.push(e)
                    }
                }
                macroRecorderDialog.rawEvents = list
                macroRecorderDialog.rebuildGeneratedDuckyScript()
            } catch (err) {}
        }

        Timer {
            id: evdevPollTimer
            interval: 40
            repeat: true
            running: false
            onTriggered: macroRecorderDialog.pollEvdevOnce()
        }

        function rebuildGeneratedDuckyScript() {
            const events = macroRecorderDialog.rawEvents
            if (!events || events.length === 0) {
                recordedScriptEditor.text = qsTr("// Click '🔴 Start Recording' above, then type keys or use the mouse.\n// Your recorded DuckyScript 3 macro will appear here in real time.")
                return
            }

            let lines = []
            const modeLabel = macroRecorderDialog.timingMode === "exact"
                ? "Exact Timings"
                : (macroRecorderDialog.timingMode === "fast" ? "As Fast As Possible" : ("Fixed Delay (" + macroRecorderDialog.fixedDelayMs + "ms)"))
            lines.push("// Recorded Macro (" + modeLabel + " • " + events.length + " events)")

            let prevElapsed = events[0].elapsed_ms || 0

            for (let i = 0; i < events.length; i++) {
                const ev = events[i]
                const dt = Math.max(0, Math.round((ev.elapsed_ms || 0) - prevElapsed))
                prevElapsed = ev.elapsed_ms || prevElapsed

                if (i > 0) {
                    if (macroRecorderDialog.timingMode === "exact") {
                        if (dt >= macroRecorderDialog.minDelayThresholdMs) {
                            lines.push("DELAY " + dt)
                        }
                    } else if (macroRecorderDialog.timingMode === "fixed") {
                        if (macroRecorderDialog.fixedDelayMs > 0) {
                            lines.push("DELAY " + macroRecorderDialog.fixedDelayMs)
                        }
                    }
                }

                if (ev.kind === "custom_delay") {
                    lines.push("DELAY " + Math.max(1, Math.round(ev.dx || 100)))
                } else if (ev.kind === "key_down") {
                    const nextEv = (i + 1 < events.length) ? events[i + 1] : null
                    const holdMs = nextEv ? Math.max(0, Math.round((nextEv.elapsed_ms || 0) - (ev.elapsed_ms || 0))) : 0
                    if (nextEv && nextEv.kind === "key_up" && nextEv.key === ev.key &&
                        (macroRecorderDialog.timingMode !== "exact" || holdMs < 95)) {
                        lines.push(ev.key)
                        prevElapsed = nextEv.elapsed_ms || prevElapsed
                        i++
                    } else {
                        lines.push("KEYDOWN " + ev.key)
                    }
                } else if (ev.kind === "key_up") {
                    lines.push("KEYUP " + ev.key)
                } else if (ev.kind === "mouse_click") {
                    lines.push(ev.key || "LMOUSE")
                } else if (ev.kind === "mouse_wheel") {
                    lines.push("MOUSE_SCROLL " + (ev.dx || 0) + " " + (ev.dy || 0))
                } else if (ev.kind === "mouse_move") {
                    let remX = Math.round(ev.dx || 0)
                    let remY = Math.round(ev.dy || 0)
                    while (remX !== 0 || remY !== 0) {
                        const stepX = Math.max(-127, Math.min(127, remX))
                        const stepY = Math.max(-127, Math.min(127, -remY))
                        lines.push("MOUSE_MOVE " + stepX + " " + stepY)
                        remX -= stepX
                        remY -= (-stepY)
                    }
                }
            }

            recordedScriptEditor.text = lines.join("\n")
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            // Header Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: macroRecorderDialog.isRecording ? Qt.alpha("#ef4444", 0.25) : Qt.alpha("#38bdf8", 0.18)
                    border.color: macroRecorderDialog.isRecording ? "#ef4444" : "#38bdf8"
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: macroRecorderDialog.isRecording ? "🔴" : "⏺"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("DuckyScript 3 Live Macro Recorder")
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Record keyboard strokes, key holds, mouse clicks, scroll wheel, and mouse movements directly into DuckyScript 3.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }

                // Start / Stop Recording Button
                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: macroRecorderDialog.isRecording
                        ? qsTr("⏹ Stop Recording (%1)").arg(macroRecorderDialog.rawEvents.length)
                        : qsTr("🔴 Start Recording")
                    highlighted: true
                    font.bold: true
                    implicitHeight: 34
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        } else {
                            macroRecorderDialog.startRecordingSession()
                        }
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("🧹 Clear")
                    enabled: macroRecorderDialog.rawEvents.length > 0 && !macroRecorderDialog.isRecording
                    implicitHeight: 34
                    onClicked: {
                        macroRecorderDialog.rawEvents = []
                        macroRecorderDialog.rebuildGeneratedDuckyScript()
                    }
                }
            }

            // Timing & Capture Filters Card
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: recOptionsCol.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1

                ColumnLayout {
                    id: recOptionsCol
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    // Row 1: Timing Mode Options
                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Timing Mode:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            height: 28
                            verticalAlignment: Text.AlignVCenter
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            text: qsTr("⏱ Exact Timings (Real Delays)")
                            highlighted: macroRecorderDialog.timingMode === "exact"
                            font.bold: macroRecorderDialog.timingMode === "exact"
                            onClicked: {
                                macroRecorderDialog.timingMode = "exact"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Inserts DELAY <ms> commands matching the exact millisecond intervals between your recorded actions")
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            text: qsTr("⚡ As Fast As Possible (No Delays)")
                            highlighted: macroRecorderDialog.timingMode === "fast"
                            font.bold: macroRecorderDialog.timingMode === "fast"
                            onClicked: {
                                macroRecorderDialog.timingMode = "fast"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Omits all DELAY commands between actions and collapses quick key taps so the macro runs at maximum hardware speed")
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            text: qsTr("📏 Fixed Step Delay")
                            highlighted: macroRecorderDialog.timingMode === "fixed"
                            font.bold: macroRecorderDialog.timingMode === "fixed"
                            onClicked: {
                                macroRecorderDialog.timingMode = "fixed"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                        }

                        QQC2.SpinBox {
                            font.pointSize: 10 * root.fontScale
                            visible: macroRecorderDialog.timingMode === "fixed"
                            from: 5
                            to: 2000
                            stepSize: 5
                            value: macroRecorderDialog.fixedDelayMs
                            onValueModified: {
                                macroRecorderDialog.fixedDelayMs = value
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                        }

                        QQC2.Label {
                            visible: macroRecorderDialog.timingMode === "fixed"
                            text: qsTr("ms")
                            font.pointSize: 10 * root.fontScale
                            height: 28
                            verticalAlignment: Text.AlignVCenter
                            color: Kirigami.Theme.disabledTextColor
                        }
                    }

                    // Row 2: Input Sources (Keyboard, Mouse Clicks/Scroll, Mouse Movements)
                    Flow {
                        Layout.fillWidth: true
                        spacing: 10

                        QQC2.Label {
                            text: qsTr("Include Inputs:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            height: 26
                            verticalAlignment: Text.AlignVCenter
                        }

                        HighContrastCheckBox {
                            checked: macroRecorderDialog.captureKeyboard
                            text: qsTr("⌨ Keyboard Keys (Taps & Holds)")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureKeyboard = checked
                        }

                        HighContrastCheckBox {
                            checked: macroRecorderDialog.captureMouseClicks
                            text: qsTr("🖱 Mouse Clicks & Scroll")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureMouseClicks = checked
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Captures LMOUSE, RMOUSE, MMOUSE, and MOUSE_SCROLL events")
                        }

                        HighContrastCheckBox {
                            checked: macroRecorderDialog.captureMouseMoves
                            text: qsTr("📐 Mouse Moves (MOUSE_MOVE)")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureMouseMoves = checked
                        }
                    }
                }
            }

            // Interactive Capture Area (Captures Keyboard + Mouse inside Window & Shows Global Evdev Status)
            Rectangle {
                id: captureFocusPad
                Layout.fillWidth: true
                Layout.preferredHeight: 96
                radius: 8
                focus: true
                activeFocusOnTab: true
                color: macroRecorderDialog.isRecording
                    ? Qt.alpha("#ef4444", 0.12)
                    : Qt.alpha(Kirigami.Theme.highlightColor, 0.08)
                border.color: macroRecorderDialog.isRecording
                    ? "#ef4444"
                    : (captureFocusPad.activeFocus ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.25))
                border.width: macroRecorderDialog.isRecording ? 2 : 1

                Keys.onPressed: (event) => {
                    if (!macroRecorderDialog.isRecording || event.isAutoRepeat) return
                    event.accepted = true
                    if (!macroRecorderDialog.captureKeyboard) return
                    if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                    const kName = macroRecorderDialog.qtKeyToDuckyName(event.key, event.text)
                    if (kName.length > 0) {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "key_down",
                            key: kName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }
                }

                Keys.onReleased: (event) => {
                    if (!macroRecorderDialog.isRecording || event.isAutoRepeat) return
                    event.accepted = true
                    if (!macroRecorderDialog.captureKeyboard) return
                    if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                    const kName = macroRecorderDialog.qtKeyToDuckyName(event.key, event.text)
                    if (kName.length > 0) {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "key_up",
                            key: kName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    acceptedButtons: Qt.AllButtons
                    cursorShape: macroRecorderDialog.isRecording ? Qt.CrossCursor : Qt.PointingHandCursor

                    onPressed: (mouse) => {
                        captureFocusPad.forceActiveFocus()
                        macroRecorderDialog.lastPadMouseX = mouse.x
                        macroRecorderDialog.lastPadMouseY = mouse.y
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseClicks) return
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        let btnName = "LMOUSE"
                        if (mouse.button === Qt.RightButton) btnName = "RMOUSE"
                        else if (mouse.button === Qt.MiddleButton) btnName = "MMOUSE"
                        else if (mouse.button === Qt.BackButton) btnName = "BMOUSE"
                        else if (mouse.button === Qt.ForwardButton) btnName = "FMOUSE"
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "mouse_click",
                            key: btnName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }

                    onWheel: (wheel) => {
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseClicks) return
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        const vStep = wheel.angleDelta.y > 0 ? 1 : (wheel.angleDelta.y < 0 ? -1 : 0)
                        const hStep = wheel.angleDelta.x > 0 ? 1 : (wheel.angleDelta.x < 0 ? -1 : 0)
                        if (vStep !== 0 || hStep !== 0) {
                            macroRecorderDialog.appendRecordedEvent({
                                kind: "mouse_wheel",
                                key: "MOUSE_SCROLL",
                                dx: hStep,
                                dy: vStep,
                                elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                            })
                        }
                    }

                    onPositionChanged: (mouse) => {
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseMoves) {
                            macroRecorderDialog.lastPadMouseX = mouse.x
                            macroRecorderDialog.lastPadMouseY = mouse.y
                            return
                        }
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        if (macroRecorderDialog.lastPadMouseX >= 0 && macroRecorderDialog.lastPadMouseY >= 0) {
                            const dx = Math.round(mouse.x - macroRecorderDialog.lastPadMouseX)
                            const dy = Math.round(mouse.y - macroRecorderDialog.lastPadMouseY)
                            if (Math.abs(dx) >= 2 || Math.abs(dy) >= 2) {
                                macroRecorderDialog.lastPadMouseX = mouse.x
                                macroRecorderDialog.lastPadMouseY = mouse.y
                                macroRecorderDialog.appendRecordedEvent({
                                    kind: "mouse_move",
                                    key: "MOUSE_MOVE",
                                    dx: dx,
                                    dy: dy,
                                    elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                                })
                            }
                        } else {
                            macroRecorderDialog.lastPadMouseX = mouse.x
                            macroRecorderDialog.lastPadMouseY = mouse.y
                        }
                    }
                }

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 4

                    Item { Layout.fillHeight: true }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        text: macroRecorderDialog.isRecording
                            ? qsTr("🔴 RECORDING LIVE — Type keys, click, scroll, or move the mouse here%1")
                                .arg(macroRecorderDialog.globalEvdevActive ? qsTr(" (or anywhere on desktop via evdev)") : "")
                            : qsTr("Click '🔴 Start Recording' then type keys or use mouse inside this Capture Pad")
                        font.bold: true
                        font.pointSize: 11 * root.fontScale
                        color: macroRecorderDialog.isRecording ? "#fca5a5" : Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        elide: Text.ElideRight
                        text: qsTr("Captured Events: %1 • Mode: %2")
                            .arg(macroRecorderDialog.rawEvents.length)
                            .arg(macroRecorderDialog.timingMode === "exact"
                                ? qsTr("Exact Timings")
                                : (macroRecorderDialog.timingMode === "fast" ? qsTr("As Fast As Possible") : qsTr("Fixed %1ms Delay").arg(macroRecorderDialog.fixedDelayMs)))
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    Item { Layout.fillHeight: true }
                }
            }

            // Generated DuckyScript 3 Preview Header + Quick Manual Insertions
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Generated DuckyScript 3 Output (Editable):")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    text: qsTr("+ DELAY 100")
                    implicitHeight: 26
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "custom_delay",
                            key: "DELAY",
                            dx: 100,
                            dy: 0,
                            elapsed_ms: macroRecorderDialog.rawEvents.length > 0
                                ? (macroRecorderDialog.rawEvents[macroRecorderDialog.rawEvents.length - 1].elapsed_ms || 0)
                                : 0
                        })
                    }
                }

                QQC2.Button {
                    text: qsTr("+ DELAY 500")
                    implicitHeight: 26
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "custom_delay",
                            key: "DELAY",
                            dx: 500,
                            dy: 0,
                            elapsed_ms: macroRecorderDialog.rawEvents.length > 0
                                ? (macroRecorderDialog.rawEvents[macroRecorderDialog.rawEvents.length - 1].elapsed_ms || 0)
                                : 0
                        })
                    }
                }
            }

            // Scrollable DuckyScript Preview Editor
            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: availableWidth
                QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                    orientation: Qt.Horizontal
                }

                QQC2.TextArea {
                    id: recordedScriptEditor
                    width: parent.width
                    font.family: "monospace"
                    font.pointSize: 10 * root.fontScale
                    selectByMouse: true
                    wrapMode: TextEdit.Wrap
                    clip: true
                    text: qsTr("// Click '🔴 Start Recording' above, then type keys or use the mouse.\n// Your recorded DuckyScript 3 macro will appear here in real time.")
                    background: Rectangle {
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                        border.width: 1
                    }
                }
            }

            // Footer Actions (Insert into Key Script / Replace Key Script)
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: root.hasSelectedSlot
                        ? qsTr("Target: %1 (%2)")
                            .arg(root.selectedSlotTitle)
                            .arg(root.editingReleaseScript ? qsTr("On Release") : qsTr("On Press"))
                        : qsTr("Select a key on the canvas to insert directly")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("Close")
                    onClicked: macroRecorderDialog.close()
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("📥 Append to Key Script")
                    enabled: root.hasSelectedSlot && recordedScriptEditor.text.trim().length > 0
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        }
                        const cur = scriptEditor.text.trim()
                        scriptEditor.text = cur.length > 0
                            ? (cur + "\n\n" + recordedScriptEditor.text.trim() + "\n")
                            : (recordedScriptEditor.text.trim() + "\n")
                        root.inspectorMode = "code"
                        macroRecorderDialog.close()
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("⚡ Replace Key Script")
                    highlighted: true
                    font.bold: true
                    enabled: root.hasSelectedSlot && recordedScriptEditor.text.trim().length > 0
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        }
                        scriptEditor.text = recordedScriptEditor.text.trim() + "\n"
                        root.inspectorMode = "code"
                        macroRecorderDialog.close()
                    }
                }
            }
        }
    }

    // =========================================================================
    // OLED Button Icon & Symbol Picker Window
    // =========================================================================
    Window {
        id: oledIconPickerDialog
        transientParent: root
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: oledIconPickerDialog.visible
            onActivated: oledIconPickerDialog.close()
        }
        title: qsTr("OLED Button Icon Picker — %1").arg(root.selectedSlotBadge)
        width: 680
        height: 540
        minimumWidth: 540
        minimumHeight: 420
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)
        color: Kirigami.Theme.backgroundColor

        // "inspector" updates label1Input/label2Input directly; "keyProps" updates keyPropertiesWindow.keyPropLabel1Field/keyPropertiesWindow.keyPropLabel2Field
        property string targetSource: "inspector"
        property int placementMode: 0 // 0 = Icon (L1) + Short Label (L2), 1 = Big Icon Only (L1, clear L2), 2 = Set Line 1, 3 = Set Line 2, 4 = Prepend to Line 1
        property string selectedCategory: "All"
        property string searchQuery: ""

        readonly property bool targetSupportsTwoLines: targetSource === "keyProps"
            ? (keyPropertiesWindow.slotNum <= 20)
            : (root.selectedSlotNumber <= 20)

        readonly property var categories: [
            "All",
            "Web Browsers",
            "Word & Docs",
            "Excel & Sheets",
            "PowerPoint & Slides",
            "Photoshop & Creative",
            "FreeCAD & Fusion 360",
            "Editing & Clipboard",
            "Arrows & Nav",
            "Media & Audio",
            "Coding & Terminal",
            "Home Automation",
            "Window & System",
            "Streaming & Apps",
            "Symbols & Shapes"
        ]

        readonly property var iconCatalog: [
            // Web Browsers (Chrome, Firefox, Edge, Brave, Safari)
            { icon: "🌐", label: "WEB", name: "Browser / Open Website / URL", cat: "Web Browsers" },
            { icon: "🧭", label: "ADDR", name: "Address Bar / Navigate / Compass", cat: "Web Browsers" },
            { icon: "📑", label: "TAB+", name: "New Tab / Duplicate Tab / Bookmark Tabs", cat: "Web Browsers" },
            { icon: "🗂", label: "TABS", name: "Tab Groups / Tab Overview / Workspaces", cat: "Web Browsers" },
            { icon: "🔄", label: "RELD", name: "Reload / Hard Refresh Page", cat: "Web Browsers" },
            { icon: "🔖", label: "BMRK", name: "Bookmark Page / Favorites", cat: "Web Browsers" },
            { icon: "🕵", label: "PRIV", name: "Incognito / Private Browsing Window", cat: "Web Browsers" },
            { icon: "📥", label: "DL", name: "Downloads / Save Page As", cat: "Web Browsers" },
            { icon: "📤", label: "SHAR", name: "Share / Upload / Send Link", cat: "Web Browsers" },
            { icon: "📜", label: "HIST", name: "Browsing History / Recently Closed", cat: "Web Browsers" },
            { icon: "🧩", label: "EXT", name: "Browser Extensions / Add-ons / Plugins", cat: "Web Browsers" },
            { icon: "🔎", label: "INSP", name: "DevTools Inspect / Find in Page", cat: "Web Browsers" },
            { icon: "⛶", label: "FULL", name: "Fullscreen Mode (F11)", cat: "Web Browsers" },
            { icon: "🖨", label: "PRNT", name: "Print Page / Save as PDF", cat: "Web Browsers" },
            { icon: "🍪", label: "COOK", name: "Clear Cookies / Site Data", cat: "Web Browsers" },
            { icon: "📰", label: "READ", name: "Reader View / Simplified Article", cat: "Web Browsers" },

            // Word & Docs (Microsoft Word, Google Docs, LibreOffice Writer, Notion)
            { icon: "📄", label: "NEW", name: "New Document / Blank Page", cat: "Word & Docs" },
            { icon: "📝", label: "DOC", name: "Edit Document / Rich Text Note", cat: "Word & Docs" },
            { icon: "🔤", label: "FONT", name: "Font Family & Size Formatting", cat: "Word & Docs" },
            { icon: "🅱", label: "BOLD", name: "Bold Text (Ctrl+B)", cat: "Word & Docs" },
            { icon: "ℹ", label: "ITAL", name: "Italic / Info Style (Ctrl+I)", cat: "Word & Docs" },
            { icon: "¶", label: "PARA", name: "Paragraph / Show Formatting Marks", cat: "Word & Docs" },
            { icon: "§", label: "SECT", name: "Section Break / Page Layout", cat: "Word & Docs" },
            { icon: "☰", label: "ALGN", name: "Align Text / Justify Paragraph", cat: "Word & Docs" },
            { icon: "•", label: "BULT", name: "Bulleted List", cat: "Word & Docs" },
            { icon: "🔢", label: "NUMS", name: "Numbered / Multilevel List", cat: "Word & Docs" },
            { icon: "🔠", label: "UPPR", name: "Uppercase / Change Case", cat: "Word & Docs" },
            { icon: "🔡", label: "LOWR", name: "Lowercase / Small Caps", cat: "Word & Docs" },
            { icon: "💬", label: "CMNT", name: "Insert Comment / Review Track Changes", cat: "Word & Docs" },
            { icon: "👁", label: "VIEW", name: "Read Mode / Print Preview", cat: "Word & Docs" },
            { icon: "📖", label: "DICT", name: "Spell Check / Thesaurus / Word Count", cat: "Word & Docs" },
            { icon: "🏷", label: "STYL", name: "Heading / Quick Style Tag", cat: "Word & Docs" },
            { icon: "📎", label: "ATCH", name: "Insert Attachment / Embedded Object", cat: "Word & Docs" },
            { icon: "✒", label: "SIGN", name: "Insert Footnote / Signature / Nib", cat: "Word & Docs" },

            // Excel & Sheets (Microsoft Excel, Google Sheets, LibreOffice Calc)
            { icon: "📊", label: "CHRT", name: "Insert Bar / Column / Pivot Chart", cat: "Excel & Sheets" },
            { icon: "📈", label: "TRND", name: "Line Chart / Trendline / Sparkline", cat: "Excel & Sheets" },
            { icon: "📉", label: "VARI", name: "Variance / Drawdown Chart", cat: "Excel & Sheets" },
            { icon: "∑", label: "SUM", name: "AutoSum / Sigma Formula (Alt+=)", cat: "Excel & Sheets" },
            { icon: "ƒ", label: "FX", name: "Insert Function / Formula Bar (fx)", cat: "Excel & Sheets" },
            { icon: "🧮", label: "CALC", name: "Calculate Sheet / Pivot Table", cat: "Excel & Sheets" },
            { icon: "⊞", label: "GRID", name: "Table Grid / All Cell Borders", cat: "Excel & Sheets" },
            { icon: "⊟", label: "UNGR", name: "Ungroup / Collapse Rows & Columns", cat: "Excel & Sheets" },
            { icon: "∇", label: "FILT", name: "Data Filter / AutoFilter Funnel", cat: "Excel & Sheets" },
            { icon: "⇅", label: "SORT", name: "Sort Ascending / Descending", cat: "Excel & Sheets" },
            { icon: "💲", label: "CURR", name: "Currency / Accounting Number Format", cat: "Excel & Sheets" },
            { icon: "％", label: "PCT", name: "Percentage Number Format", cat: "Excel & Sheets" },
            { icon: "🧊", label: "FRZE", name: "Freeze Panes / Top Row", cat: "Excel & Sheets" },
            { icon: "⇔", label: "CFIT", name: "AutoFit Column Width / Merge Across", cat: "Excel & Sheets" },
            { icon: "⇕", label: "RFIT", name: "AutoFit Row Height / Insert Row", cat: "Excel & Sheets" },

            // PowerPoint & Slides (PowerPoint, Keynote, Google Slides)
            { icon: "📽", label: "SHOW", name: "Start Slide Show Presentation (F5)", cat: "PowerPoint & Slides" },
            { icon: "🖼", label: "SLID", name: "New Slide / Insert Picture", cat: "PowerPoint & Slides" },
            { icon: "🎭", label: "ANIM", name: "Animations & Slide Transitions", cat: "PowerPoint & Slides" },
            { icon: "🪄", label: "DSGN", name: "Design Ideas / Auto-Layout Magic Wand", cat: "PowerPoint & Slides" },
            { icon: "📐", label: "MSTR", name: "Slide Master / Guides & Set Square", cat: "PowerPoint & Slides" },
            { icon: "🎙", label: "NARR", name: "Record Slide Narration / Voiceover", cat: "PowerPoint & Slides" },
            { icon: "👉", label: "LASR", name: "Laser Pointer / Presenter Highlight", cat: "PowerPoint & Slides" },
            { icon: "▦", label: "SRTR", name: "Slide Sorter Grid View", cat: "PowerPoint & Slides" },
            { icon: "🗒", label: "NOTE", name: "Speaker Notes / Presenter View", cat: "PowerPoint & Slides" },
            { icon: "⧉", label: "DUPL", name: "Duplicate Slide / Group Objects (Ctrl+D)", cat: "PowerPoint & Slides" },
            { icon: "⬛", label: "BLCK", name: "Blackout Presentation Screen (B)", cat: "PowerPoint & Slides" },
            { icon: "⬜", label: "WHTE", name: "Whiteout Presentation Screen (W)", cat: "PowerPoint & Slides" },

            // Photoshop & Creative (Photoshop, Illustrator, Lightroom, GIMP, Krita)
            { icon: "🎨", label: "CLR", name: "Color Picker / Swatches Palette", cat: "Photoshop & Creative" },
            { icon: "🖌", label: "BRSH", name: "Brush Tool / Paint Stroke (B)", cat: "Photoshop & Creative" },
            { icon: "✏", label: "PENC", name: "Pencil / Freehand Draw Tool", cat: "Photoshop & Creative" },
            { icon: "🖍", label: "FILL", name: "Paint Bucket / Gradient Fill (G)", cat: "Photoshop & Creative" },
            { icon: "🧽", label: "ERAS", name: "Eraser Tool / Clear Pixels (E)", cat: "Photoshop & Creative" },
            { icon: "◩", label: "CROP", name: "Crop / Trim Canvas Tool (C)", cat: "Photoshop & Creative" },
            { icon: "⬚", label: "MARQ", name: "Rectangular Marquee Selection (M)", cat: "Photoshop & Creative" },
            { icon: "➰", label: "LASO", name: "Lasso / Freeform Selection Tool (L)", cat: "Photoshop & Creative" },
            { icon: "🧪", label: "DROP", name: "Eyedropper / Color Sampler (I)", cat: "Photoshop & Creative" },
            { icon: "🩹", label: "HEAL", name: "Spot Healing / Patch Brush (J)", cat: "Photoshop & Creative" },
            { icon: "💠", label: "CLON", name: "Clone Stamp / Pattern Stamp (S)", cat: "Photoshop & Creative" },
            { icon: "📚", label: "LAYR", name: "Layers Panel / Merge Layers (F7)", cat: "Photoshop & Creative" },
            { icon: "◐", label: "MASK", name: "Layer Mask / Contrast / Invert", cat: "Photoshop & Creative" },
            { icon: "🔆", label: "EXPO", name: "Exposure / Levels / Curves Adjustment", cat: "Photoshop & Creative" },
            { icon: "✋", label: "HAND", name: "Hand Tool / Pan Canvas (H)", cat: "Photoshop & Creative" },
            { icon: "🅰", label: "TEXT", name: "Type / Artistic Text Tool (T)", cat: "Photoshop & Creative" },

            // FreeCAD & Fusion 360 (FreeCAD, Fusion 360, SolidWorks, Blender, Slicers)
            { icon: "⬡", label: "BODY", name: "3D Solid Body / Part Design Workbench", cat: "FreeCAD & Fusion 360" },
            { icon: "⬢", label: "MESH", name: "STL Mesh / Surface Body / Tessellate", cat: "FreeCAD & Fusion 360" },
            { icon: "✎", label: "SKCH", name: "Create 2D Sketch / Edit Sketch", cat: "FreeCAD & Fusion 360" },
            { icon: "📏", label: "DIM", name: "Sketch Dimension / Measure Distance (D)", cat: "FreeCAD & Fusion 360" },
            { icon: "⌖", label: "ORIG", name: "Origin / Center Point / Datum Crosshair", cat: "FreeCAD & Fusion 360" },
            { icon: "⏫", label: "EXTR", name: "Extrude / Pad Sketch Profile (E)", cat: "FreeCAD & Fusion 360" },
            { icon: "⏬", label: "PKT", name: "Pocket / Cut / Hole Feature", cat: "FreeCAD & Fusion 360" },
            { icon: "↻", label: "REVL", name: "Revolve Feature / Orbit 3D View", cat: "FreeCAD & Fusion 360" },
            { icon: "◡", label: "FLLT", name: "Fillet / Round Edge (F)", cat: "FreeCAD & Fusion 360" },
            { icon: "⟋", label: "CHMF", name: "Chamfer / Bevel Edge", cat: "FreeCAD & Fusion 360" },
            { icon: "∥", label: "PRLL", name: "Parallel Sketch Constraint", cat: "FreeCAD & Fusion 360" },
            { icon: "⊥", label: "PERP", name: "Perpendicular / Normal To Sketch Plane", cat: "FreeCAD & Fusion 360" },
            { icon: "⊙", label: "CONC", name: "Concentric Constraint / Center Circle (C)", cat: "FreeCAD & Fusion 360" },
            { icon: "◰", label: "ISO", name: "Isometric View / Fit All (F6)", cat: "FreeCAD & Fusion 360" },
            { icon: "∷", label: "PATN", name: "Rectangular / Circular Pattern Array", cat: "FreeCAD & Fusion 360" },
            { icon: "🐚", label: "SHLL", name: "Shell / Hollow Wall Thickness", cat: "FreeCAD & Fusion 360" },
            { icon: "⛓", label: "JNT", name: "Assembly Joint / Mate Constraint (J)", cat: "FreeCAD & Fusion 360" },
            { icon: "🧱", label: "SLCE", name: "Section Analysis / Slicer Layer Preview", cat: "FreeCAD & Fusion 360" },

            // Editing & Clipboard
            { icon: "✂", label: "CUT", name: "Cut / Scissors (Ctrl+X)", cat: "Editing & Clipboard" },
            { icon: "📋", label: "COPY", name: "Copy / Clipboard (Ctrl+C)", cat: "Editing & Clipboard" },
            { icon: "📌", label: "PASTE", name: "Paste / Pin Clipboard (Ctrl+V)", cat: "Editing & Clipboard" },
            { icon: "💾", label: "SAVE", name: "Save File / Floppy (Ctrl+S)", cat: "Editing & Clipboard" },
            { icon: "📁", label: "OPEN", name: "Open Folder / Browse File (Ctrl+O)", cat: "Editing & Clipboard" },
            { icon: "🔍", label: "FIND", name: "Search / Zoom / Find & Replace (Ctrl+F)", cat: "Editing & Clipboard" },
            { icon: "🗑", label: "DEL", name: "Delete / Trash", cat: "Editing & Clipboard" },
            { icon: "📷", label: "SHOT", name: "Screenshot / Capture Region", cat: "Editing & Clipboard" },
            { icon: "🔗", label: "LINK", name: "Insert Hyperlink / Copy URL (Ctrl+K)", cat: "Editing & Clipboard" },
            { icon: "↩", label: "UNDO", name: "Undo Action (Ctrl+Z)", cat: "Editing & Clipboard" },
            { icon: "↪", label: "REDO", name: "Redo Action (Ctrl+Y)", cat: "Editing & Clipboard" },
            { icon: "↺", label: "RESET", name: "Revert / Reset Changes", cat: "Editing & Clipboard" },

            // Arrows & Nav
            { icon: "↑", label: "UP", name: "Arrow Up", cat: "Arrows & Nav" },
            { icon: "↓", label: "DOWN", name: "Arrow Down", cat: "Arrows & Nav" },
            { icon: "←", label: "LEFT", name: "Arrow Left", cat: "Arrows & Nav" },
            { icon: "→", label: "RIGHT", name: "Arrow Right", cat: "Arrows & Nav" },
            { icon: "↖", label: "HOME", name: "Home / Top-Left", cat: "Arrows & Nav" },
            { icon: "↘", label: "END", name: "End / Bottom-Right", cat: "Arrows & Nav" },
            { icon: "▲", label: "PGUP", name: "Triangle Up / Page Up", cat: "Arrows & Nav" },
            { icon: "▼", label: "PGDN", name: "Triangle Down / Page Down", cat: "Arrows & Nav" },
            { icon: "◀", label: "BACK", name: "Navigate Back / Previous", cat: "Arrows & Nav" },
            { icon: "▷", label: "FWD", name: "Navigate Forward / Step Next", cat: "Arrows & Nav" },
            { icon: "⇄", label: "SWAP", name: "Swap Left / Right", cat: "Arrows & Nav" },
            { icon: "⤢", label: "MAX", name: "Expand / Diagonal Resize", cat: "Arrows & Nav" },

            // Media & Audio
            { icon: "▶", label: "PLAY", name: "Play Media", cat: "Media & Audio" },
            { icon: "⏸", label: "PAUSE", name: "Pause Media", cat: "Media & Audio" },
            { icon: "⏯", label: "PLPA", name: "Play / Pause Toggle", cat: "Media & Audio" },
            { icon: "⏹", label: "STOP", name: "Stop Playback", cat: "Media & Audio" },
            { icon: "⏭", label: "NEXT", name: "Next Track", cat: "Media & Audio" },
            { icon: "⏮", label: "PREV", name: "Previous Track", cat: "Media & Audio" },
            { icon: "⏩", label: "FFWD", name: "Fast Forward", cat: "Media & Audio" },
            { icon: "⏪", label: "RWND", name: "Rewind", cat: "Media & Audio" },
            { icon: "🔊", label: "VOL+", name: "Volume Up", cat: "Media & Audio" },
            { icon: "🔉", label: "VOL-", name: "Volume Down", cat: "Media & Audio" },
            { icon: "🔇", label: "MUTE", name: "Mute Audio", cat: "Media & Audio" },
            { icon: "🎤", label: "MIC", name: "Microphone Mute / Push-to-Talk", cat: "Media & Audio" },
            { icon: "🎧", label: "HEAD", name: "Headphones / Audio Output", cat: "Media & Audio" },
            { icon: "🎵", label: "MUSIC", name: "Music Note", cat: "Media & Audio" },
            { icon: "🎶", label: "SONG", name: "Music Notes / Playlist", cat: "Media & Audio" },
            { icon: "⏺", label: "REC", name: "Record Audio / Video", cat: "Media & Audio" },

            // Coding & Terminal
            { icon: "⌨", label: "TERM", name: "Terminal / Console / Shell", cat: "Coding & Terminal" },
            { icon: "💻", label: "IDE", name: "Code Editor / VS Code / IDE", cat: "Coding & Terminal" },
            { icon: "⚡", label: "BUILD", name: "Build / Compile / Flash", cat: "Coding & Terminal" },
            { icon: "🐞", label: "DEBUG", name: "Debug / Breakpoint / Bug", cat: "Coding & Terminal" },
            { icon: "🚀", label: "RUN", name: "Run / Deploy / Launch", cat: "Coding & Terminal" },
            { icon: "🔧", label: "TOOL", name: "Wrench / Refactor / Lint", cat: "Coding & Terminal" },
            { icon: "⚙", label: "PREF", name: "Settings / Configuration Gear", cat: "Coding & Terminal" },
            { icon: "📦", label: "PKG", name: "Package / Cargo / NPM Build", cat: "Coding & Terminal" },
            { icon: "🌿", label: "GIT", name: "Git Branch / Commit / Merge", cat: "Coding & Terminal" },
            { icon: "☁", label: "PUSH", name: "Git Push / Cloud Sync", cat: "Coding & Terminal" },
            { icon: "🐳", label: "DCKR", name: "Docker Container / Compose", cat: "Coding & Terminal" },
            { icon: "🔑", label: "SSH", name: "SSH Key / Auth Token / Secret", cat: "Coding & Terminal" },
            { icon: "🤖", label: "AI", name: "AI Assistant / Copilot / Agent", cat: "Coding & Terminal" },
            { icon: "⌘", label: "CMD", name: "Command Palette / Super Key", cat: "Coding & Terminal" },

            // Home Automation
            { icon: "💡", label: "BULB", name: "Light / Lamp / Bulb", cat: "Home Automation" },
            { icon: "🔌", label: "PLUG", name: "Smart Plug / Socket", cat: "Home Automation" },
            { icon: "🌡", label: "TEMP", name: "Thermostat / Temperature", cat: "Home Automation" },
            { icon: "🔥", label: "HEAT", name: "Heater / Warmth", cat: "Home Automation" },
            { icon: "❄", label: "COOL", name: "AC / Air Conditioner / Cooling", cat: "Home Automation" },
            { icon: "🌀", label: "FAN", name: "Ceiling Fan / Ventilation", cat: "Home Automation" },
            { icon: "🔒", label: "LOCK", name: "Lock / Armed", cat: "Home Automation" },
            { icon: "🔓", label: "UNLCK", name: "Unlock / Disarmed", cat: "Home Automation" },
            { icon: "🛡", label: "ALRM", name: "Security Alarm / Shield", cat: "Home Automation" },
            { icon: "📹", label: "CAM", name: "Security Camera / CCTV", cat: "Home Automation" },
            { icon: "🚪", label: "DOOR", name: "Door / Contact Sensor", cat: "Home Automation" },
            { icon: "🪟", label: "WNDW", name: "Window / Blinds / Shade", cat: "Home Automation" },
            { icon: "🧹", label: "VACC", name: "Robot Vacuum / Cleaner", cat: "Home Automation" },
            { icon: "💧", label: "LEAK", name: "Water / Flood / Leak Sensor", cat: "Home Automation" },
            { icon: "🚨", label: "SIREN", name: "Siren / Emergency Strobe", cat: "Home Automation" },
            { icon: "🏠", label: "HOME", name: "Home / Occupied Scene", cat: "Home Automation" },
            { icon: "🌙", label: "NIGHT", name: "Night / Goodnight Scene", cat: "Home Automation" },
            { icon: "☀", label: "DAY", name: "Morning / Daytime Scene", cat: "Home Automation" },
            { icon: "🚗", label: "EV", name: "EV Charger / Garage Door", cat: "Home Automation" },

            // Window & System
            { icon: "🖥", label: "DESK", name: "Desktop / Monitor Switch", cat: "Window & System" },
            { icon: "▣", label: "TILE", name: "Tile Window / Snap Layout", cat: "Window & System" },
            { icon: "⏻", label: "PWR", name: "System Power / Shutdown / Reboot", cat: "Window & System" },
            { icon: "💤", label: "SLEEP", name: "Sleep / Suspend System", cat: "Window & System" },
            { icon: "🔅", label: "BRITE", name: "Screen Brightness / Night Light", cat: "Window & System" },
            { icon: "🔋", label: "BATT", name: "Battery / Power Profile", cat: "Window & System" },
            { icon: "📡", label: "WIFI", name: "Network / Wi-Fi Toggle", cat: "Window & System" },
            { icon: "⏱", label: "TIMER", name: "Stopwatch / Macro Delay", cat: "Window & System" },
            { icon: "🔔", label: "BELL", name: "Notifications / Do Not Disturb", cat: "Window & System" },
            { icon: "⏏", label: "EJECT", name: "Eject Drive / Safe Unmount", cat: "Window & System" },

            // Streaming & Apps
            { icon: "🎥", label: "SCENE", name: "OBS Scene / Camera Switch", cat: "Streaming & Apps" },
            { icon: "📺", label: "LIVE", name: "Go Live / Stream Monitor", cat: "Streaming & Apps" },
            { icon: "🎬", label: "CLIP", name: "Save Replay Clip / Marker", cat: "Streaming & Apps" },
            { icon: "🗨", label: "CHAT", name: "Stream Chat / Discord Message", cat: "Streaming & Apps" },
            { icon: "🎮", label: "GAME", name: "Game Capture / Controller", cat: "Streaming & Apps" },
            { icon: "📻", label: "AUDIO", name: "Audio Mixer / Radio Stream", cat: "Streaming & Apps" },
            { icon: "📧", label: "MAIL", name: "Email Client / Compose Mail", cat: "Streaming & Apps" },
            { icon: "📅", label: "CAL", name: "Calendar / New Meeting", cat: "Streaming & Apps" },
            { icon: "🛒", label: "SHOP", name: "Shopping / Quick Order", cat: "Streaming & Apps" },

            // Symbols & Shapes
            { icon: "★", label: "STAR", name: "Solid Star / Favorite", cat: "Symbols & Shapes" },
            { icon: "☆", label: "FAV", name: "Outline Star / Bookmark", cat: "Symbols & Shapes" },
            { icon: "♥", label: "LIKE", name: "Heart / Like", cat: "Symbols & Shapes" },
            { icon: "✔", label: "OK", name: "Checkmark / Confirm / Apply", cat: "Symbols & Shapes" },
            { icon: "✖", label: "CLOSE", name: "Cross / Close / Cancel", cat: "Symbols & Shapes" },
            { icon: "➕", label: "ADD", name: "Plus / Zoom In / New Item", cat: "Symbols & Shapes" },
            { icon: "➖", label: "SUB", name: "Minus / Zoom Out / Remove", cat: "Symbols & Shapes" },
            { icon: "⚠", label: "WARN", name: "Warning / Alert", cat: "Symbols & Shapes" },
            { icon: "💎", label: "PRO", name: "Diamond / Macro Preset", cat: "Symbols & Shapes" },
            { icon: "🎯", label: "AIM", name: "Target / Focus Mode", cat: "Symbols & Shapes" },
            { icon: "∞", label: "LOOP", name: "Infinity / Repeat Loop", cat: "Symbols & Shapes" },
            { icon: "❓", label: "HELP", name: "Help / Shortcut Reference (F1)", cat: "Symbols & Shapes" }
        ]

        readonly property var filteredIcons: {
            const q = searchQuery.trim().toLowerCase()
            const cat = selectedCategory
            const out = []
            for (let i = 0; i < iconCatalog.length; i++) {
                const item = iconCatalog[i]
                if (cat !== "All" && item.cat !== cat) continue
                if (q.length > 0) {
                    const hay = (item.name + " " + item.label + " " + item.cat + " " + item.icon).toLowerCase()
                    if (hay.indexOf(q) === -1) continue
                }
                out.push(item)
            }
            return out
        }

        function openForInspector(defaultLine) {
            targetSource = "inspector"
            if (!targetSupportsTwoLines) {
                placementMode = 2 // Set Line 1
            } else if (defaultLine === 2) {
                placementMode = 3 // Set Line 2
            } else {
                placementMode = 0 // Icon (Line 1) + Short Label (Line 2)
            }
            show()
            raise()
            requestActivate()
            iconSearchInput.forceActiveFocus()
        }

        function openForKeyProperties(lineNum) {
            targetSource = "keyProps"
            if (!targetSupportsTwoLines) {
                placementMode = 2
            } else if (lineNum === 2) {
                placementMode = 3
            } else {
                placementMode = 2
            }
            show()
            raise()
            requestActivate()
            iconSearchInput.forceActiveFocus()
        }

        function currentLine1Text() {
            if (targetSource === "keyProps") {
                return (keyPropertiesWindow && keyPropertiesWindow.keyPropLabel1Field) ? keyPropertiesWindow.keyPropLabel1Field.text : ""
            }
            return label1Input ? label1Input.text : ""
        }

        function currentLine2Text() {
            if (targetSource === "keyProps") {
                return (keyPropertiesWindow && keyPropertiesWindow.keyPropLabel2Field) ? keyPropertiesWindow.keyPropLabel2Field.text : ""
            }
            return label2Input ? label2Input.text : ""
        }

        function applyValues(newL1, newL2) {
            const c1 = root.clampOledLabel(newL1)
            const c2 = root.clampOledLabel(newL2)
            if (targetSource === "keyProps") {
                keyPropertiesWindow.keyPropLabel1Field.text = c1
                if (targetSupportsTwoLines) {
                    keyPropertiesWindow.keyPropLabel2Field.text = c2
                }
            } else {
                label1Input.text = c1
                if (targetSupportsTwoLines) {
                    label2Input.text = c2
                }
                liveSlotAutoSaveTimer.stop()
                if (root.hasSelectedSlot && !root.loadingSlotData && root.compileOk) {
                    root.saveCurrentSlotToController()
                    hardwareCanvas.syncSlotsFromController()
                    root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                } else {
                    liveSlotAutoSaveTimer.restart()
                }
            }
        }

        function applyIconItem(item) {
            if (!item) return
            const cur1 = currentLine1Text()
            const cur2 = currentLine2Text()
            if (!targetSupportsTwoLines) {
                if (placementMode === 4) {
                    applyValues(root.clampOledLabel(item.icon + cur1.trim()), "")
                } else {
                    applyValues(item.icon, "")
                }
                return
            }

            if (placementMode === 0) {
                // Icon on Line 1 + Short Label on Line 2 (or keep existing L2 if user already typed custom text)
                const nextL2 = cur2.trim().length > 0 && !root.isSingleIconLabel(cur2)
                    ? cur2
                    : item.label
                applyValues(item.icon, nextL2)
            } else if (placementMode === 1) {
                // Big Icon Only (Line 1 = icon, Line 2 = empty)
                applyValues(item.icon, "")
            } else if (placementMode === 2) {
                // Set Line 1 Only
                applyValues(item.icon, cur2)
            } else if (placementMode === 3) {
                // Set Line 2 Only
                applyValues(cur1, item.icon)
            } else if (placementMode === 4) {
                // Prepend Icon to Line 1 (e.g. "🔊VOL+")
                const stripped = root.isSingleIconLabel(cur1) ? "" : cur1.trim()
                const fallback = stripped.length > 0 ? stripped : item.label
                applyValues(root.clampOledLabel(item.icon + fallback), cur2)
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 12
            spacing: 10

            // Top Header & Live OLED Key Preview Card
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: headerRow.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                border.width: 1

                RowLayout {
                    id: headerRow
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 12

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 3

                        QQC2.Label {
                            text: qsTr("OLED Button Icon & Symbol Picker (%1)").arg(root.selectedSlotBadge)
                            font.bold: true
                            font.pointSize: 11 * root.fontScale
                            Layout.fillWidth: true
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("Click any icon below to assign it. Desktop OLED & Studio render the graphical symbol, and Sync to SD automatically writes the 5-char 7-bit ASCII glyph for the physical duckyPad OLED.")
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }
                    }

                    // Live 1:1 Desktop OLED Preview Box
                    ColumnLayout {
                        spacing: 2
                        QQC2.Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("Desktop OLED")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        Rectangle {
                            Layout.preferredWidth: 82
                            Layout.preferredHeight: 48
                            radius: 5
                            color: "#05070a"
                            border.color: "#38bdf8"
                            border.width: 1.5

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 0

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#ffffff"
                                    font.family: root.isSingleIconLabel(oledIconPickerDialog.currentLine1Text()) ? "sans-serif" : "monospace"
                                    font.bold: true
                                    font.pointSize: {
                                        const l1 = oledIconPickerDialog.currentLine1Text()
                                        const l2 = oledIconPickerDialog.currentLine2Text()
                                        if (root.isSingleIconLabel(l1)) {
                                            return ((!oledIconPickerDialog.targetSupportsTwoLines || l2.trim().length === 0) ? 14 : 12) * root.fontScale
                                        }
                                        return 10 * root.fontScale
                                    }
                                    text: oledIconPickerDialog.currentLine1Text().length > 0
                                        ? oledIconPickerDialog.currentLine1Text()
                                        : "—"
                                }

                                Text {
                                    visible: oledIconPickerDialog.targetSupportsTwoLines && oledIconPickerDialog.currentLine2Text().trim().length > 0
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 15
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#d8f3ff"
                                    font.family: root.isSingleIconLabel(oledIconPickerDialog.currentLine2Text()) ? "sans-serif" : "monospace"
                                    font.bold: true
                                    font.pointSize: (root.isSingleIconLabel(oledIconPickerDialog.currentLine2Text()) ? 11 : 10) * root.fontScale
                                    text: oledIconPickerDialog.currentLine2Text()
                                }
                            }
                        }
                    }

                    // Live Physical duckyPad 6x8 ASCII OLED Preview Box
                    ColumnLayout {
                        spacing: 2
                        QQC2.Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("duckyPad HW")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        Rectangle {
                            Layout.preferredWidth: 82
                            Layout.preferredHeight: 48
                            radius: 5
                            color: "#05070a"
                            border.color: "#94a3b8"
                            border.width: 1.5

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 0

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#ffffff"
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: {
                                        const hw1 = deviceController.oledLabelToHardwareAscii(oledIconPickerDialog.currentLine1Text())
                                        return hw1.length > 0 ? hw1 : "—"
                                    }
                                }

                                Text {
                                    visible: oledIconPickerDialog.targetSupportsTwoLines && oledIconPickerDialog.currentLine2Text().trim().length > 0
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 15
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#d8f3ff"
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: deviceController.oledLabelToHardwareAscii(oledIconPickerDialog.currentLine2Text())
                                }
                            }
                        }
                    }
                }
            }

            // Placement Mode + Search & Category Filter Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Apply Mode:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.ComboBox {
                    font.pointSize: 10 * root.fontScale
                    id: iconPlacementCombo
                    Layout.preferredWidth: 220
                    model: oledIconPickerDialog.targetSupportsTwoLines
                        ? [
                            qsTr("Icon (L1) + Short Label (L2)"),
                            qsTr("Big Icon Only (L1, clear L2)"),
                            qsTr("Set Line 1 Only"),
                            qsTr("Set Line 2 Only"),
                            qsTr("Icon + Text on Line 1")
                        ]
                        : [
                            qsTr("Set Icon Only"),
                            qsTr("Icon + Text (5 chars)")
                        ]
                    currentIndex: {
                        if (!oledIconPickerDialog.targetSupportsTwoLines) {
                            return oledIconPickerDialog.placementMode === 4 ? 1 : 0
                        }
                        return oledIconPickerDialog.placementMode
                    }
                    onActivated: function(index) {
                        if (!oledIconPickerDialog.targetSupportsTwoLines) {
                            oledIconPickerDialog.placementMode = (index === 1) ? 4 : 2
                        } else {
                            oledIconPickerDialog.placementMode = index
                        }
                    }
                }

                QQC2.ComboBox {
                    font.pointSize: 10 * root.fontScale
                    id: iconCategoryCombo
                    Layout.preferredWidth: 165
                    model: oledIconPickerDialog.categories
                    currentIndex: Math.max(0, oledIconPickerDialog.categories.indexOf(oledIconPickerDialog.selectedCategory))
                    onActivated: function(index) {
                        oledIconPickerDialog.selectedCategory = oledIconPickerDialog.categories[index]
                    }
                }

                QQC2.TextField {
                    font.pointSize: 10 * root.fontScale
                    id: iconSearchInput
                    Layout.fillWidth: true
                    placeholderText: qsTr("Search icons (e.g. play, vol, git, arrow)...")
                    selectByMouse: true
                    onTextChanged: oledIconPickerDialog.searchQuery = text
                }
            }

            // Direct Line 1 / Line 2 Editable Preview Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Line 1:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.TextField {
                    font.pointSize: 10 * root.fontScale
                    Layout.preferredWidth: Math.max(120, Math.round(120 * root.fontScale))
                    implicitHeight: Math.max(30, Math.round(30 * root.fontScale))
                    verticalAlignment: TextInput.AlignVCenter
                    leftPadding: 8
                    rightPadding: 8
                    maximumLength: 10
                    font.family: "monospace"
                    font.bold: true
                    selectByMouse: true
                    text: oledIconPickerDialog.currentLine1Text()
                    onTextEdited: {
                        oledIconPickerDialog.applyValues(text, oledIconPickerDialog.currentLine2Text())
                    }
                }

                QQC2.Label {
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    text: qsTr("Line 2:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.TextField {
                    font.pointSize: 10 * root.fontScale
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    Layout.preferredWidth: Math.max(120, Math.round(120 * root.fontScale))
                    implicitHeight: Math.max(30, Math.round(30 * root.fontScale))
                    verticalAlignment: TextInput.AlignVCenter
                    leftPadding: 8
                    rightPadding: 8
                    maximumLength: 10
                    font.family: "monospace"
                    selectByMouse: true
                    text: oledIconPickerDialog.currentLine2Text()
                    onTextEdited: {
                        oledIconPickerDialog.applyValues(oledIconPickerDialog.currentLine1Text(), text)
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    text: qsTr("⇅ Swap L1/L2")
                    onClicked: {
                        oledIconPickerDialog.applyValues(
                            oledIconPickerDialog.currentLine2Text(),
                            oledIconPickerDialog.currentLine1Text()
                        )
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Label {
                    text: qsTr("%1 icons").arg(oledIconPickerDialog.filteredIcons.length)
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            // Scrollable Icon Grid
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Qt.darker(Kirigami.Theme.backgroundColor, 1.08)
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                border.width: 1
                clip: true

                GridView {
                    id: iconGridView
                    anchors.fill: parent
                    anchors.margins: 8
                    cellWidth: 92
                    cellHeight: 72
                    clip: true
                    boundsBehavior: Flickable.StopAtBounds
                    model: oledIconPickerDialog.filteredIcons

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: iconGridView
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: iconGridView.contentHeight > iconGridView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Item {
                        width: iconGridView.cellWidth
                        height: iconGridView.cellHeight

                        required property var modelData

                        Rectangle {
                            anchors.fill: parent
                            anchors.margins: 3
                            radius: 6
                            color: iconTileMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.24)
                                : Kirigami.Theme.alternateBackgroundColor
                            border.color: iconTileMouse.containsMouse
                                ? Kirigami.Theme.highlightColor
                                : Qt.alpha(Kirigami.Theme.textColor, 0.14)
                            border.width: iconTileMouse.containsMouse ? 1.5 : 1

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 1

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    text: modelData.icon
                                    font.pointSize: 14 * root.fontScale
                                    font.bold: true
                                    color: Kirigami.Theme.textColor
                                }

                                Text {
                                    Layout.fillWidth: true
                                    horizontalAlignment: Text.AlignHCenter
                                    text: modelData.label + " · " + deviceController.oledLabelToHardwareAscii(modelData.icon)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    color: iconTileMouse.containsMouse
                                        ? Kirigami.Theme.textColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.82)
                                    elide: Text.ElideRight
                                }
                            }

                            MouseArea {
                                id: iconTileMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: oledIconPickerDialog.applyIconItem(modelData)
                                onDoubleClicked: {
                                    oledIconPickerDialog.applyIconItem(modelData)
                                    oledIconPickerDialog.close()
                                }
                            }

                            QQC2.ToolTip.visible: iconTileMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("%1 (%2 / HW: %3) — Click to apply, double-click to apply & close")
                                .arg(modelData.name)
                                .arg(modelData.icon)
                                .arg(deviceController.oledLabelToHardwareAscii(modelData.icon))
                        }
                    }
                }
            }

            // Footer Action Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "edit-clear"
                    text: qsTr("Clear Labels")
                    onClicked: oledIconPickerDialog.applyValues("", "")
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    text: qsTr("Done")
                    highlighted: true
                    font.bold: true
                    onClicked: oledIconPickerDialog.close()
                }
            }
        }
    }

    readonly property bool closableWithEscape: true
    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: appLaunchConfigWindow.visible
        onActivated: appLaunchConfigWindow.close()
    }
    title: qsTr("Launch Desktop Application — Key %1 (%2)").arg(root.selectedSlotNumber).arg(root.selectedSlotTitle)
    width: 740
    height: 640
    minimumWidth: 580
    minimumHeight: 480
    visible: false
    transientParent: root
    modality: Qt.NonModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    function openWindow(slotNumber) {
        const targetSlot = (slotNumber !== undefined && slotNumber > 0) ? slotNumber : root.selectedSlotNumber
        if (targetSlot <= 0) return
        if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
            appLaunchConfigWindow.loadAppLaunchBuilderFromScript(scriptEditor.text)
        } else if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlot) {
            appLaunchConfigWindow.loadAppLaunchBuilderFromScript(keyPropertiesWindow.keyPropPressScriptArea.text)
        }
        appLaunchConfigWindow.show()
        appLaunchConfigWindow.raise()
        appLaunchConfigWindow.requestActivate()
    }

    function openForCurrentSlot() {
        openWindow(root.selectedSlotNumber)
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 10

        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Rectangle {
                width: 34
                height: 34
                radius: 8
                color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                border.color: Kirigami.Theme.highlightColor
                border.width: 1
                QQC2.Label {
                    anchors.centerIn: parent
                    text: "🚀"
                    font.pointSize: 13 * root.fontScale
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                Layout.preferredWidth: 1
                spacing: 2
                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    elide: Text.ElideRight
                    text: qsTr("Desktop Application & Command Launcher — %1").arg(root.selectedSlotTitle)
                    font.bold: true
                    font.pointSize: 12 * root.fontScale
                }
                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    text: qsTr("Launch any installed desktop application, shell script, or command when this duckyPad key is pressed.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    wrapMode: Text.WordWrap
                }
            }

            QQC2.Switch {
                font.pointSize: 10 * root.fontScale
                id: appLaunchEnabledCheck
                text: checked ? qsTr("Enabled") : qsTr("Disabled")
                font.bold: true
                onToggled: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
            }
        }

        Rectangle {
            Layout.fillWidth: true
            radius: 5
            color: Qt.alpha("#f59e0b", 0.14)
            border.color: Qt.alpha("#f59e0b", 0.55)
            border.width: 1
            implicitHeight: appLaunchHostWarnLbl.implicitHeight + 10

            QQC2.Label {
                id: appLaunchHostWarnLbl
                anchors.fill: parent
                anchors.margins: 5
                wrapMode: Text.WordWrap
                font.pointSize: 10 * root.fontScale
                font.bold: true
                color: root.highContrastWarningText
                text: qsTr("💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable) — Application launching is executed by DuckyDeck Studio over USB HID. If DuckyDeck Studio is not running or the duckyPad is moved to another PC, only standard DuckyScript 3 commands in this key will run.")
            }
        }

        Flickable {
            id: appLaunchFlickable
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.minimumHeight: 80
            Layout.preferredHeight: 1
            clip: true
            contentWidth: width
            contentHeight: appLaunchScrollCol.implicitHeight + 8
            boundsBehavior: Flickable.StopAtBounds
            flickableDirection: Flickable.VerticalFlick

            QQC2.ScrollBar.vertical: HighContrastScrollBar {
                parent: appLaunchFlickable
                anchors.top: appLaunchFlickable.top
                anchors.right: appLaunchFlickable.right
                anchors.bottom: appLaunchFlickable.bottom
                policy: appLaunchFlickable.contentHeight > appLaunchFlickable.height
                    ? QQC2.ScrollBar.AlwaysOn
                    : QQC2.ScrollBar.AsNeeded
            }

            ColumnLayout {
                id: appLaunchScrollCol
                width: Math.max(200, appLaunchFlickable.width - (appLaunchFlickable.contentHeight > appLaunchFlickable.height ? 18 : 2))
                spacing: 10

                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: appPickCol.implicitHeight + 16
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                    border.width: 1

                    ColumnLayout {
                        id: appPickCol
                        anchors.fill: parent
                        anchors.margins: 8
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                icon.name: "applications-all"
                                text: qsTr("Choose Installed Application...")
                                highlighted: true
                                font.bold: true
                                onClicked: {
                                    root.installedAppSearchText = ""
                                    installedAppSearchInput.text = ""
                                    appLaunchConfigWindow.refreshInstalledAppsModel()
                                    installedAppPickerDialog.show()
                                    installedAppPickerDialog.raise()
                                    installedAppPickerDialog.requestActivate()
                                }
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                icon.name: "document-open"
                                text: qsTr("Browse Script / Executable...")
                                onClicked: appExecutableFileDialog.open()
                            }

                            Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

                            QQC2.Label {
                                visible: root.appLaunchAppName.length > 0
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                horizontalAlignment: Text.AlignRight
                                text: qsTr("Selected: %1").arg(root.appLaunchAppName)
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: root.highContrastPositiveText
                                elide: Text.ElideRight
                            }
                        }

                        GridLayout {
                            Layout.fillWidth: true
                            columns: 2
                            columnSpacing: 10
                            rowSpacing: 8

                            QQC2.Label {
                                text: qsTr("Command / Binary:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }
                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchCommandField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                placeholderText: qsTr("e.g. dolphin, konsole, /usr/bin/obs, ~/scripts/deploy.sh")
                                font.family: "monospace"
                                selectByMouse: true
                                onTextEdited: {
                                    if (text.trim().length > 0 && !appLaunchEnabledCheck.checked) {
                                        appLaunchEnabledCheck.checked = true
                                    }
                                    appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Arguments:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }
                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchArgsField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                placeholderText: qsTr("Optional CLI arguments (e.g. --new-window ~/Projects)")
                                font.family: "monospace"
                                selectByMouse: true
                                onTextEdited: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }

                            QQC2.Label {
                                text: qsTr("Working Dir:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }
                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchWorkDirField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                placeholderText: qsTr("Optional working directory (e.g. /home/user/workspace)")
                                font.family: "monospace"
                                selectByMouse: true
                                onTextEdited: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: appTrigCol.implicitHeight + 16
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                    border.width: 1

                    ColumnLayout {
                        id: appTrigCol
                        anchors.fill: parent
                        anchors.margins: 8
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Trigger Mode:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchTriggerModeCombo
                                Layout.fillWidth: true
                                Layout.minimumWidth: 120
                                Layout.preferredWidth: 1
                                model: [
                                    qsTr("USB HID Listener (Instant host launch — Zero keystroke pollution)"),
                                    qsTr("F-Key Shortcut Fallback (Emits F13–F24 + host listener)")
                                ]
                                onActivated: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchFkeyCombo
                                visible: appLaunchTriggerModeCombo.currentIndex === 1
                                Layout.preferredWidth: 96
                                model: ["F13","F14","F15","F16","F17","F18","F19","F20","F21","F22","F23","F24"]
                                currentIndex: 7
                                onActivated: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 12

                            HighContrastCheckBox {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchTerminalCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Run inside Terminal Emulator (konsole)")
                                onToggled: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }

                            HighContrastCheckBox {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchLedCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                checked: true
                                text: qsTr("Flash Key LED Green on Launch")
                                onToggled: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("OLED Feedback Text:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: appLaunchOledField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                maximumLength: 20
                                placeholderText: qsTr("Optional OLED status message on press (leave blank for none)")
                                selectByMouse: true
                                onTextEdited: appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                            }
                        }
                    }
                }

                // Quick Presets
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Label {
                        text: qsTr("Quick Presets:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    QQC2.Button {
                        text: Qt.platform.os === "windows" ? qsTr("📁 Explorer") : qsTr("📁 Dolphin")
                        implicitHeight: 26
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            const isWin = (Qt.platform.os === "windows")
                            root.appLaunchAppName = isWin ? "Explorer" : "Dolphin"
                            root.appLaunchDesktopId = isWin ? "" : "org.kde.dolphin.desktop"
                            appLaunchCommandField.text = isWin ? "explorer.exe" : "dolphin"
                            appLaunchArgsField.text = ""
                            appLaunchTerminalCheck.checked = false
                            appLaunchEnabledCheck.checked = true
                            appLaunchOledField.text = isWin ? "Run: Explorer" : "Run: Dolphin"
                            appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                        }
                    }

                    QQC2.Button {
                        text: Qt.platform.os === "windows" ? qsTr("🖥 Terminal") : qsTr("🖥 Konsole")
                        implicitHeight: 26
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            const isWin = (Qt.platform.os === "windows")
                            root.appLaunchAppName = isWin ? "Terminal" : "Konsole"
                            root.appLaunchDesktopId = isWin ? "" : "org.kde.konsole.desktop"
                            appLaunchCommandField.text = isWin ? "powershell.exe" : "konsole"
                            appLaunchArgsField.text = ""
                            appLaunchTerminalCheck.checked = false
                            appLaunchEnabledCheck.checked = true
                            appLaunchOledField.text = isWin ? "Run: Terminal" : "Run: Konsole"
                            appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                        }
                    }

                    QQC2.Button {
                        text: qsTr("🌐 Browser")
                        implicitHeight: 26
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            const isWin = (Qt.platform.os === "windows")
                            root.appLaunchAppName = "Web Browser"
                            root.appLaunchDesktopId = ""
                            appLaunchCommandField.text = isWin ? "explorer.exe" : "xdg-open"
                            appLaunchArgsField.text = "https://duckypad.com"
                            appLaunchTerminalCheck.checked = false
                            appLaunchEnabledCheck.checked = true
                            appLaunchOledField.text = "Open Browser"
                            appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                        }
                    }

                    Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "media-playback-start"
                text: qsTr("Test Launch Now")
                enabled: appLaunchCommandField.text.trim().length > 0
                onClicked: {
                    deviceController.testAppLaunch(
                        appLaunchConfigWindow.buildAppLaunchConfigJson(),
                        root.selectedSlotNumber
                    )
                }
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "edit-clear"
                text: qsTr("Disable / Clear")
                onClicked: {
                    appLaunchEnabledCheck.checked = false
                    appLaunchCommandField.text = ""
                    appLaunchArgsField.text = ""
                    appLaunchWorkDirField.text = ""
                    root.appLaunchAppName = ""
                    root.appLaunchDesktopId = ""
                    appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                }
            }

            Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "dialog-ok-apply"
                text: qsTr("Save & Close")
                highlighted: true
                font.bold: true
                onClicked: {
                    appLaunchConfigWindow.syncAppLaunchBuilderToScript()
                    appLaunchConfigWindow.close()
                }
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "window-close"
                text: qsTr("Close")
                onClicked: appLaunchConfigWindow.close()
            }
        }
    }
}
