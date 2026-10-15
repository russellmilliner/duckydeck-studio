import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
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

    property ListModel sidebarProfileModel: ListModel {}

    function syncSidebarProfilesFromController() {
        if (!deviceController || !deviceController.profilesJson) return
        try {
            const arr = JSON.parse(deviceController.profilesJson)
            if (!Array.isArray(arr)) return
            root.totalProfileCount = arr.length

            const filter = root.profileFilterText.trim().toLowerCase()
            sidebarProfileModel.clear()
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                const nameStr = p.name || "Default"
                const boundStr = p.boundClass || ""
                const numStr = String(p.number)
                if (filter.length > 0
                    && !nameStr.toLowerCase().includes(filter)
                    && !boundStr.toLowerCase().includes(filter)
                    && !numStr.includes(filter)) {
                    continue
                }
                const bg = Array.isArray(p.bgColor) && p.bgColor.length === 3 ? p.bgColor : [0, 170, 255]
                sidebarProfileModel.append({
                    profNumber: p.number,
                    profName: nameStr,
                    slotCount: p.slotCount || 0,
                    boundClass: boundStr,
                    bgR: bg[0],
                    bgG: bg[1],
                    bgB: bg[2]
                })
            }
        } catch (e) {}

        try {
            const rArr = JSON.parse(deviceController.autoSwitchRulesJson || "[]")
            if (Array.isArray(rArr)) {
                root.totalRuleCount = rArr.length
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
        root.syncSidebarProfilesFromController()
        hardwareCanvas.syncProfilesFromController()
        hardwareCanvas.syncSlotsFromController()
        root.loadSlotFromController(
            root.selectedSlotNumber,
            hardwareCanvas.slotTitleFor(root.selectedSlotNumber),
            hardwareCanvas.slotColorFor(root.selectedSlotNumber)
        )
    }

    Component.onCompleted: {
        deviceController.refreshDevice()
        root.refreshSelectedSlotFromController()
    }

    DeviceController {
        id: deviceController
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

    PgvInspectorDrawer {
        id: pgvDrawer
        controller: deviceController
    }

    AutoSwitchDrawer {
        id: autoSwitchDrawer
        controller: deviceController
    }

    AiArchitectModal {
        id: aiModal
        controller: deviceController
    }

    pageStack.initialPage: Kirigami.Page {
        title: deviceController.connected
            ? qsTr("%1 (Connected)").arg(deviceController.modelName)
            : qsTr("%1").arg(deviceController.statusMessage)

        background: Rectangle {
            color: Kirigami.Theme.backgroundColor
        }

        actions: [
            Kirigami.Action {
                icon.name: "view-refresh"
                text: qsTr("Scan USB HID")
                tooltip: qsTr("Detect and connect to duckyPad Pro hardware")
                onTriggered: {
                    deviceController.refreshDevice()
                    root.refreshSelectedSlotFromController()
                }
            },
            Kirigami.Action {
                icon.name: "media-flash-sd-mmc"
                text: qsTr("Scan SD Card")
                tooltip: qsTr("Auto-detect mounted duckyPad microSD card in /run/media or /media")
                onTriggered: {
                    const found = deviceController.scanMountedSdCard()
                    if (found.length > 0) {
                        root.refreshSelectedSlotFromController()
                    } else {
                        sdFolderDialog.open()
                    }
                }
            },
            Kirigami.Action {
                icon.name: "folder-open"
                text: qsTr("Open SD Folder...")
                tooltip: qsTr("Choose a mounted duckyPad SD card or local profile backup folder")
                onTriggered: sdFolderDialog.open()
            },
            Kirigami.Action {
                icon.name: "preferences-system-windows"
                text: qsTr("KWin Auto-Switch (%1)").arg(root.totalRuleCount)
                tooltip: qsTr("Configure Wayland window-focus rules to automatically switch profiles")
                onTriggered: autoSwitchDrawer.open()
            },
            Kirigami.Action {
                icon.name: "preferences-system-time"
                text: qsTr("Sync RTC")
                tooltip: qsTr("Synchronize duckyPad RTC hardware clock with system time")
                enabled: deviceController.connected
                onTriggered: deviceController.syncRtcClock()
            },
            Kirigami.Action {
                icon.name: "format-list-ordered"
                text: qsTr("Variables (_GV)")
                tooltip: qsTr("Inspect and modify persistent global variables (_GV0.._GV31)")
                onTriggered: {
                    pgvDrawer.readAllFromDevice()
                    pgvDrawer.open()
                }
            },
            Kirigami.Action {
                icon.name: "system-run"
                text: qsTr("AI Macro Architect")
                tooltip: qsTr("Generate duckyScript using local KDE shortcuts and AI")
                onTriggered: aiModal.open()
            }
        ]

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
                                spacing: 3

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
                                    height: 34
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
                                        cursorShape: profMouse.dragging
                                            ? Qt.ClosedHandCursor
                                            : (profileSidebarList.canReorder ? Qt.OpenHandCursor : Qt.PointingHandCursor)

                                        property real pressY: 0
                                        property bool dragging: false

                                        onPressed: (mouse) => {
                                            pressY = mouse.y
                                            dragging = false
                                        }

                                        onPositionChanged: (mouse) => {
                                            if (!pressed || !profileSidebarList.canReorder) return
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

                                        onReleased: {
                                            if (dragging) {
                                                const fromIdx = profileSidebarList.draggedProfileIndex
                                                const toIdx = profileSidebarList.dropTargetIndex
                                                dragging = false
                                                profileSidebarList.draggedProfileIndex = -1
                                                profileSidebarList.dropTargetIndex = -1
                                                profileSidebarList.interactive = true
                                                if (fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                    deviceController.reorderProfile(fromIdx, toIdx)
                                                    root.refreshSelectedSlotFromController()
                                                }
                                            } else {
                                                deviceController.selectProfile(profDelegate.profNumber)
                                                root.refreshSelectedSlotFromController()
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
                                        anchors.leftMargin: 5
                                        anchors.rightMargin: 5
                                        spacing: 5

                                        // Drag Handle Grip
                                        QQC2.Label {
                                            visible: profileSidebarList.canReorder
                                            text: "⠿"
                                            font.pixelSize: 12
                                            color: profMouse.containsMouse || profDelegate.isCurrent
                                                ? Kirigami.Theme.textColor
                                                : Kirigami.Theme.disabledTextColor
                                        }

                                        // Profile Background RGB Swatch
                                        Rectangle {
                                            width: 5
                                            height: 20
                                            radius: 2.5
                                            color: profDelegate.profColor
                                        }

                                        QQC2.Label {
                                            text: profDelegate.profNumber + ". " + profDelegate.profName
                                            font.bold: profDelegate.isCurrent
                                            font.pixelSize: 12
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
                                                implicitWidth: 18
                                                implicitHeight: 20
                                                font.pixelSize: 8
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
                                                implicitWidth: 18
                                                implicitHeight: 20
                                                font.pixelSize: 8
                                                onClicked: {
                                                    deviceController.reorderProfile(profDelegate.index, profDelegate.index + 1)
                                                    root.refreshSelectedSlotFromController()
                                                }
                                                QQC2.ToolTip.visible: hovered
                                                QQC2.ToolTip.text: qsTr("Move Profile Down")
                                            }
                                        }

                                        // Bound KWin Window Class Badge or Key Count
                                        Rectangle {
                                            radius: 4
                                            color: profDelegate.boundClass.length > 0
                                                ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16))
                                                : Kirigami.Theme.alternateBackgroundColor
                                            border.color: profDelegate.boundClass.length > 0
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                                                : profilesSidebar.frameBorder
                                            border.width: 1
                                            implicitWidth: badgeText.implicitWidth + 8
                                            implicitHeight: 18

                                            QQC2.Label {
                                                id: badgeText
                                                anchors.centerIn: parent
                                                text: profDelegate.boundClass.length > 0 ? profDelegate.boundClass : (profDelegate.slotCount + "k")
                                                font.pixelSize: 9
                                                font.family: profDelegate.boundClass.length > 0 ? "monospace" : "sans-serif"
                                                color: profDelegate.boundClass.length > 0
                                                    ? Kirigami.Theme.highlightColor
                                                    : Kirigami.Theme.disabledTextColor
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

                        // + New Profile / Import SD Buttons
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "list-add"
                                text: qsTr("New Profile")
                                font.pixelSize: 11
                                onClicked: {
                                    root.showNewProfileInput = !root.showNewProfileInput
                                    if (root.showNewProfileInput) {
                                        newProfileNameField.forceActiveFocus()
                                    }
                                }
                            }

                            QQC2.Button {
                                icon.name: "folder-open"
                                text: qsTr("SD")
                                font.pixelSize: 11
                                onClicked: sdFolderDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Open duckyPad SD Card or Backup Folder")
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
                                        : qsTr("Pick Window")
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
                                    text: qsTr("Rules (%1)").arg(root.totalRuleCount)
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
                    QQC2.SplitView.preferredWidth: 540
                    QQC2.SplitView.minimumWidth: 420
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
                    QQC2.SplitView.minimumWidth: 360
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.largeSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // Inspector Header
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 10

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
                                icon.name: "system-run"
                                text: qsTr("AI Prompt")
                                onClicked: aiModal.open()
                            }
                        }

                        // Key Label & Flags Row (config.txt zN, xN, abN, drN)
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Label 1:")
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label1Input
                                Layout.preferredWidth: 86
                                maximumLength: 7
                                placeholderText: qsTr("Line 1")
                                selectByMouse: true
                            }

                            QQC2.Label {
                                text: qsTr("Label 2:")
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label2Input
                                Layout.preferredWidth: 86
                                maximumLength: 7
                                placeholderText: qsTr("Line 2")
                                selectByMouse: true
                            }

                            QQC2.CheckBox {
                                id: allowAbortCheck
                                text: qsTr("Allow Abort")
                                font.pixelSize: 11
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab%1 1)").arg(root.selectedSlotNumber)
                            }

                            QQC2.CheckBox {
                                id: disableRepeatCheck
                                text: qsTr("No Repeat")
                                font.pixelSize: 11
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Disable auto-repeat while key is held down (dr%1 1)").arg(root.selectedSlotNumber)
                            }

                            Item { Layout.fillWidth: true }
                        }

                        // Script Mode Tabs (keyN.txt vs keyN-release.txt) + Snippet Quick Insert Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.ButtonGroup { id: scriptPhaseGroup }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: !root.editingReleaseScript
                                icon.name: "input-keyboard"
                                text: qsTr("On Press (key%1.txt)").arg(root.selectedSlotNumber)
                                display: QQC2.AbstractButton.TextBesideIcon
                                onClicked: {
                                    if (root.editingReleaseScript) {
                                        root.releaseScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = false
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.pressScriptBuffer
                                        root.loadingSlotData = false
                                    }
                                }
                            }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: root.editingReleaseScript
                                icon.name: "key-enter"
                                text: root.releaseScriptBuffer.trim().length > 0
                                    ? qsTr("On Release (key%1-release.txt ●)").arg(root.selectedSlotNumber)
                                    : qsTr("On Release (key%1-release.txt)").arg(root.selectedSlotNumber)
                                display: QQC2.AbstractButton.TextBesideIcon
                                onClicked: {
                                    if (!root.editingReleaseScript) {
                                        root.pressScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = true
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.releaseScriptBuffer
                                        root.loadingSlotData = false
                                    }
                                }
                            }

                            Item { Layout.fillWidth: true }

                            QQC2.Button {
                                text: "+ DELAY"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "DELAY 100\n")
                            }

                            QQC2.Button {
                                text: "+ STRING"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "STRING Hello World\n")
                            }

                            QQC2.Button {
                                text: "+ OLED"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "OLED_CLEAR\nOLED_CPRINT Active\nOLED_UPDATE\n")
                            }
                        }

                        // Code Editor Area
                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            border.width: 1

                            QQC2.ScrollView {
                                anchors.fill: parent
                                anchors.margins: 8
                                clip: true

                                QQC2.TextArea {
                                    id: scriptEditor
                                    font.family: "monospace"
                                    font.pixelSize: 13
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

                        // Inspector Footer Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: Kirigami.Units.smallSpacing

                            QQC2.Label {
                                text: root.editingReleaseScript
                                    ? qsTr("Editing key%1-release.txt").arg(root.selectedSlotNumber)
                                    : qsTr("Editing key%1.txt").arg(root.selectedSlotNumber)
                                font.pixelSize: 11
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                            }

                            QQC2.Button {
                                icon.name: "edit-undo"
                                text: qsTr("Revert")
                                onClicked: root.loadSlotFromController(
                                    root.selectedSlotNumber,
                                    root.selectedSlotTitle,
                                    root.selectedSlotColor
                                )
                            }

                            QQC2.Button {
                                icon.name: "document-save"
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("Save Key %1 to SD").arg(root.selectedSlotNumber)
                                    : qsTr("Save Key %1").arg(root.selectedSlotNumber)
                                highlighted: true
                                onClicked: root.saveCurrentSlotToController()
                            }
                        }
                    }
                }
            }
        }
    }
}
