import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Rectangle {
    id: profilesSidebar
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property var hardwareCanvas: null
    property var autoSwitchDrawer: null
    property var profileContextMenu: null
    property var sidebarProfileModel: null
    property var importProfileFileDialog: null
    property var loadProfileGroupFileDialog: null
    property var saveProfileGroupFileDialog: null
    readonly property alias profileSidebarList: profileSidebarList
    readonly property alias mainProfilesBoxScrollBar: mainProfilesBoxScrollBar
    readonly property alias newProfileTemplateCombo: newProfileTemplateCombo
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
                QQC2.ToolTip.delay: 500
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
                                Layout.preferredWidth: 6
                                Layout.preferredHeight: Math.round(24 * root.sidebarScale)
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

                                QQC2.ToolTip.delay: 500
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

                                    QQC2.ToolTip.delay: 500
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
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Select a template for the new profile")
                }

                QQC2.Button {
                    icon.name: "bookmark-new"
                    text: qsTr("Templates...")
                    implicitHeight: Math.round(24 * root.sidebarScale)
                    font.pointSize: 10 * root.fontScale
                    onClicked: profileTemplatesDialog.openGeneral()
                    QQC2.ToolTip.delay: 500
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
                            Qt.callLater(root.ensureActiveProfileVisibleInList)
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
                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Create a new profile from blank or from a template (Right-click a profile for Templates & Export)")
            }

            QQC2.Button {
                Layout.fillWidth: true
                icon.name: "tools-wizard"
                text: qsTr("Wizard...")
                font.pointSize: 10 * root.fontScale
                onClicked: root.openProfileWizard()
                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Open New Profile Wizard: step-by-step setup with window picker, auto-switch rule, and shortcut search")
            }

            QQC2.Button {
                Layout.fillWidth: true
                icon.name: "document-import"
                text: qsTr("Import")
                font.pointSize: 10 * root.fontScale
                onClicked: importProfileFileDialog.open()
                QQC2.ToolTip.delay: 500
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
                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Manage Profile Templates and Profile Groups (.duckygroup.json)")

                QQC2.Menu {
                    font.pointSize: 10 * root.fontScale
                    id: profileGroupMenu
                    title: qsTr("Templates & Profile Groups")

                    QQC2.MenuItem {
                        icon.name: "tools-wizard"
                        text: qsTr("New Profile Wizard...")
                        onTriggered: root.openProfileWizard()
                    }

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
            Layout.preferredHeight: 1
            color: profilesSidebar.frameBorder
        }

        // AUTO-SWITCHER Section
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 6

            RowLayout {
                Layout.fillWidth: true
                spacing: 4

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
                    QQC2.ToolTip.delay: 500
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
                    : (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled
                        ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(0.96, 0.62, 0.04, 0.16))
                        : Kirigami.Theme.alternateBackgroundColor)
                border.color: deviceController.windowPickerArmed
                    ? Kirigami.Theme.highlightColor
                    : (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled
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
                            : (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled
                                ? qsTr("⏸ Paused while Key Editor is focused")
                                : qsTr("Active: %1").arg(deviceController.activeWindowClass))
                        font.family: "monospace"
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: deviceController.windowPickerArmed
                            ? root.highContrastHighlightText
                            : (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled
                                ? root.highContrastWarningText
                                : Kirigami.Theme.textColor)
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }

                    QQC2.Label {
                        visible: deviceController.activeWindowTitle.length > 0 || (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled)
                        text: (root.shouldPauseAutoSwitchForEditing && deviceController.autoSwitchEnabled)
                            ? qsTr("Auto-Switch resumes when another window is focused")
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
                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Open Auto-Switcher Rules")
            }
        }
    }
}
