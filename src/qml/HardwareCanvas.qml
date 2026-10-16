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

    readonly property real canvasScale: (canvasRoot.controller && canvasRoot.controller.canvasTextScale > 0
        ? canvasRoot.controller.canvasTextScale
        : 100) / 100.0

    readonly property int requiredWidthForFullHeight: Math.max(
        500,
        Math.ceil(topToolbarRow.implicitWidth + 12),
        Math.ceil(((Math.max(320, canvasRoot.height - topToolbarRow.implicitHeight - mainCol.spacing) - 20) / 354.0) * 430.0 + 20)
    )

    property var profileLabels: []
    property var profileNumbers: []
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
        if (idx >= 0 && profileCombo.currentIndex !== idx) {
            profileCombo.currentIndex = idx
        }
    }

    function syncProfilesFromController() {
        if (!controller || !controller.profilesJson) return
        try {
            const arr = JSON.parse(controller.profilesJson)
            if (!Array.isArray(arr)) return
            const labels = []
            const nums = []
            let activeIdx = 0
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                nums.push(p.number)
                labels.push("P" + p.number + ": " + p.name)
                if (p.number === controller.activeProfile) {
                    activeIdx = i
                }
            }
            canvasRoot.profileNumbers = nums
            canvasRoot.profileLabels = labels
            profileCombo.currentIndex = activeIdx
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
                    keyModel.setProperty(s - 1, "l1", item.l1 || "")
                    keyModel.setProperty(s - 1, "l2", item.l2 || "")
                    keyModel.setProperty(s - 1, "r", item.r !== undefined ? item.r : 0)
                    keyModel.setProperty(s - 1, "g", item.g !== undefined ? item.g : 170)
                    keyModel.setProperty(s - 1, "b", item.b !== undefined ? item.b : 255)
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
            canvasRoot.selectedSlot = targetSlotNum
            canvasRoot.slotContentMutated(targetSlotNum)
            oledSimulator.requestPaint()
        }
    }

    function clearSlotAction(slotNum) {
        if (!controller) return
        canvasRoot.beforeSlotMutationRequested()
        if (controller.clearSlot(slotNum)) {
            canvasRoot.selectedSlot = slotNum
            canvasRoot.slotContentMutated(slotNum)
            oledSimulator.requestPaint()
        }
    }

    function moveOrSwapSlotAction(fromSlot, toSlot, shiftBetween) {
        if (!controller || fromSlot === toSlot) return
        canvasRoot.beforeSlotMutationRequested()
        if (controller.moveOrSwapSlot(fromSlot, toSlot, shiftBetween)) {
            canvasRoot.selectedSlot = toSlot
            canvasRoot.slotContentMutated(toSlot)
            oledSimulator.requestPaint()
        }
    }

    function openSlotContextMenu(slotNum) {
        canvasRoot.selectedSlot = slotNum
        canvasRoot.contextMenuSlot = slotNum
        canvasRoot.slotSelected(
            slotNum,
            canvasRoot.slotTitleFor(slotNum),
            canvasRoot.slotColorFor(slotNum)
        )
        oledSimulator.requestPaint()
        slotContextMenu.popup()
    }

    QQC2.Menu {
        id: slotContextMenu

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
                canvasRoot.selectedSlot = canvasRoot.contextMenuSlot
                canvasRoot.keyColorPickerRequested(
                    canvasRoot.contextMenuSlot,
                    canvasRoot.slotColorFor(canvasRoot.contextMenuSlot).toString()
                )
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
        spacing: Kirigami.Units.smallSpacing

        RowLayout {
            id: topToolbarRow
            Layout.fillWidth: true
            spacing: 4

            QQC2.ComboBox {
                id: profileCombo
                Layout.preferredWidth: 170
                Layout.fillWidth: true
                Layout.maximumWidth: 220
                model: canvasRoot.profileLabels
                onActivated: (index) => {
                    if (index >= 0 && index < canvasRoot.profileNumbers.length && canvasRoot.controller) {
                        const profNum = canvasRoot.profileNumbers[index]
                        canvasRoot.controller.selectProfile(profNum)
                        if (canvasRoot.selectedSlot >= 1 && canvasRoot.selectedSlot <= 58) {
                            canvasRoot.slotSelected(
                                canvasRoot.selectedSlot,
                                canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                                canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                            )
                        }
                    }
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: canvasRoot.controller && canvasRoot.controller.sdMountPath.length > 0
                    ? qsTr("SD Path: %1 (%2 profiles)").arg(canvasRoot.controller.sdMountPath).arg(canvasRoot.profileNumbers.length)
                    : (canvasRoot.controller && canvasRoot.controller.connected
                        ? qsTr("%1 (%2 profiles loaded)").arg(canvasRoot.controller.modelName).arg(canvasRoot.profileNumbers.length)
                        : qsTr("Active Profile (Connect duckyPad or Load SD Card)"))
            }

            QQC2.ToolButton {
                icon.name: "edit-rename"
                display: QQC2.AbstractButton.IconOnly
                enabled: canvasRoot.controller !== null
                onClicked: {
                    if (canvasRoot.controller) {
                        canvasRoot.renameProfileRequested(
                            canvasRoot.controller.activeProfile,
                            canvasRoot.controller.activeProfileName
                        )
                    }
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: canvasRoot.controller
                    ? qsTr("Rename Profile #%1 (%2)...").arg(canvasRoot.controller.activeProfile).arg(canvasRoot.controller.activeProfileName)
                    : qsTr("Rename Profile...")
            }

            QQC2.ButtonGroup { id: modeGroup }

            QQC2.ToolButton {
                QQC2.ButtonGroup.group: modeGroup
                checkable: true
                checked: !canvasRoot.showExpansion
                icon.name: "input-keyboard"
                text: qsTr("Onboard")
                display: QQC2.AbstractButton.TextBesideIcon
                onClicked: canvasRoot.showExpansion = false
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Onboard (20 Keys + 2 Encoders)")
            }

            QQC2.ToolButton {
                QQC2.ButtonGroup.group: modeGroup
                checkable: true
                checked: canvasRoot.showExpansion
                icon.name: "network-wired"
                text: qsTr("Expansion")
                display: QQC2.AbstractButton.TextBesideIcon
                onClicked: canvasRoot.showExpansion = true
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Daisy-Chained Expansion Channels (27–58)")
            }

            Item { Layout.fillWidth: true }

            RowLayout {
                spacing: 4

                Rectangle {
                    width: 14
                    height: 14
                    radius: 7
                    color: canvasRoot.controller && canvasRoot.controller.activeProfileBgColor.length > 0
                        ? canvasRoot.controller.activeProfileBgColor
                        : "#00aaff"
                    border.width: 1
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.55)

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            if (canvasRoot.controller) {
                                canvasRoot.profileDefaultColorPickerRequested(
                                    canvasRoot.controller.activeProfile,
                                    canvasRoot.controller.activeProfileName,
                                    canvasRoot.controller.activeProfileBgColor
                                )
                            }
                        }
                    }
                }

                QQC2.ToolButton {
                    id: profileBgBtn
                    icon.name: "color-picker"
                    text: qsTr("Default Color")
                    display: QQC2.AbstractButton.TextBesideIcon
                    enabled: canvasRoot.controller !== null
                    onClicked: {
                        if (canvasRoot.controller) {
                            canvasRoot.profileDefaultColorPickerRequested(
                                canvasRoot.controller.activeProfile,
                                canvasRoot.controller.activeProfileName,
                                canvasRoot.controller.activeProfileBgColor
                            )
                        }
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: canvasRoot.controller
                        ? qsTr("Change Default Button Color (BG_COLOR) for Profile #%1 (%2)").arg(canvasRoot.controller.activeProfile).arg(canvasRoot.controller.activeProfileName)
                        : qsTr("Change Profile Default Button Color (BG_COLOR)")
                }
            }

            QQC2.ToolButton {
                checkable: true
                checked: canvasRoot.oledScriptMode
                icon.name: "video-television"
                display: QQC2.AbstractButton.TextBesideIcon
                text: canvasRoot.oledScriptMode ? qsTr("OLED VM") : qsTr("OLED Grid")
                onToggled: {
                    canvasRoot.oledScriptMode = checked
                    oledSimulator.requestPaint()
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: canvasRoot.oledScriptMode ? qsTr("Switch to Profile Grid HUD") : qsTr("Switch to duckStack VM HUD")
            }
        }

        Rectangle {
            id: pcbChassis
            Kirigami.Theme.colorSet: Kirigami.Theme.View
            Kirigami.Theme.inherit: false
            readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            readonly property real uiScale: Math.max(
                0.85,
                Math.min(
                    (pcbChassis.width - 20) / 430.0,
                    (pcbChassis.height - 20) / 354.0
                )
            )

            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 12
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
                            readonly property color ledColor: Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 1.0)
                            property real dragOffsetX: 0
                            property real dragOffsetY: 0

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
                                width: parent.width
                                height: parent.height
                                x: keycapItem.isDraggingThis ? keycapItem.dragOffsetX : 0
                                y: keycapItem.isDraggingThis ? keycapItem.dragOffsetY : 0
                                scale: keycapItem.isDraggingThis ? 1.06 : 1.0
                                opacity: keycapItem.isDraggingThis ? 0.92 : 1.0
                                radius: 8
                                color: Kirigami.Theme.backgroundColor
                                border.width: keycapItem.isDropTarget ? 2.5 : (keycapItem.isSelected ? 2 : 1)
                                border.color: keycapItem.isDropTarget
                                    ? Kirigami.Theme.positiveTextColor
                                    : (keycapItem.isSelected
                                        ? Kirigami.Theme.highlightColor
                                        : pcbChassis.frameBorderColor)

                                Rectangle {
                                    anchors.top: parent.top
                                    anchors.topMargin: 3
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    width: 28
                                    height: 4
                                    radius: 2
                                    color: keycapItem.ledColor
                                }

                                Rectangle {
                                    anchors.fill: parent
                                    anchors.margins: 5
                                    anchors.topMargin: 9
                                    radius: 6
                                    color: keycapItem.isDropTarget
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.positiveTextColor.r, Kirigami.Theme.positiveTextColor.g, Kirigami.Theme.positiveTextColor.b, 0.28))
                                        : (keyMouse.pressed
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(keycapItem.ledColor.r, keycapItem.ledColor.g, keycapItem.ledColor.b, 0.35))
                                            : (keycapItem.isSelected
                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.18))
                                                : Kirigami.Theme.alternateBackgroundColor))
                                    border.width: 1
                                    border.color: keycapItem.isDropTarget
                                        ? Kirigami.Theme.positiveTextColor
                                        : (keycapItem.isSelected
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 0.5))

                                    QQC2.Label {
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.margins: 3
                                        text: keycapItem.slot
                                        font.pixelSize: Math.round(9 * canvasRoot.canvasScale)
                                        font.bold: true
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    Row {
                                        anchors.top: parent.top
                                        anchors.right: parent.right
                                        anchors.margins: 3
                                        spacing: 2

                                        Rectangle {
                                            visible: keycapItem.hasAppLaunch
                                            radius: 3
                                            color: Qt.alpha(Kirigami.Theme.highlightColor, 0.22)
                                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.65)
                                            border.width: 1
                                            width: appBadgeLbl.implicitWidth + 5
                                            height: 12

                                            QQC2.Label {
                                                id: appBadgeLbl
                                                anchors.centerIn: parent
                                                text: "APP"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: Kirigami.Theme.highlightColor
                                            }
                                        }
                                        Rectangle {
                                            visible: keycapItem.hasWebApi
                                            radius: 3
                                            color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.22)
                                            border.color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.65)
                                            border.width: 1
                                            width: apiBadgeLbl.implicitWidth + 5
                                            height: 12

                                            QQC2.Label {
                                                id: apiBadgeLbl
                                                anchors.centerIn: parent
                                                text: "API"
                                                font.pixelSize: 7
                                                font.bold: true
                                                color: Kirigami.Theme.positiveTextColor
                                            }
                                        }
                                        QQC2.Label {
                                            visible: keycapItem.hasRel
                                            text: "↑"
                                            font.pixelSize: 10
                                            font.bold: true
                                            color: Kirigami.Theme.highlightColor
                                        }
                                        QQC2.Label {
                                            visible: keycapItem.hasLoop
                                            text: "∞"
                                            font.pixelSize: 11
                                            font.bold: true
                                            color: Kirigami.Theme.neutralTextColor
                                        }
                                    }

                                    Column {
                                        anchors.centerIn: parent
                                        anchors.verticalCenterOffset: 3
                                        width: parent.width - 4
                                        spacing: 1

                                        QQC2.Label {
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            text: keycapItem.l1
                                            font.pixelSize: Math.round(12 * canvasRoot.canvasScale)
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 7
                                            font.bold: true
                                            color: Kirigami.Theme.textColor
                                        }
                                        QQC2.Label {
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            text: keycapItem.l2
                                            font.pixelSize: Math.round(10 * canvasRoot.canvasScale)
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 6
                                            font.weight: Font.Medium
                                            color: Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    Rectangle {
                                        visible: keycapItem.isDropTarget
                                        anchors.bottom: parent.bottom
                                        anchors.bottomMargin: 2
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        radius: 3
                                        width: dropModeLbl.implicitWidth + 8
                                        height: 13
                                        color: Kirigami.Theme.positiveTextColor

                                        QQC2.Label {
                                            id: dropModeLbl
                                            anchors.centerIn: parent
                                            text: canvasRoot.dragShiftMode ? qsTr("→ Shift") : qsTr("⇄ Swap")
                                            font.pixelSize: 8
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

                                QQC2.ToolTip.visible: keyMouse.containsMouse && !didDrag && !slotContextMenu.visible
                                QQC2.ToolTip.delay: 650
                                QQC2.ToolTip.text: qsTr("Key %1 • Click to edit (click again to close editor) • Drag to swap (Shift+Drag to shift) • Right-click for Copy/Paste").arg(keycapItem.slot)
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
                            scale: 1.0
                            smooth: false

                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.fillStyle = "#000000"
                                ctx.fillRect(0, 0, 128, 128)

                                if (!canvasRoot.oledScriptMode) {
                                    ctx.fillStyle = "#38bdf8"
                                    ctx.fillRect(0, 0, 128, 13)
                                    ctx.fillStyle = "#020617"
                                    ctx.font = "bold 9px sans-serif"
                                    const profNum = canvasRoot.controller ? canvasRoot.controller.activeProfile : 1
                                    const profName = (canvasRoot.controller && canvasRoot.controller.activeProfileName)
                                        ? canvasRoot.controller.activeProfileName
                                        : "Default"
                                    ctx.fillText("P" + profNum + ": " + profName.slice(0, 16), 3, 10)

                                    const cellW = 32
                                    const cellH = 23
                                    ctx.font = "bold 8px monospace"

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
                                            ctx.fillStyle = "#020617"
                                        } else {
                                            ctx.strokeStyle = "#334155"
                                            ctx.strokeRect(x, y, cellW, cellH)
                                            ctx.fillStyle = "#f8fafc"
                                        }

                                        ctx.fillText(item.l1.slice(0, 5), x + 2, y + 9)
                                        ctx.fillText(item.l2.slice(0, 5), x + 2, y + 19)
                                    }
                                } else {
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
                            contentItem: Text {
                                text: "− Profile"
                                font.pixelSize: Math.min(10, Math.round(9 * canvasRoot.canvasScale))
                                fontSizeMode: Text.HorizontalFit
                                minimumPixelSize: 7
                                font.bold: true
                                color: Kirigami.Theme.textColor
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideNone
                            }
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && profileCombo.currentIndex > 0) {
                                        profileCombo.currentIndex -= 1
                                        const profNum = canvasRoot.profileNumbers[profileCombo.currentIndex]
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
                            contentItem: Text {
                                text: "+ Profile"
                                font.pixelSize: Math.min(10, Math.round(9 * canvasRoot.canvasScale))
                                fontSizeMode: Text.HorizontalFit
                                minimumPixelSize: 7
                                font.bold: true
                                color: Kirigami.Theme.textColor
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideNone
                            }
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && profileCombo.currentIndex + 1 < canvasRoot.profileNumbers.length) {
                                        profileCombo.currentIndex += 1
                                        const profNum = canvasRoot.profileNumbers[profileCombo.currentIndex]
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
                anchors.margins: 14
                spacing: 8
                visible: canvasRoot.showExpansion

                QQC2.Label {
                    text: qsTr("Expansion Modules (4 Modules × 8 Channels) • Drag to swap, Right-click for Copy/Paste")
                    font.bold: true
                    font.pixelSize: Math.round(12 * canvasRoot.canvasScale)
                    color: Kirigami.Theme.textColor
                }

                GridLayout {
                    id: expansionGrid
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    columns: 4
                    rowSpacing: 6
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
                            radius: 6
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
                                width: parent.width - 6
                                spacing: 1

                                QQC2.Label {
                                    width: parent.width
                                    horizontalAlignment: Text.AlignHCenter
                                    text: expLabel.length > 0
                                        ? ("M" + moduleNum + "·C" + chanNum + ": " + expLabel)
                                        : ("M" + moduleNum + "·C" + chanNum)
                                    font.bold: true
                                    font.pixelSize: Math.round(10 * canvasRoot.canvasScale)
                                    fontSizeMode: Text.HorizontalFit
                                    minimumPixelSize: 7
                                    color: Kirigami.Theme.textColor
                                }
                                QQC2.Label {
                                    width: parent.width
                                    horizontalAlignment: Text.AlignHCenter
                                    text: isDropTarget
                                        ? (canvasRoot.dragShiftMode ? qsTr("→ Shift Here") : qsTr("⇄ Swap Here"))
                                        : ("Slot " + slotNum)
                                    font.pixelSize: Math.round(9 * canvasRoot.canvasScale)
                                    fontSizeMode: Text.HorizontalFit
                                    minimumPixelSize: 7
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
                font.pixelSize: Math.round(11 * encRoot.textScale)
                fontSizeMode: Text.HorizontalFit
                minimumPixelSize: 8
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
                            font.pixelSize: 16
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.ccwLabel
                            font.pixelSize: Math.round(9 * encRoot.textScale)
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 6
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
                        width: parent.width - 6
                        spacing: 0
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: "PUSH"
                            font.pixelSize: Math.round(8 * encRoot.textScale)
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 6
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.pushLabel
                            font.pixelSize: Math.round(9 * encRoot.textScale)
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 6
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
                            font.pixelSize: 16
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                            text: encRoot.cwLabel
                            font.pixelSize: Math.round(9 * encRoot.textScale)
                            fontSizeMode: Text.HorizontalFit
                            minimumPixelSize: 6
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
