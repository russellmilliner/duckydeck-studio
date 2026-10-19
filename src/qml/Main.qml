import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
    objectName: "duckydeckMainWindow"
    title: qsTr("DuckyDeck Studio v%1 — %2").arg(deviceController.appVersion).arg(deviceController.modelName)
    readonly property int currentScreenWidth: (root.screen && root.screen.width > 1024)
        ? root.screen.width
        : (Screen.width > 1024 ? Screen.width : 1920)
    readonly property int maxCenterColumnWidth: Math.max(
        460,
        Math.round(root.currentScreenWidth * 0.30)
    )
    readonly property int compactWindowWidth: Math.max(
        profilesSidebar.QQC2.SplitView.preferredWidth
            + Math.min(hardwareCanvas.requiredWidthForFullHeight, root.maxCenterColumnWidth)
            + Kirigami.Units.largeSpacing * 2
            + 20,
        connRow.implicitWidth
            + fwBadgeRect.implicitWidth
            + syncStatusBadge.implicitWidth
            + (deviceController.sdMountPath.length > 0 ? 220 : 0)
            + Kirigami.Units.largeSpacing * 2
            + 180
    )
    readonly property int expandedWindowWidth: Math.max(
        compactWindowWidth + inspectorCard.minContentWidth + 8,
        Math.min(
            Math.max(1280, Math.round(root.currentScreenWidth * 0.96)),
            1520
        )
    )
    readonly property int requiredWindowWidth: root.hasSelectedSlot
        ? root.expandedWindowWidth
        : root.compactWindowWidth
    readonly property bool isShowcaseCaptureWindow: Boolean(
        deviceController && deviceController.isDemoShowcaseMode() && !deviceController.isUiTestSuiteMode()
    )
    visible: false
    width: root.isShowcaseCaptureWindow ? 1600 : compactWindowWidth
    height: root.isShowcaseCaptureWindow ? 900 : 800
    minimumWidth: root.isShowcaseCaptureWindow ? 1600 : (root.hasSelectedSlot ? root.expandedWindowWidth : root.compactWindowWidth)
    minimumHeight: root.isShowcaseCaptureWindow ? 900 : 680

    TextEdit {
        id: clipboardHelper
        visible: false
    }

    function copyTextToClipboard(textToCopy) {
        clipboardHelper.text = String(textToCopy !== undefined && textToCopy !== null ? textToCopy : "")
        clipboardHelper.selectAll()
        clipboardHelper.copy()
        clipboardHelper.deselect()
    }

    function syncWindowWidthToEditorState() {
        if (root.isShowcaseCaptureWindow) {
            root.width = 1600
            root.height = 900
            return
        }
        if (root.visibility === Window.Maximized || root.visibility === Window.FullScreen) {
            return
        }
        if (root.hasSelectedSlot) {
            if (root.width < root.minimumWidth) {
                root.width = root.minimumWidth
            }
        } else {
            root.width = root.minimumWidth
        }
    }

    onMinimumWidthChanged: {
        root.syncWindowWidthToEditorState()
        Qt.callLater(root.syncWindowWidthToEditorState)
    }

    color: Kirigami.Theme.backgroundColor
    readonly property bool isLightTheme: (0.2126 * root.color.r + 0.7152 * root.color.g + 0.0722 * root.color.b) > 0.45
    readonly property string effectiveThemeMode: isLightTheme ? "light" : "dark"
    readonly property color highContrastHighlightText: isLightTheme ? "#0369a1" : Kirigami.Theme.highlightColor
    readonly property color highContrastPositiveText: isLightTheme ? "#15803d" : "#4ade80"
    readonly property color highContrastWarningText: isLightTheme ? "#b45309" : "#fbbf24"
    readonly property color highContrastOrangeText: isLightTheme ? "#c2410c" : "#fb923c"

    readonly property real fontScale: (deviceController && deviceController.uiTextScale > 0
        ? deviceController.uiTextScale
        : 100) / 100.0
    readonly property real fontCaption: Math.round(10.0 * root.fontScale * 10) / 10
    readonly property real fontBody: Math.round(10.5 * root.fontScale * 10) / 10
    readonly property real fontSubheading: Math.round(11.5 * root.fontScale * 10) / 10
    readonly property real fontHeading: Math.round(13.0 * root.fontScale * 10) / 10
    readonly property real fontDisplay: Math.round(15.0 * root.fontScale * 10) / 10
    readonly property real fontMono: Math.round(10.5 * root.fontScale * 10) / 10
    font.pointSize: root.fontBody
    readonly property real sidebarScale: root.fontScale
    readonly property real canvasScale: 1.0
    readonly property real inspectorScale: root.fontScale
    readonly property real editorScale: root.fontScale
    readonly property real statusBarScale: root.fontScale
    readonly property real eventLogScale: root.fontScale

    function zoomIn() {
        if (deviceController) {
            deviceController.zoomInUiTextScale()
        }
    }

    function zoomOut() {
        if (deviceController) {
            deviceController.zoomOutUiTextScale()
        }
    }

    function resetZoom() {
        if (deviceController) {
            deviceController.resetAllTextScales()
        }
    }

    function allPopupWindows() {
        return [
            { name: "DuckyColorPickerWindow", win: duckyColorPickerWindow, openFn: function() { duckyColorPickerWindow.openForKey(1, "#38BDF8") } },
            { name: "OledIconPickerDialog", win: oledIconPickerDialog, openFn: function() { oledIconPickerDialog.openForInspector(0) } },
            { name: "InstalledAppPickerDialog", win: installedAppPickerDialog, openFn: function() { installedAppPickerDialog.show(); installedAppPickerDialog.raise(); installedAppPickerDialog.requestActivate() } },
            { name: "WebApiDestinationsDialog", win: webApiDestinationsDialog, openFn: function() { webApiDestinationsDialog.openForDestination("", "POST", "", "") } },
            { name: "NewProfileGroupDialog", win: newProfileGroupDialog, openFn: function() { newProfileGroupDialog.openWithDefaults() } },
            { name: "RenameProfileDialog", win: renameProfileDialog, openFn: function() { renameProfileDialog.openWithProfile(1, "Default") } },
            { name: "DeleteProfileDialog", win: deleteProfileDialog, openFn: function() { deleteProfileDialog.open() } },
            { name: "UnsyncedChangesDialog", win: unsyncedChangesDialog, openFn: function() { unsyncedChangesDialog.open() } },
            { name: "SdSyncDialog", win: sdSyncDialog, openFn: function() { sdSyncDialog.open() } },
            { name: "FirmwareUpdateDialog", win: firmwareUpdateDialog, openFn: function() { firmwareUpdateDialog.open() } },
            { name: "FirstRunSetupDialog", win: firstRunSetupDialog, openFn: function() { firstRunSetupDialog.open() } },
            { name: "ProfileTemplatesDialog", win: profileTemplatesDialog, openFn: function() { profileTemplatesDialog.openGeneral() } },
            { name: "SettingsDialog", win: settingsDialog, openFn: function() { settingsDialog.open() } },
            { name: "AboutDialog", win: aboutDialog, openFn: function() { aboutDialog.openAboutWindow() } },
            { name: "EventLogDialog", win: eventLogDialog, openFn: function() { eventLogDialog.openLogWindow() } },
            { name: "LightingAnimatorDialog", win: lightingAnimatorDialog, openFn: function() { lightingAnimatorDialog.openFor("profile", 1) } },
            { name: "MacroRecorderDialog", win: macroRecorderDialog, openFn: function() { macroRecorderDialog.openRecorder() } },
            { name: "AppLaunchConfigWindow", win: appLaunchConfigWindow, openFn: function() { appLaunchConfigWindow.openWindow(1) } },
            { name: "WebApiKeyConfigWindow", win: webApiKeyConfigWindow, openFn: function() { webApiKeyConfigWindow.openWindow(1) } },
            { name: "KeyPropertiesWindow", win: keyPropertiesWindow, openFn: function() { keyPropertiesWindow.openForSlot(1) } },
            { name: "ProfilePropertiesWindow", win: profilePropertiesWindow, openFn: function() { profilePropertiesWindow.openForProfile(deviceController.activeProfile || 1) } },
            { name: "AiModal", win: aiModal, openFn: function() { aiModal.open() } },
            { name: "PgvDrawer", win: pgvDrawer, openFn: function() { pgvDrawer.open() } },
            { name: "AutoSwitchDrawer", win: autoSwitchDrawer, openFn: function() { autoSwitchDrawer.open() } },
            { name: "OledDesktopWidget", win: oledDesktopWidget, openFn: function() { oledDesktopWidget.show(); oledDesktopWidget.syncFromController() } }
        ]
    }

    function openAboutWindow() {
        aboutDialog.openAboutWindow()
    }

    property bool lastPopupClosedByEsc: false

    function closeTopmostPopupWindow() {
        root.lastPopupClosedByEsc = false
        const popups = root.allPopupWindows()
        // 1. Prefer an actively focused popup window if one is active
        for (let i = 0; i < popups.length; ++i) {
            const w = popups[i].win
            if (w && w.visible && w.active === true) {
                if (w === macroRecorderDialog && macroRecorderDialog.isRecording) {
                    macroRecorderDialog.stopRecording()
                }
                if (w === oledDesktopWidget) {
                    oledDesktopWidget.hide()
                }
                w.close()
                w.visible = false
                root.lastPopupClosedByEsc = true
                return
            }
        }
        // 2. Otherwise close the topmost visible popup window in priority order
        for (let i = 0; i < popups.length; ++i) {
            const w = popups[i].win
            if (w && w.visible) {
                if (w === macroRecorderDialog && macroRecorderDialog.isRecording) {
                    macroRecorderDialog.stopRecording()
                }
                if (w === oledDesktopWidget) {
                    oledDesktopWidget.hide()
                }
                w.close()
                w.visible = false
                root.lastPopupClosedByEsc = true
                return
            }
        }
        if (root.autocompleteVisible) {
            root.autocompleteVisible = false
            root.lastPopupClosedByEsc = true
            return
        }
    }


    property int selectedSlotNumber: 0
    readonly property bool hasSelectedSlot: root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 58
    onHasSelectedSlotChanged: {
        if (deviceController) {
            deviceController.setScriptEditorOpen(root.hasSelectedSlot)
        }
        root.syncWindowWidthToEditorState()
        Qt.callLater(root.syncWindowWidthToEditorState)
    }
    property string selectedSlotTitle: "Key 1 (Front View)"
    property color selectedSlotColor: "#00aaff"
    property bool editingReleaseScript: false
    property string pressScriptBuffer: ""
    property string releaseScriptBuffer: ""
    property bool loadingSlotData: false

    // Snapshot of slot state when first selected so Revert can undo auto-saved edits
    property int slotSnapshotSlotNumber: 0
    property int slotSnapshotProfileNumber: 0
    property string slotSnapshotL1: ""
    property string slotSnapshotL2: ""
    property string slotSnapshotWL1: ""
    property string slotSnapshotWL2: ""
    property bool slotSnapshotAbort: false
    property bool slotSnapshotNoRepeat: false
    property string slotSnapshotPressScript: ""
    property string slotSnapshotReleaseScript: ""
    property string slotSnapshotColor: "#00aaff"

    // Mutually exclusive inspector workspace mode: "code" | "app_launch" | "web_api"
    property string inspectorMode: "code"
    property bool showScriptPreviewInBuilder: true

    property string profileFilterText: ""
    property int totalProfileCount: 1
    property int totalRuleCount: 0
    property bool showNewProfileInput: false

    property bool compileOk: true
    property bool compileEmpty: false
    property int compileByteCount: 0
    property int compileErrorLine: 0
    property string compileErrorMsg: ""

    property bool showWebApiBuilder: false
    onShowWebApiBuilderChanged: {
        if (showWebApiBuilder) {
            if (showAppLaunchBuilder) showAppLaunchBuilder = false
            if (showCommandReference) showCommandReference = false
            if (inspectorMode !== "web_api") inspectorMode = "web_api"
        } else if (inspectorMode === "web_api" && !showAppLaunchBuilder) {
            inspectorMode = "code"
        }
    }
    property bool syncingWebApiScript: false
    property bool webApiEnabled: false
    property string webApiSelectedDestination: ""
    property var webApiDestinationsList: {
        if (!deviceController || !deviceController.webApiDestinationsJson) return []
        try {
            const parsed = JSON.parse(deviceController.webApiDestinationsJson)
            return Array.isArray(parsed) ? parsed : []
        } catch (e) {
            return []
        }
    }
    readonly property var webApiDestinationComboItems: {
        const items = [qsTr("Custom")]
        const list = root.webApiDestinationsList || []
        for (let i = 0; i < list.length; ++i) {
            const d = list[i]
            items.push(d.name)
        }
        return items
    }
    readonly property var webApiMethods: ["POST", "GET", "PUT", "PATCH", "DELETE"]
    readonly property var webApiFkeys: ["F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24"]

    // Application Launch Builder state
    property bool showAppLaunchBuilder: false
    onShowAppLaunchBuilderChanged: {
        if (showAppLaunchBuilder) {
            if (showWebApiBuilder) showWebApiBuilder = false
            if (showCommandReference) showCommandReference = false
            if (inspectorMode !== "app_launch") inspectorMode = "app_launch"
        } else if (inspectorMode === "app_launch" && !showWebApiBuilder) {
            inspectorMode = "code"
        }
    }
    property bool syncingAppLaunchScript: false
    property bool appLaunchEnabled: false
    property string appLaunchAppName: ""
    property string appLaunchDesktopId: ""
    property string installedAppSearchText: ""
    readonly property var appLaunchTriggerModes: ["pgv", "fkey", "krunner", "guir"]

    // DuckyScript 3 Autocomplete & Command Reference state
    property bool showCommandReference: false
    property string commandRefSearchText: ""
    property string commandRefCategory: "All"
    readonly property var commandRefCategories: [
        "All",
        "Text & Keys",
        "Flow & Loops",
        "OLED & RGB",
        "Variables & Math",
        "Profiles & System"
    ]
    property bool autocompleteEnabled: true
    property bool autocompleteVisible: false
    property int autocompleteIndex: 0
    property int autocompleteTokenStart: 0
    property int autocompleteTokenEnd: 0
    property string autocompletePrefix: ""
    property bool insertingCompletion: false

    ListModel {
        id: autocompleteModel
    }
    ListModel {
        id: commandReferenceModel
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

    readonly property var duckyScriptCatalog: [
        // Text & Keys
        { token: "STRING", category: "Text & Keys", badge: "CMD", syntax: "STRING <text>", insertText: "STRING ", summary: "Type literal text without pressing Enter at the end (supports $var interpolation)." },
        { token: "STRINGLN", category: "Text & Keys", badge: "CMD", syntax: "STRINGLN <text>", insertText: "STRINGLN ", summary: "Type literal text followed by an automatic Enter keystroke." },
        { token: "STRING_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "STRING_BLOCK ... END_STRING", insertText: "STRING_BLOCK\n\nEND_STRING", summary: "Multi-line literal text block typed without trailing Enter on each line." },
        { token: "END_STRING", category: "Text & Keys", badge: "CMD", syntax: "END_STRING", insertText: "END_STRING", summary: "Closes a STRING_BLOCK block." },
        { token: "STRINGLN_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "STRINGLN_BLOCK ... END_STRINGLN", insertText: "STRINGLN_BLOCK\n\nEND_STRINGLN", summary: "Multi-line literal text block where each line is followed by Enter." },
        { token: "END_STRINGLN", category: "Text & Keys", badge: "CMD", syntax: "END_STRINGLN", insertText: "END_STRINGLN", summary: "Closes a STRINGLN_BLOCK block." },
        { token: "REM", category: "Text & Keys", badge: "CMD", syntax: "REM <comment>", insertText: "REM ", summary: "Single-line comment ignored by the compiler (or use //)." },
        { token: "REM_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "REM_BLOCK ... END_REM", insertText: "REM_BLOCK\n\nEND_REM", summary: "Multi-line comment block ignored by the compiler." },
        { token: "END_REM", category: "Text & Keys", badge: "CMD", syntax: "END_REM", insertText: "END_REM", summary: "Closes a REM_BLOCK multi-line comment." },
        { token: "KEYDOWN", category: "Text & Keys", badge: "KEY", syntax: "KEYDOWN <key>", insertText: "KEYDOWN ", summary: "Hold down a key or modifier until KEYUP is called." },
        { token: "KEYUP", category: "Text & Keys", badge: "KEY", syntax: "KEYUP <key>", insertText: "KEYUP ", summary: "Release a key previously held with KEYDOWN." },
        { token: "INJECT_MOD", category: "Text & Keys", badge: "KEY", syntax: "INJECT_MOD <modifier>", insertText: "INJECT_MOD ", summary: "Inject a standalone modifier key press compatibility wrapper." },
        { token: "CTRL", category: "Text & Keys", badge: "KEY", syntax: "CTRL <key>", insertText: "CTRL ", summary: "Left Control modifier key (can be combined up to 6 keys, e.g. CTRL SHIFT c)." },
        { token: "RCTRL", category: "Text & Keys", badge: "KEY", syntax: "RCTRL <key>", insertText: "RCTRL ", summary: "Right Control modifier key." },
        { token: "SHIFT", category: "Text & Keys", badge: "KEY", syntax: "SHIFT <key>", insertText: "SHIFT ", summary: "Left Shift modifier key." },
        { token: "RSHIFT", category: "Text & Keys", badge: "KEY", syntax: "RSHIFT <key>", insertText: "RSHIFT ", summary: "Right Shift modifier key." },
        { token: "ALT", category: "Text & Keys", badge: "KEY", syntax: "ALT <key>", insertText: "ALT ", summary: "Left Alt / Option modifier key." },
        { token: "RALT", category: "Text & Keys", badge: "KEY", syntax: "RALT <key>", insertText: "RALT ", summary: "Right Alt / AltGr modifier key." },
        { token: "GUI", category: "Text & Keys", badge: "KEY", syntax: "GUI <key>", insertText: "GUI ", summary: "Left Super / Windows / Command modifier key (aliases: WINDOWS, COMMAND)." },
        { token: "WINDOWS", category: "Text & Keys", badge: "KEY", syntax: "WINDOWS <key>", insertText: "WINDOWS ", summary: "Alias for GUI (Super / Windows / Meta key)." },
        { token: "COMMAND", category: "Text & Keys", badge: "KEY", syntax: "COMMAND <key>", insertText: "COMMAND ", summary: "Alias for GUI (macOS Command / Super key)." },
        { token: "ENTER", category: "Text & Keys", badge: "KEY", syntax: "ENTER", insertText: "ENTER", summary: "Press the Return / Enter key." },
        { token: "ESC", category: "Text & Keys", badge: "KEY", syntax: "ESC", insertText: "ESC", summary: "Press the Escape key (alias: ESCAPE)." },
        { token: "BACKSPACE", category: "Text & Keys", badge: "KEY", syntax: "BACKSPACE", insertText: "BACKSPACE", summary: "Press the Backspace key." },
        { token: "TAB", category: "Text & Keys", badge: "KEY", syntax: "TAB", insertText: "TAB", summary: "Press the Tab key." },
        { token: "SPACE", category: "Text & Keys", badge: "KEY", syntax: "SPACE", insertText: "SPACE", summary: "Press the Spacebar." },
        { token: "DELETE", category: "Text & Keys", badge: "KEY", syntax: "DELETE", insertText: "DELETE", summary: "Press the forward Delete key." },
        { token: "INSERT", category: "Text & Keys", badge: "KEY", syntax: "INSERT", insertText: "INSERT", summary: "Press the Insert key." },
        { token: "HOME", category: "Text & Keys", badge: "KEY", syntax: "HOME", insertText: "HOME", summary: "Press the Home key." },
        { token: "END", category: "Text & Keys", badge: "KEY", syntax: "END", insertText: "END", summary: "Press the End key." },
        { token: "PAGEUP", category: "Text & Keys", badge: "KEY", syntax: "PAGEUP", insertText: "PAGEUP", summary: "Press the Page Up key." },
        { token: "PAGEDOWN", category: "Text & Keys", badge: "KEY", syntax: "PAGEDOWN", insertText: "PAGEDOWN", summary: "Press the Page Down key." },
        { token: "UP", category: "Text & Keys", badge: "KEY", syntax: "UP", insertText: "UP", summary: "Press the Up Arrow key (alias: UPARROW)." },
        { token: "DOWN", category: "Text & Keys", badge: "KEY", syntax: "DOWN", insertText: "DOWN", summary: "Press the Down Arrow key (alias: DOWNARROW)." },
        { token: "LEFT", category: "Text & Keys", badge: "KEY", syntax: "LEFT", insertText: "LEFT", summary: "Press the Left Arrow key (alias: LEFTARROW)." },
        { token: "RIGHT", category: "Text & Keys", badge: "KEY", syntax: "RIGHT", insertText: "RIGHT", summary: "Press the Right Arrow key (alias: RIGHTARROW)." },
        { token: "PRINTSCREEN", category: "Text & Keys", badge: "KEY", syntax: "PRINTSCREEN", insertText: "PRINTSCREEN", summary: "Press the Print Screen / SysRq key." },
        { token: "CAPSLOCK", category: "Text & Keys", badge: "KEY", syntax: "CAPSLOCK", insertText: "CAPSLOCK", summary: "Press the Caps Lock key." },
        { token: "NUMLOCK", category: "Text & Keys", badge: "KEY", syntax: "NUMLOCK", insertText: "NUMLOCK", summary: "Press the Num Lock key." },
        { token: "SCROLLLOCK", category: "Text & Keys", badge: "KEY", syntax: "SCROLLLOCK", insertText: "SCROLLLOCK", summary: "Press the Scroll Lock key." },
        { token: "PAUSE", category: "Text & Keys", badge: "KEY", syntax: "PAUSE", insertText: "PAUSE", summary: "Press the Pause / Break key." },
        { token: "MENU", category: "Text & Keys", badge: "KEY", syntax: "MENU", insertText: "MENU", summary: "Press the Application / Context Menu key (alias: APP)." },
        { token: "F1", category: "Text & Keys", badge: "KEY", syntax: "F1 .. F24", insertText: "F1", summary: "Function keys F1 through F24 (F13–F24 great for conflict-free shortcuts)." },
        { token: "F13", category: "Text & Keys", badge: "KEY", syntax: "F13 .. F24", insertText: "F13", summary: "Extended Function key F13 (F13–F24 supported)." },
        { token: "MK_VOLUP", category: "Text & Keys", badge: "KEY", syntax: "MK_VOLUP", insertText: "MK_VOLUP", summary: "Media key: Volume Up." },
        { token: "MK_VOLDOWN", category: "Text & Keys", badge: "KEY", syntax: "MK_VOLDOWN", insertText: "MK_VOLDOWN", summary: "Media key: Volume Down." },
        { token: "MK_MUTE", category: "Text & Keys", badge: "KEY", syntax: "MK_MUTE", insertText: "MK_MUTE", summary: "Media key: Mute Audio." },
        { token: "MK_PP", category: "Text & Keys", badge: "KEY", syntax: "MK_PP", insertText: "MK_PP", summary: "Media key: Play / Pause." },
        { token: "MK_NEXT", category: "Text & Keys", badge: "KEY", syntax: "MK_NEXT", insertText: "MK_NEXT", summary: "Media key: Next Track." },
        { token: "MK_PREV", category: "Text & Keys", badge: "KEY", syntax: "MK_PREV", insertText: "MK_PREV", summary: "Media key: Previous Track." },
        { token: "MK_STOP", category: "Text & Keys", badge: "KEY", syntax: "MK_STOP", insertText: "MK_STOP", summary: "Media key: Stop Playback." },
        { token: "MOUSE_MOVE", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_MOVE <dx> <dy>", insertText: "MOUSE_MOVE 0 0", summary: "Move cursor relatively by dx (-127..127) and dy (-127..127) pixels." },
        { token: "MOUSE_SCROLL", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_SCROLL <h> <v>", insertText: "MOUSE_SCROLL 0 1", summary: "Scroll mouse wheel horizontally (h) and vertically (v)." },
        { token: "MOUSE_WHEEL", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_WHEEL <v>", insertText: "MOUSE_WHEEL 1", summary: "Scroll vertical mouse wheel (positive = up, negative = down)." },
        { token: "LMOUSE", category: "Text & Keys", badge: "KEY", syntax: "LMOUSE", insertText: "LMOUSE", summary: "Click Left Mouse Button (also works with KEYDOWN/KEYUP)." },
        { token: "RMOUSE", category: "Text & Keys", badge: "KEY", syntax: "RMOUSE", insertText: "RMOUSE", summary: "Click Right Mouse Button." },
        { token: "MMOUSE", category: "Text & Keys", badge: "KEY", syntax: "MMOUSE", insertText: "MMOUSE", summary: "Click Middle Mouse Button." },
        { token: "BMOUSE", category: "Text & Keys", badge: "KEY", syntax: "BMOUSE", insertText: "BMOUSE", summary: "Click Mouse Back Button." },
        { token: "FMOUSE", category: "Text & Keys", badge: "KEY", syntax: "FMOUSE", insertText: "FMOUSE", summary: "Click Mouse Forward Button." },

        // Flow & Loops
        { token: "DELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DELAY <ms>", insertText: "DELAY 100", summary: "Pause script execution for <ms> milliseconds." },
        { token: "DEFAULTDELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DEFAULTDELAY <ms>", insertText: "DEFAULTDELAY 20", summary: "Set delay (ms) automatically inserted before every subsequent command." },
        { token: "DEFAULTCHARDELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DEFAULTCHARDELAY <ms>", insertText: "DEFAULTCHARDELAY 20", summary: "Set delay (ms) between each character typed by STRING/STRINGLN." },
        { token: "CHARJITTER", category: "Flow & Loops", badge: "FLOW", syntax: "CHARJITTER <ms>", insertText: "CHARJITTER 5", summary: "Add random jitter (0..ms) to character typing delay." },
        { token: "IF", category: "Flow & Loops", badge: "FLOW", syntax: "IF <cond> ... END_IF", insertText: "IF $_READKEY == 0\n    \nEND_IF", summary: "Conditional execution block (supports ==, !=, <, <=, >, >=, &&, ||, !)." },
        { token: "ELSE IF", category: "Flow & Loops", badge: "FLOW", syntax: "ELSE IF <cond>", insertText: "ELSE IF ", summary: "Secondary branch inside an IF ... END_IF block." },
        { token: "ELSE", category: "Flow & Loops", badge: "FLOW", syntax: "ELSE", insertText: "ELSE", summary: "Fallback branch inside an IF ... END_IF block." },
        { token: "END_IF", category: "Flow & Loops", badge: "FLOW", syntax: "END_IF", insertText: "END_IF", summary: "Closes an IF conditional block." },
        { token: "WHILE", category: "Flow & Loops", badge: "FLOW", syntax: "WHILE <cond> ... END_WHILE", insertText: "WHILE 1\n    \nEND_WHILE", summary: "Loop while condition evaluates to non-zero." },
        { token: "END_WHILE", category: "Flow & Loops", badge: "FLOW", syntax: "END_WHILE", insertText: "END_WHILE", summary: "Closes a WHILE loop." },
        { token: "LBREAK", category: "Flow & Loops", badge: "FLOW", syntax: "LBREAK", insertText: "LBREAK", summary: "Break out of the innermost WHILE loop immediately." },
        { token: "CONTINUE", category: "Flow & Loops", badge: "FLOW", syntax: "CONTINUE", insertText: "CONTINUE", summary: "Skip to the next iteration of the innermost WHILE loop." },
        { token: "LOOP0:", category: "Flow & Loops", badge: "FLOW", syntax: "LOOP0: .. LOOP9:", insertText: "LOOP0:\n    \nLOOP1:\n    ", summary: "Cyclic step label: each press of the key advances to the next LOOP<n>: block." },
        { token: "REPEAT", category: "Flow & Loops", badge: "FLOW", syntax: "REPEAT <count>", insertText: "REPEAT 1", summary: "Repeat the previous line <count> additional times." },
        { token: "FUN", category: "Flow & Loops", badge: "FLOW", syntax: "FUN <name>(<args>) ... END_FUN", insertText: "FUN my_func(arg1)\n    RETURN 0\nEND_FUN", summary: "Define a reusable function with optional arguments and RETURN value." },
        { token: "END_FUN", category: "Flow & Loops", badge: "FLOW", syntax: "END_FUN", insertText: "END_FUN", summary: "Closes a FUN function definition." },
        { token: "RETURN", category: "Flow & Loops", badge: "FLOW", syntax: "RETURN [expr]", insertText: "RETURN ", summary: "Return from the current function (optionally returning a value)." },
        { token: "HALT", category: "Flow & Loops", badge: "FLOW", syntax: "HALT", insertText: "HALT", summary: "Immediately stop script execution." },
        { token: "PASS", category: "Flow & Loops", badge: "FLOW", syntax: "PASS", insertText: "PASS", summary: "No-operation placeholder statement." },

        // OLED & RGB
        { token: "OLED_CLEAR", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CLEAR", insertText: "OLED_CLEAR", summary: "Clear the OLED display framebuffer (call OLED_UPDATE to render)." },
        { token: "OLED_UPDATE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_UPDATE", insertText: "OLED_UPDATE", summary: "Push the OLED framebuffer to the physical screen." },
        { token: "OLED_RESTORE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_RESTORE", insertText: "OLED_RESTORE", summary: "Restore the default profile/key grid view on the OLED screen." },
        { token: "OLED_CURSOR", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CURSOR <x> <y>", insertText: "OLED_CURSOR 0 0", summary: "Move the OLED text cursor to pixel coordinates (x: 0..127, y: 0..63)." },
        { token: "OLED_PRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_PRINT <text>", insertText: "OLED_PRINT ", summary: "Print left-aligned text at the current OLED cursor position (supports $var)." },
        { token: "OLED_CPRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CPRINT <text>", insertText: "OLED_CPRINT ", summary: "Print horizontally centered text on the OLED screen." },
        { token: "OLED_LPRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_LPRINT <text>", insertText: "OLED_LPRINT ", summary: "Print text at the current OLED cursor with automatic line wrapping." },
        { token: "OLED_LINE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_LINE <x1> <y1> <x2> <y2>", insertText: "OLED_LINE 0 0 127 63", summary: "Draw a line on the OLED framebuffer between (x1,y1) and (x2,y2)." },
        { token: "OLED_RECT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_RECT <x1> <y1> <x2> <y2> <opt>", insertText: "OLED_RECT 0 0 127 63 1", summary: "Draw a rectangle on the OLED (opt: 0=clear, 1=outline, 2=filled)." },
        { token: "OLED_CIRCLE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CIRCLE <x> <y> <radius> <opt>", insertText: "OLED_CIRCLE 64 32 16 1", summary: "Draw a circle on the OLED (opt: 0=clear, 1=outline, 2=filled)." },
        { token: "SWCOLOR", category: "OLED & RGB", badge: "LED", syntax: "SWCOLOR <r> <g> <b>", insertText: "SWCOLOR 0 170 255", summary: "Set the current key's RGB LED color (0..255 per channel)." },
        { token: "SWC_SET", category: "OLED & RGB", badge: "LED", syntax: "SWC_SET <key> <r> <g> <b>", insertText: "SWC_SET $_THIS_KEYID 0 255 128", summary: "Set RGB LED color of a specific key (0 = current key, 1..20 = key slot)." },
        { token: "SWC_FILL", category: "OLED & RGB", badge: "LED", syntax: "SWC_FILL <r> <g> <b>", insertText: "SWC_FILL 0 170 255", summary: "Fill all key RGB LEDs with the specified (r, g, b) color." },
        { token: "SWC_RESET", category: "OLED & RGB", badge: "LED", syntax: "SWC_RESET <key>", insertText: "SWC_RESET 0", summary: "Reset key RGB LED(s) back to profile default colors (0 = all keys, 1..20 = slot)." },

        // Variables & Math
        { token: "VAR", category: "Variables & Math", badge: "VAR", syntax: "VAR $<name> = <expr>", insertText: "VAR $count = 0", summary: "Declare and initialize a 32-bit user variable ($ is optional)." },
        { token: "DEFINE", category: "Variables & Math", badge: "VAR", syntax: "DEFINE <NAME> <replacement>", insertText: "DEFINE MY_DELAY 50", summary: "Define a compile-time text replacement macro." },
        { token: "DPDSSTDLIB", category: "Variables & Math", badge: "FUNC", syntax: "DPDSSTDLIB", insertText: "DPDSSTDLIB", summary: "Import the standard library functions: WAITKEY(k), MEMSET(a,v,n), ABS(n), MIN(a,b), MAX(a,b)." },
        { token: "RANDINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDINT(<min>, <max>)", insertText: "RANDINT(1, 100)", summary: "Return a random signed integer between min and max (inclusive)." },
        { token: "RANDUINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDUINT(<min>, <max>)", insertText: "RANDUINT(0, 100)", summary: "Return a random unsigned integer between min and max (inclusive)." },
        { token: "RANDCHR", category: "Variables & Math", badge: "FUNC", syntax: "RANDCHR(<bitmask>)", insertText: "RANDCHR(0x103)", summary: "Type/print a random character (or use RANDOM_LETTER, RANDOM_NUMBER, RANDOM_CHAR)." },
        { token: "RANDOM_LETTER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_LETTER", insertText: "RANDOM_LETTER", summary: "Type a random uppercase or lowercase ASCII letter." },
        { token: "RANDOM_NUMBER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_NUMBER", insertText: "RANDOM_NUMBER", summary: "Type a random digit (0–9)." },
        { token: "RANDOM_CHAR", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_CHAR", insertText: "RANDOM_CHAR", summary: "Type a random printable ASCII character." },
        { token: "WAITKEY", category: "Variables & Math", badge: "FUNC", syntax: "WAITKEY(<key_id>)", insertText: "WAITKEY(1)", summary: "Block until the specified key_id (1..20) is pressed (requires DPDSSTDLIB)." },
        { token: "ABS", category: "Variables & Math", badge: "FUNC", syntax: "ABS(<n>)", insertText: "ABS(", summary: "Return absolute value of n (requires DPDSSTDLIB)." },
        { token: "MIN", category: "Variables & Math", badge: "FUNC", syntax: "MIN(<a>, <b>)", insertText: "MIN(", summary: "Return the smaller of a and b (requires DPDSSTDLIB)." },
        { token: "MAX", category: "Variables & Math", badge: "FUNC", syntax: "MAX(<a>, <b>)", insertText: "MAX(", summary: "Return the larger of a and b (requires DPDSSTDLIB)." },
        { token: "$_GV0", category: "Variables & Math", badge: "VAR", syntax: "$_GV0 .. $_GV31", insertText: "$_GV0 = 0", summary: "Persistent Global Variable (0..31) shared across profiles and saved to SD/flash." },
        { token: "$_DEFAULTDELAY", category: "Variables & Math", badge: "VAR", syntax: "$_DEFAULTDELAY", insertText: "$_DEFAULTDELAY = 20", summary: "Read/write the default delay (ms) between commands." },
        { token: "$_DEFAULTCHARDELAY", category: "Variables & Math", badge: "VAR", syntax: "$_DEFAULTCHARDELAY", insertText: "$_DEFAULTCHARDELAY = 20", summary: "Read/write the delay (ms) between typed characters." },
        { token: "$_CHARJITTER", category: "Variables & Math", badge: "VAR", syntax: "$_CHARJITTER", insertText: "$_CHARJITTER = 5", summary: "Read/write random character delay jitter (ms)." },
        { token: "$_TIME_MS", category: "Variables & Math", badge: "VAR", syntax: "$_TIME_MS", insertText: "$_TIME_MS", summary: "Milliseconds elapsed since duckyPad booted." },
        { token: "$_TIME_S", category: "Variables & Math", badge: "VAR", syntax: "$_TIME_S", insertText: "$_TIME_S", summary: "Seconds elapsed since duckyPad booted." },
        { token: "$_READKEY", category: "Variables & Math", badge: "VAR", syntax: "$_READKEY", insertText: "$_READKEY", summary: "Non-blocking read of currently pressed key ID (0 if none)." },
        { token: "$_BLOCKING_READKEY", category: "Variables & Math", badge: "VAR", syntax: "$_BLOCKING_READKEY", insertText: "$_BLOCKING_READKEY", summary: "Wait for any key press and return its key ID." },
        { token: "$_THIS_KEYID", category: "Variables & Math", badge: "VAR", syntax: "$_THIS_KEYID", insertText: "$_THIS_KEYID", summary: "Key slot ID (1..20) that triggered the currently running script." },
        { token: "$_KEYPRESS_COUNT", category: "Variables & Math", badge: "VAR", syntax: "$_KEYPRESS_COUNT", insertText: "$_KEYPRESS_COUNT", summary: "Number of times this key has been pressed." },
        { token: "$_ALLOW_ABORT", category: "Variables & Math", badge: "VAR", syntax: "$_ALLOW_ABORT = 1", insertText: "$_ALLOW_ABORT = 1", summary: "Set to 1 to allow pressing any key to abort the running script." },
        { token: "$_DONT_REPEAT", category: "Variables & Math", badge: "VAR", syntax: "$_DONT_REPEAT = 1", insertText: "$_DONT_REPEAT = 1", summary: "Set to 1 to disable auto-repeat when holding down the key." },
        { token: "$_SW_BITFIELD", category: "Variables & Math", badge: "VAR", syntax: "$_SW_BITFIELD", insertText: "$_SW_BITFIELD", summary: "Bitmask of all currently pressed physical switches on the duckyPad." },
        { token: "$_KBLED_BITFIELD", category: "Variables & Math", badge: "VAR", syntax: "$_KBLED_BITFIELD", insertText: "$_KBLED_BITFIELD", summary: "Host keyboard LED state bitmask (NumLock=1, CapsLock=2, ScrollLock=4)." },
        { token: "_IS_NUMLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_NUMLOCK_ON", insertText: "_IS_NUMLOCK_ON", summary: "Evaluates to 1 if host Num Lock LED is on, 0 otherwise." },
        { token: "_IS_CAPSLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_CAPSLOCK_ON", insertText: "_IS_CAPSLOCK_ON", summary: "Evaluates to 1 if host Caps Lock LED is on, 0 otherwise." },
        { token: "_IS_SCROLLLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_SCROLLLOCK_ON", insertText: "_IS_SCROLLLOCK_ON", summary: "Evaluates to 1 if host Scroll Lock LED is on, 0 otherwise." },
        { token: "$_RTC_HOUR", category: "Variables & Math", badge: "VAR", syntax: "$_RTC_YEAR / $_RTC_MONTH / $_RTC_DAY / $_RTC_HOUR / $_RTC_MINUTE / $_RTC_SECOND", insertText: "$_RTC_HOUR", summary: "Real-Time Clock fields synced from DuckyDeck Studio (also $_RTC_IS_VALID, $_RTC_WDAY)." },

        // Profiles & System
        { token: "GOTO_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "GOTO_PROFILE <name_or_num>", insertText: "GOTO_PROFILE ", summary: "Switch immediately to a profile by number or exact name." },
        { token: "NEXT_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "NEXT_PROFILE", insertText: "NEXT_PROFILE", summary: "Switch to the next profile in sequence." },
        { token: "PREV_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "PREV_PROFILE", insertText: "PREV_PROFILE", summary: "Switch to the previous profile in sequence." },
        { token: "SKIP_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "SKIP_PROFILE <offset>", insertText: "SKIP_PROFILE 1", summary: "Jump forward/backward by <offset> profiles." },
        { token: "DP_SLEEP", category: "Profiles & System", badge: "PROF", syntax: "DP_SLEEP", insertText: "DP_SLEEP", summary: "Turn off OLED and RGB LEDs and enter low-power sleep mode." },
        { token: "BCLR", category: "Profiles & System", badge: "PROF", syntax: "BCLR", insertText: "BCLR", summary: "Clear the internal key press queue buffer." },
        { token: "HIDTX", category: "Profiles & System", badge: "PROF", syntax: "HIDTX <addr>", insertText: "HIDTX ", summary: "Transmit raw USB HID buffer from memory address <addr>." },
        { token: "PEEK8", category: "Profiles & System", badge: "FUNC", syntax: "PEEK8(<addr>) / PEEK16 / PEEK32", insertText: "PEEK8(", summary: "Read signed/unsigned 8, 16, or 32-bit value from VM memory address." },
        { token: "POKE8", category: "Profiles & System", badge: "FUNC", syntax: "POKE8(<addr>, <val>) / POKE16 / POKE32", insertText: "POKE8(", summary: "Write 8, 16, or 32-bit value to VM memory address." }
    ]

    function badgeColorForType(badge) {
        if (root.isLightTheme) {
            if (badge === "CMD") return "#0369a1"
            if (badge === "KEY") return "#6d28d9"
            if (badge === "FLOW") return "#b45309"
            if (badge === "OLED") return "#0f766e"
            if (badge === "LED") return "#be185d"
            if (badge === "VAR") return "#15803d"
            if (badge === "FUNC") return "#1d4ed8"
            if (badge === "PROF") return "#c2410c"
            return "#334155"
        }
        if (badge === "CMD") return "#38bdf8"
        if (badge === "KEY") return "#a78bfa"
        if (badge === "FLOW") return "#f59e0b"
        if (badge === "OLED") return "#2dd4bf"
        if (badge === "LED") return "#f472b6"
        if (badge === "VAR") return "#4ade80"
        if (badge === "FUNC") return "#60a5fa"
        if (badge === "PROF") return "#fb923c"
        return "#94a3b8"
    }

    function refreshCommandReferenceModel() {
        commandReferenceModel.clear()
        const query = (root.commandRefSearchText || "").trim().toLowerCase()
        const cat = root.commandRefCategory || "All"
        for (let i = 0; i < root.duckyScriptCatalog.length; ++i) {
            const item = root.duckyScriptCatalog[i]
            if (cat !== "All" && item.category !== cat) {
                continue
            }
            if (query.length > 0) {
                const hay = (item.token + " " + item.syntax + " " + item.summary + " " + item.badge).toLowerCase()
                if (hay.indexOf(query) === -1) {
                    continue
                }
            }
            commandReferenceModel.append(item)
        }
    }

    function collectDynamicScriptSymbols(scriptText) {
        const extra = []
        const seen = {}
        if (!scriptText) return extra
        const lines = scriptText.split("\n")
        for (let i = 0; i < lines.length; ++i) {
            const trimmed = lines[i].trim()
            if (trimmed.indexOf("VAR ") === 0) {
                const eqIdx = trimmed.indexOf("=")
                if (eqIdx > 4) {
                    let rawName = trimmed.slice(4, eqIdx).trim()
                    if (rawName.length > 0) {
                        const withDollar = rawName.charAt(0) === "$" ? rawName : ("$" + rawName)
                        if (!seen[withDollar]) {
                            seen[withDollar] = true
                            extra.push({
                                token: withDollar,
                                category: "Variables & Math",
                                badge: "USER",
                                syntax: withDollar,
                                insertText: withDollar,
                                summary: "User variable declared in this script (line " + (i + 1) + ")."
                            })
                        }
                    }
                }
            } else if (trimmed.indexOf("DEFINE ") === 0) {
                const rest = trimmed.slice(7).trim()
                const sp = rest.indexOf(" ")
                const defName = (sp > 0 ? rest.slice(0, sp) : rest).trim()
                if (defName.length > 0 && !seen[defName]) {
                    seen[defName] = true
                    extra.push({
                        token: defName,
                        category: "Variables & Math",
                        badge: "USER",
                        syntax: "DEFINE " + defName,
                        insertText: defName,
                        summary: "User DEFINE macro declared in this script (line " + (i + 1) + ")."
                    })
                }
            } else if (trimmed.indexOf("FUN ") === 0) {
                const rest = trimmed.slice(4).trim()
                const paren = rest.indexOf("(")
                const fnName = (paren > 0 ? rest.slice(0, paren) : rest).trim()
                if (fnName.length > 0 && !seen[fnName]) {
                    seen[fnName] = true
                    extra.push({
                        token: fnName,
                        category: "Flow & Loops",
                        badge: "USER",
                        syntax: rest,
                        insertText: fnName + "()",
                        summary: "User function defined in this script (line " + (i + 1) + ")."
                    })
                }
            }
        }
        return extra
    }

    function updateAutocompleteSuggestions(forcePopup) {
        if (root.loadingSlotData || root.insertingCompletion || (!root.autocompleteEnabled && !forcePopup)) {
            root.autocompleteVisible = false
            return
        }
        const text = scriptEditor.text || ""
        const cursor = scriptEditor.cursorPosition
        if (cursor < 0 || cursor > text.length) {
            root.autocompleteVisible = false
            return
        }

        // Find start of current line and start of current token
        let lineStart = cursor
        while (lineStart > 0 && text.charAt(lineStart - 1) !== "\n") {
            lineStart--
        }
        const lineBeforeCursor = text.slice(lineStart, cursor)
        const trimmedLine = lineBeforeCursor.replace(/^\s+/, "")

        // Suppress automatic popup inside literal text/comment lines unless typing a $variable or forcing via Ctrl+Space
        const firstSpaceIdx = trimmedLine.indexOf(" ")
        const firstWord = firstSpaceIdx >= 0 ? trimmedLine.slice(0, firstSpaceIdx) : ""
        const inLiteralOrComment = (
            trimmedLine.indexOf("//") === 0 ||
            firstWord === "REM" ||
            firstWord === "STRING" ||
            firstWord === "STRINGLN" ||
            firstWord === "OLED_PRINT" ||
            firstWord === "OLED_CPRINT" ||
            firstWord === "OLED_LPRINT"
        )

        let tokStart = cursor
        while (tokStart > lineStart) {
            const ch = text.charAt(tokStart - 1)
            if ((ch >= "A" && ch <= "Z") || (ch >= "a" && ch <= "z") || (ch >= "0" && ch <= "9") || ch === "_" || ch === "$") {
                tokStart--
            } else {
                break
            }
        }
        let tokEnd = cursor
        while (tokEnd < text.length) {
            const ch = text.charAt(tokEnd)
            if ((ch >= "A" && ch <= "Z") || (ch >= "a" && ch <= "z") || (ch >= "0" && ch <= "9") || ch === "_" || ch === ":") {
                tokEnd++
            } else {
                break
            }
        }

        const prefix = text.slice(tokStart, cursor)
        if (!forcePopup) {
            if (prefix.length < 1) {
                root.autocompleteVisible = false
                return
            }
            if (inLiteralOrComment && prefix.charAt(0) !== "$" && prefix.charAt(0) !== "_") {
                root.autocompleteVisible = false
                return
            }
        }

        const normPrefix = prefix.toUpperCase()
        const normNoDollar = normPrefix.indexOf("$") === 0 ? normPrefix.slice(1) : normPrefix
        const prefixMatches = []
        const substringMatches = []

        const dynamicSyms = root.collectDynamicScriptSymbols(text)
        const combined = dynamicSyms.concat(root.duckyScriptCatalog)

        // Also include $_GV0..$_GV31 when user types $_GV or _GV
        if (normNoDollar.indexOf("_GV") === 0) {
            for (let g = 1; g <= 31; ++g) {
                const gvTok = "$_GV" + g
                combined.push({
                    token: gvTok,
                    category: "Variables & Math",
                    badge: "VAR",
                    syntax: gvTok,
                    insertText: gvTok,
                    summary: "Persistent Global Variable #" + g + " (shared across profiles)."
                })
            }
        }

        for (let i = 0; i < combined.length; ++i) {
            const entry = combined[i]
            const tokUpper = entry.token.toUpperCase()
            const tokNoDollar = tokUpper.indexOf("$") === 0 ? tokUpper.slice(1) : tokUpper
            if (normPrefix.length === 0) {
                prefixMatches.push(entry)
            } else if (tokUpper === normPrefix || tokNoDollar === normNoDollar) {
                // If user already typed the exact full token and didn't press Ctrl+Space, don't keep showing single identical match
                if (forcePopup) {
                    prefixMatches.push(entry)
                }
            } else if (tokUpper.indexOf(normPrefix) === 0 || (normNoDollar.length > 0 && tokNoDollar.indexOf(normNoDollar) === 0)) {
                prefixMatches.push(entry)
            } else if (normPrefix.length >= 2 && (tokUpper.indexOf(normPrefix) > 0 || tokNoDollar.indexOf(normNoDollar) > 0)) {
                substringMatches.push(entry)
            }
        }

        const results = prefixMatches.concat(substringMatches).slice(0, 10)
        autocompleteModel.clear()
        for (let j = 0; j < results.length; ++j) {
            autocompleteModel.append(results[j])
        }

        root.autocompleteTokenStart = tokStart
        root.autocompleteTokenEnd = tokEnd
        root.autocompletePrefix = prefix
        root.autocompleteIndex = 0
        root.autocompleteVisible = autocompleteModel.count > 0
    }

    function applyAutocompleteSelection(idx) {
        if (idx < 0 || idx >= autocompleteModel.count) return
        const item = autocompleteModel.get(idx)
        if (!item) return
        const replacement = item.insertText || item.token
        const start = root.autocompleteTokenStart
        const end = root.autocompleteTokenEnd
        root.insertingCompletion = true
        scriptEditor.remove(start, end)
        scriptEditor.insert(start, replacement)
        scriptEditor.cursorPosition = start + replacement.length
        root.insertingCompletion = false
        root.autocompleteVisible = false
        scriptEditor.forceActiveFocus()
    }

    function insertCommandFromReference(snippetText) {
        if (!snippetText) return
        const text = scriptEditor.text || ""
        const cur = scriptEditor.cursorPosition
        const needsLeadingNewline = cur > 0 && text.charAt(cur - 1) !== "\n" && text.charAt(cur - 1) !== " "
        const needsTrailingNewline = snippetText.slice(-1) !== " " && snippetText.slice(-1) !== "(" && snippetText.slice(-1) !== "\n"
        const toInsert = (needsLeadingNewline ? "\n" : "") + snippetText + (needsTrailingNewline ? "\n" : "")
        root.insertingCompletion = true
        scriptEditor.insert(cur, toInsert)
        root.insertingCompletion = false
        root.autocompleteVisible = false
        scriptEditor.forceActiveFocus()
    }

    ListModel {
        id: sidebarProfileModel
    }
    readonly property var lightingAnimatorDialog: duckyColorPickerWindow.lightingAnimatorDialog
    readonly property var webApiDestinationsDialog: webApiKeyConfigWindow.webApiDestinationsDialog
    readonly property var webApiUrlField: webApiKeyConfigWindow.webApiUrlField
    readonly property var installedAppPickerDialog: appLaunchConfigWindow.installedAppPickerDialog
    readonly property var macroRecorderDialog: appLaunchConfigWindow.macroRecorderDialog
    readonly property var oledIconPickerDialog: appLaunchConfigWindow.oledIconPickerDialog
    readonly property var appLaunchCommandField: appLaunchConfigWindow.appLaunchCommandField
    readonly property var unsyncedChangesDialog: sdSyncDialog.unsyncedChangesDialog
    readonly property var firmwareUpdateDialog: sdSyncDialog.firmwareUpdateDialog
    readonly property var firstRunSetupDialog: sdSyncDialog.firstRunSetupDialog
    readonly property var newProfileGroupDialog: settingsDialog.newProfileGroupDialog
    readonly property var profileTemplatesDialog: settingsDialog.profileTemplatesDialog
    readonly property var renameProfileDialog: settingsDialog.renameProfileDialog
    readonly property var aboutDialog: settingsDialog.aboutDialog
    readonly property var profilePropertiesWindow: settingsDialog.profilePropertiesWindow
    readonly property var keyPropertiesWindow: settingsDialog.keyPropertiesWindow
    readonly property var showcaseRunnerTimer: eventLogDialog.showcaseRunnerTimer
    readonly property var uiTestSuiteRunner: eventLogDialog.uiTestSuiteRunner

    function refreshEventLogModel() { eventLogDialog.refreshEventLogModel() }
    function refreshUnsyncedSummaryModel() { sdSyncDialog.refreshUnsyncedSummaryModel() }
    function openUnsyncedChangesDialog() { sdSyncDialog.openUnsyncedChangesDialog() }
    function loadWebApiBuilderFromScript(s) { webApiKeyConfigWindow.loadWebApiBuilderFromScript(s) }
    function loadAppLaunchBuilderFromScript(s) { appLaunchConfigWindow.loadAppLaunchBuilderFromScript(s) }
    function syncLightingAnimationsForActiveProfile() { duckyColorPickerWindow.syncLightingAnimationsForActiveProfile() }
    function openRenameProfileDialog(profileNumber, currentName) {
        root.contextMenuProfileNumber = profileNumber
        root.contextMenuProfileName = currentName
        renameProfileDialog.openWithProfile(profileNumber, currentName)
    }

    function updateCompilerStatus() {
        if (!deviceController) return
        try {
            const raw = deviceController.checkScriptSyntax(
                scriptEditor.text,
                allowAbortCheck.checked,
                disableRepeatCheck.checked
            )
            const res = JSON.parse(raw)
            root.compileOk = !!res.ok
            root.compileEmpty = !!res.empty
            root.compileByteCount = res.bytes || 0
            root.compileErrorLine = res.errorLine || 0
            root.compileErrorMsg = res.errorMsg || ""
        } catch (e) {
            root.compileOk = true
            root.compileEmpty = false
            root.compileByteCount = 0
            root.compileErrorLine = 0
            root.compileErrorMsg = ""
        }
    }

    property bool selectingProfileFromClick: false

    function ensureActiveProfileVisibleInList() {
        if (!deviceController || !profileSidebarList) return
        const targetProf = deviceController.activeProfile
        for (let i = 0; i < sidebarProfileModel.count; ++i) {
            if (sidebarProfileModel.get(i).profNumber === targetProf) {
                profileSidebarList.positionViewAtIndex(i, ListView.Contain)
                break
            }
        }
    }

    function syncSidebarProfilesFromController() {
        if (!deviceController || !deviceController.profilesJson) return
        const savedContentY = profileSidebarList ? profileSidebarList.contentY : 0
        let rulesByProfile = {}
        try {
            const rArr = JSON.parse(deviceController.autoSwitchRulesJson || "[]")
            if (Array.isArray(rArr)) {
                root.totalRuleCount = rArr.length
                for (let j = 0; j < rArr.length; ++j) {
                    const r = rArr[j]
                    if (!r.enabled) continue
                    const tp = r.targetProfile
                    if (!rulesByProfile[tp]) {
                        rulesByProfile[tp] = []
                    }
                    rulesByProfile[tp].push(r)
                }
            }
        } catch (e) {}

        try {
            const arr = JSON.parse(deviceController.profilesJson)
            if (!Array.isArray(arr)) return
            root.totalProfileCount = arr.length

            const filter = root.profileFilterText.trim().toLowerCase()
            const newItems = []
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                const nameStr = p.name || "Default"
                const profRules = rulesByProfile[p.number] || []
                const boundStr = p.boundClass || (profRules.length > 0 ? (profRules[0].resourceClass || "") : "")
                const boundIconStr = p.boundIcon || (profRules.length > 0 ? (profRules[0].icon || "") : "")
                const numStr = String(p.number)
                let ruleSearchText = boundStr.toLowerCase()
                let tooltipParts = []
                for (let k = 0; k < profRules.length; ++k) {
                    const pr = profRules[k]
                    const clsDisplay = pr.resourceClass && pr.resourceClass.toLowerCase() === "*browser*"
                        ? qsTr("All Web Browsers")
                        : pr.resourceClass
                    const tPart = pr.titleContains && pr.titleContains.length > 0
                        ? (" (title: \"" + pr.titleContains + "\")")
                        : ""
                    tooltipParts.push(clsDisplay + tPart)
                    ruleSearchText += " " + (pr.resourceClass || "") + " " + (pr.titleContains || "")
                    if (pr.resourceClass && pr.resourceClass.toLowerCase() === "*browser*") {
                        ruleSearchText += " all web browsers chrome zen firefox edge brave opera vivaldi"
                    }
                }
                if (filter.length > 0
                    && !nameStr.toLowerCase().includes(filter)
                    && !ruleSearchText.toLowerCase().includes(filter)
                    && !numStr.includes(filter)) {
                    continue
                }
                const tooltipStr = tooltipParts.length > 0
                    ? qsTr("Auto-Switch Application: %1").arg(tooltipParts.join(", "))
                    : (boundStr.length > 0 ? qsTr("Auto-Switch Application: %1").arg(boundStr) : "")

                const bg = Array.isArray(p.bgColor) && p.bgColor.length === 3 ? p.bgColor : [0, 170, 255]
                newItems.push({
                    profNumber: p.number,
                    profName: nameStr,
                    slotCount: p.slotCount || 0,
                    boundClass: boundStr,
                    boundIcon: boundIconStr,
                    boundRuleCount: profRules.length > 0 ? profRules.length : (boundStr.length > 0 ? 1 : 0),
                    boundTooltip: tooltipStr,
                    bgR: bg[0],
                    bgG: bg[1],
                    bgB: bg[2]
                })
            }

            for (let idx = 0; idx < newItems.length; ++idx) {
                if (idx < sidebarProfileModel.count) {
                    sidebarProfileModel.set(idx, newItems[idx])
                } else {
                    sidebarProfileModel.append(newItems[idx])
                }
            }
            if (sidebarProfileModel.count > newItems.length) {
                sidebarProfileModel.remove(newItems.length, sidebarProfileModel.count - newItems.length)
            }

            if (profileSidebarList && savedContentY > 0) {
                const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                profileSidebarList.contentY = Math.min(savedContentY, maxScroll)
            }
        } catch (e) {}
    }

    function clearSlotSelection() {
        root.flushPendingSlotEditsIfModified()
        root.selectedSlotNumber = 0
        hardwareCanvas.selectedSlot = 0
    }

    function captureSlotSnapshot() {
        if (!root.hasSelectedSlot) return
        root.slotSnapshotSlotNumber = root.selectedSlotNumber
        root.slotSnapshotProfileNumber = deviceController ? deviceController.activeProfile : 1
        root.slotSnapshotL1 = label1Input.text
        root.slotSnapshotL2 = label2Input.text
        root.slotSnapshotWL1 = widgetLabel1Input.text
        root.slotSnapshotWL2 = widgetLabel2Input.text
        root.slotSnapshotAbort = allowAbortCheck.checked
        root.slotSnapshotNoRepeat = disableRepeatCheck.checked
        root.slotSnapshotPressScript = root.pressScriptBuffer
        root.slotSnapshotReleaseScript = root.releaseScriptBuffer
        root.slotSnapshotColor = String(root.selectedSlotColor)
    }

    function revertSlotToSnapshot() {
        if (!root.hasSelectedSlot) return
        root.loadingSlotData = true
        label1Input.text = root.slotSnapshotL1
        label2Input.text = root.slotSnapshotL2
        widgetLabel1Input.text = root.slotSnapshotWL1
        widgetLabel2Input.text = root.slotSnapshotWL2
        allowAbortCheck.checked = root.slotSnapshotAbort
        disableRepeatCheck.checked = root.slotSnapshotNoRepeat
        root.pressScriptBuffer = root.slotSnapshotPressScript
        root.releaseScriptBuffer = root.slotSnapshotReleaseScript
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        root.loadingSlotData = false
        root.updateCompilerStatus()
        root.saveCurrentSlotToController()
        hardwareCanvas.syncSlotsFromController()
    }

    function clearSelectedSlotNow() {
        if (!root.hasSelectedSlot) return
        root.loadingSlotData = true
        label1Input.text = ""
        label2Input.text = ""
        widgetLabel1Input.text = ""
        widgetLabel2Input.text = ""
        allowAbortCheck.checked = false
        disableRepeatCheck.checked = false
        root.pressScriptBuffer = ""
        root.releaseScriptBuffer = ""
        scriptEditor.text = ""
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript("")
        root.loadAppLaunchBuilderFromScript("")
        root.loadingSlotData = false
        root.updateCompilerStatus()
        if (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20) {
            deviceController.clearKeyColor(root.selectedSlotNumber)
        }
        root.saveCurrentSlotToController()
        hardwareCanvas.syncSlotsFromController()
        root.selectedSlotColor = hardwareCanvas.slotColorFor(root.selectedSlotNumber)
        root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
    }

    function loadSlotFromController(slotNumber, slotTitle, slotColor) {
        if (slotNumber <= 0 || slotNumber > 58) {
            root.selectedSlotNumber = 0
            return
        }
        const isDifferentSlotOrProfile = (
            root.slotSnapshotSlotNumber !== slotNumber ||
            root.slotSnapshotProfileNumber !== (deviceController ? deviceController.activeProfile : 1)
        )
        root.loadingSlotData = true
        root.selectedSlotNumber = slotNumber
        root.selectedSlotTitle = slotTitle
        root.selectedSlotColor = slotColor

        try {
            const metaJson = deviceController.getSlotMetaJson(slotNumber)
            const meta = JSON.parse(metaJson)
            label1Input.text = meta.l1 || ""
            label2Input.text = meta.l2 || ""
            widgetLabel1Input.text = meta.wl1 || ""
            widgetLabel2Input.text = meta.wl2 || ""
            allowAbortCheck.checked = !!meta.allowAbort
            disableRepeatCheck.checked = !!meta.disableAutorepeat
        } catch (e) {
            label1Input.text = ""
            label2Input.text = ""
            widgetLabel1Input.text = ""
            widgetLabel2Input.text = ""
            allowAbortCheck.checked = false
            disableRepeatCheck.checked = false
        }

        root.pressScriptBuffer = deviceController.getSlotScript(slotNumber, false)
        root.releaseScriptBuffer = deviceController.getSlotScript(slotNumber, true)
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        if (isDifferentSlotOrProfile) {
            root.captureSlotSnapshot()
        }
        root.loadingSlotData = false
        root.updateCompilerStatus()
    }

    function isDuckyDeckStudioFirmware(verStr) {
        const raw = String(verStr || "").trim()
        if (!raw || raw === "-") return false
        const lower = raw.toLowerCase()
        if (lower.indexOf("duckydeck") >= 0 || lower.indexOf("studio") >= 0 || lower.indexOf("dds") >= 0) {
            return true
        }
        const clean = raw.replace(/^[vV]/, "").split(/\s+/)[0]
        const parts = clean.split(".")
        if (parts.length < 3) return false
        const maj = parseInt(parts[0], 10) || 0
        const min = parseInt(parts[1], 10) || 0
        if (maj === 1) return true
        return (maj > 3) || (maj === 3 && min >= 4)
    }

    function cleanFirmwareVersionLabel(verStr) {
        const raw = String(verStr || "-").trim()
        if (!raw || raw === "-") return "-"
        return raw.replace(/\s*\(DuckyDeck Studio\)/i, "").replace(/^[vV]/, "").trim()
    }

    function refreshSidebarTemplateCombo() {
        if (typeof newProfileTemplateCombo !== "undefined" && newProfileTemplateCombo) {
            newProfileTemplateCombo.reloadTemplates()
        }
    }

    function visibleOledChars(str) {
        if (!str) return []
        const raw = Array.from(String(str))
        const out = []
        for (let i = 0; i < raw.length; i++) {
            const cp = raw[i].codePointAt(0)
            if ((cp === 0xFE0E || cp === 0xFE0F || cp === 0x200D) && out.length > 0) {
                out[out.length - 1] += raw[i]
            } else {
                out.push(raw[i])
            }
        }
        return out
    }

    function oledLabelCharCount(str) {
        return visibleOledChars(str).length
    }

    function clampOledLabel(str) {
        return visibleOledChars(str).slice(0, 5).join("")
    }

    function isSingleIconLabel(str) {
        const s = String(str || "").trim()
        if (!s) return false
        const chars = visibleOledChars(s)
        return chars.length === 1 && chars[0].codePointAt(0) > 127
    }

    function saveCurrentSlotToController() {
        if (!root.hasSelectedSlot) {
            return
        }
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = scriptEditor.text
        } else {
            root.pressScriptBuffer = scriptEditor.text
        }

        deviceController.saveSlotConfig(
            root.selectedSlotNumber,
            label1Input.text.trim(),
            label2Input.text.trim(),
            root.pressScriptBuffer,
            root.releaseScriptBuffer,
            allowAbortCheck.checked,
            disableRepeatCheck.checked
        )
        deviceController.saveSlotWidgetLabels(
            root.selectedSlotNumber,
            widgetLabel1Input.text.trim(),
            widgetLabel2Input.text.trim()
        )
    }

    function flushPendingSlotEditsIfModified() {
        if (!root.hasSelectedSlot || root.loadingSlotData || !root.compileOk) {
            return
        }
        const currentPress = root.editingReleaseScript ? root.pressScriptBuffer : scriptEditor.text
        const currentRelease = root.editingReleaseScript ? scriptEditor.text : root.releaseScriptBuffer
        const currentL1 = label1Input.text.trim()
        const currentL2 = label2Input.text.trim()
        const currentWL1 = widgetLabel1Input.text.trim()
        const currentWL2 = widgetLabel2Input.text.trim()
        const currentAbort = allowAbortCheck.checked
        const currentNoRepeat = disableRepeatCheck.checked

        try {
            const meta = JSON.parse(deviceController.getSlotMetaJson(root.selectedSlotNumber))
            const savedL1 = meta.l1 || ""
            const savedL2 = meta.l2 || ""
            const savedWL1 = meta.wl1 || ""
            const savedWL2 = meta.wl2 || ""
            const savedAbort = !!meta.allowAbort
            const savedNoRepeat = !!meta.disableAutorepeat
            const savedPress = deviceController.getSlotScript(root.selectedSlotNumber, false)
            const savedRelease = deviceController.getSlotScript(root.selectedSlotNumber, true)

            if (currentL1 !== savedL1 ||
                currentL2 !== savedL2 ||
                currentWL1 !== savedWL1 ||
                currentWL2 !== savedWL2 ||
                currentAbort !== savedAbort ||
                currentNoRepeat !== savedNoRepeat ||
                currentPress !== savedPress ||
                currentRelease !== savedRelease) {
                root.saveCurrentSlotToController()
                hardwareCanvas.syncSlotsFromController()
            }
        } catch (e) {}
    }

    function refreshSelectedSlotFromController() {
        if (sidebarProfileModel.count === 0) {
            root.syncSidebarProfilesFromController()
        }
        hardwareCanvas.syncSlotsFromController()
        if (root.hasSelectedSlot) {
            root.loadSlotFromController(
                root.selectedSlotNumber,
                hardwareCanvas.slotTitleFor(root.selectedSlotNumber),
                hardwareCanvas.slotColorFor(root.selectedSlotNumber)
            )
        }
    }

    property bool isDarkSystemTheme: true

    Connections {
        target: Kirigami.Theme
        function onBackgroundColorChanged() {
            const dark = Kirigami.Theme.backgroundColor.hslLightness < 0.5
            if (root.isDarkSystemTheme !== dark) {
                root.isDarkSystemTheme = dark
                Qt.callLater(function() {
                    if (deviceController) {
                        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
                    }
                })
            }
        }
    }

    onVisibilityChanged: (visibility) => {
        if (visibility === Window.Minimized) {
            Qt.callLater(function() {
                if (deviceController) {
                    deviceController.minimizeToTray()
                }
            })
        }
    }

    onClosing: (close) => {
        if (deviceController && deviceController.closeToTray) {
            close.accepted = false
            Qt.callLater(function() {
                if (deviceController) {
                    deviceController.minimizeToTray()
                }
            })
        }
    }

    Component.onCompleted: {
        if (deviceController.isStartMinimizedMode()) {
            root.visible = false
            root.hide()
            deviceController.minimizeToTray()
        }
        root.isDarkSystemTheme = Kirigami.Theme.backgroundColor.hslLightness < 0.5
        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
        root.refreshCommandReferenceModel()
        deviceController.refreshDevice()

        const startProf = deviceController.startupProfileArg()
        if (startProf.length > 0) {
            const parsedNum = parseInt(startProf, 10)
            if (!isNaN(parsedNum) && parsedNum >= 1 && parsedNum <= 64) {
                deviceController.switchProfile(parsedNum)
            } else {
                deviceController.switchProfileByName(startProf)
            }
        }

        root.syncSidebarProfilesFromController()
        hardwareCanvas.syncProfilesFromController()
        root.refreshSelectedSlotFromController()
        root.syncLightingAnimationsForActiveProfile()
        Qt.callLater(function() {
            if (deviceController.isStartMinimizedMode()) {
                root.visible = false
                root.hide()
                deviceController.minimizeToTray()
            }
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
            root.refreshSelectedSlotFromController()
            root.syncLightingAnimationsForActiveProfile()
            if (deviceController.connected) {
                root.ensureActiveProfileVisibleInList()
            }
            if (deviceController.isStartupSyncSdRequested()) {
                root.triggerSyncToDeviceSd()
            } else if (deviceController.showFirstRunDialog && !deviceController.isDemoShowcaseMode() && !deviceController.isUiTestSuiteMode()) {
                firstRunSetupDialog.open()
            }
        })
    }

    function triggerSyncToDeviceSd() {
        root.flushPendingSlotEditsIfModified()
        sdSyncDialog.open()
        deviceController.syncProfilesToDeviceSd()
    }

    function triggerReloadFromDeviceSd() {
        sdSyncDialog.open()
        deviceController.reloadProfilesFromDeviceSd()
    }

    DeviceController {
        id: deviceController
        onShowFirstRunDialogChanged: {
            if (deviceController.showFirstRunDialog && !deviceController.isDemoShowcaseMode() && !deviceController.isUiTestSuiteMode()) {
                firstRunSetupDialog.open()
            }
        }
        onSdSyncBusyChanged: {
            if (deviceController.sdSyncBusy) {
                sdSyncDialog.open()
            } else {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                root.refreshSelectedSlotFromController()
            }
        }
        onActiveProfileChanged: {
            if (!root.selectingProfileFromClick) {
                root.ensureActiveProfileVisibleInList()
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
            root.syncLightingAnimationsForActiveProfile()
        }
        onActiveWindowClassChanged: {
            if (deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onAutoSwitchEnabledChanged: {
            if (deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onActiveProfileSlotsJsonChanged: {
            root.refreshSelectedSlotFromController()
        }
        onProfilesJsonChanged: {
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
        }
        onAutoSwitchRulesJsonChanged: {
            root.syncSidebarProfilesFromController()
        }
        onGeneratedScriptPreviewChanged: {
            if (deviceController.generatedScriptPreview.length > 0) {
                scriptEditor.text = deviceController.generatedScriptPreview
                if (root.editingReleaseScript) {
                    root.releaseScriptBuffer = scriptEditor.text
                } else {
                    root.pressScriptBuffer = scriptEditor.text
                }
            }
        }
        onStatusMessageChanged: {
            if (deviceController.statusMessage.length > 0) {
                deviceController.recordStatusLog(deviceController.statusMessage)
            }
        }
        onEventLogJsonChanged: {
            if (eventLogDialog.visible) {
                root.refreshEventLogModel()
            }
        }
    }

    FolderDialog {
        id: sdFolderDialog
        title: qsTr("Select duckyPad microSD Card Root or Profile Directory")
        onAccepted: {
            if (deviceController.loadSdDirectory(selectedFolder.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    FileDialog {
        id: importProfileFileDialog
        title: qsTr("Import DuckyDeck Profile (.duckyprofile.json or config.txt)")
        fileMode: FileDialog.OpenFile
        nameFilters: [
            qsTr("DuckyDeck Profiles (*.duckyprofile.json *.json config.txt)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            if (deviceController.importProfileFromPath(selectedFile.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    FolderDialog {
        id: importProfileFolderDialog
        title: qsTr("Select duckyPad Profile Folder (profile_<Name>) to Import")
        onAccepted: {
            if (deviceController.importProfileFromPath(selectedFolder.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    property int contextMenuProfileNumber: 1
    property string contextMenuProfileName: "Default"
    property int contextMenuProfileIndex: 0
    property int exportTargetProfileNumber: 0

    function openProfileDefaultColorPicker(profNumber, profName, hexColor) {
        root.contextMenuProfileNumber = profNumber
        root.contextMenuProfileName = profName
        let initialHex = "#00aaff"
        if (hexColor && hexColor.length > 0) {
            initialHex = hexColor
        } else if (deviceController && deviceController.activeProfileBgColor.length > 0) {
            initialHex = deviceController.activeProfileBgColor
        }
        duckyColorPickerWindow.openForProfileBg(profNumber, profName, initialHex)
    }

    function openKeyColorPicker(slotNumber, hexColor) {
        const targetSlot = (slotNumber >= 1 && slotNumber <= 20) ? slotNumber : root.selectedSlotNumber
        if (targetSlot < 1 || targetSlot > 20) return
        const initialHex = (hexColor && hexColor.length > 0)
            ? hexColor
            : (hardwareCanvas ? hardwareCanvas.slotColorFor(targetSlot) : root.selectedSlotColor)
        duckyColorPickerWindow.openForKey(targetSlot, initialHex || "#00aaff")
    }

    FileDialog {
        id: saveProfileGroupFileDialog
        title: qsTr("Save All Profiles as Group Bundle (.duckygroup.json)")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "duckygroup.json"
        nameFilters: [
            qsTr("DuckyDeck Profile Group (*.duckygroup.json *.json)")
        ]
        onAccepted: {
            root.flushPendingSlotEditsIfModified()
            deviceController.saveProfileGroup(selectedFile.toString(), "")
        }
    }

    FileDialog {
        id: loadProfileGroupFileDialog
        title: qsTr("Load DuckyDeck Profile Group (.duckygroup.json)")
        fileMode: FileDialog.OpenFile
        nameFilters: [
            qsTr("DuckyDeck Profile Group (*.duckygroup.json *.json)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            if (deviceController.loadProfileGroup(selectedFile.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }


    FileDialog {
        id: exportProfileFileDialog
        title: qsTr("Export Profile #%1 (%2) Bundle (.duckyprofile.json)")
            .arg(root.exportTargetProfileNumber > 0 ? root.exportTargetProfileNumber : deviceController.activeProfile)
            .arg(root.contextMenuProfileName)
        fileMode: FileDialog.SaveFile
        defaultSuffix: "duckyprofile.json"
        nameFilters: [
            qsTr("DuckyDeck Profile Bundle (*.duckyprofile.json *.json)")
        ]
        onAccepted: {
            const targetNum = root.exportTargetProfileNumber > 0
                ? root.exportTargetProfileNumber
                : deviceController.activeProfile
            deviceController.exportProfile(targetNum, selectedFile.toString())
        }
    }

    FolderDialog {
        id: exportProfileFolderDialog
        title: qsTr("Select Destination Folder to Export Profile #%1 (%2)")
            .arg(root.exportTargetProfileNumber > 0 ? root.exportTargetProfileNumber : deviceController.activeProfile)
            .arg(root.contextMenuProfileName)
        onAccepted: {
            const targetNum = root.exportTargetProfileNumber > 0
                ? root.exportTargetProfileNumber
                : deviceController.activeProfile
            deviceController.exportProfile(targetNum, selectedFolder.toString())
        }
    }

    ColorPickerWindow {
        id: duckyColorPickerWindow
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        isLightTheme: root.isLightTheme
        highContrastPositiveText: root.highContrastPositiveText
        hardwareCanvas: hardwareCanvas
        scriptEditor: scriptEditor
        profilePropertiesWindow: settingsDialog.profilePropertiesWindow
        keyPropertiesWindow: settingsDialog.keyPropertiesWindow
    }

    // Profile Right-Click Context Menu
    QQC2.Menu {
        font.pointSize: 10 * root.fontScale
        id: profileContextMenu
        title: qsTr("Profile Actions")

        QQC2.MenuItem {
            text: qsTr("Profile #%1 — %2")
                .arg(root.contextMenuProfileNumber)
                .arg(root.contextMenuProfileName)
            enabled: false
            font.bold: true
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-properties"
            text: qsTr("Properties...")
            onTriggered: {
                profilePropertiesWindow.openForProfile(root.contextMenuProfileNumber)
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-export"
            text: qsTr("Export Profile (.duckyprofile.json)...")
            onTriggered: {
                root.exportTargetProfileNumber = root.contextMenuProfileNumber
                exportProfileFileDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "folder"
            text: qsTr("Export Profile to Folder...")
            onTriggered: {
                root.exportTargetProfileNumber = root.contextMenuProfileNumber
                exportProfileFolderDialog.open()
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "bookmark-new"
            text: qsTr("Save Profile as Template...")
            onTriggered: {
                profileTemplatesDialog.openSaveFromProfile(
                    root.contextMenuProfileNumber,
                    root.contextMenuProfileName
                )
            }
        }

        QQC2.MenuItem {
            icon.name: "document-new-from-template"
            text: qsTr("Apply Template to Profile...")
            onTriggered: {
                profileTemplatesDialog.openApplyToProfile(
                    root.contextMenuProfileNumber,
                    root.contextMenuProfileName
                )
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "edit-delete"
            text: root.contextMenuProfileNumber === deviceController.defaultFallbackProfile
                ? qsTr("Delete Profile... (Fallback Profile)")
                : qsTr("Delete Profile...")
            enabled: profileSidebarList.count > 1
                && root.contextMenuProfileNumber !== deviceController.defaultFallbackProfile
            onTriggered: {
                deleteProfileDialog.open()
            }
        }
    }


    QQC2.Dialog {
        id: deleteProfileDialog
        readonly property bool closableWithEscape: true
        enter: null
        exit: null
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: deleteProfileDialog.visible
            onActivated: deleteProfileDialog.close()
        }
        anchors.centerIn: parent
        modal: true
        width: 400
        title: qsTr("Delete Profile #%1 — %2?")
            .arg(root.contextMenuProfileNumber)
            .arg(root.contextMenuProfileName)
        standardButtons: QQC2.Dialog.Yes | QQC2.Dialog.Cancel

        ColumnLayout {
            width: parent.width
            spacing: 8

            QQC2.Label {
                font.pointSize: 10 * root.fontScale
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: qsTr("Are you sure you want to delete Profile #%1 (%2)?\n\nAny Auto-Switcher rules targeting this profile will be updated to reference the Fallback Profile, and remaining profiles will be renumbered.")
                    .arg(root.contextMenuProfileNumber)
                    .arg(root.contextMenuProfileName)
            }
        }

        onAccepted: {
            if (root.contextMenuProfileNumber === deviceController.defaultFallbackProfile) {
                return
            }
            const savedY = profileSidebarList.contentY
            root.selectingProfileFromClick = true
            if (deviceController.deleteProfile(root.contextMenuProfileNumber)) {
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = Math.min(
                    savedY,
                    Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                )
            }
            root.selectingProfileFromClick = false
        }
    }

    PgvInspectorDrawer {
        id: pgvDrawer
        controller: deviceController
    }

    AutoSwitchDrawer {
        id: autoSwitchDrawer
        transientParent: root
        controller: deviceController
    }

    AiArchitectModal {
        id: aiModal
        controller: deviceController
    }

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

    // Settings Window (Appearance, System Integration, HTTP REST API, and Global Webhook)
    SettingsPage {
        id: settingsDialog
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        isLightTheme: root.isLightTheme
        highContrastHighlightText: root.highContrastHighlightText
        highContrastPositiveText: root.highContrastPositiveText
        highContrastWarningText: root.highContrastWarningText
        saveProfileGroupFileDialog: saveProfileGroupFileDialog
        duckyColorPickerWindow: duckyColorPickerWindow
        autoSwitchDrawer: autoSwitchDrawer
        profileSidebarList: profileSidebarList
        hardwareCanvas: hardwareCanvas
        appLaunchConfigWindow: appLaunchConfigWindow
        webApiKeyConfigWindow: webApiKeyConfigWindow
        oledDesktopWidget: oledDesktopWidget
    }

    // Dedicated Window for Configuring Named Reusable Web API Destinations

    // Profile Sync Status Menu
    QQC2.Menu {
        font.pointSize: 10 * root.fontScale
        id: profileSyncMenu
        title: qsTr("Profile Sync Status")

        QQC2.MenuItem {
            text: deviceController.profileSyncState === "synced"
                ? qsTr("✓ Validated & Synced with duckyPad SD")
                : (deviceController.profileSyncState === "unsynced"
                    ? qsTr("⚠ Unsynced Profile Changes")
                    : qsTr("⚠ Profile Mismatch Detected"))
            enabled: false
            font.bold: true
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "view-list-details"
            text: qsTr("📋 Review Unsynced Changes Summary...")
            onTriggered: root.openUnsyncedChangesDialog()
        }

        QQC2.MenuItem {
            icon.name: "document-save"
            text: qsTr("⬆ Sync (Write) Changes to duckyPad SD")
            enabled: !deviceController.sdSyncBusy
                && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerSyncToDeviceSd()
        }

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("⬇ Reload (Read) Profiles from duckyPad SD")
            enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerReloadFromDeviceSd()
        }

        QQC2.MenuItem {
            icon.name: "dialog-information"
            text: qsTr("View Sync Progress & Validation Details...")
            onTriggered: {
                root.refreshUnsyncedSummaryModel()
                sdSyncDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
        }
    }

    // Hamburger Menu for Action Buttons
    QQC2.Menu {
        font.pointSize: 10 * root.fontScale
        id: hamburgerMenu
        title: qsTr("Menu")
        onAboutToShow: deviceController.refreshSdMountStatus()

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("Scan USB HID")
            onTriggered: {
                deviceController.refreshDevice()
                root.refreshSelectedSlotFromController()
            }
        }

        QQC2.MenuItem {
            icon.name: "view-list-details"
            text: qsTr("📋 Review Unsynced Changes Summary...")
            onTriggered: root.openUnsyncedChangesDialog()
        }

        QQC2.MenuItem {
            icon.name: "document-save"
            text: deviceController.profileSyncState === "unsynced"
                ? qsTr("⬆ Sync Changes to duckyPad SD (Unsynced)")
                : qsTr("Sync Profiles to duckyPad SD")
            enabled: !deviceController.sdSyncBusy
                && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerSyncToDeviceSd()
        }

        QQC2.MenuItem {
            visible: deviceController.sdCardDetected && deviceController.sdMountPath.length === 0
            height: visible ? implicitHeight : 0
            icon.name: "media-mount"
            text: qsTr("Mount SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdCardDetected && deviceController.sdMountPath.length === 0
            onTriggered: {
                deviceController.mountSdCard()
            }
        }

        QQC2.MenuItem {
            icon.name: "media-mount"
            text: qsTr("Mount and Scan SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdMountPath.length === 0 && (deviceController.sdCardDetected || deviceController.connected)
            onTriggered: root.triggerReloadFromDeviceSd()
        }

        QQC2.MenuItem {
            visible: deviceController.sdMountPath.length > 0
            height: visible ? implicitHeight : 0
            icon.name: "media-flash-sd-mmc"
            text: qsTr("Scan Mounted SD Card")
            onTriggered: {
                const found = deviceController.scanMountedSdCard()
                if (found.length > 0) {
                    root.refreshSelectedSlotFromController()
                } else {
                    sdFolderDialog.open()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "media-eject"
            text: qsTr("Unmount SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdMountPath.length > 0
            onTriggered: {
                if (deviceController.unmountSdCard()) {
                    root.refreshSelectedSlotFromController()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-new-from-template"
            text: qsTr("Profile Templates Manager...")
            onTriggered: profileTemplatesDialog.openGeneral()
        }

        QQC2.MenuItem {
            icon.name: "document-new"
            text: qsTr("New Profile Group...")
            onTriggered: newProfileGroupDialog.openWithDefaults()
        }

        QQC2.MenuItem {
            icon.name: "document-open"
            text: qsTr("Load Profile Group (.duckygroup.json)...")
            onTriggered: loadProfileGroupFileDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "document-save-all"
            text: qsTr("Save All Profiles as Group (.duckygroup.json)...")
            onTriggered: saveProfileGroupFileDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-import"
            text: qsTr("Import Single Profile (File / config.txt)...")
            onTriggered: importProfileFileDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Import Single Profile Folder...")
            onTriggered: importProfileFolderDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "system-switch-user"
            text: qsTr("Import Deck / dekuNukem Setup & Auto-Switcher...")
            onTriggered: firstRunSetupDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "preferences-system-windows"
            text: qsTr("Auto-Switcher Rules...")
            onTriggered: autoSwitchDrawer.open()
        }

        QQC2.MenuItem {
            icon.name: "preferences-system-time"
            text: qsTr("Sync RTC Clock")
            enabled: deviceController.connected
            onTriggered: deviceController.syncRtcClock()
        }

        QQC2.MenuItem {
            icon.name: "system-reboot"
            text: qsTr("Reboot duckyPad")
            enabled: deviceController.connected
            onTriggered: deviceController.rebootDevice()
        }

        QQC2.MenuItem {
            icon.name: "system-software-update"
            text: deviceController.firmwareUpdateAvailable
                ? qsTr("⬆ Firmware Manager & Updater (v%1 Available)...").arg(deviceController.latestFirmwareVersion)
                : qsTr("Firmware Manager & Updater...")
            onTriggered: {
                deviceController.checkFirmwareUpdate(true)
                firmwareUpdateDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "format-list-ordered"
            text: qsTr("Variables (_GV)...")
            onTriggered: {
                pgvDrawer.readAllFromDevice()
                pgvDrawer.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "system-run"
            text: qsTr("Macro Template Generator (experimental)...")
            onTriggered: aiModal.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "settings-configure"
            text: qsTr("Settings...")
            onTriggered: settingsDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "view-list-text"
            text: qsTr("System Event Log (%1)...").arg(deviceController.eventLogCount)
            onTriggered: eventLogDialog.openLogWindow()
        }

        QQC2.MenuItem {
            id: menuUserDocumentationItem
            icon.name: "help-contents"
            text: qsTr("User Documentation (Web Guide)...")
            onTriggered: deviceController.openUserGuide()
        }

        QQC2.MenuItem {
            icon.name: "emblem-favorite"
            text: qsTr("☕ Buy Me a Coffee (Ko-fi)...")
            onTriggered: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
        }

        QQC2.MenuItem {
            icon.name: "help-about"
            text: qsTr("About DuckyDeck Studio...")
            onTriggered: aboutDialog.openAboutWindow()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "application-exit"
            text: qsTr("Quit DuckyDeck Studio")
            onTriggered: deviceController.quitApplication()
        }
    }

    EventLogDialog {
        id: eventLogDialog
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        isLightTheme: root.isLightTheme
        highContrastHighlightText: root.highContrastHighlightText
        duckyColorPickerWindow: duckyColorPickerWindow
        webApiKeyConfigWindow: webApiKeyConfigWindow
        appLaunchConfigWindow: appLaunchConfigWindow
        sdSyncDialog: sdSyncDialog
        settingsDialog: settingsDialog
        pgvDrawer: pgvDrawer
        autoSwitchDrawer: autoSwitchDrawer
        aiModal: aiModal
        oledDesktopWidget: oledDesktopWidget
        hardwareCanvas: hardwareCanvas
        profilesBoxScrollBar: mainProfilesBoxScrollBar
        scriptScrollView: scriptScrollView
        scriptEditorVerticalScrollBar: mainScriptEditorVerticalScrollBar
        scriptEditor: scriptEditor
    }


    SdSyncDialog {
        id: sdSyncDialog
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        effectiveThemeMode: root.effectiveThemeMode
        hardwareCanvas: hardwareCanvas
        inspectorCard: inspectorCard
    }

    pageStack.globalToolBar.style: Kirigami.ApplicationHeaderStyle.None

    pageStack.initialPage: Kirigami.Page {
        padding: 0

        background: Rectangle {
            color: Kirigami.Theme.backgroundColor
        }

        header: Rectangle {
            Kirigami.Theme.colorSet: Kirigami.Theme.Window
            Kirigami.Theme.inherit: false
            width: parent.width
            implicitHeight: 46
            color: Kirigami.Theme.backgroundColor

            Rectangle {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                height: 1
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Kirigami.Units.largeSpacing
                anchors.rightMargin: Kirigami.Units.largeSpacing
                spacing: Kirigami.Units.mediumSpacing

                QQC2.Label {
                    text: qsTr("DuckyDeck Studio v%1").arg(deviceController.appVersion)
                    font.bold: true
                    font.pointSize: 12 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                Rectangle {
                    radius: 5
                    color: Qt.tint(
                        Kirigami.Theme.alternateBackgroundColor,
                        Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16)
                    )
                    border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                    border.width: 1
                    implicitWidth: activeProfHeaderRow.implicitWidth + 16
                    implicitHeight: 28

                    RowLayout {
                        id: activeProfHeaderRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            id: headerProfColorSwatch
                            width: 12
                            height: 12
                            radius: 6
                            color: deviceController.activeProfileBgColor.length > 0
                                ? deviceController.activeProfileBgColor
                                : "#00aaff"
                            border.width: 1
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.55)

                            MouseArea {
                                id: headerSwatchMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: root.openProfileDefaultColorPicker(
                                    deviceController.activeProfile,
                                    deviceController.activeProfileName,
                                    deviceController.activeProfileBgColor
                                )
                            }

                            QQC2.ToolTip.visible: headerSwatchMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("Click to change Default Button Color (BG_COLOR) for Profile #%1").arg(deviceController.activeProfile)
                        }

                        QQC2.Label {
                            id: activeProfHeaderLabel
                            text: qsTr("Profile #%1 — %2")
                                .arg(deviceController.activeProfile)
                                .arg(deviceController.activeProfileName)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }
                }

                Item { Layout.fillWidth: true }

                // Ko-fi Buy Me a Coffee Widget beside Hamburger Menu
                Rectangle {
                    id: headerKofiWidget
                    radius: 15
                    color: headerKofiMouse.containsMouse ? "#c81e1e" : "#d93834"
                    border.color: "#ffffff33"
                    border.width: 1
                    implicitWidth: headerKofiRow.implicitWidth + 20
                    implicitHeight: 30

                    RowLayout {
                        id: headerKofiRow
                        anchors.centerIn: parent
                        spacing: 6

                        QQC2.Label {
                            text: "☕"
                            font.pointSize: 11 * root.fontScale
                        }

                        QQC2.Label {
                            text: qsTr("Buy Me a Coffee")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: "#ffffff"
                        }
                    }

                    MouseArea {
                        id: headerKofiMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                    }

                    QQC2.ToolTip.visible: headerKofiMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Buy Russell Milliner a coffee on Ko-fi (https://ko-fi.com/russellmilliner)")
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    id: hamburgerMenuBtn
                    icon.name: "application-menu"
                    display: QQC2.AbstractButton.IconOnly
                    implicitWidth: 36
                    implicitHeight: 32
                    onClicked: hamburgerMenu.popup(hamburgerMenuBtn, 0, hamburgerMenuBtn.height)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Menu (Device Actions, Profile Groups, Import/Export, Rules, Variables, AI & Settings)")
                }
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            UdevBanner {
                controller: deviceController
            }

            QQC2.SplitView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                orientation: Qt.Horizontal

                handle: Rectangle {
                    implicitWidth: 8
                    color: "transparent"

                    Rectangle {
                        anchors.centerIn: parent
                        width: 2
                        height: 48
                        radius: 1
                        color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    }
                }

                // Column 1: Profiles & Auto-Switcher Sidebar
                Rectangle {
                    id: profilesSidebar
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property color frameBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

                    QQC2.SplitView.preferredWidth: Math.max(
                        Math.round(300 * Math.max(1.0, root.sidebarScale * 0.95)),
                        profileActionsRow.implicitWidth + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.minimumWidth: Math.max(
                        Math.round(280 * Math.max(1.0, root.sidebarScale * 0.92)),
                        profileActionsRow.implicitWidth + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.maximumWidth: 460
                    QQC2.SplitView.fillHeight: true
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: profilesSidebar.frameBorder
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // PROFILES Header
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("PROFILES (%1/64)").arg(root.totalProfileCount)
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true

                                HoverHandler {
                                    id: profilesHeaderHover
                                }

                                QQC2.ToolTip.visible: profilesHeaderHover.hovered
                                QQC2.ToolTip.delay: 200
                                QQC2.ToolTip.text: qsTr("Drag ⠿ to Reorder • Double-click to Rename • Right-click for Options")
                            }
                        }

                        // Filter Search Input
                        QQC2.TextField {
                            id: profileFilterField
                            objectName: "profileFilterField"
                            Layout.fillWidth: true
                            placeholderText: qsTr("Filter profiles or apps...")
                            selectByMouse: true
                            font.pointSize: 10 * root.fontScale
                            rightPadding: clearProfileFilterBtn.visible ? 28 : 8
                            onTextChanged: {
                                root.profileFilterText = text
                                root.syncSidebarProfilesFromController()
                            }

                            QQC2.ToolButton {
                                id: clearProfileFilterBtn
                                objectName: "clearProfileFilterButton"
                                visible: profileFilterField.text.length > 0
                                anchors.right: parent.right
                                anchors.rightMargin: 3
                                anchors.verticalCenter: parent.verticalCenter
                                width: 22
                                height: 22
                                implicitWidth: 22
                                implicitHeight: 22
                                padding: 0
                                flat: true
                                text: "✕"
                                font.pointSize: 9 * root.fontScale
                                onClicked: {
                                    profileFilterField.text = ""
                                    profileFilterField.forceActiveFocus()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Clear profile filter")
                            }
                        }

                        // Scrollable 64-Profile List with Drag-and-Drop Reordering
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            ListView {
                                id: profileSidebarList
                                anchors.fill: parent
                                clip: true
                                model: sidebarProfileModel
                                spacing: 4
                                boundsBehavior: Flickable.StopAtBounds
                                highlightFollowsCurrentItem: false

                                property int draggedProfileIndex: -1
                                property int dropTargetIndex: -1
                                readonly property bool canReorder: root.profileFilterText.trim().length === 0

                                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                    id: mainProfilesBoxScrollBar
                                    parent: profileSidebarList
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    policy: profileSidebarList.contentHeight > profileSidebarList.height
                                        ? QQC2.ScrollBar.AlwaysOn
                                        : QQC2.ScrollBar.AsNeeded
                                }

                                delegate: Item {
                                    id: profDelegate
                                    required property int index
                                    required property int profNumber
                                    required property string profName
                                    required property int slotCount
                                    required property string boundClass
                                    required property string boundIcon
                                    required property int boundRuleCount
                                    required property string boundTooltip
                                    required property int bgR
                                    required property int bgG
                                    required property int bgB

                                    readonly property bool isCurrent: deviceController.activeProfile === profNumber
                                    readonly property bool isBeingDragged: profileSidebarList.draggedProfileIndex === index
                                    readonly property bool isDropTarget: profileSidebarList.draggedProfileIndex !== -1
                                        && profileSidebarList.dropTargetIndex === index
                                        && profileSidebarList.draggedProfileIndex !== index
                                    readonly property bool isRowHovered: profRowHover.hovered
                                        || profMouse.containsMouse
                                        || profSwatchMouse.containsMouse
                                    readonly property color profColor: Qt.rgba(bgR / 255.0, bgG / 255.0, bgB / 255.0, 1.0)
                                    readonly property real rowStride: height + profileSidebarList.spacing
                                    readonly property real shiftOffsetY: {
                                        const fromIdx = profileSidebarList.draggedProfileIndex
                                        const toIdx = profileSidebarList.dropTargetIndex
                                        if (fromIdx < 0 || toIdx < 0 || fromIdx === toIdx || index === fromIdx) {
                                            return 0
                                        }
                                        if (fromIdx < toIdx && index > fromIdx && index <= toIdx) {
                                            return -rowStride
                                        }
                                        if (fromIdx > toIdx && index >= toIdx && index < fromIdx) {
                                            return rowStride
                                        }
                                        return 0
                                    }

                                    width: Math.max(
                                        100,
                                        profileSidebarList.width - (profileSidebarList.contentHeight > profileSidebarList.height ? 14 : 2)
                                    )
                                    height: Math.round(40 * root.sidebarScale)
                                    z: isBeingDragged ? 100 : 1

                                    HoverHandler {
                                        id: profRowHover
                                    }

                                    QQC2.ToolTip.visible: profMouse.containsMouse
                                        && !profMouse.dragging
                                        && !profSwatchMouse.containsMouse
                                        && !(appIconHover.hovered && profDelegate.boundTooltip.length > 0)
                                    QQC2.ToolTip.delay: 300
                                    QQC2.ToolTip.text: profileSidebarList.canReorder
                                        ? qsTr("Drag ⠿ to Reorder • Double-click to Rename • Right-click for Options")
                                        : qsTr("Double-click to Rename • Right-click for Options")

                                    // Drop target slot outline shown in the target gap while dragging
                                    Rectangle {
                                        visible: profileSidebarList.draggedProfileIndex !== -1
                                            && profileSidebarList.dropTargetIndex === profDelegate.index
                                        anchors.fill: parent
                                        radius: 6
                                        color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.10)
                                        border.width: 1.5
                                        border.color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.55)
                                    }

                                    MouseArea {
                                        id: profMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                                        cursorShape: profMouse.dragging
                                            ? Qt.ClosedHandCursor
                                            : Qt.PointingHandCursor

                                        property real pressX: 0
                                        property real pressY: 0
                                        property real dragOffsetX: 0
                                        property real dragOffsetY: 0
                                        property bool dragging: false

                                        onPressed: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                root.contextMenuProfileNumber = profDelegate.profNumber
                                                root.contextMenuProfileName = profDelegate.profName
                                                root.contextMenuProfileIndex = profDelegate.index
                                                root.exportTargetProfileNumber = profDelegate.profNumber
                                                profileContextMenu.popup()
                                                return
                                            }
                                            pressX = mouse.x
                                            pressY = mouse.y
                                            dragOffsetX = 0
                                            dragOffsetY = 0
                                            dragging = false
                                        }

                                        onPositionChanged: (mouse) => {
                                            if (!pressed || (mouse.buttons & Qt.RightButton) || !profileSidebarList.canReorder) return
                                            if (!dragging && Math.abs(mouse.y - pressY) > 5) {
                                                dragging = true
                                                profileSidebarList.interactive = false
                                                profileSidebarList.draggedProfileIndex = profDelegate.index
                                                profileSidebarList.dropTargetIndex = profDelegate.index
                                            }
                                            if (dragging) {
                                                // Auto-scroll when dragging near the top or bottom of the viewport
                                                const posInView = profMouse.mapToItem(profileSidebarList, mouse.x, mouse.y)
                                                if (posInView.y < 24 && profileSidebarList.contentY > 0) {
                                                    profileSidebarList.contentY = Math.max(0, profileSidebarList.contentY - 10)
                                                } else if (posInView.y > profileSidebarList.height - 24) {
                                                    const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                                                    profileSidebarList.contentY = Math.min(maxScroll, profileSidebarList.contentY + 10)
                                                }

                                                dragOffsetX = Math.max(-8, Math.min(8, mouse.x - pressX))
                                                dragOffsetY = mouse.y - pressY

                                                const draggedCenterY = profDelegate.y + dragOffsetY + profDelegate.height * 0.5
                                                const targetIdx = Math.max(
                                                    0,
                                                    Math.min(
                                                        Math.max(0, profileSidebarList.count - 1),
                                                        Math.floor(draggedCenterY / profDelegate.rowStride)
                                                    )
                                                )
                                                profileSidebarList.dropTargetIndex = targetIdx
                                            }
                                        }

                                        onReleased: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                return
                                            }
                                            if (dragging) {
                                                const fromIdx = profileSidebarList.draggedProfileIndex
                                                const toIdx = profileSidebarList.dropTargetIndex
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                profileSidebarList.draggedProfileIndex = -1
                                                profileSidebarList.dropTargetIndex = -1
                                                profileSidebarList.interactive = true
                                                if (fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                    const savedY = profileSidebarList.contentY
                                                    deviceController.reorderProfile(fromIdx, toIdx)
                                                    root.refreshSelectedSlotFromController()
                                                    profileSidebarList.contentY = savedY
                                                }
                                            } else {
                                                root.flushPendingSlotEditsIfModified()
                                                const savedY = profileSidebarList.contentY
                                                root.selectingProfileFromClick = true
                                                deviceController.selectProfile(profDelegate.profNumber)
                                                root.refreshSelectedSlotFromController()
                                                profileSidebarList.contentY = savedY
                                                root.selectingProfileFromClick = false
                                            }
                                        }

                                        onDoubleClicked: (mouse) => {
                                            if (mouse.button === Qt.LeftButton) {
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                root.openRenameProfileDialog(profDelegate.profNumber, profDelegate.profName)
                                            }
                                        }

                                        onCanceled: {
                                            dragging = false
                                            dragOffsetX = 0
                                            dragOffsetY = 0
                                            profileSidebarList.draggedProfileIndex = -1
                                            profileSidebarList.dropTargetIndex = -1
                                            profileSidebarList.interactive = true
                                        }
                                    }

                                    Rectangle {
                                        id: profCard
                                        width: parent.width
                                        height: parent.height
                                        x: profDelegate.isBeingDragged ? profMouse.dragOffsetX : 0
                                        y: profDelegate.isBeingDragged ? profMouse.dragOffsetY : profDelegate.shiftOffsetY
                                        scale: profDelegate.isBeingDragged ? 1.02 : 1.0
                                        radius: 6
                                        opacity: profDelegate.isBeingDragged ? 0.96 : 1.0
                                        color: profDelegate.isBeingDragged
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.32))
                                            : (profDelegate.isCurrent
                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.22))
                                                : "transparent")
                                        border.width: (profDelegate.isCurrent || profDelegate.isBeingDragged) ? 1.5 : 0
                                        border.color: (profDelegate.isCurrent || profDelegate.isBeingDragged)
                                            ? Kirigami.Theme.highlightColor
                                            : "transparent"

                                        Behavior on y {
                                            enabled: profileSidebarList.draggedProfileIndex !== -1 && !profDelegate.isBeingDragged
                                            NumberAnimation {
                                                duration: 120
                                                easing.type: Easing.OutCubic
                                            }
                                        }

                                        Behavior on scale {
                                            NumberAnimation {
                                                duration: 100
                                                easing.type: Easing.OutCubic
                                            }
                                        }

                                        RowLayout {
                                            anchors.fill: parent
                                            anchors.leftMargin: 6
                                            anchors.rightMargin: 6
                                            spacing: 6

                                            // Drag Handle Grip
                                            QQC2.Label {
                                                visible: profileSidebarList.canReorder
                                                text: "⠿"
                                                font.pointSize: 11 * root.fontScale
                                                color: profDelegate.isCurrent || profDelegate.isBeingDragged
                                                    ? Kirigami.Theme.textColor
                                                    : Kirigami.Theme.disabledTextColor
                                            }

                                            // Profile Default Button Color Swatch (Click to change BG_COLOR)
                                            Rectangle {
                                                width: 6
                                                height: Math.round(24 * root.sidebarScale)
                                                radius: 3
                                                color: profDelegate.profColor
                                                border.width: profSwatchMouse.containsMouse ? 1 : 0
                                                border.color: Kirigami.Theme.textColor

                                                MouseArea {
                                                    id: profSwatchMouse
                                                    anchors.fill: parent
                                                    anchors.margins: -3
                                                    hoverEnabled: true
                                                    cursorShape: Qt.PointingHandCursor
                                                    onClicked: {
                                                        root.openProfileDefaultColorPicker(
                                                            profDelegate.profNumber,
                                                            profDelegate.profName,
                                                            profDelegate.profColor.toString()
                                                        )
                                                    }
                                                }

                                                QQC2.ToolTip.visible: profSwatchMouse.containsMouse
                                                QQC2.ToolTip.text: qsTr("Click to change Default Button Color (BG_COLOR) for Profile #%1").arg(profDelegate.profNumber)
                                            }

                                            QQC2.Label {
                                                text: profDelegate.profNumber + ". " + profDelegate.profName
                                                font.bold: profDelegate.isCurrent || profDelegate.isBeingDragged
                                                font.pointSize: 12 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                            }

                                            // Auto-Switch Associated Application Icon on the right side of the profile
                                            RowLayout {
                                                visible: profDelegate.boundIcon.length > 0 || profDelegate.boundClass.length > 0
                                                spacing: 3
                                                Layout.alignment: Qt.AlignVCenter

                                                Kirigami.Icon {
                                                    source: profDelegate.boundIcon.length > 0
                                                        ? profDelegate.boundIcon
                                                        : "application-x-executable"
                                                    Layout.preferredWidth: Math.round(22 * root.sidebarScale)
                                                    Layout.preferredHeight: Math.round(22 * root.sidebarScale)
                                                    isMask: false

                                                    HoverHandler {
                                                        id: appIconHover
                                                    }

                                                    QQC2.ToolTip.visible: appIconHover.hovered && profDelegate.boundTooltip.length > 0
                                                    QQC2.ToolTip.text: profDelegate.boundTooltip
                                                }

                                                QQC2.Label {
                                                    visible: profDelegate.boundRuleCount > 1
                                                    text: "+" + (profDelegate.boundRuleCount - 1)
                                                    font.pointSize: 10 * root.fontScale
                                                    font.bold: true
                                                    color: root.highContrastHighlightText
                                                }
                                            }

                                            // Fallback Key Count Badge when no Auto-Switch App is associated
                                            Rectangle {
                                                visible: profDelegate.boundIcon.length === 0 && profDelegate.boundClass.length === 0
                                                radius: 4
                                                color: Kirigami.Theme.alternateBackgroundColor
                                                border.color: profilesSidebar.frameBorder
                                                border.width: 1
                                                implicitWidth: badgeText.implicitWidth + 8
                                                implicitHeight: Math.round(20 * root.sidebarScale)

                                                QQC2.Label {
                                                    id: badgeText
                                                    anchors.centerIn: parent
                                                    text: profDelegate.slotCount + "k"
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Inline New Profile Creator (when toggled)
                        ColumnLayout {
                            Layout.fillWidth: true
                            visible: root.showNewProfileInput
                            spacing: 4

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.ComboBox {
                                    id: newProfileTemplateCombo
                                    Layout.fillWidth: true
                                    font.pointSize: 10 * root.fontScale
                                    implicitHeight: Math.round(24 * root.sidebarScale)
                                    property var templateIds: ["builtin_blank"]
                                    property var templateDefaultNames: ["NewProfile"]

                                    function reloadTemplates() {
                                        let ids = []
                                        let defNames = []
                                        let labels = []
                                        try {
                                            const arr = JSON.parse(deviceController.listProfileTemplatesJson())
                                            for (let i = 0; i < arr.length; i++) {
                                                ids.push(arr[i].id)
                                                defNames.push(arr[i].defaultProfileName || arr[i].name)
                                                labels.push((arr[i].builtIn ? "★ " : "📋 ") + arr[i].name)
                                            }
                                        } catch (e) {}
                                        if (ids.length === 0) {
                                            ids = ["builtin_blank"]
                                            defNames = ["NewProfile"]
                                            labels = [qsTr("★ Blank Profile")]
                                        }
                                        templateIds = ids
                                        templateDefaultNames = defNames
                                        model = labels
                                        currentIndex = 0
                                    }

                                    Component.onCompleted: reloadTemplates()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Select a template for the new profile")
                                }

                                QQC2.Button {
                                    icon.name: "bookmark-new"
                                    text: qsTr("Templates...")
                                    implicitHeight: Math.round(24 * root.sidebarScale)
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: profileTemplatesDialog.openGeneral()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Manage profile templates or save an existing profile as a template")
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.TextField {
                                    id: newProfileNameField
                                    Layout.fillWidth: true
                                    maximumLength: 16
                                    placeholderText: qsTr("Profile name (optional)...")
                                    font.pointSize: 10 * root.fontScale
                                    onAccepted: createProfBtn.clicked()
                                }

                                QQC2.Button {
                                    id: createProfBtn
                                    text: qsTr("Add")
                                    implicitHeight: Math.round(26 * root.sidebarScale)
                                    font.pointSize: 10 * root.fontScale
                                    highlighted: true
                                    onClicked: {
                                        const tIdx = Math.max(0, newProfileTemplateCombo.currentIndex)
                                        const tplId = newProfileTemplateCombo.templateIds[tIdx] || "builtin_blank"
                                        const fallbackName = newProfileTemplateCombo.templateDefaultNames[tIdx] || "NewProfile"
                                        const chosenName = newProfileNameField.text.trim().length > 0
                                            ? newProfileNameField.text.trim()
                                            : fallbackName
                                        if (deviceController.createProfileFromTemplate(chosenName, tplId)) {
                                            newProfileNameField.text = ""
                                            root.showNewProfileInput = false
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncProfilesFromController()
                                            root.refreshSelectedSlotFromController()
                                        }
                                    }
                                }
                            }
                        }

                        // Single Row: + New Profile / Import Profile / Groups ▾ Menu
                        RowLayout {
                            id: profileActionsRow
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "list-add"
                                text: qsTr("New")
                                font.pointSize: 10 * root.fontScale
                                onClicked: {
                                    root.showNewProfileInput = !root.showNewProfileInput
                                    if (root.showNewProfileInput) {
                                        newProfileTemplateCombo.reloadTemplates()
                                        newProfileNameField.forceActiveFocus()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Create a new profile from blank or from a template (Right-click a profile for Templates & Export)")
                            }

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "document-import"
                                text: qsTr("Import")
                                font.pointSize: 10 * root.fontScale
                                onClicked: importProfileFileDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Import a profile from .duckyprofile.json or config.txt")
                            }

                            QQC2.Button {
                                id: profileGroupsMenuBtn
                                Layout.fillWidth: true
                                icon.name: "folder"
                                text: qsTr("Groups ▾")
                                font.pointSize: 10 * root.fontScale
                                onClicked: profileGroupMenu.popup(
                                    profileGroupsMenuBtn,
                                    0,
                                    -profileGroupMenu.implicitHeight
                                )
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Manage Profile Templates and Profile Groups (.duckygroup.json)")

                                QQC2.Menu {
                                    font.pointSize: 10 * root.fontScale
                                    id: profileGroupMenu
                                    title: qsTr("Templates & Profile Groups")

                                    QQC2.MenuItem {
                                        icon.name: "document-new-from-template"
                                        text: qsTr("Profile Templates Manager...")
                                        onTriggered: profileTemplatesDialog.openGeneral()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "bookmark-new"
                                        text: qsTr("Save Active Profile as Template...")
                                        onTriggered: profileTemplatesDialog.openSaveFromProfile(
                                            deviceController.activeProfile,
                                            deviceController.activeProfileName
                                        )
                                    }

                                    QQC2.MenuSeparator {}

                                    QQC2.MenuItem {
                                        icon.name: "document-new"
                                        text: qsTr("New Profile Group...")
                                        onTriggered: newProfileGroupDialog.openWithDefaults()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "document-open"
                                        text: qsTr("Load Profile Group (.duckygroup.json)...")
                                        onTriggered: loadProfileGroupFileDialog.open()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "document-save-all"
                                        text: qsTr("Save All Profiles as Group (.duckygroup.json)...")
                                        onTriggered: saveProfileGroupFileDialog.open()
                                    }
                                }
                            }
                        }

                        // Divider
                        Rectangle {
                            Layout.fillWidth: true
                            height: 1
                            color: profilesSidebar.frameBorder
                        }

                        // AUTO-SWITCHER Section
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true

                                QQC2.Label {
                                    text: qsTr("AUTO-SWITCHER")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                }

                                QQC2.Switch {
                                    font.pointSize: 10 * root.fontScale
                                    checked: deviceController.autoSwitchEnabled
                                    implicitHeight: 22
                                    onToggled: {
                                        deviceController.setAutoSwitchConfig(
                                            checked,
                                            deviceController.defaultFallbackProfile
                                        )
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Toggle automatic profile switching")
                                }
                            }

                            // Active Window Status Box
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: activeWinCol.implicitHeight + 12
                                radius: 6
                                color: deviceController.windowPickerArmed
                                    ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(0.96, 0.62, 0.04, 0.16))
                                        : Kirigami.Theme.alternateBackgroundColor)
                                border.color: deviceController.windowPickerArmed
                                    ? Kirigami.Theme.highlightColor
                                    : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? "#f59e0b"
                                        : profilesSidebar.frameBorder)
                                border.width: 1

                                ColumnLayout {
                                    id: activeWinCol
                                    anchors.fill: parent
                                    anchors.margins: 6
                                    spacing: 1

                                    QQC2.Label {
                                        text: deviceController.windowPickerArmed
                                            ? qsTr("🎯 Click target window...")
                                            : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                                ? qsTr("⏸ Paused while Key Editor is open")
                                                : qsTr("Active: %1").arg(deviceController.activeWindowClass))
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: deviceController.windowPickerArmed
                                            ? root.highContrastHighlightText
                                            : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                                ? root.highContrastWarningText
                                                : Kirigami.Theme.textColor)
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }

                                    QQC2.Label {
                                        visible: deviceController.activeWindowTitle.length > 0 || (root.hasSelectedSlot && deviceController.autoSwitchEnabled)
                                        text: (root.hasSelectedSlot && deviceController.autoSwitchEnabled)
                                            ? qsTr("Auto-Switch resumes when Key Editor closes")
                                            : deviceController.activeWindowTitle
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }
                                }
                            }

                            // Rules Window Button
                            QQC2.Button {
                                Layout.fillWidth: true
                                Layout.minimumWidth: implicitWidth
                                icon.name: "configure"
                                text: qsTr("Rules")
                                font.pointSize: 10 * root.fontScale
                                onClicked: autoSwitchDrawer.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Open Auto-Switcher Rules")
                            }
                        }
                    }
                }

                // Column 2: Interactive Hardware Canvas
                HardwareCanvas {
                    id: hardwareCanvas
                    QQC2.SplitView.preferredWidth: Math.min(requiredWidthForFullHeight, root.maxCenterColumnWidth)
                    QQC2.SplitView.minimumWidth: Math.min(440, root.maxCenterColumnWidth)
                    QQC2.SplitView.maximumWidth: root.maxCenterColumnWidth
                    QQC2.SplitView.fillWidth: !root.hasSelectedSlot
                    QQC2.SplitView.fillHeight: true
                    controller: deviceController

                    onSlotSelected: (slotNumber, slotTitle, slotColor) => {
                        root.flushPendingSlotEditsIfModified()
                        root.loadSlotFromController(slotNumber, slotTitle, slotColor)
                    }

                    onBeforeSlotMutationRequested: {
                        root.flushPendingSlotEditsIfModified()
                    }

                    onSlotContentMutated: (newSelectedSlot) => {
                        hardwareCanvas.syncSlotsFromController()
                        if (root.hasSelectedSlot) {
                            if (newSelectedSlot >= 1 && newSelectedSlot <= 58 && root.selectedSlotNumber === newSelectedSlot) {
                                root.loadSlotFromController(
                                    newSelectedSlot,
                                    hardwareCanvas.slotTitleFor(newSelectedSlot),
                                    hardwareCanvas.slotColorFor(newSelectedSlot)
                                )
                            } else {
                                root.refreshSelectedSlotFromController()
                            }
                        }
                    }

                    onRenameProfileRequested: (profileNumber, currentName) => {
                        root.openRenameProfileDialog(profileNumber, currentName)
                    }

                    onKeyColorPickerRequested: (slotNumber, currentColorHex) => {
                        root.openKeyColorPicker(slotNumber, currentColorHex)
                    }

                    onProfileDefaultColorPickerRequested: (profileNumber, profileName, currentColorHex) => {
                        root.openProfileDefaultColorPicker(profileNumber, profileName, currentColorHex)
                    }

                    onOpenLightingAnimatorRequested: (targetScope, targetSlot) => {
                        lightingAnimatorDialog.openFor(targetScope, targetSlot)
                    }

                    onOpenKeyPropertiesRequested: (slotNumber) => {
                        keyPropertiesWindow.openForSlot(slotNumber)
                    }
                }

                // Column 3: Right Inspector Card matching system View colorSet
                Rectangle {
                    id: inspectorCard
                    visible: root.hasSelectedSlot
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property bool isMechanicalKey: root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20
                    readonly property bool isRotaryEncoder: root.selectedSlotNumber >= 21 && root.selectedSlotNumber <= 26
                    readonly property bool hasRevertibleChanges: (
                        root.hasSelectedSlot && (
                            label1Input.text !== root.slotSnapshotL1 ||
                            label2Input.text !== root.slotSnapshotL2 ||
                            allowAbortCheck.checked !== root.slotSnapshotAbort ||
                            disableRepeatCheck.checked !== root.slotSnapshotNoRepeat ||
                            root.pressScriptBuffer !== root.slotSnapshotPressScript ||
                            root.releaseScriptBuffer !== root.slotSnapshotReleaseScript ||
                            String(root.selectedSlotColor) !== root.slotSnapshotColor
                        )
                    )
                    readonly property int minContentWidth: Math.max(
                        440,
                        Math.ceil(Math.max(
                            inspectorHeaderRow.implicitWidth + 16,
                            inspectorLabelsRow.implicitWidth + 16,
                            inspectorPhaseRow.implicitWidth + 12,
                            holdFlagsRow.implicitWidth + 16,
                            inspectorTabsRow.implicitWidth + 12,
                            editorHeaderRow.implicitWidth + 16,
                            inspectorFooterRow.implicitWidth + 12
                        )) + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.fillWidth: true
                    QQC2.SplitView.fillHeight: true
                    QQC2.SplitView.minimumWidth: minContentWidth
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    clip: true

                    Timer {
                        id: liveSlotAutoSaveTimer
                        interval: 350
                        repeat: false
                        onTriggered: {
                            if (root.hasSelectedSlot && !root.loadingSlotData && root.compileOk) {
                                root.saveCurrentSlotToController()
                                hardwareCanvas.syncSlotsFromController()
                                root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                            }
                        }
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: 6

                        // =========================================================================
                        // ZONE 1: Identity & Key Appearance Card (Header + Live OLED Labels + Color)
                        // =========================================================================
                        Rectangle {
                            Layout.fillWidth: true
                            implicitHeight: zone1Col.implicitHeight + 16
                            radius: 6
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            border.width: 1

                            ColumnLayout {
                                id: zone1Col
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6

                                // Row 1A: Interactive Keycap Badge, Slot Title, Slot Type Pill, Key Color Swatch & Close
                                RowLayout {
                                    id: inspectorHeaderRow
                                    Layout.fillWidth: true
                                    spacing: 6

                                    // Interactive Mini Keycap Badge (click to pick key color on slots 1..20)
                                    Rectangle {
                                        width: Math.round(32 * root.inspectorScale)
                                        height: Math.round(32 * root.inspectorScale)
                                        radius: 6
                                        color: inspectorBadgeSwatchMouse.containsMouse && inspectorCard.isMechanicalKey
                                            ? Qt.lighter(Kirigami.Theme.backgroundColor, 1.2)
                                            : Kirigami.Theme.backgroundColor
                                        border.color: root.selectedSlotColor
                                        border.width: 2

                                        // Mini top LED bar matching physical keycap
                                        Rectangle {
                                            anchors.top: parent.top
                                            anchors.topMargin: 3
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            width: parent.width - 10
                                            height: 3
                                            radius: 1.5
                                            color: root.selectedSlotColor
                                        }

                                        QQC2.Label {
                                            anchors.centerIn: parent
                                            anchors.verticalCenterOffset: 2
                                            text: root.selectedSlotNumber
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: inspectorBadgeSwatchMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            enabled: inspectorCard.isMechanicalKey
                                            cursorShape: inspectorCard.isMechanicalKey ? Qt.PointingHandCursor : Qt.ArrowCursor
                                            onClicked: root.openKeyColorPicker(root.selectedSlotNumber, root.selectedSlotColor)
                                        }

                                        QQC2.ToolTip.visible: inspectorBadgeSwatchMouse.containsMouse && inspectorCard.isMechanicalKey
                                        QQC2.ToolTip.text: qsTr("Click to change Key %1 RGB Color (%2)")
                                            .arg(root.selectedSlotNumber)
                                            .arg(root.selectedSlotColor)
                                    }

                                    ColumnLayout {
                                        spacing: 1
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 60

                                        QQC2.Label {
                                            id: inspectorTitle
                                            font.weight: Font.Bold
                                            font.pointSize: 11 * root.fontScale
                                            text: root.selectedSlotTitle
                                            color: Kirigami.Theme.textColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }

                                        RowLayout {
                                            Layout.fillWidth: true
                                            spacing: 5

                                            // Slot Type Badge (MECHANICAL KEY / ROTARY ENCODER / EXPANSION)
                                            Rectangle {
                                                radius: 3
                                                color: inspectorCard.isMechanicalKey
                                                    ? Qt.alpha("#38bdf8", 0.16)
                                                    : (inspectorCard.isRotaryEncoder ? Qt.alpha("#a78bfa", 0.16) : Qt.alpha("#f59e0b", 0.16))
                                                border.color: inspectorCard.isMechanicalKey
                                                    ? Qt.alpha("#38bdf8", 0.5)
                                                    : (inspectorCard.isRotaryEncoder ? Qt.alpha("#a78bfa", 0.5) : Qt.alpha("#f59e0b", 0.5))
                                                border.width: 1
                                                implicitWidth: slotTypeBadgeLbl.implicitWidth + 8
                                                implicitHeight: Math.round(15 * root.inspectorScale)

                                                QQC2.Label {
                                                    id: slotTypeBadgeLbl
                                                    anchors.centerIn: parent
                                                    text: inspectorCard.isMechanicalKey
                                                        ? qsTr("KEY %1").arg(root.selectedSlotNumber)
                                                        : (inspectorCard.isRotaryEncoder
                                                            ? qsTr("ENCODER")
                                                            : qsTr("CH %1").arg(root.selectedSlotNumber - 26))
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: inspectorCard.isMechanicalKey
                                                        ? root.highContrastHighlightText
                                                        : (inspectorCard.isRotaryEncoder
                                                            ? (root.isLightTheme ? "#6d28d9" : "#a78bfa")
                                                            : root.highContrastWarningText)
                                                }
                                            }

                                            QQC2.Label {
                                                text: qsTr("P%1: %2")
                                                    .arg(deviceController.activeProfile)
                                                    .arg(deviceController.activeProfileName)
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.disabledTextColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                            }
                                        }
                                    }

                                    // Interactive Key RGB FX Animation Pill (Slots 1..20)
                                    Rectangle {
                                        visible: inspectorCard.isMechanicalKey
                                        readonly property bool hasKeyFx: {
                                            const _cfgs = hardwareCanvas.keyAnimationConfigs
                                            return hardwareCanvas.keyHasCustomAnimation(root.selectedSlotNumber)
                                        }
                                        radius: 5
                                        color: keyFxPillMouse.containsMouse
                                            ? Qt.alpha("#c084fc", 0.25)
                                            : (hasKeyFx ? Qt.alpha("#a855f7", 0.18) : Kirigami.Theme.backgroundColor)
                                        border.color: hasKeyFx
                                            ? "#c084fc"
                                            : (keyFxPillMouse.containsMouse ? "#a855f7" : Qt.alpha(Kirigami.Theme.textColor, 0.25))
                                        border.width: 1
                                        implicitWidth: keyFxPillRow.implicitWidth + 10
                                        implicitHeight: Math.round(26 * root.inspectorScale)

                                        RowLayout {
                                            id: keyFxPillRow
                                            anchors.centerIn: parent
                                            spacing: 3

                                            QQC2.Label {
                                                text: parent.parent.hasKeyFx ? qsTr("✨ Key FX ●") : qsTr("✨ Key FX")
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: parent.parent.hasKeyFx
                                                    ? (root.isLightTheme ? "#6d28d9" : "#e9d5ff")
                                                    : Kirigami.Theme.textColor
                                            }
                                        }

                                        MouseArea {
                                            id: keyFxPillMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.openFor("key", root.selectedSlotNumber)
                                        }

                                        QQC2.ToolTip.visible: keyFxPillMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Configure gaming RGB lighting animations (Pulse, Wave, Ripple, Starlight, Spectrum, Scanner) for Key %1 or Profile #%2")
                                            .arg(root.selectedSlotNumber)
                                            .arg(deviceController.activeProfile)
                                    }

                                    // Interactive Key RGB Color Swatch Button (Slots 1..20)
                                    Rectangle {
                                        visible: inspectorCard.isMechanicalKey
                                        radius: 5
                                        color: keyColorPillMouse.containsMouse
                                            ? Qt.lighter(Kirigami.Theme.backgroundColor, 1.25)
                                            : Kirigami.Theme.backgroundColor
                                        border.color: keyColorPillMouse.containsMouse
                                            ? root.selectedSlotColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.25)
                                        border.width: 1
                                        implicitWidth: keyColorPillRow.implicitWidth + 12
                                        implicitHeight: Math.round(26 * root.inspectorScale)

                                        RowLayout {
                                            id: keyColorPillRow
                                            anchors.centerIn: parent
                                            spacing: 5

                                            Rectangle {
                                                width: Math.round(12 * root.inspectorScale)
                                                height: Math.round(12 * root.inspectorScale)
                                                radius: 3
                                                color: root.selectedSlotColor
                                                border.color: "#ffffff44"
                                                border.width: 1
                                            }

                                            QQC2.Label {
                                                text: String(root.selectedSlotColor).toUpperCase()
                                                font.family: "monospace"
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                            }
                                        }

                                        MouseArea {
                                            id: keyColorPillMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: root.openKeyColorPicker(root.selectedSlotNumber, root.selectedSlotColor)
                                        }

                                        QQC2.ToolTip.visible: keyColorPillMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Set Key %1 Color (SWCOLOR) or pick from Saved & Used Palettes").arg(root.selectedSlotNumber)
                                    }

                                    QQC2.ToolButton {
                                        text: "✕"
                                        implicitWidth: Math.round(26 * root.inspectorScale)
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: root.clearSlotSelection()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Close Key Inspector (or click the selected key again)")
                                    }
                                }

                                Rectangle {
                                    Layout.fillWidth: true
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.08)
                                }

                                // Row 1B: OLED Key Labels (Line 1 & Line 2 with 5-char indicators + Icon Picker) + Clear Slot Action
                                RowLayout {
                                    id: inspectorLabelsRow
                                    Layout.fillWidth: true
                                    spacing: 5

                                    QQC2.Label {
                                        text: inspectorCard.isMechanicalKey ? qsTr("OLED L1:") : qsTr("OLED Label:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    RowLayout {
                                        spacing: 3
                                        QQC2.TextField {
                                            id: label1Input
                                            Layout.preferredWidth: Math.round(66 * root.inspectorScale)
                                            Layout.minimumWidth: 52
                                            implicitHeight: Math.round(25 * root.inspectorScale)
                                            maximumLength: 10
                                            placeholderText: qsTr("Top (5ch)")
                                            selectByMouse: true
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            onTextChanged: {
                                                const clamped = root.clampOledLabel(text)
                                                if (text !== clamped) {
                                                    text = clamped
                                                }
                                            }
                                            onTextEdited: liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.Label {
                                            text: qsTr("%1/5").arg(root.oledLabelCharCount(label1Input.text))
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: root.oledLabelCharCount(label1Input.text) >= 5
                                                ? Kirigami.Theme.neutralTextColor
                                                : Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    QQC2.Label {
                                        visible: inspectorCard.isMechanicalKey
                                        text: qsTr("L2:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    RowLayout {
                                        visible: inspectorCard.isMechanicalKey
                                        spacing: 3
                                        QQC2.TextField {
                                            id: label2Input
                                            Layout.preferredWidth: Math.round(66 * root.inspectorScale)
                                            Layout.minimumWidth: 52
                                            implicitHeight: Math.round(25 * root.inspectorScale)
                                            maximumLength: 10
                                            placeholderText: qsTr("Bot (5ch)")
                                            selectByMouse: true
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            onTextChanged: {
                                                const clamped = root.clampOledLabel(text)
                                                if (text !== clamped) {
                                                    text = clamped
                                                }
                                            }
                                            onTextEdited: liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.Label {
                                            text: qsTr("%1/5").arg(root.oledLabelCharCount(label2Input.text))
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: root.oledLabelCharCount(label2Input.text) >= 5
                                                ? Kirigami.Theme.neutralTextColor
                                                : Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("★ Icon")
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        onClicked: oledIconPickerDialog.openForInspector(1)
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Slot %1").arg(root.selectedSlotNumber)
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.Button {
                                        icon.name: "edit-clear"
                                        text: qsTr("Clear")
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: root.clearSelectedSlotNow()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Clear labels, scripts, and custom RGB color on Slot %1").arg(root.selectedSlotNumber)
                                    }
                                }

                                // Row 1C: Desktop Widget Alternate Labels (longer labels up to 24 chars for Desktop OLED Widget)
                                RowLayout {
                                    id: inspectorWidgetLabelsRow
                                    Layout.fillWidth: true
                                    spacing: 5

                                    QQC2.Label {
                                        text: inspectorCard.isMechanicalKey ? qsTr("💻 Widget L1:") : qsTr("💻 Widget Label:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: (widgetLabel1Input.text.trim().length > 0 || widgetLabel2Input.text.trim().length > 0)
                                            ? root.highContrastWarningText
                                            : Kirigami.Theme.disabledTextColor
                                        QQC2.ToolTip.visible: widgetLblHover.hovered
                                        QQC2.ToolTip.text: qsTr("💻 Host App Only (Not Hardware-Portable): Longer labels (up to 24 chars) displayed only on the Desktop OLED Widget while DuckyDeck Studio is running. Physical OLED always uses the 5-char L1/L2 labels above.")
                                        HoverHandler { id: widgetLblHover }
                                    }

                                    QQC2.TextField {
                                        id: widgetLabel1Input
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 70
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        maximumLength: 24
                                        placeholderText: inspectorCard.isMechanicalKey
                                            ? qsTr("Desktop Widget L1 (up to 24ch, host widget only)")
                                            : qsTr("Desktop Widget Label (up to 24ch, host widget only)")
                                        selectByMouse: true
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        onTextEdited: liveSlotAutoSaveTimer.restart()
                                    }

                                    QQC2.Label {
                                        visible: inspectorCard.isMechanicalKey
                                        text: qsTr("L2:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.TextField {
                                        id: widgetLabel2Input
                                        visible: inspectorCard.isMechanicalKey
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 70
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        maximumLength: 24
                                        placeholderText: qsTr("Widget L2 (host widget only)")
                                        selectByMouse: true
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        onTextEdited: liveSlotAutoSaveTimer.restart()
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 2: Event Phase Tabs (On Press / On Release) & Key Hold Behavior
                        // =========================================================================
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            Item {
                                Layout.fillWidth: true
                                implicitHeight: Math.round(30 * root.inspectorScale)

                                // Bottom tab baseline
                                Rectangle {
                                    anchors.left: parent.left
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.22)
                                }

                                RowLayout {
                                    id: inspectorPhaseRow
                                    anchors.fill: parent
                                    spacing: 2

                                    // On Press Tab
                                    Rectangle {
                                        readonly property bool isActiveTab: !root.editingReleaseScript
                                        Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                        implicitWidth: onPressTabLbl.implicitWidth + 24
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        bottomLeftRadius: 0
                                        bottomRightRadius: 0
                                        color: isActiveTab
                                            ? Kirigami.Theme.backgroundColor
                                            : (onPressTabMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                                : Kirigami.Theme.alternateBackgroundColor)
                                        border.color: isActiveTab
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                        border.width: 1

                                        // Active tab top indicator bar
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.top: parent.top
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            height: 3
                                            topLeftRadius: 6
                                            topRightRadius: 6
                                            color: Kirigami.Theme.highlightColor
                                        }

                                        // Mask bottom border when active so tab connects to panel below
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.bottom: parent.bottom
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.leftMargin: 1
                                            anchors.rightMargin: 1
                                            height: 2
                                            color: Kirigami.Theme.backgroundColor
                                        }

                                        QQC2.Label {
                                            id: onPressTabLbl
                                            anchors.centerIn: parent
                                            text: root.pressScriptBuffer.trim().length > 0
                                                ? qsTr("⬇ On Press ●")
                                                : qsTr("⬇ On Press")
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: parent.isActiveTab
                                            color: parent.isActiveTab
                                                ? root.highContrastHighlightText
                                                : Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: onPressTabMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                if (root.editingReleaseScript) {
                                                    root.releaseScriptBuffer = scriptEditor.text
                                                    root.editingReleaseScript = false
                                                    root.loadingSlotData = true
                                                    scriptEditor.text = root.pressScriptBuffer
                                                    root.showWebApiBuilder = false
                                                    root.showAppLaunchBuilder = false
                                                    root.inspectorMode = "code"
                                                    root.loadWebApiBuilderFromScript(scriptEditor.text)
                                                    root.loadAppLaunchBuilderFromScript(scriptEditor.text)
                                                    root.loadingSlotData = false
                                                    root.updateCompilerStatus()
                                                }
                                            }
                                        }

                                        QQC2.ToolTip.visible: onPressTabMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Edit script executed when pressed down")
                                    }

                                    // On Release Tab
                                    Rectangle {
                                        readonly property bool isActiveTab: root.editingReleaseScript
                                        Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                        implicitWidth: onReleaseTabLbl.implicitWidth + 24
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        bottomLeftRadius: 0
                                        bottomRightRadius: 0
                                        color: isActiveTab
                                            ? Kirigami.Theme.backgroundColor
                                            : (onReleaseTabMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                                : Kirigami.Theme.alternateBackgroundColor)
                                        border.color: isActiveTab
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                        border.width: 1

                                        // Active tab top indicator bar
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.top: parent.top
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            height: 3
                                            topLeftRadius: 6
                                            topRightRadius: 6
                                            color: Kirigami.Theme.highlightColor
                                        }

                                        // Mask bottom border when active
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.bottom: parent.bottom
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.leftMargin: 1
                                            anchors.rightMargin: 1
                                            height: 2
                                            color: Kirigami.Theme.backgroundColor
                                        }

                                        QQC2.Label {
                                            id: onReleaseTabLbl
                                            anchors.centerIn: parent
                                            text: root.releaseScriptBuffer.trim().length > 0
                                                ? qsTr("⬆ On Release ●")
                                                : qsTr("⬆ On Release")
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: parent.isActiveTab
                                            color: parent.isActiveTab
                                                ? root.highContrastHighlightText
                                                : Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: onReleaseTabMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                if (!root.editingReleaseScript) {
                                                    root.pressScriptBuffer = scriptEditor.text
                                                    root.editingReleaseScript = true
                                                    root.loadingSlotData = true
                                                    scriptEditor.text = root.releaseScriptBuffer
                                                    root.showWebApiBuilder = false
                                                    root.showAppLaunchBuilder = false
                                                    root.inspectorMode = "code"
                                                    root.loadWebApiBuilderFromScript(scriptEditor.text)
                                                    root.loadAppLaunchBuilderFromScript(scriptEditor.text)
                                                    root.loadingSlotData = false
                                                    root.updateCompilerStatus()
                                                }
                                            }
                                        }

                                        QQC2.ToolTip.visible: onReleaseTabMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Edit script executed when released")
                                    }

                                    Item { Layout.fillWidth: true }
                                }
                            }

                            // Key Hold Behavior Bar: Compact No Repeat on Key Held Down (Run Once) + Allow Abort
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: holdFlagsRow.implicitHeight + 6
                                radius: 5
                                color: disableRepeatCheck.checked
                                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.12)
                                    : Kirigami.Theme.alternateBackgroundColor
                                border.color: disableRepeatCheck.checked
                                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                                    : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                border.width: 1

                                RowLayout {
                                    id: holdFlagsRow
                                    anchors.fill: parent
                                    anchors.leftMargin: 6
                                    anchors.rightMargin: 6
                                    anchors.topMargin: 3
                                    anchors.bottomMargin: 3
                                    spacing: 6

                                    HighContrastCheckBox {
                                        id: disableRepeatCheck
                                        text: qsTr("No Repeat on Held (Run Once)")
                                        font.bold: checked
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.updateCompilerStatus()
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Execute this key action only once per press even if the key is held down — essential for Delete or destructive actions (dr %1 / $_DONT_REPEAT = 1)").arg(root.selectedSlotNumber)
                                    }

                                    Item { Layout.fillWidth: true }

                                    HighContrastCheckBox {
                                        id: allowAbortCheck
                                        text: qsTr("Allow Abort")
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.updateCompilerStatus()
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab %1 / $_ALLOW_ABORT = 1)").arg(root.selectedSlotNumber)
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 3: Folder-Style Action Mode Tabs (DuckyScript IDE / Launch App / Web API) + Tools
                        // =========================================================================
                        Item {
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)

                            // Bottom tab baseline
                            Rectangle {
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                height: 1
                                color: Qt.alpha(Kirigami.Theme.textColor, 0.22)
                            }

                            RowLayout {
                                id: inspectorTabsRow
                                anchors.fill: parent
                                spacing: 2

                                // Tab 1: DuckyScript IDE
                                Rectangle {
                                    id: codeModeBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "code"
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: codeTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (codeTabMouse.containsMouse
                                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab
                                        ? Kirigami.Theme.highlightColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
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
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: codeTabLbl
                                        anchors.centerIn: parent
                                        text: qsTr("📜 DuckyScript IDE")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab
                                        color: parent.isActiveTab
                                            ? root.highContrastHighlightText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: codeTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showAppLaunchBuilder = false
                                            root.showWebApiBuilder = false
                                            root.inspectorMode = "code"
                                        }
                                    }

                                    QQC2.ToolTip.visible: codeTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Full-height DuckyScript 3 editor with syntax highlighting, autocomplete & command reference")
                                }

                                // Tab 2: Launch App (Opens Launch App Configuration Window)
                                Rectangle {
                                    id: appLaunchToggleBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "app_launch" || appLaunchConfigWindow.visible
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: appLaunchTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (appLaunchTabMouse.containsMouse
                                            ? Qt.alpha("#fb923c", 0.14)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab || root.appLaunchEnabled
                                        ? "#fb923c"
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab || root.appLaunchEnabled
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        color: "#fb923c"
                                    }

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.bottom: parent.bottom
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 1
                                        anchors.rightMargin: 1
                                        height: 2
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: appLaunchTabLbl
                                        anchors.centerIn: parent
                                        text: root.appLaunchEnabled
                                            ? qsTr("🚀 Launch App ●")
                                            : qsTr("🚀 Launch App")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab || root.appLaunchEnabled
                                        color: parent.isActiveTab || root.appLaunchEnabled
                                            ? root.highContrastOrangeText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: appLaunchTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showWebApiBuilder = false
                                            root.showAppLaunchBuilder = false
                                            root.inspectorMode = "app_launch"
                                        }
                                    }

                                    QQC2.ToolTip.visible: appLaunchTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Switch to Launch App tab")
                                }

                                // Tab 3: Web API
                                Rectangle {
                                    id: webApiToggleBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "web_api" || webApiKeyConfigWindow.visible
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: webApiTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (webApiTabMouse.containsMouse
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab || root.webApiEnabled
                                        ? Kirigami.Theme.positiveTextColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab || root.webApiEnabled
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        color: Kirigami.Theme.positiveTextColor
                                    }

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.bottom: parent.bottom
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 1
                                        anchors.rightMargin: 1
                                        height: 2
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: webApiTabLbl
                                        anchors.centerIn: parent
                                        text: root.webApiEnabled
                                            ? qsTr("🌐 Web API ●")
                                            : qsTr("🌐 Web API")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab || root.webApiEnabled
                                        color: parent.isActiveTab || root.webApiEnabled
                                            ? root.highContrastPositiveText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: webApiTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showAppLaunchBuilder = false
                                            root.showWebApiBuilder = false
                                            root.inspectorMode = "web_api"
                                        }
                                    }

                                    QQC2.ToolTip.visible: webApiTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Switch to Web API tab")
                                }

                                Item { Layout.fillWidth: true }

                                QQC2.Button {
                                    text: qsTr("🔴 Record")
                                    implicitHeight: Math.round(24 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    leftPadding: 6
                                    rightPadding: 6
                                    onClicked: macroRecorderDialog.openRecorder()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Record live keyboard keys, mouse clicks, and mouse movements into DuckyScript 3 (with Exact Timings or As Fast As Possible)")
                                }

                                QQC2.Button {
                                    text: qsTr("✨ Template")
                                    implicitHeight: Math.round(24 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    leftPadding: 6
                                    rightPadding: 6
                                    onClicked: aiModal.open()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Generate DuckyScript 3 macro templates from KDE shortcuts using Macro Template Generator (experimental)")
                                }
                            }
                        }

                        // Portability / Host-Dependency Status Strip for Selected Key
                        Rectangle {
                            id: slotPortabilityStrip
                            readonly property bool requiresHostApp: root.appLaunchEnabled || root.webApiEnabled
                            Layout.fillWidth: true
                            implicitHeight: Math.round(24 * root.inspectorScale)
                            radius: 4
                            color: requiresHostApp
                                ? Qt.alpha("#f59e0b", 0.14)
                                : Qt.alpha("#22c55e", 0.12)
                            border.color: requiresHostApp
                                ? Qt.alpha("#f59e0b", 0.55)
                                : Qt.alpha("#22c55e", 0.45)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: slotPortabilityStrip.requiresHostApp ? root.highContrastWarningText : root.highContrastPositiveText
                                    text: slotPortabilityStrip.requiresHostApp
                                        ? qsTr("💻 Requires DuckyDeck Studio Running on Host PC — Not Hardware-Portable (uses USB HID host listener)")
                                        : qsTr("✅ Hardware-Portable DuckyScript 3 — Runs standalone from microSD on any PC without DuckyDeck Studio")
                                }
                            }

                            HoverHandler { id: slotPortabilityHover }
                            QQC2.ToolTip.visible: slotPortabilityHover.hovered
                            QQC2.ToolTip.text: slotPortabilityStrip.requiresHostApp
                                ? qsTr("This key uses Launch App or Web API, which are executed by the DuckyDeck Studio host process over USB HID. If DuckyDeck Studio is closed or the keypad is plugged into another computer, host actions will not run.")
                                : qsTr("Standard DuckyScript 3 keystrokes, delays, mouse moves, and SWC_SET LED commands compile to bytecode (.dsb) on the microSD card and run 100% standalone on the duckyPad hardware.")
                        }

                        // Compact status strip when Launch App or Web API is configured on this key
                        Rectangle {
                            visible: root.appLaunchEnabled || root.inspectorMode === "app_launch"
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            radius: 5
                            color: Qt.alpha("#fb923c", 0.14)
                            border.color: Qt.alpha("#fb923c", 0.55)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: root.highContrastOrangeText
                                    text: root.appLaunchEnabled
                                        ? qsTr("🚀 Launch App [💻 Host App Required]: %1").arg(root.appLaunchAppName || appLaunchCommandField.text || qsTr("Configured"))
                                        : qsTr("🚀 Launch Application Configurator [💻 Host App Required]")
                                }

                                QQC2.Button {
                                    text: qsTr("Configure Launch App...")
                                    implicitHeight: Math.round(22 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    onClicked: appLaunchConfigWindow.openForCurrentSlot()
                                }
                            }
                        }

                        Rectangle {
                            visible: root.webApiEnabled || root.inspectorMode === "web_api"
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            radius: 5
                            color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                            border.color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: root.highContrastPositiveText
                                    text: root.webApiEnabled
                                        ? qsTr("🌐 Web API [💻 Host App Required]: %1").arg(root.webApiSelectedDestination || webApiUrlField.text || qsTr("Configured"))
                                        : qsTr("🌐 Web API Call Configurator [💻 Host App Required]")
                                }

                                QQC2.Button {
                                    text: qsTr("Configure Web API...")
                                    implicitHeight: Math.round(22 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    onClicked: webApiKeyConfigWindow.openForCurrentSlot()
                                }
                            }
                        }

                        // Collapsible DuckyScript 3 Command Reference & Snippet Browser
                        Rectangle {
                            id: commandReferenceCard
                            visible: root.showCommandReference
                            Layout.fillWidth: true
                            implicitHeight: visible ? Math.round(220 * root.inspectorScale) : 0
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                            border.width: 1
                            clip: true

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6

                                // Search & Autocomplete Toggle Row
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.TextField {
                                        id: commandRefSearchField
                                        Layout.fillWidth: true
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        selectByMouse: true
                                        placeholderText: qsTr("🔍 Filter duckyScript 3 commands, keys, $_VARS, OLED, RGB...")
                                        text: root.commandRefSearchText
                                        rightPadding: clearCmdRefBtn.visible ? 28 : 8
                                        onTextEdited: {
                                            root.commandRefSearchText = text
                                            root.refreshCommandReferenceModel()
                                        }

                                        QQC2.ToolButton {
                                            id: clearCmdRefBtn
                                            visible: commandRefSearchField.text.length > 0
                                            anchors.right: parent.right
                                            anchors.rightMargin: 3
                                            anchors.verticalCenter: parent.verticalCenter
                                            width: 20
                                            height: 20
                                            implicitWidth: 20
                                            implicitHeight: 20
                                            padding: 0
                                            flat: true
                                            text: "✕"
                                            font.pointSize: 9 * root.fontScale
                                            onClicked: {
                                                commandRefSearchField.text = ""
                                                root.commandRefSearchText = ""
                                                root.refreshCommandReferenceModel()
                                                commandRefSearchField.forceActiveFocus()
                                            }
                                            QQC2.ToolTip.visible: hovered
                                            QQC2.ToolTip.text: qsTr("Clear search filter")
                                        }
                                    }

                                    HighContrastCheckBox {
                                        id: autocompleteToggleCheck
                                        checked: root.autocompleteEnabled
                                        text: qsTr("⚡ Live Autocomplete")
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.autocompleteEnabled = checked
                                            if (!checked) {
                                                root.autocompleteVisible = false
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Show inline duckyScript 3 suggestions while typing (or press Ctrl+Space anytime)")
                                    }
                                }

                                // Category Filter Pills Row
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 4

                                    Repeater {
                                        model: root.commandRefCategories

                                        delegate: QQC2.Button {
                                            required property string modelData
                                            readonly property bool isSelectedCat: root.commandRefCategory === modelData
                                            text: modelData
                                            checkable: true
                                            checked: isSelectedCat
                                            highlighted: isSelectedCat
                                            implicitHeight: Math.round(22 * root.inspectorScale)
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: isSelectedCat
                                            onClicked: {
                                                root.commandRefCategory = modelData
                                                root.refreshCommandReferenceModel()
                                            }
                                        }
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.Label {
                                        text: qsTr("%1 items • Click to insert").arg(commandReferenceListView.count)
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }
                                }

                                // Scrollable Command Reference List
                                QQC2.ScrollView {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    clip: true
                                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                        policy: commandReferenceListView.contentHeight > commandReferenceListView.height
                                            ? QQC2.ScrollBar.AlwaysOn
                                            : QQC2.ScrollBar.AsNeeded
                                    }
                                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                                        orientation: Qt.Horizontal
                                    }

                                    ListView {
                                        id: commandReferenceListView
                                        model: commandReferenceModel
                                        spacing: 3
                                        boundsBehavior: Flickable.StopAtBounds

                                        delegate: Rectangle {
                                            id: refRow
                                            required property int index
                                            required property string token
                                            required property string category
                                            required property string badge
                                            required property string syntax
                                            required property string insertText
                                            required property string summary

                                            width: commandReferenceListView.width - 6
                                            implicitHeight: Math.max(Math.round(30 * root.inspectorScale), refRowLayout.implicitHeight + 8)
                                            radius: 4
                                            color: refRowMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                                : Qt.alpha(Kirigami.Theme.backgroundColor, 0.65)
                                            border.color: refRowMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                                                : Qt.alpha(Kirigami.Theme.textColor, 0.08)
                                            border.width: 1

                                            RowLayout {
                                                id: refRowLayout
                                                anchors.fill: parent
                                                anchors.leftMargin: 6
                                                anchors.rightMargin: 6
                                                spacing: 8

                                                Rectangle {
                                                    radius: 3
                                                    color: Qt.alpha(root.badgeColorForType(refRow.badge), 0.2)
                                                    border.color: Qt.alpha(root.badgeColorForType(refRow.badge), 0.6)
                                                    border.width: 1
                                                    Layout.preferredWidth: Math.round(42 * root.inspectorScale)
                                                    implicitHeight: Math.round(18 * root.inspectorScale)

                                                    QQC2.Label {
                                                        anchors.centerIn: parent
                                                        text: refRow.badge
                                                        font.family: "monospace"
                                                        font.bold: true
                                                        font.pointSize: 10 * root.fontScale
                                                        color: root.badgeColorForType(refRow.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: refRow.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(185 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: refRow.summary
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                    Layout.fillWidth: true
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    visible: refRowMouse.containsMouse
                                                    text: qsTr("+ Insert")
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: root.highContrastHighlightText
                                                }
                                            }

                                            MouseArea {
                                                id: refRowMouse
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: Qt.PointingHandCursor
                                                onClicked: root.insertCommandFromReference(refRow.insertText)
                                            }

                                            QQC2.ToolTip.visible: refRowMouse.containsMouse
                                            QQC2.ToolTip.delay: 400
                                            QQC2.ToolTip.text: refRow.syntax + " — " + refRow.summary + "\n" + qsTr("Click to insert at cursor")
                                        }
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 4: Code Editor Area with Dedicated IDE Header Toolbar,
                        //         Synchronized Line-Number Gutter & Live Autocomplete Popup
                        // =========================================================================
                        Rectangle {
                            id: editorContainer
                            readonly property bool editorExpanded: root.inspectorMode === "code" || root.showScriptPreviewInBuilder
                            Layout.fillWidth: true
                            Layout.fillHeight: editorExpanded
                            Layout.preferredHeight: editorExpanded ? -1 : (editorHeaderStrip.height + 2)
                            Layout.minimumHeight: editorExpanded ? Math.round(135 * root.inspectorScale) : (editorHeaderStrip.height + 2)
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: root.compileOk
                                ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                : Kirigami.Theme.negativeTextColor
                            border.width: 1
                            clip: true

                            // Dedicated IDE Header Toolbar
                            Rectangle {
                                id: editorHeaderStrip
                                anchors.top: parent.top
                                anchors.left: parent.left
                                anchors.right: parent.right
                                height: Math.round(30 * root.inspectorScale)
                                z: 5
                                color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.85)

                                Rectangle {
                                    anchors.left: parent.left
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.12)
                                }

                                RowLayout {
                                    id: editorHeaderRow
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 6
                                    spacing: 4

                                    // Collapse / Expand toggle when inside a visual builder
                                    QQC2.ToolButton {
                                        visible: root.inspectorMode !== "code"
                                        text: root.showScriptPreviewInBuilder ? qsTr("▾ Script") : qsTr("▸ Script")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.showScriptPreviewInBuilder = !root.showScriptPreviewInBuilder
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Show or hide the live generated DuckyScript 3 source preview while using a visual builder")
                                    }

                                    // Active script filename + line count badge
                                    Rectangle {
                                        radius: 3
                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.38)
                                        border.width: 1
                                        implicitWidth: fileBadgeRow.implicitWidth + 10
                                        implicitHeight: Math.round(20 * root.inspectorScale)

                                        RowLayout {
                                            id: fileBadgeRow
                                            anchors.centerIn: parent
                                            spacing: 5

                                            QQC2.Label {
                                                text: root.editingReleaseScript
                                                    ? ("key" + root.selectedSlotNumber + "-release.txt")
                                                    : ("key" + root.selectedSlotNumber + ".txt")
                                                font.family: "monospace"
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: root.highContrastHighlightText
                                            }

                                            QQC2.Label {
                                                text: qsTr("• %1 ln").arg(Math.max(1, scriptEditor.lineCount))
                                                font.family: "monospace"
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.disabledTextColor
                                            }
                                        }
                                    }

                                    Item { Layout.fillWidth: true }

                                    // Quick Snippet Insertion Buttons
                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+DELAY")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("DELAY 100")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert DELAY 100 (pause 100ms)")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+STRING")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("STRINGLN ")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert STRINGLN command")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+OLED")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("OLED_CLEAR\nOLED_CURSOR 10 24\nOLED_PRINT Hello!\nOLED_UPDATE")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert OLED screen drawing block")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("⚡")
                                        font.pointSize: 10 * root.fontScale
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: {
                                            scriptEditor.forceActiveFocus()
                                            root.updateAutocompleteSuggestions(true)
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Trigger DuckyScript 3 Autocomplete (Ctrl+Space)")
                                    }

                                    // Integrated Command Reference Toggle
                                    QQC2.Button {
                                        text: qsTr("📖 Reference")
                                        checkable: true
                                        checked: root.showCommandReference
                                        highlighted: root.showCommandReference
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: root.showCommandReference
                                        onClicked: {
                                            root.showCommandReference = !root.showCommandReference
                                            if (root.showCommandReference) {
                                                if (!editorContainer.editorExpanded) {
                                                    root.showScriptPreviewInBuilder = true
                                                }
                                                root.refreshCommandReferenceModel()
                                                commandRefSearchField.forceActiveFocus()
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Toggle interactive DuckyScript 3 Command Reference browser")
                                    }
                                }
                            }

                            Flickable {
                                id: scriptScrollView
                                objectName: "scriptEditorFlickable"
                                visible: editorContainer.editorExpanded
                                anchors.top: editorHeaderStrip.bottom
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                anchors.margins: 2
                                clip: true
                                boundsBehavior: Flickable.StopAtBounds
                                flickableDirection: Flickable.HorizontalAndVerticalFlick

                                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                    id: mainScriptEditorVerticalScrollBar
                                    objectName: "highContrastScrollBar"
                                    parent: scriptScrollView
                                    anchors.top: scriptScrollView.top
                                    anchors.right: scriptScrollView.right
                                    anchors.bottom: scriptScrollView.bottom
                                    policy: QQC2.ScrollBar.AlwaysOn
                                }

                                QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                                    id: scriptEditorHorizontalScrollBar
                                    objectName: "highContrastScrollBar"
                                    parent: scriptScrollView
                                    anchors.left: scriptScrollView.left
                                    anchors.right: scriptScrollView.right
                                    anchors.bottom: scriptScrollView.bottom
                                    orientation: Qt.Horizontal
                                    policy: scriptScrollView.contentWidth > scriptScrollView.width
                                        ? QQC2.ScrollBar.AlwaysOn
                                        : QQC2.ScrollBar.AsNeeded
                                }

                                T.TextArea.flickable: T.TextArea {
                                    id: scriptEditor
                                    objectName: "scriptEditorTextArea"
                                    font.family: "monospace"
                                    font.pointSize: 11 * root.fontScale
                                    leftPadding: lineGutter.width + 10
                                    topPadding: 6
                                    bottomPadding: 6
                                    rightPadding: 16
                                    background: Rectangle { color: "transparent" }
                                    color: Kirigami.Theme.textColor
                                    selectionColor: Kirigami.Theme.highlightColor
                                    selectedTextColor: Kirigami.Theme.highlightedTextColor
                                    selectByMouse: true
                                    wrapMode: TextEdit.NoWrap
                                    onTextChanged: {
                                        if (!root.loadingSlotData) {
                                            if (root.editingReleaseScript) {
                                                root.releaseScriptBuffer = text
                                            } else {
                                                root.pressScriptBuffer = text
                                            }
                                            root.updateCompilerStatus()
                                            if (activeFocus && !root.insertingCompletion) {
                                                root.updateAutocompleteSuggestions(false)
                                            }
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                    }
                                    onCursorPositionChanged: {
                                        if (root.autocompleteVisible && !root.insertingCompletion) {
                                            if (cursorPosition < root.autocompleteTokenStart || cursorPosition > root.autocompleteTokenEnd + 1) {
                                                root.autocompleteVisible = false
                                            }
                                        }
                                    }
                                    onActiveFocusChanged: {
                                        if (!activeFocus) {
                                            root.autocompleteVisible = false
                                        }
                                    }
                                    Keys.onPressed: (event) => {
                                        if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Space) {
                                            root.updateAutocompleteSuggestions(true)
                                            event.accepted = true
                                            return
                                        }
                                        if (root.autocompleteVisible && autocompleteListView.count > 0) {
                                            if (event.key === Qt.Key_Down) {
                                                root.autocompleteIndex = (root.autocompleteIndex + 1) % autocompleteListView.count
                                                autocompleteListView.positionViewAtIndex(root.autocompleteIndex, ListView.Contain)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Up) {
                                                root.autocompleteIndex = (root.autocompleteIndex - 1 + autocompleteListView.count) % autocompleteListView.count
                                                autocompleteListView.positionViewAtIndex(root.autocompleteIndex, ListView.Contain)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Tab || event.key === Qt.Key_Return || event.key === Qt.Key_Enter) {
                                                root.applyAutocompleteSelection(root.autocompleteIndex)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Escape) {
                                                root.autocompleteVisible = false
                                                event.accepted = true
                                                return
                                            }
                                        }
                                    }

                                    // Synchronized Line-Number Gutter
                                    Rectangle {
                                        id: lineGutter
                                        x: scriptScrollView.contentX
                                        y: 0
                                        z: 2
                                        width: Math.round(38 * root.editorScale)
                                        height: Math.max(
                                            scriptEditor.contentHeight + scriptEditor.topPadding + scriptEditor.bottomPadding,
                                            scriptScrollView.height
                                        )
                                        color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.65)

                                        Rectangle {
                                            anchors.right: parent.right
                                            anchors.top: parent.top
                                            anchors.bottom: parent.bottom
                                            width: 1
                                            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                        }

                                        Column {
                                            y: scriptEditor.topPadding
                                            width: parent.width - 6

                                            Repeater {
                                                model: Math.max(1, scriptEditor.lineCount)

                                                Text {
                                                    required property int index
                                                    readonly property bool isErrLine: !root.compileOk && root.compileErrorLine === (index + 1)
                                                    width: lineGutter.width - 7
                                                    horizontalAlignment: Text.AlignRight
                                                    text: (index + 1)
                                                    font.family: scriptEditor.font.family
                                                    font.pointSize: scriptEditor.font.pointSize
                                                    font.bold: isErrLine
                                                    color: isErrLine
                                                        ? Kirigami.Theme.negativeTextColor
                                                        : Kirigami.Theme.disabledTextColor
                                                }
                                            }
                                        }
                                    }

                                    KSyntax.SyntaxHighlighter {
                                        textEdit: scriptEditor
                                        definition: "duckyScript 3"
                                        repository: KSyntax.Repository
                                        theme: Kirigami.Theme.backgroundColor.hslLightness < 0.5
                                            ? KSyntax.Repository.defaultTheme(KSyntax.Repository.DarkTheme)
                                            : KSyntax.Repository.defaultTheme(KSyntax.Repository.LightTheme)
                                    }
                                }
                            }

                            // Floating DuckyScript 3 Autocomplete Popup
                            Rectangle {
                                id: autocompletePopup
                                visible: editorContainer.editorExpanded && root.autocompleteVisible && autocompleteListView.count > 0
                                z: 25
                                readonly property real headerOffset: editorHeaderStrip.height
                                readonly property real scrollX: scriptScrollView.contentX || 0
                                readonly property real scrollY: scriptScrollView.contentY || 0
                                readonly property real rowH: Math.round(26 * root.inspectorScale)
                                readonly property real footerH: Math.round(20 * root.inspectorScale)
                                readonly property real popupH: Math.min(autocompleteListView.count, 7) * rowH + footerH + 8
                                readonly property real cursorBottomY: headerOffset + scriptEditor.cursorRectangle.y - scrollY + scriptEditor.cursorRectangle.height + 6
                                readonly property real cursorTopY: headerOffset + scriptEditor.cursorRectangle.y - scrollY - popupH - 4

                                x: Math.max(
                                    lineGutter.width + 6,
                                    Math.min(
                                        scriptEditor.cursorRectangle.x - scrollX,
                                        editorContainer.width - width - 6
                                    )
                                )
                                y: (cursorBottomY + popupH <= editorContainer.height - 4 || cursorTopY < headerOffset + 4)
                                    ? Math.max(headerOffset + 4, Math.min(cursorBottomY, editorContainer.height - popupH - 4))
                                    : cursorTopY
                                width: Math.min(
                                    Math.max(360, Math.round(460 * root.inspectorScale)),
                                    editorContainer.width - lineGutter.width - 12
                                )
                                height: popupH
                                radius: 6
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.7)
                                border.width: 1

                                ColumnLayout {
                                    anchors.fill: parent
                                    anchors.margins: 4
                                    spacing: 2

                                    ListView {
                                        id: autocompleteListView
                                        Layout.fillWidth: true
                                        Layout.fillHeight: true
                                        clip: true
                                        model: autocompleteModel
                                        currentIndex: root.autocompleteIndex
                                        boundsBehavior: Flickable.StopAtBounds

                                        QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                            parent: autocompleteListView
                                            anchors.top: parent.top
                                            anchors.right: parent.right
                                            anchors.bottom: parent.bottom
                                            policy: autocompleteListView.contentHeight > autocompleteListView.height
                                                ? QQC2.ScrollBar.AlwaysOn
                                                : QQC2.ScrollBar.AsNeeded
                                        }

                                        delegate: Rectangle {
                                            id: acDelegate
                                            required property int index
                                            required property string token
                                            required property string badge
                                            required property string syntax
                                            required property string insertText
                                            required property string summary

                                            readonly property bool isSelected: index === root.autocompleteIndex
                                            width: autocompleteListView.width
                                            height: autocompletePopup.rowH
                                            radius: 4
                                            color: isSelected
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.25)
                                                : (acMouse.containsMouse
                                                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.12)
                                                    : "transparent")

                                            RowLayout {
                                                anchors.fill: parent
                                                anchors.leftMargin: 6
                                                anchors.rightMargin: 6
                                                spacing: 6

                                                Rectangle {
                                                    radius: 3
                                                    color: Qt.alpha(root.badgeColorForType(acDelegate.badge), 0.2)
                                                    border.color: Qt.alpha(root.badgeColorForType(acDelegate.badge), 0.6)
                                                    border.width: 1
                                                    Layout.preferredWidth: Math.round(38 * root.inspectorScale)
                                                    implicitHeight: Math.round(16 * root.inspectorScale)

                                                    QQC2.Label {
                                                        anchors.centerIn: parent
                                                        text: acDelegate.badge
                                                        font.family: "monospace"
                                                        font.bold: true
                                                        font.pointSize: 10 * root.fontScale
                                                        color: root.badgeColorForType(acDelegate.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: acDelegate.isSelected
                                                        ? Kirigami.Theme.highlightColor
                                                        : Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(150 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.summary
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                    Layout.fillWidth: true
                                                    elide: Text.ElideRight
                                                }
                                            }

                                            MouseArea {
                                                id: acMouse
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: Qt.PointingHandCursor
                                                onEntered: root.autocompleteIndex = acDelegate.index
                                                onClicked: root.applyAutocompleteSelection(acDelegate.index)
                                            }
                                        }
                                    }

                                    Rectangle {
                                        Layout.fillWidth: true
                                        height: 1
                                        color: Qt.alpha(Kirigami.Theme.textColor, 0.1)
                                    }

                                    RowLayout {
                                        Layout.fillWidth: true
                                        Layout.preferredHeight: autocompletePopup.footerH - 3
                                        spacing: 6

                                        QQC2.Label {
                                            Layout.leftMargin: 4
                                            text: qsTr("↑↓ Navigate  •  Tab / Enter Insert  •  Esc Dismiss  •  Ctrl+Space Trigger")
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 5: Inspector Footer Bar — Live DSVM v2 Status, Snapshot Revert & Save
                        // =========================================================================
                        RowLayout {
                            id: inspectorFooterRow
                            Layout.fillWidth: true
                            spacing: 6

                            Rectangle {
                                radius: 4
                                color: !root.compileOk
                                    ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.15)
                                    : (root.compileEmpty
                                        ? Kirigami.Theme.alternateBackgroundColor
                                        : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.15))
                                border.color: !root.compileOk
                                    ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.5)
                                    : (root.compileEmpty
                                        ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                        : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5))
                                border.width: 1
                                Layout.fillWidth: true
                                Layout.minimumWidth: 60
                                implicitHeight: Math.round(28 * root.inspectorScale)

                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 8
                                    spacing: 6

                                    QQC2.Label {
                                        Layout.fillWidth: true
                                        verticalAlignment: Text.AlignVCenter
                                        elide: Text.ElideRight
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        color: !root.compileOk
                                            ? Kirigami.Theme.negativeTextColor
                                            : (root.compileEmpty
                                                ? Kirigami.Theme.disabledTextColor
                                                : Kirigami.Theme.positiveTextColor)
                                        text: {
                                            const baseName = root.editingReleaseScript
                                                ? ("key" + root.selectedSlotNumber + "-release")
                                                : ("key" + root.selectedSlotNumber)
                                            if (!root.compileOk) {
                                                return qsTr("⚠ Line %1: %2").arg(root.compileErrorLine).arg(root.compileErrorMsg)
                                            }
                                            if (root.compileEmpty) {
                                                return qsTr("%1.txt — Empty script (0 B)").arg(baseName)
                                            }
                                            return qsTr("✓ DSVM v2: %1 B (%2.dsb)").arg(root.compileByteCount).arg(baseName)
                                        }
                                    }

                                    Rectangle {
                                        visible: inspectorCard.hasRevertibleChanges
                                        radius: 3
                                        color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                                        border.color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.5)
                                        border.width: 1
                                        implicitWidth: modifiedBadgeLabel.implicitWidth + 8
                                        implicitHeight: Math.round(18 * root.inspectorScale)

                                        QQC2.Label {
                                            id: modifiedBadgeLabel
                                            anchors.centerIn: parent
                                            text: qsTr("● Edited")
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.neutralTextColor
                                        }
                                    }
                                }
                            }

                            QQC2.Button {
                                text: qsTr("↺ Revert")
                                enabled: inspectorCard.hasRevertibleChanges
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.revertSlotToSnapshot()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: inspectorCard.hasRevertibleChanges
                                    ? qsTr("Restore Key %1 labels, color, flags, and scripts to their state when this key was selected").arg(root.selectedSlotNumber)
                                    : qsTr("No changes made to Key %1 since it was selected").arg(root.selectedSlotNumber)
                            }

                            QQC2.Button {
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("✓ Apply Key %1 (SD)").arg(root.selectedSlotNumber)
                                    : qsTr("✓ Apply Key %1").arg(root.selectedSlotNumber)
                                enabled: inspectorCard.hasRevertibleChanges
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                highlighted: inspectorCard.hasRevertibleChanges
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.saveCurrentSlotToController()
                                    root.captureSlotSnapshot()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: inspectorCard.hasRevertibleChanges
                                    ? qsTr("Compile Key %1 DSVM v2 bytecode and commit this key's current state as the new baseline").arg(root.selectedSlotNumber)
                                    : qsTr("No uncommitted changes on Key %1").arg(root.selectedSlotNumber)
                            }

                            QQC2.Button {
                                text: qsTr("⬆ Sync SD")
                                enabled: !deviceController.sdSyncBusy
                                    && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                                    && (deviceController.connected || deviceController.sdMountPath.length > 0)
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                highlighted: deviceController.profileSyncState !== "synced"
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.saveCurrentSlotToController()
                                    root.captureSlotSnapshot()
                                    root.openUnsyncedChangesDialog()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                                    ? qsTr("Review and push unsynced local profile and bytecode changes to the duckyPad SD card")
                                    : qsTr("All profiles and keys are already synced with the duckyPad SD card")
                            }
                        }
                    }
                }
            }
        }
    }

    // Bottom Status Bar (2-Line Layout: Line 1 = Connection, Sync & Live Status; Line 2 = Firmware Version, SD Card Space & OLED Widget Toggle)
    footer: Rectangle {
        id: bottomStatusBar
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        width: parent.width
        implicitHeight: Math.round(72 * root.statusBarScale)
        color: Kirigami.Theme.alternateBackgroundColor

        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.leftMargin: Kirigami.Units.largeSpacing
            anchors.rightMargin: Kirigami.Units.largeSpacing
            anchors.topMargin: 4
            anchors.bottomMargin: 4
            spacing: 4

            // Status Bar Line 1: Connection Status Pill, Sync Status Badge & Live Status Message
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                // Connection Status Pill
                Rectangle {
                    radius: 4
                    color: deviceController.connected
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                        : (deviceController.sdMountPath.length > 0
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                            : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.16))
                    border.color: deviceController.connected
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                        : (deviceController.sdMountPath.length > 0
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                    border.width: 1
                    implicitWidth: connRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: connRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: deviceController.connected
                                ? Kirigami.Theme.positiveTextColor
                                : (deviceController.sdMountPath.length > 0
                                    ? Kirigami.Theme.highlightColor
                                    : Kirigami.Theme.neutralTextColor)
                        }

                        QQC2.Label {
                            text: deviceController.connected
                                ? qsTr("USB HID Connected — %1").arg(deviceController.modelName)
                                : (deviceController.sdMountPath.length > 0
                                    ? qsTr("SD Card Mode — %1").arg(deviceController.modelName)
                                    : qsTr("Disconnected"))
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }
                }

                // Profile Sync / Mismatch Status Indicator Badge
                Rectangle {
                    id: syncStatusBadge
                    radius: 4
                    color: deviceController.sdSyncBusy
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                        : (deviceController.profileSyncState === "synced"
                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                            : (deviceController.profileSyncState === "unsynced"
                                ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                                : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.18)))
                    border.color: deviceController.sdSyncBusy
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.6)
                        : (deviceController.profileSyncState === "synced"
                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                            : (deviceController.profileSyncState === "unsynced"
                                ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                                : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.55)))
                    border.width: 1
                    implicitWidth: syncRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: syncRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: deviceController.sdSyncBusy
                                ? Kirigami.Theme.highlightColor
                                : (deviceController.profileSyncState === "synced"
                                    ? Kirigami.Theme.positiveTextColor
                                    : (deviceController.profileSyncState === "unsynced"
                                        ? Kirigami.Theme.neutralTextColor
                                        : Kirigami.Theme.negativeTextColor))
                        }

                        QQC2.Label {
                            text: deviceController.sdSyncBusy
                                ? qsTr("⟳ Syncing (%1%)").arg(deviceController.sdSyncProgress)
                                : (deviceController.profileSyncState === "synced"
                                    ? qsTr("✓ SD Validated & Synced")
                                    : (deviceController.profileSyncState === "unsynced"
                                        ? qsTr("⚠ Unsynced Changes")
                                        : qsTr("⚠ Profile Mismatch")))
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: syncBadgeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        cursorShape: Qt.PointingHandCursor
                        onClicked: (mouse) => {
                            if (deviceController.sdSyncBusy) {
                                sdSyncDialog.open()
                            } else if (mouse.button === Qt.RightButton) {
                                profileSyncMenu.popup(syncStatusBadge, 0, -profileSyncMenu.implicitHeight)
                            } else {
                                root.openUnsyncedChangesDialog()
                            }
                        }
                    }

                    QQC2.ToolTip.visible: syncBadgeMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.sdSyncBusy
                        ? deviceController.sdSyncStep
                        : (deviceController.profileSyncDetail + "\n" + qsTr("Click to open Unsynced Changes Summary (Right-click for Sync Menu)"))
                }

                Rectangle {
                    width: 1
                    height: Math.round(18 * root.statusBarScale)
                    color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                }

                // Live Status Message (Click to open Event Log Viewer)
                Rectangle {
                    id: statusLogLineBox
                    Layout.fillWidth: true
                    implicitHeight: Math.round(26 * root.statusBarScale)
                    radius: 4
                    color: statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                        : "transparent"
                    border.color: statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                        : "transparent"
                    border.width: 1

                    QQC2.Label {
                        anchors.fill: parent
                        anchors.leftMargin: 6
                        anchors.rightMargin: 6
                        verticalAlignment: Text.AlignVCenter
                        text: deviceController.statusMessage
                        font.pointSize: 11 * root.fontScale
                        color: Kirigami.Theme.textColor
                        opacity: statusLogLineMouse.containsMouse ? 1.0 : 0.92
                        elide: Text.ElideRight
                    }

                    MouseArea {
                        id: statusLogLineMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: eventLogDialog.openLogWindow()
                    }

                    QQC2.ToolTip.visible: statusLogLineMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.statusMessage.length > 0
                        ? (deviceController.statusMessage + "\n" + qsTr("Click to open the Event Log Viewer"))
                        : qsTr("Click to open the Event Log Viewer")
                }
            }

            Rectangle {
                Layout.fillWidth: true
                height: 1
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast * 0.65)
            }

            // Status Bar Line 2: Firmware Version & Serial Number, microSD Card Space Data & Desktop OLED Widget Toggle
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                // Firmware Version & Serial Number Badge (Click to check for firmware updates)
                Rectangle {
                    id: fwBadgeRect
                    radius: 4
                    readonly property bool isStudioFw: root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                    color: deviceController.firmwareUpdateAvailable
                        ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                        : (isStudioFw
                            ? Qt.alpha("#a855f7", 0.14)
                            : Kirigami.Theme.backgroundColor)
                    border.color: deviceController.firmwareUpdateAvailable
                        ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                        : (isStudioFw
                            ? Qt.alpha("#c084fc", 0.55)
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                    border.width: 1
                    implicitWidth: fwBadgeRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: fwBadgeRow
                        anchors.centerIn: parent
                        spacing: 5

                        Image {
                            visible: deviceController.firmwareVersion !== "-" && fwBadgeRect.isStudioFw
                            source: root.effectiveThemeMode === "light"
                                ? "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg"
                                : "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg"
                            width: Math.round(14 * root.statusBarScale)
                            height: Math.round(14 * root.statusBarScale)
                            sourceSize.width: 28
                            sourceSize.height: 28
                            fillMode: Image.PreserveAspectFit
                        }

                        QQC2.Label {
                            id: fwLabel
                            text: {
                                const rawVer = deviceController.firmwareVersion
                                const fwVer = root.cleanFirmwareVersionLabel(rawVer)
                                const vendorTag = (rawVer && rawVer !== "-")
                                    ? (fwBadgeRect.isStudioFw ? " [Studio]" : " [✔ Official]")
                                    : ""
                                const base = (deviceController.serialNumber && deviceController.serialNumber !== "-")
                                    ? qsTr("FW: %1%2  •  SN: %3")
                                        .arg(fwVer)
                                        .arg(vendorTag)
                                        .arg(deviceController.serialNumber)
                                    : qsTr("Firmware: %1%2").arg(fwVer).arg(vendorTag)
                                return deviceController.firmwareUpdateAvailable
                                    ? qsTr("%1  (⬆ v%2)").arg(base).arg(deviceController.latestFirmwareVersion)
                                    : base
                            }
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: fwBadgeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            deviceController.checkFirmwareUpdate(true)
                            firmwareUpdateDialog.open()
                        }
                    }

                    QQC2.ToolTip.visible: fwBadgeMouse.containsMouse
                    QQC2.ToolTip.text: {
                        const originDesc = root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                            ? qsTr("DuckyDeck Studio Custom Firmware v1.0.0 (Monochrome Icons & 0°/90°/180°/270° OLED)")
                            : qsTr("Official dekuNukem Firmware Release")
                        return deviceController.firmwareUpdateAvailable
                            ? qsTr("%1 — Update v%2 is available! Click to open Firmware Manager.").arg(originDesc).arg(deviceController.latestFirmwareVersion)
                            : qsTr("%1 — Click to open Firmware Manager & switch between Official dekuNukem or DuckyDeck Studio firmware").arg(originDesc)
                    }
                }

                // microSD Card Used / Free Space Badge
                Rectangle {
                    id: sdSpaceBadge
                    visible: deviceController.sdSpaceSummary.length > 0
                    radius: 4
                    color: Kirigami.Theme.backgroundColor
                    border.color: deviceController.sdSpaceUsedPercent >= 90
                        ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.55)
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    implicitWidth: sdSpaceRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: sdSpaceRow
                        anchors.centerIn: parent
                        spacing: 6

                        QQC2.Label {
                            text: "💾 " + deviceController.sdSpaceSummary
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: sdSpaceMouse
                        anchors.fill: parent
                        hoverEnabled: true
                    }

                    QQC2.ToolTip.visible: sdSpaceMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.sdMountPath.length > 0
                        ? (deviceController.sdSpaceDetail + "\n" + qsTr("Mount Path: %1").arg(deviceController.sdMountPath))
                        : deviceController.sdSpaceDetail
                }

                Item {
                    Layout.fillWidth: true
                }

                // Desktop OLED Widget Toggle Badge
                Rectangle {
                    id: oledWidgetStatusBtn
                    radius: 4
                    color: oledDesktopWidget.visible
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                        : Kirigami.Theme.backgroundColor
                    border.color: oledDesktopWidget.visible
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    implicitWidth: oledWidgetBtnRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: oledWidgetBtnRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: oledDesktopWidget.visible
                                ? Kirigami.Theme.positiveTextColor
                                : Kirigami.Theme.disabledTextColor
                        }

                        Image {
                            source: "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/oled-widget-icon.svg"
                            Layout.preferredWidth: Math.round(16 * root.statusBarScale)
                            Layout.preferredHeight: Math.round(16 * root.statusBarScale)
                            sourceSize.width: 32
                            sourceSize.height: 32
                            fillMode: Image.PreserveAspectFit
                        }

                        QQC2.Label {
                            text: oledDesktopWidget.visible
                                ? qsTr("Desktop OLED: Active")
                                : qsTr("Desktop OLED Widget")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: oledWidgetBtnMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            deviceController.toggleOledDesktopWidget()
                        }
                    }

                    QQC2.ToolTip.visible: oledWidgetBtnMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Toggle the floating, resizable, always-on-top KDE Desktop OLED Widget")
                }
            }
        }
    }

    AppLaunchBuilder {
        id: appLaunchConfigWindow
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        highContrastPositiveText: root.highContrastPositiveText
        highContrastWarningText: root.highContrastWarningText
        hardwareCanvas: hardwareCanvas
        liveSlotAutoSaveTimer: liveSlotAutoSaveTimer
        label1Input: label1Input
        label2Input: label2Input
        scriptEditor: scriptEditor
        keyPropertiesWindow: settingsDialog.keyPropertiesWindow
    }

    // =========================================================================
    // Standalone Key Web API / Webhook Configurator Window
    // =========================================================================
    WebApiBuilder {
        id: webApiKeyConfigWindow
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        highContrastHighlightText: root.highContrastHighlightText
        highContrastWarningText: root.highContrastWarningText
        hardwareCanvas: hardwareCanvas
        scriptEditor: scriptEditor
        keyPropertiesWindow: settingsDialog.keyPropertiesWindow
    }

    // =========================================================================
    // Profile Properties Window (Editable Fields for All Profile Properties)
    // =========================================================================

    property int showcaseStep: 0

    function selectShowcaseSlot(slotNum) {
        const title = hardwareCanvas.slotTitleFor(slotNum)
        const color = hardwareCanvas.slotColorFor(slotNum)
        hardwareCanvas.selectedSlot = slotNum
        root.loadSlotFromController(slotNum, title, color)
    }

    // =========================================================================
    // RGB Lighting Animator Window (Profile-Wide & Per-Key Gaming FX)
    // =========================================================================

    // =========================================================================
    // Live Macro Recorder Window (Creates DuckyScript 3 with Exact Timings,
    // As Fast As Possible, Fixed Delay, Keyboard Keys & Mouse Moves/Clicks)
    // =========================================================================

    OledDesktopWidget {
        id: oledDesktopWidget
        controller: deviceController
        selectedSlot: root.selectedSlotNumber
    }

    // =========================================================================
    // AUTOMATED UI TEST SUITE & SCENE-GRAPH AUDITOR (--ui-test-suite)
    // Verifies behavioral specifications, zero window/element overflow, and
    // WCAG color contrast across all windows/dialogs in Dark and Light themes.
    // =========================================================================
}


