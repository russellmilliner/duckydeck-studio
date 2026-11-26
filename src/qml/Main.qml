import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import io.github.duckydeck 1.0

// Qt <= 6.10 qmllint cascades [missing-property] onto every member access of
// compiled CXX-Qt DeviceController (and Window.screen) in Main.qml.
// MissingProperty remains enforced across all other src/qml/*.qml files via .qmllint.ini.
// qmllint disable missing-property
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
        (profilesSidebar ? profilesSidebar.QQC2.SplitView.preferredWidth : 300)
            + Math.min(hardwareCanvas.requiredWidthForFullHeight, root.maxCenterColumnWidth)
            + Kirigami.Units.largeSpacing * 2
            + 20,
        (bottomStatusBar && bottomStatusBar.connStatusPill ? bottomStatusBar.connStatusPill.implicitWidth : 0)
            + (bottomStatusBar && bottomStatusBar.syncStatusBadge ? bottomStatusBar.syncStatusBadge.implicitWidth : 0)
            + Kirigami.Units.largeSpacing * 2
            + 220,
        (bottomStatusBar && bottomStatusBar.fwBadgeRect ? bottomStatusBar.fwBadgeRect.implicitWidth : 0)
            + (bottomStatusBar && bottomStatusBar.sdSpaceBadge && bottomStatusBar.sdSpaceBadge.visible ? bottomStatusBar.sdSpaceBadge.implicitWidth + 10 : 0)
            + (bottomStatusBar && bottomStatusBar.oledWidgetStatusBtn && bottomStatusBar.oledWidgetStatusBtn.visible ? bottomStatusBar.oledWidgetStatusBtn.implicitWidth + 10 : 0)
            + Kirigami.Units.largeSpacing * 2
            + 16
    )
    readonly property int expandedWindowWidth: Math.max(
        compactWindowWidth + (inspectorCard ? inspectorCard.minContentWidth : 440) + 8,
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
    property bool pendingHiddenSync: false
    readonly property bool shouldSyncUiLive: root.visible
        || Boolean(deviceController && (deviceController.isUiTestSuiteMode() || deviceController.isDemoShowcaseMode()))

    onVisibleChanged: {
        if (root.visible && root.pendingHiddenSync) {
            root.pendingHiddenSync = false
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
            hardwareCanvas.syncSlotsFromController()
            hardwareCanvas.syncActiveProfileSelection()
            root.refreshSelectedSlotFromController()
            root.ensureActiveProfileVisibleInList()
            Qt.callLater(root.ensureActiveProfileVisibleInList)
            root.syncLightingAnimationsForActiveProfile()
        }
    }
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

    Timer {
        id: windowWidthSyncTimer
        interval: 40
        repeat: false
        onTriggered: {
            if (root.isShowcaseCaptureWindow) return
            if (root.visibility === Window.Maximized || root.visibility === Window.FullScreen) {
                root.maximumWidth = 16777215
                return
            }
            if (!root.hasSelectedSlot) {
                const targetW = root.compactWindowWidth
                if (typeof profilesSidebar !== "undefined" && profilesSidebar) {
                    profilesSidebar.width = profilesSidebar.QQC2.SplitView.preferredWidth
                }
                root.maximumWidth = targetW
                root.width = targetW
                windowMaxReleaseTimer.restart()
            } else {
                root.maximumWidth = 16777215
                if (root.width < root.minimumWidth) {
                    root.width = root.minimumWidth
                }
            }
        }
    }

    Timer {
        id: windowMaxReleaseTimer
        interval: 90
        repeat: false
        onTriggered: {
            root.maximumWidth = 16777215
            if (!root.hasSelectedSlot
                && !root.isShowcaseCaptureWindow
                && root.visibility !== Window.Maximized
                && root.visibility !== Window.FullScreen) {
                root.width = root.compactWindowWidth
            }
        }
    }

    function syncWindowWidthToEditorState() {
        if (root.isShowcaseCaptureWindow) {
            windowWidthSyncTimer.stop()
            windowMaxReleaseTimer.stop()
            root.maximumWidth = 16777215
            root.width = 1600
            root.height = 900
            return
        }
        if (root.visibility === Window.Maximized || root.visibility === Window.FullScreen) {
            windowWidthSyncTimer.stop()
            windowMaxReleaseTimer.stop()
            root.maximumWidth = 16777215
            return
        }
        if (root.hasSelectedSlot) {
            windowWidthSyncTimer.stop()
            windowMaxReleaseTimer.stop()
            root.maximumWidth = 16777215
            if (root.width < root.minimumWidth) {
                root.width = root.minimumWidth
            }
        } else {
            const targetW = root.compactWindowWidth
            if (typeof profilesSidebar !== "undefined" && profilesSidebar) {
                profilesSidebar.width = profilesSidebar.QQC2.SplitView.preferredWidth
            }
            if (!deviceController || !deviceController.isUiTestSuiteMode()) {
                root.maximumWidth = targetW
                windowWidthSyncTimer.restart()
                windowMaxReleaseTimer.restart()
            } else {
                root.maximumWidth = 16777215
            }
            root.width = targetW
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
            { name: "RenameDeviceDialog", win: renameDeviceDialog, openFn: function() { renameDeviceDialog.openForDevice("BD762874", "duckyPad Pro", "") } },
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
            { name: "TutorialsDialog", win: tutorialsDialog, openFn: function() { root.openTutorial("oled_vm") } },
            { name: "OledDesktopWidget", win: oledDesktopWidget, openFn: function() { oledDesktopWidget.show(); oledDesktopWidget.syncFromController() } }
        ]
    }

    function openAboutWindow() {
        aboutDialog.openAboutWindow()
    }

    function openAiArchitectWindow() {
        aiModal.open()
    }

    function openEventLogWindow() {
        eventLogDialog.openLogWindow()
    }

    function openSdSyncWindow() {
        sdSyncDialog.open()
        if (deviceController) {
            deviceController.syncProfilesToDeviceSd()
        }
    }

    function openAutoSwitchDrawerWindow() {
        autoSwitchDrawer.open()
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
    readonly property bool isAnyEditorModalActive: Boolean(
        (typeof appLaunchConfigWindow !== "undefined" && appLaunchConfigWindow && appLaunchConfigWindow.visible && appLaunchConfigWindow.active)
        || (typeof webApiKeyConfigWindow !== "undefined" && webApiKeyConfigWindow && webApiKeyConfigWindow.visible && webApiKeyConfigWindow.active)
        || (typeof duckyColorPickerWindow !== "undefined" && duckyColorPickerWindow && duckyColorPickerWindow.visible && duckyColorPickerWindow.active)
        || (typeof keyPropertiesWindow !== "undefined" && keyPropertiesWindow && keyPropertiesWindow.visible && keyPropertiesWindow.active)
        || (typeof macroRecorderDialog !== "undefined" && macroRecorderDialog && macroRecorderDialog.visible && macroRecorderDialog.active)
        || (typeof lightingAnimatorDialog !== "undefined" && lightingAnimatorDialog && lightingAnimatorDialog.visible && lightingAnimatorDialog.active)
        || (typeof oledIconPickerDialog !== "undefined" && oledIconPickerDialog && oledIconPickerDialog.visible && oledIconPickerDialog.active)
    )
    readonly property bool shouldPauseAutoSwitchForEditing: root.hasSelectedSlot
        && root.visible
        && root.visibility !== Window.Minimized
        && root.visibility !== Window.Hidden
        && (root.active || root.isAnyEditorModalActive)
    onShouldPauseAutoSwitchForEditingChanged: {
        if (deviceController) {
            deviceController.setScriptEditorOpen(root.shouldPauseAutoSwitchForEditing)
        }
    }
    onHasSelectedSlotChanged: {
        if (deviceController) {
            deviceController.setScriptEditorOpen(root.shouldPauseAutoSwitchForEditing)
        }
        root.syncWindowWidthToEditorState()
        Qt.callLater(root.syncWindowWidthToEditorState)
    }
    property string selectedSlotTitle: "Key 1 (Front View)"
    property color selectedSlotColor: "#00aaff"
    // 0 = 1x Single Click (On Press), 1 = 2x Double Click, 2 = 3x Triple Click, 3 = On Release
    property int editingClickPhase: 0
    property bool editingReleaseScript: false
    property string pressScriptBuffer: ""
    property string singleClickScriptBuffer: ""
    property string doubleClickScriptBuffer: ""
    property string tripleClickScriptBuffer: ""
    property string releaseScriptBuffer: ""
    property bool loadingSlotData: false

    function splitPressBufferToPhaseBuffers(rawPress) {
        const raw = rawPress || ""
        if (deviceController) {
            try {
                const sp = JSON.parse(deviceController.splitMultiClickScriptJson(raw))
                root.singleClickScriptBuffer = sp.single || ""
                root.doubleClickScriptBuffer = sp.double || ""
                root.tripleClickScriptBuffer = sp.triple || ""
                return
            } catch (e) {}
        }
        root.singleClickScriptBuffer = raw
        root.doubleClickScriptBuffer = ""
        root.tripleClickScriptBuffer = ""
    }

    function scriptForCurrentPhase() {
        if (root.editingClickPhase === 3 || root.editingReleaseScript) return root.releaseScriptBuffer
        if (root.editingClickPhase === 1) return root.doubleClickScriptBuffer
        if (root.editingClickPhase === 2) return root.tripleClickScriptBuffer
        return root.singleClickScriptBuffer
    }

    function syncActiveEditorToPhaseBuffers() {
        if (root.editingClickPhase === 3 || root.editingReleaseScript) {
            root.releaseScriptBuffer = scriptEditor.text
        } else if (root.editingClickPhase === 1) {
            root.doubleClickScriptBuffer = scriptEditor.text
        } else if (root.editingClickPhase === 2) {
            root.tripleClickScriptBuffer = scriptEditor.text
        } else {
            root.singleClickScriptBuffer = scriptEditor.text
        }
        if (deviceController) {
            root.pressScriptBuffer = deviceController.composeMultiClickScript(
                root.singleClickScriptBuffer,
                root.doubleClickScriptBuffer,
                root.tripleClickScriptBuffer
            )
        } else {
            root.pressScriptBuffer = root.singleClickScriptBuffer
        }
        if ((root.doubleClickScriptBuffer.trim().length > 0 || root.tripleClickScriptBuffer.trim().length > 0)
            && typeof disableRepeatCheck !== "undefined" && disableRepeatCheck) {
            disableRepeatCheck.checked = true
        }
    }

    function switchEditingClickPhase(newPhase) {
        if (root.editingClickPhase === newPhase && root.editingReleaseScript === (newPhase === 3)) return
        root.syncActiveEditorToPhaseBuffers()
        root.editingClickPhase = newPhase
        root.editingReleaseScript = (newPhase === 3)
        root.loadingSlotData = true
        scriptEditor.text = root.scriptForCurrentPhase()
        scriptScrollView.contentY = 0
        scriptScrollView.contentX = 0
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        root.loadingSlotData = false
        root.updateCompilerStatus()
    }

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
        { token: "SWC_FADE_TO", category: "OLED & RGB", badge: "LED", syntax: "SWC_FADE_TO <key> <r> <g> <b> <ms>", insertText: "SWC_FADE_TO $_THIS_KEYID 255 128 0 250", summary: "Hardware smooth RGB color fade over specified duration in milliseconds (0 = all keys, 1..20 = slot)." },
        { token: "SWC_FILL", category: "OLED & RGB", badge: "LED", syntax: "SWC_FILL <r> <g> <b>", insertText: "SWC_FILL 0 170 255", summary: "Fill all key RGB LEDs with the specified (r, g, b) color." },
        { token: "SWC_RESET", category: "OLED & RGB", badge: "LED", syntax: "SWC_RESET <key>", insertText: "SWC_RESET 0", summary: "Reset key RGB LED(s) back to profile default colors (0 = all keys, 1..20 = slot)." },

        // Variables & Math
        { token: "VAR", category: "Variables & Math", badge: "VAR", syntax: "VAR $<name> = <expr>", insertText: "VAR $count = 0", summary: "Declare and initialize a 32-bit user variable ($ is optional)." },
        { token: "DEFINE", category: "Variables & Math", badge: "VAR", syntax: "DEFINE <NAME> <replacement>", insertText: "DEFINE MY_DELAY 50", summary: "Define a compile-time text replacement macro." },
        { token: "USE_STDLIB", category: "Variables & Math", badge: "FUNC", syntax: "USE_STDLIB", insertText: "USE_STDLIB", summary: "Import the standard library functions: WAITKEY(k), MEMSET(a,v,n), ABS(n), MIN(a,b), MAX(a,b)." },
        { token: "DPDSSTDLIB", category: "Variables & Math", badge: "FUNC", syntax: "DPDSSTDLIB", insertText: "DPDSSTDLIB", summary: "Alias for USE_STDLIB — import standard library functions: WAITKEY(k), MEMSET(a,v,n), ABS(n), MIN(a,b), MAX(a,b)." },
        { token: "RANDINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDINT(<min>, <max>)", insertText: "RANDINT(1, 100)", summary: "Return a random signed integer between min and max (inclusive)." },
        { token: "RANDUINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDUINT(<min>, <max>)", insertText: "RANDUINT(0, 100)", summary: "Return a random unsigned integer between min and max (inclusive)." },
        { token: "RANDCHR", category: "Variables & Math", badge: "FUNC", syntax: "RANDCHR(<bitmask>)", insertText: "RANDCHR(0x103)", summary: "Type/print a random character (or use RANDOM_LETTER, RANDOM_NUMBER, RANDOM_CHAR)." },
        { token: "RANDOM_LETTER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_LETTER", insertText: "RANDOM_LETTER", summary: "Type a random uppercase or lowercase ASCII letter." },
        { token: "RANDOM_NUMBER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_NUMBER", insertText: "RANDOM_NUMBER", summary: "Type a random digit (0–9)." },
        { token: "RANDOM_CHAR", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_CHAR", insertText: "RANDOM_CHAR", summary: "Type a random printable ASCII character." },
        { token: "WAITKEY", category: "Variables & Math", badge: "FUNC", syntax: "WAITKEY(<key_id>)", insertText: "WAITKEY(1)", summary: "Block until the specified key_id (1..20) is pressed (requires USE_STDLIB)." },
        { token: "MEMSET", category: "Variables & Math", badge: "FUNC", syntax: "MEMSET(<addr>, <val>, <len>)", insertText: "MEMSET(", summary: "Fill len bytes at memory address addr with val (requires USE_STDLIB)." },
        { token: "ABS", category: "Variables & Math", badge: "FUNC", syntax: "ABS(<n>)", insertText: "ABS(", summary: "Return absolute value of n (requires USE_STDLIB)." },
        { token: "MIN", category: "Variables & Math", badge: "FUNC", syntax: "MIN(<a>, <b>)", insertText: "MIN(", summary: "Return the smaller of a and b (requires USE_STDLIB)." },
        { token: "MAX", category: "Variables & Math", badge: "FUNC", syntax: "MAX(<a>, <b>)", insertText: "MAX(", summary: "Return the larger of a and b (requires USE_STDLIB)." },
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
    readonly property alias mainHardwareCanvas: hardwareCanvas
    readonly property alias mainInspectorCard: inspectorCard
    readonly property alias mainSidebarProfileModel: sidebarProfileModel
    readonly property alias mainProfileContextMenu: profileContextMenu
    readonly property alias mainImportProfileFileDialog: importProfileFileDialog
    readonly property alias mainLoadProfileGroupFileDialog: loadProfileGroupFileDialog
    readonly property alias mainSaveProfileGroupFileDialog: saveProfileGroupFileDialog
    readonly property alias mainAutoSwitchDrawer: autoSwitchDrawer
    readonly property alias mainAiModal: aiModal
    readonly property alias mainAppLaunchConfigWindow: appLaunchConfigWindow
    readonly property alias mainWebApiKeyConfigWindow: webApiKeyConfigWindow
    readonly property alias mainAutocompleteModel: autocompleteModel
    readonly property alias mainCommandReferenceModel: commandReferenceModel
    readonly property alias mainDeviceSelectorMenu: deviceSelectorMenu
    readonly property alias mainEventLogDialog: eventLogDialog
    readonly property alias mainOledDesktopWidget: oledDesktopWidget
    readonly property alias mainProfileSyncMenu: profileSyncMenu
    readonly property alias mainSdSyncDialog: sdSyncDialog
    readonly property var profileSidebarList: profilesSidebar ? profilesSidebar.profileSidebarList : null
    readonly property var mainProfileSidebarList: profilesSidebar ? profilesSidebar.profileSidebarList : null
    readonly property var mainProfilesBoxScrollBar: profilesSidebar ? profilesSidebar.mainProfilesBoxScrollBar : null
    readonly property var newProfileTemplateCombo: profilesSidebar ? profilesSidebar.newProfileTemplateCombo : null
    readonly property var scriptEditor: inspectorCard ? inspectorCard.scriptEditor : null
    readonly property var mainScriptEditor: inspectorCard ? inspectorCard.scriptEditor : null
    readonly property var scriptScrollView: inspectorCard ? inspectorCard.scriptScrollView : null
    readonly property var mainScriptScrollView: inspectorCard ? inspectorCard.scriptScrollView : null
    readonly property var liveSlotAutoSaveTimer: inspectorCard ? inspectorCard.liveSlotAutoSaveTimer : null
    readonly property var mainLiveSlotAutoSaveTimer: inspectorCard ? inspectorCard.liveSlotAutoSaveTimer : null
    readonly property var label1Input: inspectorCard ? inspectorCard.label1Input : null
    readonly property var mainLabel1Input: inspectorCard ? inspectorCard.label1Input : null
    readonly property var label2Input: inspectorCard ? inspectorCard.label2Input : null
    readonly property var mainLabel2Input: inspectorCard ? inspectorCard.label2Input : null
    readonly property var widgetLabel1Input: inspectorCard ? inspectorCard.widgetLabel1Input : null
    readonly property var widgetLabel2Input: inspectorCard ? inspectorCard.widgetLabel2Input : null
    readonly property var allowAbortCheck: inspectorCard ? inspectorCard.allowAbortCheck : null
    readonly property var disableRepeatCheck: inspectorCard ? inspectorCard.disableRepeatCheck : null
    readonly property var gamingHoldModeCheck: inspectorCard ? inspectorCard.gamingHoldModeCheck : null
    readonly property var mainScriptEditorVerticalScrollBar: inspectorCard ? inspectorCard.mainScriptEditorVerticalScrollBar : null
    readonly property var shortcutsLibraryCard: inspectorCard ? inspectorCard.shortcutsLibraryCard : null

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
        if (typeof gamingHoldModeCheck !== "undefined" && gamingHoldModeCheck) {
            const curPress = (root.editingClickPhase === 0 && !root.editingReleaseScript)
                ? scriptEditor.text
                : root.singleClickScriptBuffer
            const curRel = root.editingReleaseScript ? scriptEditor.text : root.releaseScriptBuffer
            gamingHoldModeCheck.checked = deviceController.isGamingHoldScript(curPress || "", curRel || "")
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

            const prevCount = sidebarProfileModel.count
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

            if (newItems.length > prevCount) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            } else if (profileSidebarList && savedContentY > 0) {
                const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                profileSidebarList.contentY = Math.min(savedContentY, maxScroll)
            }
        } catch (e) {}
    }

    function clearSlotSelection() {
        if (typeof liveSlotAutoSaveTimer !== "undefined" && liveSlotAutoSaveTimer && liveSlotAutoSaveTimer.running) {
            liveSlotAutoSaveTimer.stop()
        }
        root.flushPendingSlotEditsIfModified()
        root.selectedSlotNumber = 0
        hardwareCanvas.selectedSlot = 0
        if (typeof keyPropertiesWindow !== "undefined" && keyPropertiesWindow && keyPropertiesWindow.visible) {
            keyPropertiesWindow.close()
            keyPropertiesWindow.visible = false
        }
        if (settingsDialog && settingsDialog.keyPropertiesWindow && settingsDialog.keyPropertiesWindow.visible) {
            settingsDialog.keyPropertiesWindow.close()
            settingsDialog.keyPropertiesWindow.visible = false
        }
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
        root.splitPressBufferToPhaseBuffers(root.pressScriptBuffer)
        root.releaseScriptBuffer = root.slotSnapshotReleaseScript
        scriptEditor.text = root.scriptForCurrentPhase()
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
        root.singleClickScriptBuffer = ""
        root.doubleClickScriptBuffer = ""
        root.tripleClickScriptBuffer = ""
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
            deviceController.clearKeyCustomRgb(root.selectedSlotNumber)
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
        if (isDifferentSlotOrProfile || slotNumber > 20) {
            root.editingClickPhase = 0
            root.editingReleaseScript = false
        }

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
        root.splitPressBufferToPhaseBuffers(root.pressScriptBuffer)
        root.releaseScriptBuffer = deviceController.getSlotScript(slotNumber, true)
        scriptEditor.text = root.scriptForCurrentPhase()
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        if (isDifferentSlotOrProfile) {
            scriptScrollView.contentY = 0
            scriptScrollView.contentX = 0
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
        root.syncActiveEditorToPhaseBuffers()

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
        root.syncActiveEditorToPhaseBuffers()
        const currentPress = root.pressScriptBuffer
        const currentRelease = root.releaseScriptBuffer
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
        } else if (visibility === Window.Windowed || visibility === Window.Maximized) {
            if (deviceController
                && deviceController.checkUpdatesOnStartup
                && !deviceController.isDemoShowcaseMode()
                && !deviceController.isUiTestSuiteMode()) {
                deviceController.checkAppUpdates(false)
            }
        }
    }

    property bool appUpdateBannerDismissed: false

    Timer {
        id: appUpdateStartupTimer
        interval: 2500
        repeat: false
        running: false
        onTriggered: {
            if (deviceController
                && deviceController.checkUpdatesOnStartup
                && !deviceController.isDemoShowcaseMode()
                && !deviceController.isUiTestSuiteMode()) {
                deviceController.checkAppUpdates(false)
            }
        }
    }

    Timer {
        id: appUpdatePeriodicTimer
        interval: 21600000 // 6 hours
        repeat: true
        running: Boolean(
            deviceController
            && deviceController.checkUpdatesOnStartup
            && !deviceController.isDemoShowcaseMode()
            && !deviceController.isUiTestSuiteMode()
        )
        onTriggered: {
            if (deviceController) {
                deviceController.checkAppUpdates(false)
            }
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
            if (deviceController.checkUpdatesOnStartup
                && !deviceController.isDemoShowcaseMode()
                && !deviceController.isUiTestSuiteMode()) {
                appUpdateStartupTimer.start()
            }
        })
    }

    function triggerSyncToDeviceSd() {
        if (sdSyncDialog && sdSyncDialog.unsyncedChangesDialog && sdSyncDialog.unsyncedChangesDialog.visible) {
            sdSyncDialog.unsyncedChangesDialog.close()
            sdSyncDialog.unsyncedChangesDialog.visible = false
        }
        root.clearSlotSelection()
        root.syncWindowWidthToEditorState()
        Qt.callLater(function() {
            root.syncWindowWidthToEditorState()
            sdSyncDialog.open()
            deviceController.syncProfilesToDeviceSd()
        })
    }

    function triggerReloadFromDeviceSd() {
        if (sdSyncDialog && sdSyncDialog.unsyncedChangesDialog && sdSyncDialog.unsyncedChangesDialog.visible) {
            sdSyncDialog.unsyncedChangesDialog.close()
            sdSyncDialog.unsyncedChangesDialog.visible = false
        }
        root.clearSlotSelection()
        root.syncWindowWidthToEditorState()
        Qt.callLater(function() {
            root.syncWindowWidthToEditorState()
            sdSyncDialog.open()
            deviceController.reloadProfilesFromDeviceSd()
        })
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
                sdSyncCooldownTimer.stop()
                root.sdSyncCooldownActive = true
                root.clearSlotSelection()
                root.syncWindowWidthToEditorState()
                sdSyncDialog.open()
            } else {
                root.sdSyncCooldownActive = true
                sdSyncCooldownTimer.restart()
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                root.refreshSelectedSlotFromController()
                root.syncWindowWidthToEditorState()
                Qt.callLater(root.syncWindowWidthToEditorState)
            }
        }
        onActiveProfileChanged: {
            if (root.shouldSyncUiLive) {
                if (!root.selectingProfileFromClick) {
                    root.ensureActiveProfileVisibleInList()
                    Qt.callLater(root.ensureActiveProfileVisibleInList)
                }
                root.syncLightingAnimationsForActiveProfile()
            } else {
                root.pendingHiddenSync = true
                if (deviceController && deviceController.connected) {
                    root.syncLightingAnimationsForActiveProfile()
                }
            }
        }
        onActiveWindowClassChanged: {
            if (root.shouldSyncUiLive && deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onAutoSwitchEnabledChanged: {
            if (root.shouldSyncUiLive && deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onActiveProfileSlotsJsonChanged: {
            if (root.shouldSyncUiLive) {
                root.refreshSelectedSlotFromController()
            } else {
                root.pendingHiddenSync = true
            }
        }
        onProfilesJsonChanged: {
            if (root.shouldSyncUiLive) {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
            } else {
                root.pendingHiddenSync = true
            }
        }
        onAutoSwitchRulesJsonChanged: {
            if (root.shouldSyncUiLive) {
                root.syncSidebarProfilesFromController()
            } else {
                root.pendingHiddenSync = true
            }
        }
        onGeneratedScriptPreviewChanged: {
            if (deviceController.generatedScriptPreview.length > 0) {
                scriptEditor.text = deviceController.generatedScriptPreview
                root.syncActiveEditorToPhaseBuffers()
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
        onConnectedChanged: {
            if (!deviceController.connected
                && !deviceController.sdSyncBusy
                && !root.sdSyncCooldownActive
                && root.connectedDevicesList.length === 0
                && !deviceController.isUiTestSuiteMode()
                && !deviceController.isDemoShowcaseMode()) {
                if (oledDesktopWidget.visible) {
                    oledDesktopWidget.hide()
                    oledDesktopWidget.visible = false
                }
            }
        }
    }

    property bool sdSyncCooldownActive: false
    Timer {
        id: sdSyncCooldownTimer
        interval: 8000
        repeat: false
        onTriggered: {
            root.sdSyncCooldownActive = false
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
        hardwareCanvas: root.mainHardwareCanvas
        scriptEditor: root.mainScriptEditor
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
        root: root
        controller: deviceController
    }

    AiArchitectModal {
        id: aiModal
        transientParent: root
        root: root
        controller: deviceController
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
        profileSidebarList: root.mainProfileSidebarList
        hardwareCanvas: root.mainHardwareCanvas
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

    // Managed duckyPad & Hardware Model Selector Menu
    property var openDeviceWidgetIds: ({})
    readonly property var openDeviceWidgetIdList: Object.keys(openDeviceWidgetIds)

    function isDeviceOledWidgetOpen(devId, isSelectedDev) {
        const clean = String(devId || "").trim()
        if (!clean) return oledDesktopWidget.visible
        if (isSelectedDev && !root.openDeviceWidgetIds[clean]) {
            return oledDesktopWidget.visible
        }
        return !!root.openDeviceWidgetIds[clean]
    }

    function toggleDeviceOledWidget(devId, devLabel, isSelectedDev) {
        const clean = String(devId || "").trim()
        if (!clean) {
            deviceController.toggleOledDesktopWidget()
            return
        }
        if (isSelectedDev && !root.openDeviceWidgetIds[clean]) {
            deviceController.toggleOledDesktopWidget()
            return
        }
        const copy = Object.assign({}, root.openDeviceWidgetIds)
        if (copy[clean]) {
            delete copy[clean]
        } else {
            copy[clean] = String(devLabel || clean)
        }
        root.openDeviceWidgetIds = copy
    }

    property var availableDevicesList: {
        if (!deviceController || !deviceController.availableDevicesJson) return []
        try {
            const parsed = JSON.parse(deviceController.availableDevicesJson)
            return Array.isArray(parsed) ? parsed : []
        } catch (e) {
            return []
        }
    }

    readonly property var connectedDevicesList: {
        const list = root.availableDevicesList
        const out = []
        for (let i = 0; i < list.length; ++i) {
            if (list[i] && list[i].usbConnected) {
                out.push(list[i])
            }
        }
        return out
    }

    onConnectedDevicesListChanged: {
        if (deviceController && (deviceController.sdSyncBusy || root.sdSyncCooldownActive)) {
            return
        }
        const connectedMap = {}
        for (let i = 0; i < root.connectedDevicesList.length; ++i) {
            const d = root.connectedDevicesList[i]
            if (d && d.id) {
                connectedMap[String(d.id)] = true
            }
        }
        const openKeys = Object.keys(root.openDeviceWidgetIds || {})
        let changed = false
        const copy = Object.assign({}, root.openDeviceWidgetIds)
        for (let j = 0; j < openKeys.length; ++j) {
            const k = openKeys[j]
            if (!connectedMap[k]) {
                delete copy[k]
                changed = true
            }
        }
        if (changed) {
            root.openDeviceWidgetIds = copy
        }
        if (root.connectedDevicesList.length === 0
            && (!deviceController || (!deviceController.connected && !deviceController.sdSyncBusy && !root.sdSyncCooldownActive && !deviceController.isUiTestSuiteMode() && !deviceController.isDemoShowcaseMode()))) {
            if (oledDesktopWidget.visible) {
                oledDesktopWidget.hide()
                oledDesktopWidget.visible = false
            }
        }
    }

    QQC2.Dialog {
        id: renameDeviceDialog
        readonly property bool closableWithEscape: true
        property string targetSerial: ""
        property string targetModelName: ""
        enter: null
        exit: null
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: renameDeviceDialog.visible
            onActivated: renameDeviceDialog.close()
        }
        anchors.centerIn: parent
        modal: true
        width: 440
        title: qsTr("Name Hardware duckyPad")
        standardButtons: QQC2.Dialog.Save | QQC2.Dialog.Cancel

        function openForDevice(serialNum, modelLabel, currentCustomName) {
            renameDeviceDialog.targetSerial = String(serialNum || (deviceController ? deviceController.serialNumber : "")).trim()
            renameDeviceDialog.targetModelName = String(modelLabel || (deviceController ? deviceController.modelName : "duckyPad")).trim()
            renameDeviceInput.text = String(currentCustomName !== undefined ? currentCustomName : (deviceController ? deviceController.customDeviceName : ""))
            renameDeviceDialog.open()
            Qt.callLater(function() {
                renameDeviceInput.forceActiveFocus()
                renameDeviceInput.selectAll()
            })
        }

        ColumnLayout {
            width: parent.width
            spacing: 10

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.textColor
                text: qsTr("Assign a friendly name to %1 (SN: %2). Leave blank to use the default hardware model and serial label.")
                    .arg(renameDeviceDialog.targetModelName || "duckyPad")
                    .arg(renameDeviceDialog.targetSerial || "-")
            }

            QQC2.TextField {
                id: renameDeviceInput
                Layout.fillWidth: true
                font.pointSize: 10.5 * root.fontScale
                placeholderText: qsTr("e.g. Desk Pro, Stream Deck, Home Assistant Pad...")
                onAccepted: renameDeviceDialog.accept()
            }
        }

        onAccepted: {
            if (deviceController && renameDeviceDialog.targetSerial.length > 0 && renameDeviceDialog.targetSerial !== "-") {
                deviceController.assignCustomDeviceName(renameDeviceDialog.targetSerial, renameDeviceInput.text.trim())
            }
        }
    }

    QQC2.Menu {
        id: deviceSelectorMenu
        font.pointSize: 10 * root.fontScale
        title: qsTr("Select Managed duckyPad")
        onAboutToShow: {
            if (deviceController) {
                deviceController.scanAvailableDevices()
            }
        }

        QQC2.MenuItem {
            enabled: false
            font.bold: true
            text: deviceController && deviceController.connectedDeviceCount > 1
                ? qsTr("Attached duckyPads (%1 Connected)").arg(deviceController.connectedDeviceCount)
                : qsTr("Managed duckyPad & SD Card Target")
        }

        Repeater {
            model: root.availableDevicesList
            delegate: QQC2.MenuItem {
                required property var modelData
                icon.name: modelData.usbConnected
                    ? "input-keyboard"
                    : (modelData.sdMounted ? "media-flash-sd-mmc" : "folder-saved-search")
                font.bold: !!modelData.isSelected
                text: {
                    const prefix = modelData.isSelected ? "✓ " : "    "
                    const tags = []
                    if (modelData.usbConnected) tags.push(qsTr("USB HID"))
                    if (modelData.sdMounted) {
                        tags.push(modelData.diskLabel ? qsTr("SD: %1").arg(modelData.diskLabel) : qsTr("SD Mounted"))
                    } else if (!modelData.usbConnected && modelData.hasCachedProfiles) {
                        tags.push(qsTr("Cached"))
                    }
                    const tagStr = tags.length > 0 ? (" [" + tags.join(" + ") + "]") : ""
                    return prefix + (modelData.displayLabel || modelData.id) + tagStr
                }
                onTriggered: {
                    if (deviceController && modelData.id) {
                        if (deviceController.selectManagedDevice(modelData.id)) {
                            root.syncSidebarProfilesFromController()
                            hardwareCanvas.syncProfilesFromController()
                            hardwareCanvas.syncSlotsFromController()
                            root.refreshSelectedSlotFromController()
                        }
                    }
                }
            }
        }

        QQC2.MenuItem {
            visible: deviceController && deviceController.serialNumber && deviceController.serialNumber !== "-"
            icon.name: "edit-rename"
            text: deviceController && deviceController.customDeviceName && deviceController.customDeviceName.length > 0
                ? qsTr("✏️ Rename This duckyPad (\"%1\")...").arg(deviceController.customDeviceName)
                : qsTr("✏️ Give This duckyPad a Name (SN: %1)...").arg(deviceController ? deviceController.serialNumber : "")
            onTriggered: {
                if (deviceController) {
                    renameDeviceDialog.openForDevice(
                        deviceController.serialNumber,
                        deviceController.modelName,
                        deviceController.customDeviceName
                    )
                }
            }
        }

        QQC2.MenuSeparator {
            visible: root.connectedDevicesList.length > 0
        }

        QQC2.MenuItem {
            visible: root.connectedDevicesList.length > 0
            enabled: false
            font.bold: true
            text: qsTr("Desktop OLED Widgets (Connected duckyPads)")
        }

        Repeater {
            model: root.connectedDevicesList
            delegate: QQC2.MenuItem {
                required property var modelData
                readonly property bool widgetOpen: root.isDeviceOledWidgetOpen(modelData.id, !!modelData.isSelected)
                icon.name: "view-fullscreen"
                font.bold: widgetOpen
                text: (widgetOpen ? "✓ 🖥️ " : "    🖥️ ")
                    + qsTr("OLED Widget: %1").arg(modelData.displayLabel || modelData.id)
                onTriggered: {
                    const shortBadge = modelData.customName && modelData.customName.length > 0
                        ? modelData.customName
                        : ((modelData.hwModel === 20 ? "DP20·" : "DP24·")
                            + String(modelData.serialNumber || modelData.id).slice(-4))
                    root.toggleDeviceOledWidget(modelData.id, shortBadge, !!modelData.isSelected)
                }
            }
        }

        QQC2.MenuSeparator {
            visible: deviceController && deviceController.esp32HaDetected
        }

        QQC2.MenuItem {
            visible: deviceController && deviceController.esp32HaDetected
            icon.name: "network-server"
            font.bold: deviceController && deviceController.esp32HaServerRunning
            text: deviceController && deviceController.esp32HaServerRunning
                ? qsTr("✓ 🌐 Stop ESP32-HA Web Server (:%1)").arg(deviceController.esp32HaServerPort)
                : qsTr("🌐 Start ESP32-HA Web Server (:%1)").arg(deviceController ? deviceController.esp32HaServerPort : 8099)
            onTriggered: {
                if (deviceController) {
                    deviceController.toggleEsp32HaWebServer()
                }
            }
        }

        QQC2.MenuItem {
            visible: deviceController && deviceController.esp32HaDetected
            icon.name: "internet-web-browser"
            text: qsTr("↗ Open ESP32-HA Web UI in Browser (%1)").arg(deviceController ? deviceController.esp32HaServerUrl : "http://127.0.0.1:8099")
            onTriggered: {
                if (deviceController) {
                    deviceController.openEsp32HaWebUi()
                }
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "input-keyboard"
            font.bold: deviceController && !deviceController.isDuckyPad2020
            text: (deviceController && !deviceController.isDuckyPad2020 ? "✓ " : "    ")
                + qsTr("Layout: duckyPad Pro (20 Keys + 2 Encoders)")
            onTriggered: {
                if (deviceController) {
                    deviceController.setSimulatedDeviceModel(24)
                    hardwareCanvas.syncSlotsFromController()
                    root.refreshSelectedSlotFromController()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "input-keyboard"
            font.bold: deviceController && deviceController.isDuckyPad2020
            text: (deviceController && deviceController.isDuckyPad2020 ? "✓ " : "    ")
                + qsTr("Layout: duckyPad (2020) (15 Keys • 3×5)")
            onTriggered: {
                if (deviceController) {
                    deviceController.setSimulatedDeviceModel(20)
                    hardwareCanvas.syncSlotsFromController()
                    root.refreshSelectedSlotFromController()
                }
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("Scan Attached duckyPads & Mounted SD Cards")
            onTriggered: {
                if (deviceController) {
                    deviceController.scanAvailableDevices()
                    deviceController.refreshDevice()
                    root.refreshSelectedSlotFromController()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "settings-configure"
            text: qsTr("Manage Devices in Settings...")
            onTriggered: settingsDialog.open()
        }
    }

    // Hamburger Menu for Action Buttons
    QQC2.Menu {
        font.pointSize: 10 * root.fontScale
        id: hamburgerMenu
        title: qsTr("Menu")
        onAboutToShow: {
            deviceController.refreshSdMountStatus()
            deviceController.scanAvailableDevices()
        }

        QQC2.MenuItem {
            icon.name: "network-connect"
            text: deviceController.connectedDeviceCount > 1
                ? qsTr("🔀 Switch Managed duckyPad (%1 Connected)...").arg(deviceController.connectedDeviceCount)
                : qsTr("🔀 Switch Managed duckyPad / Model (%1)...").arg(deviceController.modelName)
            onTriggered: deviceSelectorMenu.popup()
        }

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
            id: menuTutorialsItem
            icon.name: "help-contextual"
            text: qsTr("🎓 Interactive Tutorials & Step-by-Step Guides...")
            onTriggered: root.openTutorial("oled_vm")
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
            icon.name: "system-software-update"
            text: deviceController.appUpdateAvailable
                ? qsTr("⬇ Update DuckyDeck Studio (v%1 Available)...").arg(deviceController.latestAppVersion)
                : qsTr("Check for App Updates...")
            onTriggered: {
                if (deviceController.appUpdateAvailable) {
                    aboutDialog.openAboutWindow()
                } else {
                    deviceController.checkAppUpdates(true)
                }
            }
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
        tutorialsDialog: root.tutorialsDialog
        aiModal: aiModal
        oledDesktopWidget: oledDesktopWidget
        hardwareCanvas: root.mainHardwareCanvas
        profilesBoxScrollBar: root.mainProfilesBoxScrollBar
        scriptScrollView: root.mainScriptScrollView
        scriptEditorVerticalScrollBar: root.mainScriptEditorVerticalScrollBar
        scriptEditor: root.mainScriptEditor
        label1Input: root.mainLabel1Input
        label2Input: root.mainLabel2Input
    }


    SdSyncDialog {
        id: sdSyncDialog
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        effectiveThemeMode: root.effectiveThemeMode
        hardwareCanvas: root.mainHardwareCanvas
        inspectorCard: root.mainInspectorCard
    }

    Loader {
        id: profileWizardLoader
        active: false
        source: Qt.resolvedUrl("ProfileWizardDialog.qml")
        onLoaded: {
            if (item) {
                item.root = root
                item.controller = deviceController
            }
        }
    }

    function openProfileWizard() {
        if (!profileWizardLoader.active) {
            profileWizardLoader.active = true
        }
        if (profileWizardLoader.item) {
            profileWizardLoader.item.root = root
            profileWizardLoader.item.controller = deviceController
            profileWizardLoader.item.openWizard()
        }
    }

    Loader {
        id: tutorialsDialogLoader
        active: deviceController ? deviceController.isUiTestSuiteMode() : false
        source: Qt.resolvedUrl("TutorialsDialog.qml")
        onLoaded: {
            if (item) {
                item.root = root
                item.controller = deviceController
            }
        }
    }
    readonly property var tutorialsDialog: tutorialsDialogLoader.item

    function openTutorial(topicId) {
        if (!tutorialsDialogLoader.active) {
            tutorialsDialogLoader.active = true
        }
        if (tutorialsDialogLoader.item) {
            tutorialsDialogLoader.item.root = root
            tutorialsDialogLoader.item.controller = deviceController
            tutorialsDialogLoader.item.openTopic(topicId || "oled_vm")
        }
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
                            Layout.preferredWidth: 12
                            Layout.preferredHeight: 12
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

                            QQC2.ToolTip.delay: 500
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

                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: headerKofiMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Buy Russell Milliner a coffee on Ko-fi (https://ko-fi.com/russellmilliner)")
                }

                QQC2.Button {
                    id: headerTutorialsBtn
                    icon.name: "help-contextual"
                    text: qsTr("🎓 Tutorials")
                    font.pointSize: 10 * root.fontScale
                    implicitHeight: 30
                    onClicked: root.openTutorial("oled_vm")
                    QQC2.ToolTip.delay: 400
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Open Interactive Tutorials (OLED VM & HUD, Home Assistant, Webhooks, App Launching, Auto-Switcher & Modifier Layers, Multi-Click, Global Variables)")
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    id: hamburgerMenuBtn
                    icon.name: "application-menu"
                    display: QQC2.AbstractButton.IconOnly
                    implicitWidth: 36
                    implicitHeight: 32
                    onClicked: hamburgerMenu.popup(hamburgerMenuBtn, 0, hamburgerMenuBtn.height)
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Menu (Device Actions, Profile Groups, Import/Export, Rules, Variables, AI & Settings)")
                }
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            Rectangle {
                id: appUpdateBanner
                visible: deviceController.appUpdateAvailable && !root.appUpdateBannerDismissed
                Layout.fillWidth: true
                implicitHeight: appUpdateBannerRow.implicitHeight + 16
                radius: 6
                color: Qt.tint(
                    Kirigami.Theme.alternateBackgroundColor,
                    Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.18)
                )
                border.color: Kirigami.Theme.highlightColor
                border.width: 1

                RowLayout {
                    id: appUpdateBannerRow
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 10

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                        text: deviceController.appUpdateStatus.length > 0
                            ? deviceController.appUpdateStatus
                            : qsTr("⬆ DuckyDeck Studio v%1 is available! (Installed: v%2)")
                                .arg(deviceController.latestAppVersion)
                                .arg(deviceController.appVersion)
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        icon.name: "system-software-update"
                        font.bold: true
                        highlighted: true
                        enabled: !deviceController.appUpdateInstallBusy
                        text: deviceController.appUpdateInstallBusy
                            ? qsTr("Installing...")
                            : qsTr("⬇ Install v%1").arg(deviceController.latestAppVersion)
                        onClicked: deviceController.installAppUpdate()
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        icon.name: "internet-web-browser"
                        text: qsTr("Release Notes")
                        onClicked: deviceController.openAppUpdatePage()
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        text: "✕"
                        flat: true
                        implicitWidth: 28
                        onClicked: root.appUpdateBannerDismissed = true
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Dismiss update banner")
                    }
                }
            }

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
                ProfileSidebar {
                    id: profilesSidebar
                    root: root
                    controller: deviceController
                    hardwareCanvas: root.mainHardwareCanvas
                    autoSwitchDrawer: root.mainAutoSwitchDrawer
                    profileContextMenu: root.mainProfileContextMenu
                    sidebarProfileModel: root.mainSidebarProfileModel
                    importProfileFileDialog: root.mainImportProfileFileDialog
                    loadProfileGroupFileDialog: root.mainLoadProfileGroupFileDialog
                    saveProfileGroupFileDialog: root.mainSaveProfileGroupFileDialog
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
                    activeSlotScript: root.hasSelectedSlot ? scriptEditor.text : ""

                    onOpenTutorialRequested: (topicId) => {
                        root.openTutorial(topicId)
                    }

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
                SlotInspectorPane {
                    id: inspectorCard
                    root: root
                    controller: deviceController
                    hardwareCanvas: root.mainHardwareCanvas
                    aiModal: root.mainAiModal
                    appLaunchConfigWindow: root.mainAppLaunchConfigWindow
                    webApiKeyConfigWindow: root.mainWebApiKeyConfigWindow
                    autocompleteModel: root.mainAutocompleteModel
                    commandReferenceModel: root.mainCommandReferenceModel
                }
            }
        }
    }

    // Bottom Status Bar (2- or 3-Line Layout: Line 1 = Connection, Sync & Live Status; Line 2 = Firmware Version, SD Card Space & OLED Widget Toggle; Line 3 = Per-Device OLED Widgets & ESP32-HA Web Server)
    footer: BottomStatusBar {
        id: bottomStatusBar
        root: root
        controller: deviceController
        hardwareCanvas: root.mainHardwareCanvas
        deviceSelectorMenu: root.mainDeviceSelectorMenu
        eventLogDialog: root.mainEventLogDialog
        oledDesktopWidget: root.mainOledDesktopWidget
        profileSyncMenu: root.mainProfileSyncMenu
        sdSyncDialog: root.mainSdSyncDialog
    }

    AppLaunchBuilder {
        id: appLaunchConfigWindow
        transientParent: root
        root: root
        controller: deviceController
        fontScale: root.fontScale
        highContrastPositiveText: root.highContrastPositiveText
        highContrastWarningText: root.highContrastWarningText
        hardwareCanvas: root.mainHardwareCanvas
        liveSlotAutoSaveTimer: root.mainLiveSlotAutoSaveTimer
        label1Input: root.mainLabel1Input
        label2Input: root.mainLabel2Input
        scriptEditor: root.mainScriptEditor
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
        hardwareCanvas: root.mainHardwareCanvas
        scriptEditor: root.mainScriptEditor
        keyPropertiesWindow: settingsDialog.keyPropertiesWindow
    }

    // =========================================================================
    // Profile Properties Window (Editable Fields for All Profile Properties)
    // =========================================================================

    property int showcaseStep: 0
    property var draggingShortcutItem: null
    property real shortcutDragOverlayX: 0
    property real shortcutDragOverlayY: 0

    // Floating Lifted Keycap Overlay during Shortcut Drag-and-Drop in Main Window
    Item {
        id: mainFloatingShortcutOverlay
        parent: root.contentItem
        visible: root.draggingShortcutItem !== null
        x: root.shortcutDragOverlayX
        y: root.shortcutDragOverlayY
        width: 66
        height: 66
        z: 9999
        scale: 1.08
        opacity: 0.94

        readonly property color dragLedColor: root.draggingShortcutItem
            ? shortcutsLibraryCard.shortcutCategoryColor(root.draggingShortcutItem.category)
            : Kirigami.Theme.highlightColor

        Rectangle {
            anchors.fill: parent
            radius: 8
            color: Kirigami.Theme.backgroundColor
            border.width: 2
            border.color: hardwareCanvas.dropTargetSlot >= 1
                ? Kirigami.Theme.positiveTextColor
                : Kirigami.Theme.highlightColor

            Rectangle {
                x: 22
                y: 4
                width: 34
                height: 4
                radius: 2
                color: mainFloatingShortcutOverlay.dragLedColor
            }

            Rectangle {
                anchors.fill: parent
                anchors.margins: 5
                anchors.topMargin: 12
                radius: 6
                clip: true
                color: Qt.tint(
                    Kirigami.Theme.backgroundColor,
                    Qt.rgba(
                        mainFloatingShortcutOverlay.dragLedColor.r,
                        mainFloatingShortcutOverlay.dragLedColor.g,
                        mainFloatingShortcutOverlay.dragLedColor.b,
                        0.28
                    )
                )
                border.width: 1
                border.color: hardwareCanvas.dropTargetSlot >= 1
                    ? Kirigami.Theme.positiveTextColor
                    : Kirigami.Theme.highlightColor

                Column {
                    anchors.centerIn: parent
                    width: parent.width - 6
                    spacing: 0

                    QQC2.Label {
                        width: parent.width
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: root.draggingShortcutItem ? (root.draggingShortcutItem.label_l1 || "") : ""
                        font.pixelSize: 10
                        fontSizeMode: Text.HorizontalFit
                        minimumPixelSize: 7
                        elide: Text.ElideRight
                        font.bold: true
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        visible: !!(root.draggingShortcutItem && root.draggingShortcutItem.label_l2)
                        width: parent.width
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: root.draggingShortcutItem ? (root.draggingShortcutItem.label_l2 || "") : ""
                        font.pixelSize: 9
                        fontSizeMode: Text.HorizontalFit
                        minimumPixelSize: 7
                        elide: Text.ElideRight
                        font.weight: Font.Medium
                        color: Kirigami.Theme.disabledTextColor
                    }
                }
            }
        }
    }

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
        targetDeviceLabel: deviceController ? deviceController.customDeviceName : ""
        selectedSlot: root.selectedSlotNumber
        keyEditorOpen: root.shouldPauseAutoSwitchForEditing
        onRequestOpenMainWindow: {
            if (deviceController) {
                deviceController.showFromTray()
            } else {
                root.visible = true
                root.showNormal()
                root.raise()
                root.requestActivate()
            }
        }
    }

    Repeater {
        id: perDeviceOledWidgetRepeater
        model: root.openDeviceWidgetIdList
        delegate: OledDesktopWidget {
            id: secondaryOledWidget
            required property string modelData
            required property int index
            property bool initializedVisible: false
            controller: deviceController
            targetDeviceId: modelData
            targetDeviceLabel: String(root.openDeviceWidgetIds[modelData] || "")
            visible: true
            x: oledDesktopWidget.x + (index + 1) * 304
            y: oledDesktopWidget.y
            Component.onCompleted: {
                secondaryOledWidget.show()
                secondaryOledWidget.raise()
                secondaryOledWidget.initializedVisible = true
            }
            onVisibleChanged: {
                if (visible) {
                    secondaryOledWidget.initializedVisible = true
                } else if (secondaryOledWidget.initializedVisible && root.openDeviceWidgetIds[modelData]) {
                    const copy = Object.assign({}, root.openDeviceWidgetIds)
                    delete copy[modelData]
                    root.openDeviceWidgetIds = copy
                }
            }
            onRequestOpenMainWindow: {
                if (deviceController) {
                    deviceController.showFromTray()
                } else {
                    root.visible = true
                    root.showNormal()
                    root.raise()
                    root.requestActivate()
                }
            }
        }
    }

    // =========================================================================
    // AUTOMATED UI TEST SUITE & SCENE-GRAPH AUDITOR (--ui-test-suite)
    // Verifies behavioral specifications, zero window/element overflow, and
    // WCAG color contrast across all windows/dialogs in Dark and Light themes.
    // =========================================================================
}


