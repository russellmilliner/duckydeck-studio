import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
    objectName: "duckydeckMainWindow"
    title: qsTr("DuckyDeck Studio — %1").arg(deviceController.modelName)
    width: 1420
    height: 800
    minimumWidth: 1080
    minimumHeight: 680

    color: Kirigami.Theme.backgroundColor

    property int selectedSlotNumber: 1
    property string selectedSlotTitle: "Key 1 (Front View)"
    property color selectedSlotColor: "#00aaff"
    property bool editingReleaseScript: false
    property string pressScriptBuffer: ""
    property string releaseScriptBuffer: ""
    property bool loadingSlotData: false

    property string profileFilterText: ""
    property int totalProfileCount: 1
    property int totalRuleCount: 0
    property bool showNewProfileInput: false

    property bool compileOk: true
    property bool compileEmpty: false
    property int compileByteCount: 0
    property int compileErrorLine: 0
    property string compileErrorMsg: ""

    property ListModel sidebarProfileModel: ListModel {}

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
                if (filter.length > 0
                    && !nameStr.toLowerCase().includes(filter)
                    && !boundStr.toLowerCase().includes(filter)
                    && !numStr.includes(filter)) {
                    continue
                }
                let tooltipParts = []
                for (let k = 0; k < profRules.length; ++k) {
                    const pr = profRules[k]
                    const tPart = pr.titleContains && pr.titleContains.length > 0
                        ? (" (title: \"" + pr.titleContains + "\")")
                        : ""
                    tooltipParts.push(pr.resourceClass + tPart)
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

    function loadSlotFromController(slotNumber, slotTitle, slotColor) {
        root.loadingSlotData = true
        root.selectedSlotNumber = slotNumber
        root.selectedSlotTitle = slotTitle
        root.selectedSlotColor = slotColor

        try {
            const metaJson = deviceController.getSlotMetaJson(slotNumber)
            const meta = JSON.parse(metaJson)
            label1Input.text = meta.l1 || ""
            label2Input.text = meta.l2 || ""
            allowAbortCheck.checked = !!meta.allowAbort
            disableRepeatCheck.checked = !!meta.disableAutorepeat
        } catch (e) {
            label1Input.text = ""
            label2Input.text = ""
            allowAbortCheck.checked = false
            disableRepeatCheck.checked = false
        }

        root.pressScriptBuffer = deviceController.getSlotScript(slotNumber, false)
        root.releaseScriptBuffer = deviceController.getSlotScript(slotNumber, true)
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.loadingSlotData = false
        root.updateCompilerStatus()
    }

    function saveCurrentSlotToController() {
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
    }

    function refreshSelectedSlotFromController() {
        if (sidebarProfileModel.count === 0) {
            root.syncSidebarProfilesFromController()
        }
        hardwareCanvas.syncSlotsFromController()
        root.loadSlotFromController(
            root.selectedSlotNumber,
            hardwareCanvas.slotTitleFor(root.selectedSlotNumber),
            hardwareCanvas.slotColorFor(root.selectedSlotNumber)
        )
    }

    readonly property bool isDarkSystemTheme: Kirigami.Theme.backgroundColor.hslLightness < 0.5
    onIsDarkSystemThemeChanged: {
        if (deviceController) {
            deviceController.notifySystemDarkTheme(isDarkSystemTheme)
        }
    }

    onVisibilityChanged: (visibility) => {
        if (visibility === Window.Minimized) {
            Qt.callLater(function() {
                if (deviceController) {
                    deviceController.minimizeToTray()
                }
                root.hide()
            })
        }
    }

    onClosing: (close) => {
        if (deviceController && deviceController.closeToTray) {
            close.accepted = false
            deviceController.minimizeToTray()
            root.hide()
        }
    }

    Component.onCompleted: {
        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
        deviceController.refreshDevice()
        root.syncSidebarProfilesFromController()
        hardwareCanvas.syncProfilesFromController()
        root.refreshSelectedSlotFromController()
    }

    DeviceController {
        id: deviceController
        onActiveProfileChanged: {
            if (!root.selectingProfileFromClick && deviceController.autoSwitchEnabled) {
                root.ensureActiveProfileVisibleInList()
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
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

    // Profile Right-Click Context Menu
    QQC2.Menu {
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
            icon.name: "checkmark"
            text: qsTr("Select Profile")
            onTriggered: {
                const savedY = profileSidebarList.contentY
                root.selectingProfileFromClick = true
                deviceController.selectProfile(root.contextMenuProfileNumber)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                root.selectingProfileFromClick = false
            }
        }

        QQC2.MenuItem {
            icon.name: "crosshairs"
            text: qsTr("Bind Window to This Profile (Pick Window)...")
            onTriggered: {
                const savedY = profileSidebarList.contentY
                root.selectingProfileFromClick = true
                deviceController.selectProfile(root.contextMenuProfileNumber)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                root.selectingProfileFromClick = false
                deviceController.armWindowPicker(true)
            }
        }

        QQC2.MenuItem {
            icon.name: "preferences-system-windows"
            text: qsTr("Auto-Switch Rules...")
            onTriggered: autoSwitchDrawer.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "go-up"
            text: qsTr("Move Profile Up")
            enabled: profileSidebarList.canReorder && root.contextMenuProfileIndex > 0
            onTriggered: {
                const savedY = profileSidebarList.contentY
                deviceController.reorderProfile(root.contextMenuProfileIndex, root.contextMenuProfileIndex - 1)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
            }
        }

        QQC2.MenuItem {
            icon.name: "go-down"
            text: qsTr("Move Profile Down")
            enabled: profileSidebarList.canReorder && root.contextMenuProfileIndex < root.sidebarProfileModel.count - 1
            onTriggered: {
                const savedY = profileSidebarList.contentY
                deviceController.reorderProfile(root.contextMenuProfileIndex, root.contextMenuProfileIndex + 1)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
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

    // Settings Menu (Theme Light / Dark / System, Taskbar Icon Theme, KDE System Autostart)
    QQC2.Menu {
        id: settingsMenu
        title: qsTr("Settings")

        QQC2.MenuItem {
            text: qsTr("Application Theme")
            enabled: false
            font.bold: true
        }

        QQC2.MenuItem {
            text: qsTr("◐ System Theme")
            checkable: true
            checked: deviceController.appThemeMode === "system"
            onTriggered: deviceController.applyAppThemeMode("system")
        }

        QQC2.MenuItem {
            text: qsTr("☀ Light Theme")
            checkable: true
            checked: deviceController.appThemeMode === "light"
            onTriggered: deviceController.applyAppThemeMode("light")
        }

        QQC2.MenuItem {
            text: qsTr("🌙 Dark Theme")
            checkable: true
            checked: deviceController.appThemeMode === "dark"
            onTriggered: deviceController.applyAppThemeMode("dark")
        }

        QQC2.MenuSeparator {}

        QQC2.Menu {
            title: qsTr("Taskbar Tray Icon Theme")

            QQC2.MenuItem {
                text: qsTr("◐ Auto (Match KDE Theme)")
                checkable: true
                checked: deviceController.trayIconMode === "auto"
                onTriggered: deviceController.applyTrayIconMode("auto")
            }

            QQC2.MenuItem {
                text: qsTr("🌙 Dark Panel (Light Icon)")
                checkable: true
                checked: deviceController.trayIconMode === "dark"
                onTriggered: deviceController.applyTrayIconMode("dark")
            }

            QQC2.MenuItem {
                text: qsTr("☀ Light Panel (Dark Icon)")
                checkable: true
                checked: deviceController.trayIconMode === "light"
                onTriggered: deviceController.applyTrayIconMode("light")
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            text: qsTr("Start on KDE System Login (Autostart)")
            checkable: true
            checked: deviceController.autostartEnabled
            onTriggered: deviceController.applyAutostartEnabled(checked)
        }

        QQC2.MenuItem {
            text: qsTr("Keep Running in Taskbar on Window Close")
            checkable: true
            checked: deviceController.closeToTray
            onTriggered: deviceController.setCloseToTrayEnabled(checked)
        }
    }

    // Hamburger Menu for Action Buttons
    QQC2.Menu {
        id: hamburgerMenu
        title: qsTr("Menu")

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("Scan USB HID")
            onTriggered: {
                deviceController.refreshDevice()
                root.refreshSelectedSlotFromController()
            }
        }

        QQC2.MenuItem {
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
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-import"
            text: qsTr("Import Profile (File / config.txt)...")
            onTriggered: importProfileFileDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Import Profile Folder...")
            onTriggered: importProfileFolderDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "preferences-system-windows"
            text: qsTr("KWin Auto-Switch Rules (%1)...").arg(root.totalRuleCount)
            onTriggered: autoSwitchDrawer.open()
        }

        QQC2.MenuItem {
            icon.name: "preferences-system-time"
            text: qsTr("Sync RTC Clock")
            enabled: deviceController.connected
            onTriggered: deviceController.syncRtcClock()
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
            text: qsTr("AI Macro Architect...")
            onTriggered: aiModal.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "application-exit"
            text: qsTr("Exit")
            onTriggered: deviceController.quitApplication()
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
                    text: qsTr("DuckyDeck Studio")
                    font.bold: true
                    font.pixelSize: 16
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
                    implicitWidth: activeProfHeaderLabel.implicitWidth + 16
                    implicitHeight: 26

                    QQC2.Label {
                        id: activeProfHeaderLabel
                        anchors.centerIn: parent
                        text: qsTr("Profile #%1 — %2")
                            .arg(deviceController.activeProfile)
                            .arg(deviceController.activeProfileName)
                        font.bold: true
                        font.pixelSize: 12
                        color: Kirigami.Theme.textColor
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    id: settingsMenuBtn
                    icon.name: "settings-configure"
                    text: qsTr("Settings")
                    implicitHeight: 32
                    onClicked: settingsMenu.popup(settingsMenuBtn, 0, settingsMenuBtn.height)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Application Theme (Light / Dark / System), Taskbar Icon & KDE Autostart")
                }

                QQC2.Button {
                    id: hamburgerMenuBtn
                    icon.name: "application-menu"
                    text: qsTr("☰")
                    implicitHeight: 32
                    onClicked: hamburgerMenu.popup(hamburgerMenuBtn, 0, hamburgerMenuBtn.height)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Device Actions, Import/Export Profiles, KWin Auto-Switch, Variables & AI")
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

                // Column 1: Profiles & KWin Wayland Auto-Switch Sidebar
                Rectangle {
                    id: profilesSidebar
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property color frameBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

                    QQC2.SplitView.preferredWidth: 255
                    QQC2.SplitView.minimumWidth: 210
                    QQC2.SplitView.maximumWidth: 380
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

                            QQC2.Label {
                                text: qsTr("PROFILES (%1/64)").arg(root.totalProfileCount)
                                font.bold: true
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                            }

                            QQC2.Button {
                                text: qsTr("↻ SD")
                                implicitHeight: 22
                                font.pixelSize: 10
                                onClicked: {
                                    const found = deviceController.scanMountedSdCard()
                                    if (found.length > 0) {
                                        root.refreshSelectedSlotFromController()
                                    } else {
                                        sdFolderDialog.open()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Scan / Reload duckyPad SD Card")
                            }
                        }

                        // Filter Search Input
                        QQC2.TextField {
                            id: profileFilterField
                            Layout.fillWidth: true
                            placeholderText: qsTr("Filter profiles or apps (drag ⠿ to reorder)...")
                            selectByMouse: true
                            font.pixelSize: 11
                            onTextChanged: {
                                root.profileFilterText = text
                                root.syncSidebarProfilesFromController()
                            }
                        }

                        // Scrollable 64-Profile List with Drag-and-Drop Reordering
                        QQC2.ScrollView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            ListView {
                                id: profileSidebarList
                                model: root.sidebarProfileModel
                                spacing: 4
                                highlightFollowsCurrentItem: false

                                property int draggedProfileIndex: -1
                                property int dropTargetIndex: -1
                                readonly property bool canReorder: root.profileFilterText.trim().length === 0

                                delegate: Rectangle {
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
                                    readonly property color profColor: Qt.rgba(bgR / 255.0, bgG / 255.0, bgB / 255.0, 1.0)

                                    width: profileSidebarList.width - 8
                                    height: 40
                                    radius: 6
                                    z: isBeingDragged ? 10 : 1
                                    opacity: isBeingDragged ? 0.85 : 1.0
                                    color: isDropTarget
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.35))
                                        : (isCurrent
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.22))
                                            : (profMouse.containsMouse ? Kirigami.Theme.alternateBackgroundColor : "transparent"))
                                    border.width: (isCurrent || isDropTarget || isBeingDragged) ? 1.5 : 1
                                    border.color: (isCurrent || isDropTarget || isBeingDragged)
                                        ? Kirigami.Theme.highlightColor
                                        : (profMouse.containsMouse ? profilesSidebar.frameBorder : "transparent")

                                    // Drop target indicator line
                                    Rectangle {
                                        visible: profDelegate.isDropTarget
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        y: profileSidebarList.draggedProfileIndex > profDelegate.index ? 0 : parent.height - 2
                                        height: 2
                                        radius: 1
                                        color: Kirigami.Theme.highlightColor
                                    }

                                    MouseArea {
                                        id: profMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                                        cursorShape: profMouse.dragging
                                            ? Qt.ClosedHandCursor
                                            : Qt.ArrowCursor

                                        property real pressY: 0
                                        property bool dragging: false

                                        onPressed: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                dragging = false
                                                root.contextMenuProfileNumber = profDelegate.profNumber
                                                root.contextMenuProfileName = profDelegate.profName
                                                root.contextMenuProfileIndex = profDelegate.index
                                                root.exportTargetProfileNumber = profDelegate.profNumber
                                                profileContextMenu.popup()
                                                return
                                            }
                                            pressY = mouse.y
                                            dragging = false
                                        }

                                        onPositionChanged: (mouse) => {
                                            if (!pressed || (mouse.buttons & Qt.RightButton) || !profileSidebarList.canReorder) return
                                            if (!dragging && Math.abs(mouse.y - pressY) > 6) {
                                                dragging = true
                                                profileSidebarList.interactive = false
                                                profileSidebarList.draggedProfileIndex = profDelegate.index
                                                profileSidebarList.dropTargetIndex = profDelegate.index
                                            }
                                            if (dragging) {
                                                const posInContent = profMouse.mapToItem(profileSidebarList.contentItem, mouse.x, mouse.y)
                                                const rowStride = profDelegate.height + profileSidebarList.spacing
                                                const targetIdx = Math.max(
                                                    0,
                                                    Math.min(
                                                        root.sidebarProfileModel.count - 1,
                                                        Math.floor(posInContent.y / rowStride)
                                                    )
                                                )
                                                profileSidebarList.dropTargetIndex = targetIdx

                                                // Auto-scroll when dragging near the top or bottom of the viewport
                                                const posInView = profMouse.mapToItem(profileSidebarList, mouse.x, mouse.y)
                                                if (posInView.y < 24 && profileSidebarList.contentY > 0) {
                                                    profileSidebarList.contentY = Math.max(0, profileSidebarList.contentY - 10)
                                                } else if (posInView.y > profileSidebarList.height - 24) {
                                                    const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                                                    profileSidebarList.contentY = Math.min(maxScroll, profileSidebarList.contentY + 10)
                                                }
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
                                                const savedY = profileSidebarList.contentY
                                                root.selectingProfileFromClick = true
                                                deviceController.selectProfile(profDelegate.profNumber)
                                                root.refreshSelectedSlotFromController()
                                                profileSidebarList.contentY = savedY
                                                root.selectingProfileFromClick = false
                                            }
                                        }

                                        onCanceled: {
                                            dragging = false
                                            profileSidebarList.draggedProfileIndex = -1
                                            profileSidebarList.dropTargetIndex = -1
                                            profileSidebarList.interactive = true
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
                                            font.pixelSize: 14
                                            color: profMouse.containsMouse || profDelegate.isCurrent
                                                ? Kirigami.Theme.textColor
                                                : Kirigami.Theme.disabledTextColor
                                        }

                                        // Profile Background RGB Swatch
                                        Rectangle {
                                            width: 5
                                            height: 24
                                            radius: 2.5
                                            color: profDelegate.profColor
                                        }

                                        QQC2.Label {
                                            text: profDelegate.profNumber + ". " + profDelegate.profName
                                            font.bold: profDelegate.isCurrent
                                            font.pixelSize: 15
                                            color: Kirigami.Theme.textColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }

                                        // Quick Up/Down reorder buttons on hover
                                        RowLayout {
                                            visible: profileSidebarList.canReorder && profMouse.containsMouse && !profMouse.dragging
                                            spacing: 1

                                            QQC2.ToolButton {
                                                text: "▲"
                                                enabled: profDelegate.index > 0
                                                implicitWidth: 20
                                                implicitHeight: 22
                                                font.pixelSize: 9
                                                onClicked: {
                                                    deviceController.reorderProfile(profDelegate.index, profDelegate.index - 1)
                                                    root.refreshSelectedSlotFromController()
                                                }
                                                QQC2.ToolTip.visible: hovered
                                                QQC2.ToolTip.text: qsTr("Move Profile Up")
                                            }

                                            QQC2.ToolButton {
                                                text: "▼"
                                                enabled: profDelegate.index < root.sidebarProfileModel.count - 1
                                                implicitWidth: 20
                                                implicitHeight: 22
                                                font.pixelSize: 9
                                                onClicked: {
                                                    deviceController.reorderProfile(profDelegate.index, profDelegate.index + 1)
                                                    root.refreshSelectedSlotFromController()
                                                }
                                                QQC2.ToolTip.visible: hovered
                                                QQC2.ToolTip.text: qsTr("Move Profile Down")
                                            }
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
                                                Layout.preferredWidth: 22
                                                Layout.preferredHeight: 22
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
                                                font.pixelSize: 10
                                                font.bold: true
                                                color: Kirigami.Theme.highlightColor
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
                                            implicitHeight: 20

                                            QQC2.Label {
                                                id: badgeText
                                                anchors.centerIn: parent
                                                text: profDelegate.slotCount + "k"
                                                font.pixelSize: 10
                                                color: Kirigami.Theme.disabledTextColor
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Inline New Profile Creator (when toggled)
                        RowLayout {
                            Layout.fillWidth: true
                            visible: root.showNewProfileInput
                            spacing: 4

                            QQC2.TextField {
                                id: newProfileNameField
                                Layout.fillWidth: true
                                maximumLength: 16
                                placeholderText: qsTr("Profile name...")
                                font.pixelSize: 11
                                onAccepted: createProfBtn.clicked()
                            }

                            QQC2.Button {
                                id: createProfBtn
                                text: qsTr("Add")
                                implicitHeight: 26
                                font.pixelSize: 11
                                highlighted: true
                                enabled: newProfileNameField.text.trim().length > 0
                                onClicked: {
                                    if (deviceController.createNewProfile(newProfileNameField.text.trim())) {
                                        newProfileNameField.text = ""
                                        root.showNewProfileInput = false
                                        root.refreshSelectedSlotFromController()
                                    }
                                }
                            }
                        }

                        // + New Profile / Import Profile Buttons (Right-click any profile to Export)
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "list-add"
                                text: qsTr("New")
                                font.pixelSize: 11
                                onClicked: {
                                    root.showNewProfileInput = !root.showNewProfileInput
                                    if (root.showNewProfileInput) {
                                        newProfileNameField.forceActiveFocus()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Create a new profile (Right-click a profile for Export & actions)")
                            }

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "document-import"
                                text: qsTr("Import")
                                font.pixelSize: 11
                                onClicked: importProfileFileDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Import a profile from .duckyprofile.json or config.txt")
                            }
                        }

                        // Divider
                        Rectangle {
                            Layout.fillWidth: true
                            height: 1
                            color: profilesSidebar.frameBorder
                        }

                        // KWIN AUTO-SWITCH Section
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true

                                QQC2.Label {
                                    text: qsTr("KWIN AUTO-SWITCH")
                                    font.bold: true
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                }

                                QQC2.Switch {
                                    checked: deviceController.autoSwitchEnabled
                                    implicitHeight: 22
                                    onToggled: {
                                        deviceController.setAutoSwitchConfig(
                                            checked,
                                            deviceController.defaultFallbackProfile
                                        )
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Toggle KWin Wayland automatic profile switching")
                                }
                            }

                            // Active Wayland Window Status Box
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: activeWinCol.implicitHeight + 12
                                radius: 6
                                color: deviceController.windowPickerArmed
                                    ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : Kirigami.Theme.alternateBackgroundColor
                                border.color: deviceController.windowPickerArmed
                                    ? Kirigami.Theme.highlightColor
                                    : profilesSidebar.frameBorder
                                border.width: 1

                                ColumnLayout {
                                    id: activeWinCol
                                    anchors.fill: parent
                                    anchors.margins: 6
                                    spacing: 1

                                    QQC2.Label {
                                        text: deviceController.windowPickerArmed
                                            ? qsTr("🎯 Click target window...")
                                            : qsTr("Active: %1").arg(deviceController.activeWindowClass)
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: 11
                                        color: deviceController.windowPickerArmed
                                            ? Kirigami.Theme.highlightColor
                                            : Kirigami.Theme.textColor
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }

                                    QQC2.Label {
                                        visible: deviceController.activeWindowTitle.length > 0
                                        text: deviceController.activeWindowTitle
                                        font.pixelSize: 9
                                        color: Kirigami.Theme.disabledTextColor
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }
                                }
                            }

                            // Pick Window & Rules Drawer Buttons
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.Button {
                                    Layout.fillWidth: true
                                    icon.name: "crosshairs"
                                    text: deviceController.windowPickerArmed
                                        ? qsTr("Armed...")
                                        : qsTr("🎯 Pick Window")
                                    highlighted: deviceController.windowPickerArmed
                                    font.pixelSize: 11
                                    onClicked: deviceController.armWindowPicker(true)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Click this, then click any open Plasma 6 window to bind it to Profile #%1 (%2)")
                                        .arg(deviceController.activeProfile)
                                        .arg(deviceController.activeProfileName)
                                }

                                QQC2.Button {
                                    icon.name: "configure"
                                    text: qsTr("⚙ Rules (%1)").arg(root.totalRuleCount)
                                    font.pixelSize: 11
                                    onClicked: autoSwitchDrawer.open()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Open KWin Wayland Auto-Switch Rules Drawer")
                                }
                            }
                        }
                    }
                }

                // Column 2: Interactive Hardware Canvas
                HardwareCanvas {
                    id: hardwareCanvas
                    QQC2.SplitView.preferredWidth: 460
                    QQC2.SplitView.minimumWidth: 340
                    QQC2.SplitView.fillHeight: true
                    controller: deviceController

                    onSlotSelected: (slotNumber, slotTitle, slotColor) => {
                        root.loadSlotFromController(slotNumber, slotTitle, slotColor)
                    }
                }

                // Column 3: Right Inspector Card matching system View colorSet
                Rectangle {
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    QQC2.SplitView.fillWidth: true
                    QQC2.SplitView.fillHeight: true
                    QQC2.SplitView.minimumWidth: 380
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // Inspector Header
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Rectangle {
                                width: 28
                                height: 28
                                radius: 6
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: root.selectedSlotColor
                                border.width: 2

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: root.selectedSlotNumber
                                    font.bold: true
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.textColor
                                }
                            }

                            ColumnLayout {
                                spacing: 1
                                Layout.fillWidth: true
                                Layout.minimumWidth: 60

                                Kirigami.Heading {
                                    id: inspectorTitle
                                    level: 3
                                    text: qsTr("%1 — duckyScript 3").arg(root.selectedSlotTitle)
                                    color: Kirigami.Theme.textColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                }

                                QQC2.Label {
                                    text: deviceController.sdMountPath.length > 0
                                        ? qsTr("Profile #%1 (%2) • SD: %3")
                                            .arg(deviceController.activeProfile)
                                            .arg(deviceController.activeProfileName)
                                            .arg(deviceController.sdMountPath)
                                        : qsTr("RGB LED: %1 • Target Slot: %2 (In-Memory / No SD Loaded)")
                                            .arg(root.selectedSlotColor)
                                            .arg(root.selectedSlotNumber)
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideMiddle
                                }
                            }

                            QQC2.Button {
                                text: qsTr("✨ AI")
                                implicitHeight: 28
                                font.pixelSize: 11
                                onClicked: aiModal.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Open AI Macro Architect")
                            }

                            QQC2.Button {
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("💾 Save Key %1").arg(root.selectedSlotNumber)
                                    : qsTr("💾 Save Key %1").arg(root.selectedSlotNumber)
                                implicitHeight: 28
                                font.pixelSize: 11
                                font.bold: true
                                highlighted: true
                                onClicked: root.saveCurrentSlotToController()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Compile .dsb bytecode and save key%1.txt / key%1.dsb & config.txt").arg(root.selectedSlotNumber)
                            }
                        }

                        // Key Label & Flags Row (config.txt zN, xN, ab N, dr N)
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("L1:")
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label1Input
                                Layout.preferredWidth: 72
                                Layout.minimumWidth: 52
                                maximumLength: 7
                                placeholderText: qsTr("Line 1")
                                selectByMouse: true
                                font.pixelSize: 11
                            }

                            QQC2.Label {
                                text: qsTr("L2:")
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label2Input
                                Layout.preferredWidth: 72
                                Layout.minimumWidth: 52
                                maximumLength: 7
                                placeholderText: qsTr("Line 2")
                                selectByMouse: true
                                font.pixelSize: 11
                            }

                            QQC2.CheckBox {
                                id: allowAbortCheck
                                text: qsTr("Allow Abort")
                                font.pixelSize: 11
                                onToggled: root.updateCompilerStatus()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab %1 / $_ALLOW_ABORT = 1)").arg(root.selectedSlotNumber)
                            }

                            QQC2.CheckBox {
                                id: disableRepeatCheck
                                text: qsTr("No Repeat")
                                font.pixelSize: 11
                                onToggled: root.updateCompilerStatus()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Disable auto-repeat while key is held down (dr %1 / $_DONT_REPEAT = 1)").arg(root.selectedSlotNumber)
                            }

                            Item { Layout.fillWidth: true }
                        }

                        // Script Mode Tabs (keyN.txt vs keyN-release.txt) + Snippet Quick Insert Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.ButtonGroup { id: scriptPhaseGroup }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: !root.editingReleaseScript
                                text: qsTr("▾ On Press (key%1)").arg(root.selectedSlotNumber)
                                font.pixelSize: 11
                                font.bold: !root.editingReleaseScript
                                implicitHeight: 26
                                onClicked: {
                                    if (root.editingReleaseScript) {
                                        root.releaseScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = false
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.pressScriptBuffer
                                        root.loadingSlotData = false
                                        root.updateCompilerStatus()
                                    }
                                }
                            }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: root.editingReleaseScript
                                text: root.releaseScriptBuffer.trim().length > 0
                                    ? qsTr("▴ On Release (key%1-rel ●)").arg(root.selectedSlotNumber)
                                    : qsTr("▴ On Release (key%1-rel)").arg(root.selectedSlotNumber)
                                font.pixelSize: 11
                                font.bold: root.editingReleaseScript
                                implicitHeight: 26
                                onClicked: {
                                    if (!root.editingReleaseScript) {
                                        root.pressScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = true
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.releaseScriptBuffer
                                        root.loadingSlotData = false
                                        root.updateCompilerStatus()
                                    }
                                }
                            }

                            Item { Layout.fillWidth: true }

                            QQC2.Button {
                                text: "+DELAY"
                                implicitHeight: 24
                                font.pixelSize: 10
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "DELAY 100\n")
                            }

                            QQC2.Button {
                                text: "+STRING"
                                implicitHeight: 24
                                font.pixelSize: 10
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "STRING Hello World\n")
                            }

                            QQC2.Button {
                                text: "+OLED"
                                implicitHeight: 24
                                font.pixelSize: 10
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "OLED_CLEAR\nOLED_CPRINT Active\nOLED_UPDATE\n")
                            }
                        }

                        // Code Editor Area with Synchronized Line Number Gutter
                        Rectangle {
                            id: editorContainer
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: root.compileOk
                                ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                : Kirigami.Theme.negativeTextColor
                            border.width: 1
                            clip: true

                            QQC2.ScrollView {
                                id: scriptScrollView
                                anchors.fill: parent
                                anchors.margins: 2
                                clip: true

                                QQC2.TextArea {
                                    id: scriptEditor
                                    font.family: "monospace"
                                    font.pixelSize: 13
                                    leftPadding: lineGutter.width + 10
                                    topPadding: 6
                                    bottomPadding: 6
                                    rightPadding: 8
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
                                        }
                                    }

                                    // Synchronized Line-Number Gutter
                                    Rectangle {
                                        id: lineGutter
                                        x: scriptScrollView.contentItem ? scriptScrollView.contentItem.contentX : 0
                                        y: 0
                                        z: 2
                                        width: 38
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
                                                    font.pixelSize: scriptEditor.font.pixelSize
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
                        }

                        // Inspector Footer Bar with Live DSVM v2 .dsb Bytecode Status
                        RowLayout {
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
                                implicitHeight: 26

                                QQC2.Label {
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 8
                                    verticalAlignment: Text.AlignVCenter
                                    elide: Text.ElideRight
                                    font.family: "monospace"
                                    font.pixelSize: 11
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
                            }

                            QQC2.Button {
                                text: qsTr("↺ Revert")
                                implicitHeight: 28
                                font.pixelSize: 11
                                onClicked: root.loadSlotFromController(
                                    root.selectedSlotNumber,
                                    root.selectedSlotTitle,
                                    root.selectedSlotColor
                                )
                            }

                            QQC2.Button {
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("💾 Save Key %1 to SD").arg(root.selectedSlotNumber)
                                    : qsTr("💾 Save Key %1").arg(root.selectedSlotNumber)
                                implicitHeight: 28
                                font.pixelSize: 11
                                font.bold: true
                                highlighted: true
                                onClicked: root.saveCurrentSlotToController()
                            }
                        }
                    }
                }
            }
        }
    }

    // Bottom Status Bar (Connection Status, Firmware Version, Serial Number, SD Card Path & Live Status)
    footer: Rectangle {
        id: bottomStatusBar
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        width: parent.width
        implicitHeight: 34
        color: Kirigami.Theme.alternateBackgroundColor

        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Kirigami.Units.largeSpacing
            anchors.rightMargin: Kirigami.Units.largeSpacing
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
                implicitWidth: connRow.implicitWidth + 14
                implicitHeight: 22

                RowLayout {
                    id: connRow
                    anchors.centerIn: parent
                    spacing: 6

                    Rectangle {
                        width: 8
                        height: 8
                        radius: 4
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
                        font.pixelSize: 11
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            // Firmware Version & Serial Number Badge
            Rectangle {
                radius: 4
                color: Kirigami.Theme.backgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                implicitWidth: fwLabel.implicitWidth + 14
                implicitHeight: 22

                QQC2.Label {
                    id: fwLabel
                    anchors.centerIn: parent
                    text: (deviceController.serialNumber && deviceController.serialNumber !== "-")
                        ? qsTr("FW: %1  •  SN: %2")
                            .arg(deviceController.firmwareVersion)
                            .arg(deviceController.serialNumber)
                        : qsTr("Firmware: %1").arg(deviceController.firmwareVersion)
                    font.family: "monospace"
                    font.pixelSize: 11
                    color: Kirigami.Theme.textColor
                }
            }

            // Mounted SD Card Badge (when loaded)
            Rectangle {
                visible: deviceController.sdMountPath.length > 0
                radius: 4
                color: Kirigami.Theme.backgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                implicitWidth: Math.min(sdPathLabel.implicitWidth + 14, 260)
                implicitHeight: 22

                QQC2.Label {
                    id: sdPathLabel
                    anchors.fill: parent
                    anchors.leftMargin: 7
                    anchors.rightMargin: 7
                    verticalAlignment: Text.AlignVCenter
                    text: qsTr("SD: %1").arg(deviceController.sdMountPath)
                    font.family: "monospace"
                    font.pixelSize: 11
                    elide: Text.ElideMiddle
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            Rectangle {
                width: 1
                height: 16
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            }

            // Live Status Message
            QQC2.Label {
                text: deviceController.statusMessage
                font.pixelSize: 11
                color: Kirigami.Theme.disabledTextColor
                elide: Text.ElideRight
                Layout.fillWidth: true
            }
        }
    }
}
