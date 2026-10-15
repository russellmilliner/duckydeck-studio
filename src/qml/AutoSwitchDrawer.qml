import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Window {
    id: drawerRoot
    title: qsTr("KWin Wayland Auto-Switcher Rules — DuckyDeck Studio")
    width: 680
    height: 660
    minimumWidth: 560
    minimumHeight: 520
    visible: false
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

    property var controller: null
    property var profileChoices: []
    property var profileNumbers: []
    property var fallbackChoices: [qsTr("None (Stay on active profile)")]
    property var fallbackNumbers: [0]
    property int editingRuleIndex: -1

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor
    readonly property color subtleBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

    property ListModel rulesModel: ListModel {}

    function open() {
        syncFromController()
        drawerRoot.show()
        drawerRoot.raise()
        drawerRoot.requestActivate()
    }

    function syncFromController() {
        if (!controller) return
        const savedScrollY = rulesListView ? rulesListView.contentY : 0

        // 1. Sync profiles for dropdowns
        try {
            const pArr = JSON.parse(controller.profilesJson || "[]")
            const choices = []
            const nums = []
            const fbChoices = [qsTr("None (Stay on active profile)")]
            const fbNums = [0]

            for (let i = 0; i < pArr.length; ++i) {
                const p = pArr[i]
                const label = "P" + p.number + ": " + p.name
                choices.push(label)
                nums.push(p.number)
                fbChoices.push(label)
                fbNums.push(p.number)
            }
            drawerRoot.profileChoices = choices
            drawerRoot.profileNumbers = nums
            drawerRoot.fallbackChoices = fbChoices
            drawerRoot.fallbackNumbers = fbNums

            const fbIdx = fbNums.indexOf(controller.defaultFallbackProfile)
            fallbackCombo.currentIndex = fbIdx >= 0 ? fbIdx : 0

            const activeIdx = nums.indexOf(controller.activeProfile)
            if (targetProfileCombo.currentIndex < 0 && activeIdx >= 0) {
                targetProfileCombo.currentIndex = activeIdx
            }
        } catch (e) {}

        // 2. Sync rules list in-place to preserve scroll position
        try {
            const rArr = JSON.parse(controller.autoSwitchRulesJson || "[]")
            const nextItems = []
            for (let i = 0; i < rArr.length; ++i) {
                const r = rArr[i]
                nextItems.push({
                    ruleIndex: r.index !== undefined ? r.index : i,
                    ruleEnabled: !!r.enabled,
                    resourceClass: r.resourceClass || "",
                    titleContains: r.titleContains || "",
                    targetProfile: r.targetProfile || 1,
                    targetProfileName: r.targetProfileName || "Default",
                    ruleIcon: r.icon || "application-x-executable"
                })
            }

            for (let idx = 0; idx < nextItems.length; ++idx) {
                if (idx < rulesModel.count) {
                    rulesModel.set(idx, nextItems[idx])
                } else {
                    rulesModel.append(nextItems[idx])
                }
            }
            if (rulesModel.count > nextItems.length) {
                rulesModel.remove(nextItems.length, rulesModel.count - nextItems.length)
            }

            if (rulesListView && savedScrollY > 0) {
                const maxScroll = Math.max(0, rulesListView.contentHeight - rulesListView.height)
                rulesListView.contentY = Math.min(savedScrollY, maxScroll)
            }
        } catch (e) {}
    }

    onVisibleChanged: {
        if (visible) {
            syncFromController()
        }
    }

    Connections {
        target: drawerRoot.controller
        function onAutoSwitchRulesJsonChanged() {
            drawerRoot.syncFromController()
        }
        function onProfilesJsonChanged() {
            drawerRoot.syncFromController()
        }
        function onLastPickedWindowClassChanged() {
            if (drawerRoot.controller && drawerRoot.controller.lastPickedWindowClass.length > 0) {
                classInput.text = drawerRoot.controller.lastPickedWindowClass
            }
        }
    }

    Rectangle {
        anchors.fill: parent
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        Shortcut {
            sequence: "Escape"
            onActivated: drawerRoot.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: Kirigami.Units.mediumSpacing

            // Header
            RowLayout {
                Layout.fillWidth: true
                spacing: Kirigami.Units.smallSpacing

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2

                    Kirigami.Heading {
                        text: qsTr("KWin Wayland Auto-Switcher Rules")
                        level: 2
                        color: Kirigami.Theme.textColor
                    }
                    QQC2.Label {
                        text: qsTr("Zero-polling D-Bus hook (org.kde.KWin) switches duckyPad profiles when windows focus")
                        color: Kirigami.Theme.disabledTextColor
                        font.pixelSize: 11
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: drawerRoot.close()
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Close KWin Auto-Switcher Rules Window")
                }
            }

            // Global Enable + Fallback Desktop Profile Card
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                implicitHeight: globalCfgLayout.implicitHeight + 20
                radius: 8
                color: Kirigami.Theme.backgroundColor
                border.color: drawerRoot.subtleBorder
                border.width: 1

                ColumnLayout {
                    id: globalCfgLayout
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8

                    RowLayout {
                        Layout.fillWidth: true

                        QQC2.Switch {
                            id: masterEnableSwitch
                            checked: drawerRoot.controller ? drawerRoot.controller.autoSwitchEnabled : true
                            text: qsTr("Enable Automatic Profile Switching on Window Focus")
                            font.bold: true
                            onToggled: {
                                if (drawerRoot.controller) {
                                    const fbNum = drawerRoot.fallbackNumbers[fallbackCombo.currentIndex] || 0
                                    drawerRoot.controller.setAutoSwitchConfig(checked, fbNum)
                                }
                            }
                        }

                        Item { Layout.fillWidth: true }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Fallback Profile (when no rule matches):")
                            font.pixelSize: 11
                            color: Kirigami.Theme.disabledTextColor
                        }

                        QQC2.ComboBox {
                            id: fallbackCombo
                            Layout.fillWidth: true
                            model: drawerRoot.fallbackChoices
                            onActivated: (index) => {
                                if (drawerRoot.controller && index >= 0 && index < drawerRoot.fallbackNumbers.length) {
                                    drawerRoot.controller.setAutoSwitchConfig(
                                        masterEnableSwitch.checked,
                                        drawerRoot.fallbackNumbers[index]
                                    )
                                }
                            }
                        }
                    }
                }
            }

            // Live KWin Active Window Monitor + Picker Card
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                implicitHeight: liveWinRow.implicitHeight + 18
                radius: 8
                color: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.22))
                    : Kirigami.Theme.alternateBackgroundColor
                border.color: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                    ? Kirigami.Theme.highlightColor
                    : drawerRoot.subtleBorder
                border.width: drawerRoot.controller && drawerRoot.controller.windowPickerArmed ? 2 : 1

                RowLayout {
                    id: liveWinRow
                    anchors.fill: parent
                    anchors.margins: 9
                    spacing: 10

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2

                        QQC2.Label {
                            text: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                ? qsTr("🎯 Click any open KDE Plasma 6 window to capture its class...")
                                : qsTr("Detected Wayland Window: %1").arg(
                                    drawerRoot.controller ? drawerRoot.controller.activeWindowClass : "-"
                                  )
                            font.bold: true
                            font.family: "monospace"
                            font.pixelSize: 12
                            color: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                ? Kirigami.Theme.highlightColor
                                : Kirigami.Theme.textColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }

                        QQC2.Label {
                            text: drawerRoot.controller && drawerRoot.controller.activeWindowTitle.length > 0
                                ? qsTr("Title: %1").arg(drawerRoot.controller.activeWindowTitle)
                                : qsTr("Switch focus to another app window or click 'Pick Window' to grab its class")
                            font.pixelSize: 10
                            color: Kirigami.Theme.disabledTextColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                    }

                    QQC2.Button {
                        icon.name: "crosshairs"
                        text: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                            ? qsTr("Armed...")
                            : qsTr("🎯 Pick Window")
                        highlighted: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                        onClicked: {
                            if (drawerRoot.controller) {
                                drawerRoot.controller.armWindowPicker(false)
                            }
                        }
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Click this button, then click any open window to fill its resourceClass below")
                    }

                    QQC2.Button {
                        icon.name: "edit-paste"
                        text: qsTr("Use Current")
                        enabled: drawerRoot.controller
                            && drawerRoot.controller.activeWindowClass.length > 0
                            && drawerRoot.controller.activeWindowClass !== "Listening for KWin..."
                        onClicked: {
                            if (drawerRoot.controller) {
                                classInput.text = drawerRoot.controller.activeWindowClass
                            }
                        }
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Copy the last detected window class into the rule editor below")
                    }
                }
            }

            // Add / Edit Rule Form Card
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                implicitHeight: ruleFormCol.implicitHeight + 20
                radius: 8
                color: Kirigami.Theme.backgroundColor
                border.color: drawerRoot.subtleBorder
                border.width: 1

                ColumnLayout {
                    id: ruleFormCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8

                    QQC2.Label {
                        text: drawerRoot.editingRuleIndex >= 0
                            ? qsTr("Edit Rule #%1").arg(drawerRoot.editingRuleIndex + 1)
                            : qsTr("Add New Window Rule")
                        font.bold: true
                        font.pixelSize: 12
                        color: Kirigami.Theme.textColor
                    }

                    GridLayout {
                        Layout.fillWidth: true
                        columns: 2
                        rowSpacing: 6
                        columnSpacing: 8

                        QQC2.Label {
                            text: qsTr("Window Class:")
                            font.pixelSize: 11
                            color: Kirigami.Theme.disabledTextColor
                        }

                        QQC2.TextField {
                            id: classInput
                            Layout.fillWidth: true
                            placeholderText: qsTr("e.g. firefox, chrome, code, org.kde.dolphin")
                            font.family: "monospace"
                            selectByMouse: true
                        }

                        QQC2.Label {
                            text: qsTr("Title Substring:")
                            font.pixelSize: 11
                            color: Kirigami.Theme.disabledTextColor
                        }

                        QQC2.TextField {
                            id: titleInput
                            Layout.fillWidth: true
                            placeholderText: qsTr("Optional (e.g. YouTube, Gmail, Twitch)")
                            selectByMouse: true
                        }

                        QQC2.Label {
                            text: qsTr("Target Profile:")
                            font.pixelSize: 11
                            color: Kirigami.Theme.disabledTextColor
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.ComboBox {
                                id: targetProfileCombo
                                Layout.fillWidth: true
                                model: drawerRoot.profileChoices
                            }

                            QQC2.Button {
                                visible: drawerRoot.editingRuleIndex >= 0
                                text: qsTr("Cancel")
                                onClicked: {
                                    drawerRoot.editingRuleIndex = -1
                                    classInput.text = ""
                                    titleInput.text = ""
                                }
                            }

                            QQC2.Button {
                                icon.name: drawerRoot.editingRuleIndex >= 0 ? "document-save" : "list-add"
                                text: drawerRoot.editingRuleIndex >= 0 ? qsTr("Update Rule") : qsTr("Save Rule")
                                highlighted: true
                                enabled: classInput.text.trim().length > 0 && targetProfileCombo.currentIndex >= 0
                                onClicked: {
                                    if (!drawerRoot.controller) return
                                    const profNum = drawerRoot.profileNumbers[targetProfileCombo.currentIndex] || 1
                                    drawerRoot.controller.addOrUpdateAutoSwitchRule(
                                        drawerRoot.editingRuleIndex,
                                        true,
                                        classInput.text.trim(),
                                        titleInput.text.trim(),
                                        profNum
                                    )
                                    drawerRoot.editingRuleIndex = -1
                                    classInput.text = ""
                                    titleInput.text = ""
                                }
                            }
                        }
                    }
                }
            }

            // Rules List Header
            RowLayout {
                Layout.fillWidth: true
                QQC2.Label {
                    text: qsTr("Configured Rules (%1) — Evaluated Top to Bottom (Drag ⠿ or ▲/▼ to Reorder)").arg(rulesModel.count)
                    font.bold: true
                    font.pixelSize: 12
                    color: Kirigami.Theme.textColor
                }
                Item { Layout.fillWidth: true }
            }

            // Scrollable Rules List with Drag-and-Drop & Button Reordering
            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true

                ListView {
                    id: rulesListView
                    model: drawerRoot.rulesModel
                    spacing: 6
                    highlightFollowsCurrentItem: false

                    property int draggedRuleIndex: -1
                    property int dropTargetRuleIndex: -1

                    delegate: Rectangle {
                        id: ruleDelegate
                        required property int index
                        required property int ruleIndex
                        required property bool ruleEnabled
                        required property string resourceClass
                        required property string titleContains
                        required property int targetProfile
                        required property string targetProfileName
                        required property string ruleIcon

                        readonly property bool isBeingDragged: rulesListView.draggedRuleIndex === index
                        readonly property bool isDropTarget: rulesListView.draggedRuleIndex !== -1
                            && rulesListView.dropTargetRuleIndex === index
                            && rulesListView.draggedRuleIndex !== index

                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        width: rulesListView.width - 12
                        height: 52
                        radius: 6
                        z: isBeingDragged ? 10 : 1
                        opacity: isBeingDragged ? 0.85 : 1.0
                        color: isDropTarget
                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.28))
                            : Kirigami.Theme.backgroundColor
                        border.color: (drawerRoot.editingRuleIndex === ruleIndex || isDropTarget || isBeingDragged)
                            ? Kirigami.Theme.highlightColor
                            : drawerRoot.subtleBorder
                        border.width: (drawerRoot.editingRuleIndex === ruleIndex || isDropTarget || isBeingDragged) ? 2 : 1

                        // Drop indicator line
                        Rectangle {
                            visible: ruleDelegate.isDropTarget
                            anchors.left: parent.left
                            anchors.right: parent.right
                            y: rulesListView.draggedRuleIndex > ruleDelegate.index ? 0 : parent.height - 2
                            height: 2
                            radius: 1
                            color: Kirigami.Theme.highlightColor
                        }

                        RowLayout {
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            // Drag Handle for Reordering Rules
                            Item {
                                Layout.preferredWidth: 18
                                Layout.fillHeight: true

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: "⠿"
                                    font.pixelSize: 15
                                    color: ruleDragMouse.containsMouse || ruleDelegate.isBeingDragged
                                        ? Kirigami.Theme.textColor
                                        : Kirigami.Theme.disabledTextColor
                                }

                                MouseArea {
                                    id: ruleDragMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: ruleDragMouse.dragging
                                        ? Qt.ClosedHandCursor
                                        : Qt.ArrowCursor

                                    property real pressY: 0
                                    property bool dragging: false

                                    onPressed: (mouse) => {
                                        pressY = mouse.y
                                        dragging = false
                                    }

                                    onPositionChanged: (mouse) => {
                                        if (!pressed) return
                                        if (!dragging && Math.abs(mouse.y - pressY) > 5) {
                                            dragging = true
                                            rulesListView.interactive = false
                                            rulesListView.draggedRuleIndex = ruleDelegate.index
                                            rulesListView.dropTargetRuleIndex = ruleDelegate.index
                                        }
                                        if (dragging) {
                                            const posInContent = ruleDragMouse.mapToItem(rulesListView.contentItem, mouse.x, mouse.y)
                                            const stride = ruleDelegate.height + rulesListView.spacing
                                            const targetIdx = Math.max(
                                                0,
                                                Math.min(
                                                    drawerRoot.rulesModel.count - 1,
                                                    Math.floor(posInContent.y / stride)
                                                )
                                            )
                                            rulesListView.dropTargetRuleIndex = targetIdx

                                            const posInView = ruleDragMouse.mapToItem(rulesListView, mouse.x, mouse.y)
                                            if (posInView.y < 24 && rulesListView.contentY > 0) {
                                                rulesListView.contentY = Math.max(0, rulesListView.contentY - 10)
                                            } else if (posInView.y > rulesListView.height - 24) {
                                                const maxScroll = Math.max(0, rulesListView.contentHeight - rulesListView.height)
                                                rulesListView.contentY = Math.min(maxScroll, rulesListView.contentY + 10)
                                            }
                                        }
                                    }

                                    onReleased: {
                                        if (dragging) {
                                            const fromIdx = rulesListView.draggedRuleIndex
                                            const toIdx = rulesListView.dropTargetRuleIndex
                                            dragging = false
                                            rulesListView.draggedRuleIndex = -1
                                            rulesListView.dropTargetRuleIndex = -1
                                            rulesListView.interactive = true
                                            if (drawerRoot.controller && fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                if (drawerRoot.editingRuleIndex === fromIdx) {
                                                    drawerRoot.editingRuleIndex = toIdx
                                                }
                                                drawerRoot.controller.reorderAutoSwitchRule(fromIdx, toIdx)
                                            }
                                        }
                                    }

                                    onCanceled: {
                                        dragging = false
                                        rulesListView.draggedRuleIndex = -1
                                        rulesListView.dropTargetRuleIndex = -1
                                        rulesListView.interactive = true
                                    }

                                    QQC2.ToolTip.visible: containsMouse && !dragging
                                    QQC2.ToolTip.text: qsTr("Drag to reorder rule priority")
                                }
                            }

                            QQC2.CheckBox {
                                checked: ruleEnabled
                                onToggled: {
                                    if (drawerRoot.controller) {
                                        drawerRoot.controller.toggleAutoSwitchRule(ruleIndex, checked)
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Enable or disable this auto-switch rule")
                            }

                            Kirigami.Icon {
                                source: ruleIcon.length > 0 ? ruleIcon : "application-x-executable"
                                Layout.preferredWidth: 24
                                Layout.preferredHeight: 24
                                opacity: ruleEnabled ? 1.0 : 0.45
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 1

                                RowLayout {
                                    spacing: 6
                                    QQC2.Label {
                                        text: "#" + (index + 1) + "  " + resourceClass
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: 12
                                        color: ruleEnabled ? Kirigami.Theme.textColor : Kirigami.Theme.disabledTextColor
                                    }

                                    Rectangle {
                                        visible: titleContains.length > 0
                                        radius: 4
                                        color: Kirigami.Theme.alternateBackgroundColor
                                        border.color: drawerRoot.subtleBorder
                                        border.width: 1
                                        implicitWidth: titleTag.implicitWidth + 10
                                        implicitHeight: 18

                                        QQC2.Label {
                                            id: titleTag
                                            anchors.centerIn: parent
                                            text: qsTr("title: \"%1\"").arg(titleContains)
                                            font.pixelSize: 10
                                            color: Kirigami.Theme.disabledTextColor
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("Switches to Profile #%1 (%2)").arg(targetProfile).arg(targetProfileName)
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.highlightColor
                                }
                            }

                            // Up / Down Rule Priority Reorder Buttons
                            RowLayout {
                                spacing: 2

                                QQC2.ToolButton {
                                    text: "▲"
                                    enabled: ruleDelegate.index > 0
                                    implicitWidth: 24
                                    implicitHeight: 26
                                    font.pixelSize: 10
                                    onClicked: {
                                        if (drawerRoot.controller) {
                                            drawerRoot.controller.reorderAutoSwitchRule(ruleDelegate.index, ruleDelegate.index - 1)
                                        }
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Move Rule Up (Higher Priority)")
                                }

                                QQC2.ToolButton {
                                    text: "▼"
                                    enabled: ruleDelegate.index < drawerRoot.rulesModel.count - 1
                                    implicitWidth: 24
                                    implicitHeight: 26
                                    font.pixelSize: 10
                                    onClicked: {
                                        if (drawerRoot.controller) {
                                            drawerRoot.controller.reorderAutoSwitchRule(ruleDelegate.index, ruleDelegate.index + 1)
                                        }
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Move Rule Down (Lower Priority)")
                                }
                            }

                            QQC2.Button {
                                text: qsTr("Edit")
                                implicitHeight: 28
                                font.pixelSize: 11
                                onClicked: {
                                    drawerRoot.editingRuleIndex = ruleIndex
                                    classInput.text = resourceClass
                                    titleInput.text = titleContains
                                    const pIdx = drawerRoot.profileNumbers.indexOf(targetProfile)
                                    if (pIdx >= 0) {
                                        targetProfileCombo.currentIndex = pIdx
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Edit Rule")
                            }

                            QQC2.Button {
                                text: qsTr("✕")
                                implicitWidth: 28
                                implicitHeight: 28
                                font.pixelSize: 11
                                onClicked: {
                                    if (drawerRoot.controller) {
                                        drawerRoot.controller.removeAutoSwitchRule(ruleIndex)
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Delete Rule")
                            }
                        }
                    }
                }
            }
        }
    }
}
