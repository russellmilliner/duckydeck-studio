import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Window {
    id: settingsDialog
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property bool isLightTheme: root ? root.isLightTheme : false
    property color highContrastHighlightText: root ? root.highContrastHighlightText : "#38bdf8"
    property color highContrastPositiveText: root ? root.highContrastPositiveText : "#4ade80"
    property color highContrastWarningText: root ? root.highContrastWarningText : "#fbbf24"
    property var saveProfileGroupFileDialog: null
    property var duckyColorPickerWindow: null
    property var autoSwitchDrawer: null
    property var profileSidebarList: null
    property var hardwareCanvas: null
    property var appLaunchConfigWindow: null
    property var webApiKeyConfigWindow: null
    property var oledDesktopWidget: null
    readonly property var lightingAnimatorDialog: duckyColorPickerWindow ? duckyColorPickerWindow.lightingAnimatorDialog : null
    readonly property var macroRecorderDialog: appLaunchConfigWindow ? appLaunchConfigWindow.macroRecorderDialog : null
    readonly property var oledIconPickerDialog: appLaunchConfigWindow ? appLaunchConfigWindow.oledIconPickerDialog : null
    property alias newProfileGroupDialog: newProfileGroupDialog
    property alias profileTemplatesDialog: profileTemplatesDialog
    property alias renameProfileDialog: renameProfileDialog
    property alias aboutDialog: aboutDialog
    property alias profilePropertiesWindow: profilePropertiesWindow
    property alias keyPropertiesWindow: keyPropertiesWindow

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

    Window {
        id: newProfileGroupDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Create New Profile Group — DuckyDeck Studio")
        width: 590
        height: 255
        minimumWidth: 560
        minimumHeight: 240
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openWithDefaults() {
            newGroupNameInput.text = qsTr("New Group")
            newGroupFirstProfileInput.text = qsTr("Default")
            newProfileGroupDialog.show()
            newProfileGroupDialog.raise()
            newProfileGroupDialog.requestActivate()
            Qt.callLater(function() {
                newGroupNameInput.forceActiveFocus()
                newGroupNameInput.selectAll()
            })
        }

        function commitCreateNewGroup() {
            const groupName = newGroupNameInput.text.trim() || qsTr("New Group")
            const firstProf = newGroupFirstProfileInput.text.trim() || qsTr("Default")
            if (deviceController.createNewProfileGroup(groupName, firstProf)) {
                root.refreshSelectedSlotFromController()
                newProfileGroupDialog.close()
            }
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: newProfileGroupDialog.visible
            onActivated: newProfileGroupDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10
            Keys.onEscapePressed: newProfileGroupDialog.close()

            QQC2.Label {
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                Layout.preferredWidth: 1
                wrapMode: Text.WordWrap
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.textColor
                text: qsTr("Start a fresh profile group workspace. Tip: Use 'Save Current Group...' first if you want to back up your existing profiles and Auto-Switcher rules.")
            }

            GridLayout {
                Layout.fillWidth: true
                columns: 2
                columnSpacing: 10
                rowSpacing: 8

                QQC2.Label {
                    text: qsTr("Group Name:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupNameInput
                    Layout.fillWidth: true
                    Layout.minimumWidth: 120
                    Layout.preferredWidth: 1
                    placeholderText: qsTr("e.g. Work & Coding")
                    selectByMouse: true
                    font.pointSize: 10 * root.fontScale
                    onAccepted: newProfileGroupDialog.commitCreateNewGroup()
                    Keys.onEscapePressed: newProfileGroupDialog.close()
                }

                QQC2.Label {
                    text: qsTr("Initial Profile #1:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupFirstProfileInput
                    Layout.fillWidth: true
                    Layout.minimumWidth: 120
                    Layout.preferredWidth: 1
                    maximumLength: 16
                    placeholderText: qsTr("Default")
                    selectByMouse: true
                    font.pointSize: 10 * root.fontScale
                    onAccepted: newProfileGroupDialog.commitCreateNewGroup()
                    Keys.onEscapePressed: newProfileGroupDialog.close()
                }
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "document-save-all"
                    text: qsTr("Save Current Group...")
                    onClicked: saveProfileGroupFileDialog.open()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "dialog-cancel"
                    text: qsTr("Cancel")
                    onClicked: newProfileGroupDialog.close()
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "document-new"
                    text: qsTr("Create New Group")
                    highlighted: true
                    onClicked: newProfileGroupDialog.commitCreateNewGroup()
                }
            }
        }
    }

    Window {
        id: profileTemplatesDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Profile Templates Manager — DuckyDeck Studio")
        width: 840
        height: 660
        minimumWidth: 640
        minimumHeight: 520
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property var templatesList: []
        property int selectedIndex: 0
        property int targetProfileNumber: 0
        property string targetProfileName: ""
        property string statusMessage: ""

        function refreshTemplates() {
            let parsed = []
            try {
                parsed = JSON.parse(deviceController.listProfileTemplatesJson())
            } catch (e) {
                parsed = []
            }
            templatesList = parsed
            if (selectedIndex >= templatesList.length) {
                selectedIndex = Math.max(0, templatesList.length - 1)
            }
        }

        function selectedTemplate() {
            if (selectedIndex >= 0 && selectedIndex < templatesList.length) {
                return templatesList[selectedIndex]
            }
            return null
        }

        function deleteTemplateById(tplId, tplName) {
            if (!tplId) return
            if (deviceController.deleteProfileTemplate(tplId)) {
                profileTemplatesDialog.refreshTemplates()
                root.refreshSidebarTemplateCombo()
                profileTemplatesDialog.statusMessage = qsTr("✓ Deleted custom template '%1'.").arg(tplName || tplId)
            } else {
                profileTemplatesDialog.statusMessage = deviceController.statusMessage
            }
        }

        function openGeneral() {
            refreshTemplates()
            targetProfileNumber = deviceController.activeProfile
            targetProfileName = deviceController.activeProfileName
            saveSourceProfileCombo.syncChoices()
            tplSaveNameField.text = deviceController.activeProfileName + " Template"
            tplSaveDescField.text = qsTr("Saved from Profile #%1 (%2)")
                .arg(deviceController.activeProfile)
                .arg(deviceController.activeProfileName)
            tplNewProfileNameField.text = ""
            statusMessage = ""
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
        }

        function openSaveFromProfile(profNum, profName) {
            refreshTemplates()
            targetProfileNumber = profNum
            targetProfileName = profName
            saveSourceProfileCombo.syncChoices(profNum)
            tplSaveNameField.text = profName + " Template"
            tplSaveDescField.text = qsTr("Copied from Profile #%1 (%2)").arg(profNum).arg(profName)
            statusMessage = ""
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
            Qt.callLater(function() {
                tplSaveNameField.forceActiveFocus()
                tplSaveNameField.selectAll()
            })
        }

        function openApplyToProfile(profNum, profName) {
            refreshTemplates()
            targetProfileNumber = profNum
            targetProfileName = profName
            saveSourceProfileCombo.syncChoices(profNum)
            statusMessage = qsTr("Select a template below and click 'Apply to Profile #%1 (%2)'.").arg(profNum).arg(profName)
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: profileTemplatesDialog.visible
            onActivated: profileTemplatesDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2
                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: qsTr("Profile Templates — Create Profiles from Templates or Save Existing Profiles as Templates")
                    font.bold: true
                    font.pointSize: 11 * root.fontScale
                    color: Kirigami.Theme.textColor
                }
                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: qsTr("Choose a built-in or custom template to create a new profile or overwrite an existing profile, or copy any existing profile into a reusable template.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            // Copy Existing Profile to Template Card
            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                border.width: 1
                implicitHeight: saveTplCol.implicitHeight + 16

                ColumnLayout {
                    id: saveTplCol
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.top: parent.top
                    anchors.margins: 8
                    spacing: 6

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        text: qsTr("Copy Existing Profile to Reusable Template:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Source Profile:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.ComboBox {
                            font.pointSize: 10 * root.fontScale
                            id: saveSourceProfileCombo
                            Layout.preferredWidth: 185
                            Layout.minimumWidth: 130
                            property var profileNums: []
                            property var profileNames: []
                            function syncChoices(preferredNum) {
                                let nums = []
                                let names = []
                                let labels = []
                                try {
                                    const arr = JSON.parse(deviceController.profilesJson)
                                    for (let i = 0; i < arr.length; i++) {
                                        nums.push(arr[i].number)
                                        names.push(arr[i].name)
                                        labels.push("#" + arr[i].number + " — " + arr[i].name)
                                    }
                                } catch (e) {}
                                profileNums = nums
                                profileNames = names
                                model = labels
                                const target = preferredNum || deviceController.activeProfile
                                const idx = nums.indexOf(target)
                                currentIndex = idx >= 0 ? idx : 0
                            }
                            onActivated: {
                                if (currentIndex >= 0 && currentIndex < profileNames.length) {
                                    tplSaveNameField.text = profileNames[currentIndex] + " Template"
                                    tplSaveDescField.text = qsTr("Copied from Profile #%1 (%2)")
                                        .arg(profileNums[currentIndex])
                                        .arg(profileNames[currentIndex])
                                }
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Template Name:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextField {
                            font.pointSize: 10 * root.fontScale
                            id: tplSaveNameField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 140
                            maximumLength: 36
                            placeholderText: qsTr("Template Name...")
                            selectByMouse: true
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            icon.name: "bookmark-new"
                            text: qsTr("Save as Template")
                            highlighted: true
                            enabled: tplSaveNameField.text.trim().length > 0 && saveSourceProfileCombo.profileNums.length > 0
                            onClicked: {
                                const srcIdx = Math.max(0, saveSourceProfileCombo.currentIndex)
                                const srcProf = saveSourceProfileCombo.profileNums[srcIdx] || deviceController.activeProfile
                                if (deviceController.saveProfileAsTemplate(
                                    srcProf,
                                    tplSaveNameField.text.trim(),
                                    tplSaveDescField.text.trim()
                                )) {
                                    profileTemplatesDialog.refreshTemplates()
                                    root.refreshSidebarTemplateCombo()
                                    profileTemplatesDialog.selectedIndex = Math.max(0, profileTemplatesDialog.templatesList.length - 1)
                                    profileTemplatesDialog.statusMessage = qsTr("✓ Saved Profile #%1 as template '%2'.")
                                        .arg(srcProf)
                                        .arg(tplSaveNameField.text.trim())
                                }
                            }
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Description:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextField {
                            font.pointSize: 10 * root.fontScale
                            id: tplSaveDescField
                            Layout.fillWidth: true
                            maximumLength: 140
                            placeholderText: qsTr("Description (optional)...")
                            selectByMouse: true
                        }
                    }
                }
            }

            // Templates List
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 200
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                border.width: 1
                clip: true

                ListView {
                    id: tplListView
                    anchors.fill: parent
                    anchors.margins: 6
                    spacing: 6
                    model: profileTemplatesDialog.templatesList
                    boundsBehavior: Flickable.StopAtBounds
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: tplListView
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: tplListView.contentHeight > tplListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Rectangle {
                        id: tplDelegate
                        width: Math.max(200, tplListView.width - (tplListView.contentHeight > tplListView.height ? 16 : 4))
                        implicitHeight: tplDelegateCol.implicitHeight + 16
                        radius: 5
                        readonly property bool isSel: profileTemplatesDialog.selectedIndex === index
                        readonly property bool isBuiltIn: Boolean(modelData.builtIn !== undefined ? modelData.builtIn : modelData.builtin)
                        readonly property int keyCount: Number(modelData.configuredSlotCount !== undefined ? modelData.configuredSlotCount : (modelData.slotCount || 0))
                        color: isSel
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.22)
                            : (tplRowMouse.containsMouse ? Qt.alpha(Kirigami.Theme.textColor, 0.06) : Kirigami.Theme.backgroundColor)
                        border.color: isSel
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                        border.width: isSel ? 2 : 1

                        MouseArea {
                            id: tplRowMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            z: 0
                            onClicked: {
                                profileTemplatesDialog.selectedIndex = index
                            }
                        }

                        ColumnLayout {
                            id: tplDelegateCol
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.top: parent.top
                            anchors.margins: 8
                            spacing: 4
                            z: 2

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                Rectangle {
                                    width: 14
                                    height: 14
                                    radius: 3
                                    color: modelData.bgColor || "#38bdf8"
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.Label {
                                    text: modelData.name || ""
                                    font.bold: true
                                    font.pointSize: 11 * root.fontScale
                                    color: Kirigami.Theme.textColor
                                    elide: Text.ElideRight
                                    Layout.maximumWidth: 260
                                }

                                Rectangle {
                                    radius: 4
                                    color: tplDelegate.isBuiltIn
                                        ? Qt.alpha("#0ea5e9", 0.20)
                                        : Qt.alpha("#a855f7", 0.22)
                                    border.color: tplDelegate.isBuiltIn
                                        ? Qt.alpha("#38bdf8", 0.65)
                                        : Qt.alpha("#c084fc", 0.70)
                                    border.width: 1
                                    implicitWidth: tplKindLbl.implicitWidth + 10
                                    implicitHeight: tplKindLbl.implicitHeight + 4

                                    QQC2.Label {
                                        id: tplKindLbl
                                        anchors.centerIn: parent
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        text: tplDelegate.isBuiltIn ? qsTr("BUILT-IN") : qsTr("CUSTOM TEMPLATE")
                                        color: Kirigami.Theme.textColor
                                    }
                                }

                                QQC2.Label {
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.82)
                                    text: qsTr("%1 keys configured  •  OLED %2°")
                                        .arg(tplDelegate.keyCount)
                                        .arg(modelData.oledRotation || 0)
                                    elide: Text.ElideRight
                                    Layout.fillWidth: true
                                }

                                QQC2.Button {
                                    visible: !tplDelegate.isBuiltIn
                                    icon.name: "edit-delete"
                                    text: qsTr("Delete")
                                    implicitHeight: 26
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: {
                                        profileTemplatesDialog.deleteTemplateById(modelData.id, modelData.name)
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                                font.pointSize: 10 * root.fontScale
                                text: modelData.description || ""
                                color: Qt.alpha(Kirigami.Theme.textColor, 0.85)
                            }
                        }
                    }
                }
            }

            // Status message
            QQC2.Label {
                visible: profileTemplatesDialog.statusMessage.length > 0
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: profileTemplatesDialog.statusMessage
                font.bold: true
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.positiveTextColor
            }

            // Footer Actions: two clean rows to prevent horizontal overflow
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.Label {
                        text: qsTr("New Profile Name:")
                        font.pointSize: 10 * root.fontScale
                    }

                    QQC2.TextField {
                        font.pointSize: 10 * root.fontScale
                        id: tplNewProfileNameField
                        Layout.preferredWidth: 180
                        Layout.minimumWidth: 120
                        maximumLength: 16
                        placeholderText: profileTemplatesDialog.selectedTemplate()
                            ? (profileTemplatesDialog.selectedTemplate().defaultProfileName || profileTemplatesDialog.selectedTemplate().name || qsTr("New profile name..."))
                            : qsTr("New profile name...")
                        selectByMouse: true
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        icon.name: "list-add"
                        text: qsTr("Create New Profile from Selected")
                        highlighted: true
                        enabled: profileTemplatesDialog.selectedTemplate() !== null
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            const pName = tplNewProfileNameField.text.trim().length > 0
                                ? tplNewProfileNameField.text.trim()
                                : (tpl.defaultProfileName || tpl.name)
                            if (deviceController.createProfileFromTemplate(pName, tpl.id)) {
                                root.syncSidebarProfilesFromController()
                                hardwareCanvas.syncProfilesFromController()
                                root.refreshSelectedSlotFromController()
                                profileTemplatesDialog.statusMessage = qsTr("✓ Created new profile '%1' from template '%2'.").arg(pName).arg(tpl.name)
                            }
                        }
                    }

                    Item { Layout.fillWidth: true }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        visible: profileTemplatesDialog.targetProfileNumber > 0
                        icon.name: "document-new-from-template"
                        text: qsTr("Apply to Profile #%1 (%2)")
                            .arg(profileTemplatesDialog.targetProfileNumber)
                            .arg(profileTemplatesDialog.targetProfileName || deviceController.activeProfileName)
                        enabled: profileTemplatesDialog.selectedTemplate() !== null
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            if (deviceController.applyTemplateToProfile(profileTemplatesDialog.targetProfileNumber, tpl.id)) {
                                root.syncSidebarProfilesFromController()
                                hardwareCanvas.syncProfilesFromController()
                                root.refreshSelectedSlotFromController()
                                profileTemplatesDialog.statusMessage = qsTr("✓ Applied template '%1' to Profile #%2.")
                                    .arg(tpl.name)
                                    .arg(profileTemplatesDialog.targetProfileNumber)
                            }
                        }
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        readonly property var curTpl: profileTemplatesDialog.selectedTemplate()
                        readonly property bool canDelete: curTpl !== null && !(curTpl.builtIn !== undefined ? curTpl.builtIn : curTpl.builtin)
                        visible: canDelete
                        enabled: canDelete
                        icon.name: "edit-delete"
                        text: qsTr("Delete Selected Template")
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            profileTemplatesDialog.deleteTemplateById(tpl.id, tpl.name)
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        icon.name: "window-close"
                        text: qsTr("Close")
                        onClicked: profileTemplatesDialog.close()
                    }
                }
            }
        }
    }

    Window {
        id: renameProfileDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Rename Profile #%1 — DuckyDeck Studio").arg(renameProfileDialog.targetProfileNumber)
        width: 420
        height: 175
        minimumWidth: 380
        minimumHeight: 165
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        property int targetProfileNumber: 1
        property string initialProfileName: ""

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openWithProfile(profileNumber, currentName) {
            renameProfileDialog.targetProfileNumber = profileNumber
            renameProfileDialog.initialProfileName = currentName
            renameProfileInput.text = currentName
            renameProfileDialog.show()
            renameProfileDialog.raise()
            renameProfileDialog.requestActivate()
            Qt.callLater(function() {
                renameProfileInput.forceActiveFocus()
                renameProfileInput.selectAll()
            })
        }

        function commitRename() {
            const trimmed = renameProfileInput.text.trim()
            if (trimmed.length === 0) return
            const savedY = profileSidebarList.contentY
            root.selectingProfileFromClick = true
            if (deviceController.renameProfile(renameProfileDialog.targetProfileNumber, trimmed)) {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                renameProfileDialog.close()
            }
            root.selectingProfileFromClick = false
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: renameProfileDialog.visible
            onActivated: renameProfileDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12
            Keys.onEscapePressed: renameProfileDialog.close()

            QQC2.Label {
                Layout.fillWidth: true
                font.bold: true
                font.pointSize: 10 * root.fontScale
                text: qsTr("New name for Profile #%1 (max 16 characters):").arg(renameProfileDialog.targetProfileNumber)
                color: Kirigami.Theme.textColor
            }

            QQC2.TextField {
                id: renameProfileInput
                Layout.fillWidth: true
                maximumLength: 16
                placeholderText: qsTr("Profile name...")
                selectByMouse: true
                font.pointSize: 11 * root.fontScale
                onAccepted: renameProfileDialog.commitRename()
                Keys.onEscapePressed: renameProfileDialog.close()
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "dialog-cancel"
                    text: qsTr("Cancel")
                    onClicked: renameProfileDialog.close()
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Rename")
                    highlighted: true
                    enabled: renameProfileInput.text.trim().length > 0
                    onClicked: renameProfileDialog.commitRename()
                }
            }
        }
    }

    Window {
        id: aboutDialog
        readonly property bool closableWithEscape: true
        title: qsTr("About DuckyDeck Studio v%1").arg(deviceController.appVersion)
        width: 720
        height: 550
        minimumWidth: 680
        minimumHeight: 510
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openAboutWindow() {
            aboutDialog.show()
            aboutDialog.raise()
            aboutDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: aboutDialog.visible
            onActivated: aboutDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            // App Header Banner
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: aboutHeaderCol.implicitHeight + 24
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                border.width: 1

                ColumnLayout {
                    id: aboutHeaderCol
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 6

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 10

                        // Application Icon matching data/icons/io.github.duckydeck.Studio.svg
                        Rectangle {
                            width: 32
                            height: 32
                            radius: 6
                            color: "#16181d"
                            border.color: "#38bdf8"
                            border.width: 1.5

                            // 3x4 RGB key matrix
                            Repeater {
                                model: [
                                    { x: 4,  y: 5,  c: "#00aaff" }, { x: 10, y: 5,  c: "#00aaff" }, { x: 16, y: 5,  c: "#00aaff" },
                                    { x: 4,  y: 11, c: "#28dc64" }, { x: 10, y: 11, c: "#28dc64" }, { x: 16, y: 11, c: "#28dc64" },
                                    { x: 4,  y: 17, c: "#ffb400" }, { x: 10, y: 17, c: "#ffb400" }, { x: 16, y: 17, c: "#ffb400" },
                                    { x: 4,  y: 23, c: "#ff3c3c" }, { x: 10, y: 23, c: "#b450ff" }, { x: 16, y: 23, c: "#b450ff" }
                                ]
                                delegate: Rectangle {
                                    required property var modelData
                                    x: modelData.x
                                    y: modelData.y
                                    width: 4.5
                                    height: 4.5
                                    radius: 1
                                    color: modelData.c
                                }
                            }

                            // Rotary encoders & OLED on right
                            Rectangle { x: 22.5; y: 5;  width: 5.5; height: 5.5; radius: 2.75; color: "#262b36"; border.color: "#38bdf8"; border.width: 1 }
                            Rectangle { x: 22.5; y: 12; width: 5.5; height: 5.5; radius: 2.75; color: "#262b36"; border.color: "#38bdf8"; border.width: 1 }
                            Rectangle { x: 22;   y: 19; width: 6.5; height: 8.5; radius: 1;    color: "#000000"; border.color: "#94a3b8"; border.width: 1 }
                        }

                        QQC2.Label {
                            text: qsTr("DuckyDeck Studio")
                            font.bold: true
                            font.pointSize: 14 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }

                        Rectangle {
                            radius: 4
                            color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.6)
                            border.width: 1
                            implicitWidth: verPillLbl.implicitWidth + 14
                            implicitHeight: 24

                            QQC2.Label {
                                id: verPillLbl
                                anchors.centerIn: parent
                                text: qsTr("v%1").arg(deviceController.appVersion)
                                font.family: "monospace"
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: root.highContrastHighlightText
                            }
                        }

                        Item { Layout.fillWidth: true }

                        // Ko-fi Buy Me a Coffee Widget in About Header
                        Rectangle {
                            id: aboutKofiWidget
                            radius: 16
                            color: aboutKofiMouse.containsMouse ? "#c81e1e" : "#d93834"
                            border.color: "#ffffff33"
                            border.width: 1
                            implicitWidth: aboutKofiRow.implicitWidth + 22
                            implicitHeight: 30

                            RowLayout {
                                id: aboutKofiRow
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
                                id: aboutKofiMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                            }

                            QQC2.ToolTip.visible: aboutKofiMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("Support Russell Milliner on Ko-fi (https://ko-fi.com/russellmilliner)")
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        Layout.preferredWidth: 1
                        text: qsTr("KDE Plasma 6 / Kirigami Configurator, DSVM v2 Compiler & Wayland Auto-Switcher for duckyPad Pro & duckyPad (2020)")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                    }
                }
            }

            // Metadata Details Card
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                Flickable {
                    id: aboutDetailsFlickable
                    anchors.fill: parent
                    anchors.margins: 14
                    clip: true
                    contentWidth: width
                    contentHeight: aboutMetadataGrid.implicitHeight + 4
                    boundsBehavior: Flickable.StopAtBounds
                    flickableDirection: Flickable.VerticalFlick

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: aboutDetailsFlickable
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: aboutDetailsFlickable.contentHeight > aboutDetailsFlickable.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    GridLayout {
                        id: aboutMetadataGrid
                        width: Math.max(
                            200,
                            aboutDetailsFlickable.width - (aboutDetailsFlickable.contentHeight > aboutDetailsFlickable.height ? 16 : 0)
                        )
                        columns: 2
                        rowSpacing: 8
                        columnSpacing: 14

                        QQC2.Label {
                            text: qsTr("Version:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("v%1 (DSVM v2 Bytecode Compiler)").arg(deviceController.appVersion)
                            font.family: "monospace"
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("Author:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: deviceController.appAuthor
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("Documentation:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 8

                            QQC2.Label {
                                id: docsLinkLbl
                                text: "https://duckydeckstudio.com"
                                font.family: "monospace"
                                font.pointSize: 10.5 * root.fontScale
                                font.underline: docsLinkMouse.containsMouse
                                color: root.highContrastHighlightText
                                elide: Text.ElideMiddle
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1

                                MouseArea {
                                    id: docsLinkMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://duckydeckstudio.com")
                                }
                            }

                            QQC2.Button {
                                icon.name: "edit-copy"
                                text: qsTr("Copy")
                                implicitHeight: 24
                                font.pointSize: 10 * root.fontScale
                                onClicked: root.copyTextToClipboard("https://duckydeckstudio.com")
                            }
                        }

                        QQC2.Label {
                            text: qsTr("GitHub:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 8

                            QQC2.Label {
                                id: githubLinkLbl
                                text: deviceController.appGithubUrl
                                font.family: "monospace"
                                font.pointSize: 10.5 * root.fontScale
                                font.underline: githubMouse.containsMouse
                                color: root.highContrastHighlightText
                                elide: Text.ElideMiddle
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1

                                MouseArea {
                                    id: githubMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally(deviceController.appGithubUrl)
                                }
                            }

                            QQC2.Button {
                                icon.name: "edit-copy"
                                text: qsTr("Copy")
                                implicitHeight: 24
                                font.pointSize: 10 * root.fontScale
                                onClicked: root.copyTextToClipboard(deviceController.appGithubUrl)
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Ko-fi Support:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 8

                            QQC2.Label {
                                id: kofiLinkLbl
                                text: "https://ko-fi.com/russellmilliner"
                                font.family: "monospace"
                                font.pointSize: 10.5 * root.fontScale
                                font.underline: kofiLinkMouse.containsMouse
                                color: root.isLightTheme ? "#c81e1e" : "#ff5e5b"
                                elide: Text.ElideMiddle
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1

                                MouseArea {
                                    id: kofiLinkMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                                }
                            }

                            QQC2.Button {
                                icon.name: "edit-copy"
                                text: qsTr("Copy")
                                implicitHeight: 24
                                font.pointSize: 10 * root.fontScale
                                onClicked: root.copyTextToClipboard("https://ko-fi.com/russellmilliner")
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Special Thanks:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("Thank you to dekuNukem for creating the duckyPad & duckyPad Pro!")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: root.highContrastPositiveText
                            wrapMode: Text.WordWrap
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }

                        QQC2.Label {
                            text: qsTr("duckyPad Links:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 12

                            QQC2.Label {
                                text: "duckypad.com"
                                font.family: "monospace"
                                font.pointSize: 10.5 * root.fontScale
                                font.underline: duckySiteMouse.containsMouse
                                color: root.highContrastHighlightText

                                MouseArea {
                                    id: duckySiteMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://www.duckypad.com")
                                }
                            }

                            QQC2.Label {
                                text: "•"
                                font.pointSize: 10.5 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.Label {
                                text: "Tindie Store (dekuNukem)"
                                font.family: "monospace"
                                font.pointSize: 10.5 * root.fontScale
                                font.underline: tindieSiteMouse.containsMouse
                                color: root.highContrastHighlightText

                                MouseArea {
                                    id: tindieSiteMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://www.tindie.com/stores/dekunukem/")
                                }
                            }

                            Item { Layout.fillWidth: true }
                        }

                        QQC2.Label {
                            text: qsTr("License:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("GPL-3.0-or-later")
                            font.family: "monospace"
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }

                        QQC2.Label {
                            text: qsTr("Device Status:")
                            font.bold: true
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: deviceController.connected
                                ? qsTr("%1 (FW: %2 • SN: %3)")
                                    .arg(deviceController.modelName)
                                    .arg(deviceController.firmwareVersion)
                                    .arg(deviceController.serialNumber)
                                : deviceController.modelName
                            font.family: "monospace"
                            font.pointSize: 10.5 * root.fontScale
                            color: Kirigami.Theme.textColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }
                    }
                }
            }

            // Footer Buttons
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("GitHub")
                    font.pointSize: 10.5 * root.fontScale
                    onClicked: Qt.openUrlExternally(deviceController.appGithubUrl)
                }

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("duckyPad Site")
                    font.pointSize: 10.5 * root.fontScale
                    onClicked: Qt.openUrlExternally("https://www.duckypad.com")
                }

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("Tindie Store")
                    font.pointSize: 10.5 * root.fontScale
                    onClicked: Qt.openUrlExternally("https://www.tindie.com/stores/dekunukem/")
                }

                QQC2.Button {
                    id: aboutUserGuideBtn
                    icon.name: "help-contents"
                    text: qsTr("User Guide")
                    font.pointSize: 10.5 * root.fontScale
                    onClicked: deviceController.openUserGuide()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Close")
                    font.pointSize: 10.5 * root.fontScale
                    highlighted: true
                    onClicked: aboutDialog.close()
                }
            }
        }
    }

    Window {
        id: profilePropertiesWindow
        property alias profBgColorField: profBgColorField
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: profilePropertiesWindow.visible
            onActivated: profilePropertiesWindow.close()
        }
        title: qsTr("Profile #%1 Properties — %2").arg(profilePropertiesWindow.profNumber).arg(profilePropertiesWindow.profName)
        width: 820
        height: 740
        minimumWidth: 560
        minimumHeight: 480
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property int profNumber: 1
        property string profName: ""
        property string profFolderName: ""
        property string profBgColor: "#38bdf8"
        property bool profHasKeydownColor: false
        property string profKeydownColor: "#ffffff"
        property bool profDimUnusedKeys: true
        property bool profIsLandscape: false
        property int profOledRotation: 0 // 0 | 90 | 180 | 270
        property int profWidgetRotation: -1 // -1 (Follow OLED) | 0 | 90 | 180 | 270
        property bool profIsUpperHs: false
        property string profExtraConfigLines: ""
        property int profConfiguredSlots: 0
        property bool profWebhookEnabled: false
        property string profWebhookMethod: "POST"
        property string profWebhookUrl: ""
        property string profWebhookHeaders: "Content-Type: application/json"
        property string profWebhookBody: "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"source\":\"{{source}}\"}"
        property string webhookTestStatus: ""
        property string statusFeedback: ""

        function openForProfile(profileNumber) {
            const targetNum = profileNumber > 0 ? profileNumber : deviceController.activeProfile
            if (targetNum <= 0) return
            if (deviceController.activeProfile !== targetNum) {
                deviceController.selectProfile(targetNum)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                oledDesktopWidget.syncFromController()
            }
            profilePropertiesWindow.profNumber = targetNum
            const rawJson = deviceController.getProfilePropertiesJson(targetNum)
            try {
                const obj = JSON.parse(rawJson)
                profilePropertiesWindow.profName = obj.name || ("Profile " + targetNum)
                profilePropertiesWindow.profFolderName = obj.folderName || ("profile" + targetNum + "_" + profilePropertiesWindow.profName)
                profilePropertiesWindow.profBgColor = obj.bgColor || "#38bdf8"
                profilePropertiesWindow.profHasKeydownColor = Boolean(obj.hasKeydownColor)
                profilePropertiesWindow.profKeydownColor = obj.keydownColor || "#ffffff"
                profilePropertiesWindow.profDimUnusedKeys = obj.dimUnusedKeys !== undefined ? Boolean(obj.dimUnusedKeys) : true
                profilePropertiesWindow.profIsLandscape = Boolean(obj.isLandscape)
                profilePropertiesWindow.profOledRotation = obj.oledRotation !== undefined
                    ? Number(obj.oledRotation)
                    : (obj.isLandscape ? 90 : 0)
                profilePropertiesWindow.profWidgetRotation = obj.widgetRotation !== undefined
                    ? Number(obj.widgetRotation)
                    : -1
                profilePropertiesWindow.profIsUpperHs = Boolean(obj.isUpperHs)
                profilePropertiesWindow.profExtraConfigLines = Array.isArray(obj.extraConfigLines) ? obj.extraConfigLines.join("\n") : ""
                profilePropertiesWindow.profConfiguredSlots = obj.configuredSlotCount || 0
                const wh = obj.webhook || {}
                profilePropertiesWindow.profWebhookEnabled = Boolean(wh.enabled)
                profilePropertiesWindow.profWebhookMethod = wh.method || "POST"
                profilePropertiesWindow.profWebhookUrl = wh.url || ""
                profilePropertiesWindow.profWebhookHeaders = wh.headers !== undefined ? String(wh.headers) : "Content-Type: application/json"
                profilePropertiesWindow.profWebhookBody = wh.body !== undefined
                    ? String(wh.body)
                    : "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"source\":\"{{source}}\"}"
            } catch (e) {
                profilePropertiesWindow.profName = deviceController.activeProfileName
                profilePropertiesWindow.profBgColor = deviceController.activeProfileBgColor
                profilePropertiesWindow.profOledRotation = 0
                profilePropertiesWindow.profWidgetRotation = -1
                profilePropertiesWindow.profWebhookEnabled = false
                profilePropertiesWindow.profWebhookMethod = "POST"
                profilePropertiesWindow.profWebhookUrl = ""
            }
            profNameField.text = profilePropertiesWindow.profName
            profBgColorField.text = profilePropertiesWindow.profBgColor
            profKeydownEnableCheck.checked = profilePropertiesWindow.profHasKeydownColor
            profKeydownColorField.text = profilePropertiesWindow.profKeydownColor
            profDimUnusedCheck.checked = profilePropertiesWindow.profDimUnusedKeys
            profUpperHsCheck.checked = profilePropertiesWindow.profIsUpperHs
            profExtraLinesArea.text = profilePropertiesWindow.profExtraConfigLines
            profWebhookEnableCheck.checked = profilePropertiesWindow.profWebhookEnabled
            const mIdx = profWebhookMethodCombo.indexOfValue(profilePropertiesWindow.profWebhookMethod.toUpperCase())
            profWebhookMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 1
            profWebhookUrlField.text = profilePropertiesWindow.profWebhookUrl
            profWebhookHeadersArea.text = profilePropertiesWindow.profWebhookHeaders
            profWebhookBodyArea.text = profilePropertiesWindow.profWebhookBody
            profilePropertiesWindow.webhookTestStatus = ""
            profilePropertiesWindow.statusFeedback = ""

            profilePropertiesWindow.show()
            profilePropertiesWindow.raise()
            profilePropertiesWindow.requestActivate()
        }

        function saveAllProfileProperties() {
            const extraArr = profExtraLinesArea.text
                .split("\n")
                .map(s => s.trim())
                .filter(s => s.length > 0)
            const payload = {
                name: profNameField.text.trim(),
                bgColor: profBgColorField.text.trim(),
                hasKeydownColor: profKeydownEnableCheck.checked,
                keydownColor: profKeydownColorField.text.trim(),
                dimUnusedKeys: profDimUnusedCheck.checked,
                isLandscape: profilePropertiesWindow.profOledRotation === 90 || profilePropertiesWindow.profOledRotation === 270,
                oledRotation: profilePropertiesWindow.profOledRotation,
                widgetRotation: profilePropertiesWindow.profWidgetRotation,
                isUpperHs: profUpperHsCheck.checked,
                extraConfigLines: extraArr,
                webhook: {
                    enabled: profWebhookEnableCheck.checked,
                    method: profWebhookMethodCombo.currentText || "POST",
                    url: profWebhookUrlField.text.trim(),
                    headers: profWebhookHeadersArea.text,
                    body: profWebhookBodyArea.text
                }
            }
            const ok = deviceController.saveProfilePropertiesJson(
                profilePropertiesWindow.profNumber,
                JSON.stringify(payload)
            )
            if (ok) {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                oledDesktopWidget.syncFromController()
                profilePropertiesWindow.statusFeedback = qsTr("✓ Saved all properties for Profile #%1.").arg(profilePropertiesWindow.profNumber)
            }
            return ok
        }

        Connections {
            target: deviceController
            function onProfilesJsonChanged() {
                if (!profilePropertiesWindow.visible || profilePropertiesWindow.profNumber <= 0) return
                try {
                    const obj = JSON.parse(deviceController.getProfilePropertiesJson(profilePropertiesWindow.profNumber))
                    if (obj.oledRotation !== undefined) {
                        profilePropertiesWindow.profOledRotation = Number(obj.oledRotation)
                    }
                    if (obj.widgetRotation !== undefined) {
                        profilePropertiesWindow.profWidgetRotation = Number(obj.widgetRotation)
                    }
                } catch (e) {}
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: profilePropertiesWindow.profBgColor
                    border.color: "#ffffff66"
                    border.width: 1
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Profile #%1 Properties").arg(profilePropertiesWindow.profNumber)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Folder: %1  •  Configured Slots: %2")
                            .arg(profilePropertiesWindow.profFolderName)
                            .arg(profilePropertiesWindow.profConfiguredSlots)
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        wrapMode: Text.WrapAnywhere
                    }
                }
            }

            Flickable {
                id: profPropsFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: profPropsScrollCol.implicitHeight + 12
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: profPropsFlickable
                    anchors.top: profPropsFlickable.top
                    anchors.right: profPropsFlickable.right
                    anchors.bottom: profPropsFlickable.bottom
                    policy: profPropsFlickable.contentHeight > profPropsFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: profPropsScrollCol
                    width: Math.max(
                        200,
                        profPropsFlickable.width - (profPropsFlickable.contentHeight > profPropsFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // Identity & Colors Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profBasicCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profBasicCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                QQC2.Label {
                                    text: qsTr("Profile Name (max 13 chars):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 195
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: profNameField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 13
                                    selectByMouse: true
                                    font.bold: true
                                    placeholderText: qsTr("Profile Name")
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Default Key Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    width: 195
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                Rectangle {
                                    width: 26
                                    height: 26
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(profBgColorField.text.trim())
                                        ? profBgColorField.text.trim()
                                        : profilePropertiesWindow.profBgColor
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: profBgColorField
                                    width: 100
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            profilePropertiesWindow.profBgColor = text.trim()
                                        }
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "color-picker"
                                    text: qsTr("Open Color Picker...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        duckyColorPickerWindow.openForProfileBg(
                                            profilePropertiesWindow.profNumber,
                                            profNameField.text.trim(),
                                            profBgColorField.text.trim()
                                        )
                                    }
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Keydown Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    width: 195
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                HighContrastCheckBox {
                                    font.pointSize: 10 * root.fontScale
                                    id: profKeydownEnableCheck
                                    text: qsTr("Enable Custom Keydown Color")
                                    onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                }

                                Rectangle {
                                    visible: profKeydownEnableCheck.checked
                                    width: 26
                                    height: 26
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(profKeydownColorField.text.trim())
                                        ? profKeydownColorField.text.trim()
                                        : "#ffffff"
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: profKeydownColorField
                                    visible: profKeydownEnableCheck.checked
                                    width: 100
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onEditingFinished: profilePropertiesWindow.saveAllProfileProperties()
                                }
                            }
                        }
                    }

                    // Hardware Flags & Orientation Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profFlagsCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profFlagsCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Hardware LED & Orientation Directives (config.txt):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            HighContrastCheckBox {
                                id: profDimUnusedCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Dim Unused Keys (DIM_UNUSED_KEYS 1 — turn off LEDs on unconfigured keys)")
                                onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                contentItem: QQC2.Label {
                                    leftPadding: (profDimUnusedCheck.indicator ? profDimUnusedCheck.indicator.width : 18) + profDimUnusedCheck.spacing
                                    text: profDimUnusedCheck.text
                                    font: profDimUnusedCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.Label {
                                    text: qsTr("OLED Orientation (Physical Screen):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                Flow {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("0° (Portrait)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 0
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 0
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("90° (Landscape CW)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 90
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 90
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("180° (Inverted)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 180
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 180
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("270° (Landscape CCW)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 270
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 270
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.Label {
                                    text: qsTr("💻 Desktop Widget Orientation (Instant — No microSD Sync Required):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    QQC2.ToolTip.visible: dwRotHover.hovered
                                    QQC2.ToolTip.text: qsTr("💻 Host App Only: Configure the Desktop OLED Widget orientation separately from the physical hardware OLED screen. Applies immediately without requiring a microSD card sync.")
                                    HoverHandler { id: dwRotHover }
                                }

                                Flow {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("Follow OLED (%1°)").arg(profilePropertiesWindow.profOledRotation)
                                        highlighted: profilePropertiesWindow.profWidgetRotation < 0
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("0°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 0
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 0
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("90°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 90
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 90
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("180°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 180
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 180
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        text: qsTr("270°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 270
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 270
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                text: qsTr("Note: Physical OLED 180° & 270° require DuckyDeck Studio Firmware v1.0.0+. Desktop Widget Orientation (💻) is a host-only setting that updates the on-screen Desktop OLED Widget immediately without requiring a microSD card sync.")
                            }

                            HighContrastCheckBox {
                                id: profUpperHsCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Inverted / Upper Hot-Swap Orientation (IS_UPPER_HS 1)")
                                onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                contentItem: QQC2.Label {
                                    leftPadding: (profUpperHsCheck.indicator ? profUpperHsCheck.indicator.width : 18) + profUpperHsCheck.spacing
                                    text: profUpperHsCheck.text
                                    font: profUpperHsCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Related Profile Features Quick Access
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profQuickCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profQuickCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("Profile Integrations:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("✨ Profile RGB Lighting FX...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        lightingAnimatorDialog.openFor("profile", 0)
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "bookmark-new"
                                    text: qsTr("Save as Template...")
                                    onClicked: {
                                        profileTemplatesDialog.openSaveFromProfile(
                                            profilePropertiesWindow.profNumber,
                                            profNameField.text.trim()
                                        )
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "crosshairs"
                                    text: qsTr("Bind Window...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        deviceController.armWindowPicker(true)
                                    }
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "preferences-system-windows"
                                    text: qsTr("Auto-Switch Rules...")
                                    onClicked: autoSwitchDrawer.open()
                                }
                            }
                        }
                    }

                    // Per-Profile Change Webhook Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profWebhookCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: profWebhookEnableCheck.checked
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profWebhookCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("🔔 Profile Change Webhook (On Profile Activated)")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "network-transmit-receive"
                                    text: deviceController.webApiTestBusy ? qsTr("Testing...") : qsTr("Test Webhook")
                                    enabled: !deviceController.webApiTestBusy && profWebhookUrlField.text.trim().length > 0
                                    onClicked: {
                                        deviceController.testProfileWebhookJson(
                                            JSON.stringify({
                                                enabled: true,
                                                method: profWebhookMethodCombo.currentText || "POST",
                                                url: profWebhookUrlField.text.trim(),
                                                headers: profWebhookHeadersArea.text,
                                                body: profWebhookBodyArea.text
                                            }),
                                            profilePropertiesWindow.profNumber
                                        )
                                        profilePropertiesWindow.webhookTestStatus = qsTr("⏳ Testing webhook → %1 (see status bar / event log)").arg(profWebhookUrlField.text.trim())
                                    }
                                }
                            }

                            HighContrastCheckBox {
                                id: profWebhookEnableCheck
                                Layout.fillWidth: true
                                text: qsTr("Trigger this HTTP Webhook whenever Profile #%1 becomes active (UI, Hardware, Auto-Switch, or REST API)").arg(profilePropertiesWindow.profNumber)
                                contentItem: QQC2.Label {
                                    leftPadding: (profWebhookEnableCheck.indicator ? profWebhookEnableCheck.indicator.width : 18) + profWebhookEnableCheck.spacing
                                    text: profWebhookEnableCheck.text
                                    font: profWebhookEnableCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.ComboBox {
                                    font.pointSize: 10 * root.fontScale
                                    id: profWebhookMethodCombo
                                    Layout.preferredWidth: 105
                                    model: ["GET", "POST", "PUT", "PATCH", "DELETE"]
                                    currentIndex: 1
                                }

                                QQC2.TextField {
                                    id: profWebhookUrlField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    placeholderText: qsTr("http://homeassistant.local:8123/api/webhook/duckypad_profile")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Headers (Optional, one 'Header-Name: Value' per line):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: profWebhookHeadersArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 46
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                placeholderText: qsTr("Content-Type: application/json\nAuthorization: Bearer <token>")
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Payload Body (Template placeholders: {{profile_number}}, {{profile_name}}, {{previous_profile_number}}, {{previous_profile_name}}, {{source}}, {{device_serial}}, {{timestamp}}):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: profWebhookBodyArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 58
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                placeholderText: qsTr("{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\"}")
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                visible: profilePropertiesWindow.webhookTestStatus.length > 0
                                text: profilePropertiesWindow.webhookTestStatus
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: profilePropertiesWindow.webhookTestStatus.startsWith("✓")
                                    ? Kirigami.Theme.positiveTextColor
                                    : Kirigami.Theme.neutralTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }
                        }
                    }

                    // Extra config.txt lines
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        QQC2.Label {
                            text: qsTr("Additional Custom config.txt Directives (Optional, one per line):")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: profExtraLinesArea
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 68
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// Optional custom config.txt directives")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }

                    QQC2.Label {
                        visible: profilePropertiesWindow.statusFeedback.length > 0
                        text: profilePropertiesWindow.statusFeedback
                        color: Kirigami.Theme.positiveTextColor
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        if (profilePropertiesWindow.saveAllProfileProperties()) {
                            profilePropertiesWindow.close()
                        }
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: profilePropertiesWindow.close()
                }
            }
        }
    }

    // =========================================================================
    // Key / Slot Properties Window (Editable Fields for All Key Properties)
    // =========================================================================
    Window {
        id: keyPropertiesWindow
        property alias keyPropLabel1Field: keyPropLabel1Field
        property alias keyPropLabel2Field: keyPropLabel2Field
        property alias keyPropColorField: keyPropColorField
        property alias keyPropPressScriptArea: keyPropPressScriptArea
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: keyPropertiesWindow.visible
            onActivated: keyPropertiesWindow.close()
        }
        title: qsTr("Key Properties — Slot %1 (%2)").arg(keyPropertiesWindow.slotNum).arg(keyPropertiesWindow.slotTitle)
        width: 780
        height: 680
        minimumWidth: 660
        minimumHeight: 540
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property int slotNum: 1
        property string slotTitle: "Key 1"
        property string slotColor: "#38bdf8"

        function openForSlot(slotNumber) {
            const targetSlot = slotNumber > 0 ? slotNumber : root.selectedSlotNumber
            if (targetSlot <= 0 || targetSlot > 58) return

            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.flushPendingSlotEditsIfModified()
            }

            keyPropertiesWindow.slotNum = targetSlot
            keyPropertiesWindow.slotTitle = hardwareCanvas.slotTitleFor(targetSlot)
            keyPropertiesWindow.slotColor = String(hardwareCanvas.slotColorFor(targetSlot))

            let l1 = ""
            let l2 = ""
            let wl1 = ""
            let wl2 = ""
            let allowAbort = false
            let disableRepeat = false
            try {
                const metaJson = deviceController.getSlotMetaJson(targetSlot)
                const meta = JSON.parse(metaJson)
                l1 = meta.l1 || ""
                l2 = meta.l2 || ""
                wl1 = meta.wl1 || ""
                wl2 = meta.wl2 || ""
                allowAbort = !!meta.allowAbort
                disableRepeat = !!meta.disableAutorepeat
            } catch (e) {}

            keyPropLabel1Field.text = root.clampOledLabel(l1)
            keyPropLabel2Field.text = root.clampOledLabel(l2)
            keyPropWidgetLabel1Field.text = wl1
            keyPropWidgetLabel2Field.text = wl2
            keyPropColorField.text = keyPropertiesWindow.slotColor
            keyPropDontRepeatCheck.checked = disableRepeat
            keyPropAllowAbortCheck.checked = allowAbort
            keyPropPressScriptArea.text = deviceController.getSlotScript(targetSlot, false)
            keyPropReleaseScriptArea.text = deviceController.getSlotScript(targetSlot, true)

            keyPropertiesWindow.show()
            keyPropertiesWindow.raise()
            keyPropertiesWindow.requestActivate()
        }

        function applyKeyProperties() {
            const targetSlot = keyPropertiesWindow.slotNum
            if (targetSlot <= 0 || targetSlot > 58) return

            const l1 = root.clampOledLabel(keyPropLabel1Field.text.trim())
            const l2 = root.clampOledLabel(keyPropLabel2Field.text.trim())
            const wl1 = keyPropWidgetLabel1Field.text.trim()
            const wl2 = keyPropWidgetLabel2Field.text.trim()
            const disableRepeat = keyPropDontRepeatCheck.checked
            const allowAbort = keyPropAllowAbortCheck.checked
            const pressScript = keyPropPressScriptArea.text
            const releaseScript = keyPropReleaseScriptArea.text

            const hexCandidate = keyPropColorField.text.trim()
            if (targetSlot <= 20 && /^#[0-9A-Fa-f]{6}$/.test(hexCandidate)) {
                const rVal = parseInt(hexCandidate.substring(1, 3), 16)
                const gVal = parseInt(hexCandidate.substring(3, 5), 16)
                const bVal = parseInt(hexCandidate.substring(5, 7), 16)
                if (hexCandidate.toLowerCase() !== keyPropertiesWindow.slotColor.toLowerCase()) {
                    hardwareCanvas.setKeyRgbAt(targetSlot, rVal, gVal, bVal)
                }
                keyPropertiesWindow.slotColor = hexCandidate
            }

            deviceController.saveSlotConfig(
                targetSlot,
                l1,
                l2,
                pressScript,
                releaseScript,
                allowAbort,
                disableRepeat
            )
            deviceController.saveSlotWidgetLabels(
                targetSlot,
                wl1,
                wl2
            )

            hardwareCanvas.syncSlotsFromController()

            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.loadSlotFromController(
                    targetSlot,
                    hardwareCanvas.slotTitleFor(targetSlot),
                    hardwareCanvas.slotColorFor(targetSlot)
                )
            }
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
                    color: keyPropertiesWindow.slotColor
                    border.color: "#ffffff66"
                    border.width: 1
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Key Properties — %1 (Slot %2)").arg(keyPropertiesWindow.slotTitle).arg(keyPropertiesWindow.slotNum)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Profile #%1 (%2) • Edit OLED labels, Desktop Widget alternate labels, per-key RGB color, execution flags, and scripts.")
                            .arg(deviceController.activeProfile)
                            .arg(deviceController.activeProfileName)
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }
                }
            }

            Flickable {
                id: keyPropsFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: keyPropsScrollCol.implicitHeight + 8
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: keyPropsFlickable
                    anchors.top: keyPropsFlickable.top
                    anchors.right: keyPropsFlickable.right
                    anchors.bottom: keyPropsFlickable.bottom
                    policy: keyPropsFlickable.contentHeight > keyPropsFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: keyPropsScrollCol
                    width: Math.max(
                        200,
                        keyPropsFlickable.width - (keyPropsFlickable.contentHeight > keyPropsFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // OLED Labels & Per-Key RGB Color Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyBasicGrid.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        GridLayout {
                            id: keyBasicGrid
                            anchors.fill: parent
                            anchors.margins: 9
                            columns: 2
                            columnSpacing: 12
                            rowSpacing: 8

                            QQC2.Label {
                                text: qsTr("OLED Line 1 (max 5 chars):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: keyPropLabel1Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 10
                                    font.family: "monospace"
                                    font.bold: true
                                    selectByMouse: true
                                    placeholderText: qsTr("Top line (max 5 chars)")
                                    onTextChanged: {
                                        const clamped = root.clampOledLabel(text)
                                        if (text !== clamped) {
                                            text = clamped
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("%1/5").arg(root.oledLabelCharCount(keyPropLabel1Field.text))
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("★ Icon...")
                                    font.bold: true
                                    onClicked: oledIconPickerDialog.openForKeyProperties(1)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Line 1")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("OLED Line 2 (max 5 chars):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: keyPropLabel2Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 10
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Bottom line (max 5 chars, optional)")
                                    onTextChanged: {
                                        const clamped = root.clampOledLabel(text)
                                        if (text !== clamped) {
                                            text = clamped
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("%1/5").arg(root.oledLabelCharCount(keyPropLabel2Field.text))
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("★ Icon...")
                                    font.bold: true
                                    onClicked: oledIconPickerDialog.openForKeyProperties(2)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Line 2")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Desktop Widget Label L1 (optional):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: keyPropWidgetLabel1Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 24
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Longer label for Desktop OLED Widget (up to 24 chars, falls back to OLED L1)")
                                }

                                QQC2.Label {
                                    text: qsTr("%1/24").arg(keyPropWidgetLabel1Field.text.length)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Desktop Widget Label L2 (optional):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: keyPropWidgetLabel2Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 24
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Longer bottom line for Desktop OLED Widget (up to 24 chars, optional)")
                                }

                                QQC2.Label {
                                    text: qsTr("%1/24").arg(keyPropWidgetLabel2Field.text.length)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Key RGB Color (SWCOLOR):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(keyPropColorField.text.trim())
                                        ? keyPropColorField.text.trim()
                                        : keyPropertiesWindow.slotColor
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    font.pointSize: 10 * root.fontScale
                                    id: keyPropColorField
                                    Layout.preferredWidth: 96
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "color-picker"
                                    text: qsTr("Color Picker...")
                                    onClicked: duckyColorPickerWindow.openForKey(keyPropertiesWindow.slotNum, keyPropColorField.text.trim())
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    icon.name: "edit-clear"
                                    text: qsTr("Use Profile Default")
                                    onClicked: {
                                        deviceController.clearKeyColor(keyPropertiesWindow.slotNum)
                                        hardwareCanvas.syncSlotsFromController()
                                        const defCol = hardwareCanvas.slotColorFor(keyPropertiesWindow.slotNum)
                                        keyPropertiesWindow.slotColor = defCol
                                        keyPropColorField.text = defCol
                                        if (root.selectedSlotNumber === keyPropertiesWindow.slotNum) {
                                            root.selectedSlotColor = defCol
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }
                        }
                    }

                    // Execution Behavior Flags Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyFlagsCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: keyFlagsCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("Hardware Execution Flags (config.txt):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            HighContrastCheckBox {
                                id: keyPropDontRepeatCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("No Repeat on Key Held Down (dr%1 — run once per physical press instead of auto-repeating)").arg(keyPropertiesWindow.slotNum)
                                contentItem: QQC2.Label {
                                    leftPadding: (keyPropDontRepeatCheck.indicator ? keyPropDontRepeatCheck.indicator.width : 18) + keyPropDontRepeatCheck.spacing
                                    text: keyPropDontRepeatCheck.text
                                    font: keyPropDontRepeatCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            HighContrastCheckBox {
                                id: keyPropAllowAbortCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Allow Early Abort (ab%1 — pressing any key while this macro runs terminates it immediately)").arg(keyPropertiesWindow.slotNum)
                                contentItem: QQC2.Label {
                                    leftPadding: (keyPropAllowAbortCheck.indicator ? keyPropAllowAbortCheck.indicator.width : 18) + keyPropAllowAbortCheck.spacing
                                    text: keyPropAllowAbortCheck.text
                                    font: keyPropAllowAbortCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Quick Action Windows Row
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyQuickWinsRow.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        RowLayout {
                            id: keyQuickWinsRow
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                                text: qsTr("✨ Key RGB FX...")
                                onClicked: lightingAnimatorDialog.openFor("key", keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                text: qsTr("🚀 Launch App Window...")
                                onClicked: appLaunchConfigWindow.openWindow(keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                text: qsTr("🌐 Web API Window...")
                                onClicked: webApiKeyConfigWindow.openWindow(keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                text: qsTr("🔴 Record Macro...")
                                onClicked: macroRecorderDialog.openRecorder()
                            }

                            Item { Layout.fillWidth: true }
                        }
                    }

                    // On Press & On Release Scripts
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        QQC2.Label {
                            text: qsTr("⬇ On Press DuckyScript 3 (key%1.txt):").arg(keyPropertiesWindow.slotNum)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: keyPropPressScriptArea
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 110
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// On Press DuckyScript 3 commands")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }

                        QQC2.Label {
                            visible: keyPropertiesWindow.slotNum <= 20
                            text: qsTr("⬆ On Release DuckyScript 3 (key%1-release.txt):").arg(keyPropertiesWindow.slotNum)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: keyPropReleaseScriptArea
                            visible: keyPropertiesWindow.slotNum <= 20
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 80
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// Optional On Release DuckyScript 3 commands")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        keyPropertiesWindow.applyKeyProperties()
                        keyPropertiesWindow.close()
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: keyPropertiesWindow.close()
                }
            }
        }
    }

    readonly property bool closableWithEscape: true
    title: qsTr("Settings — DuckyDeck Studio")
    readonly property int availScreenHeight: (root.screen && root.screen.desktopAvailableHeight > 0)
        ? root.screen.desktopAvailableHeight
        : (Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight : 960)
    readonly property int targetFullHeight: Math.max(720, settingsContentCol.implicitHeight + 132)
    width: 780
    height: Math.min(availScreenHeight - 48, targetFullHeight)
    minimumWidth: 560
    minimumHeight: 440
    visible: false
    transientParent: root
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor


        function open() {
            restApiSettingsSection.resetAndSync()
            settingsDialog.height = Math.min(settingsDialog.availScreenHeight - 48, settingsDialog.targetFullHeight)
            settingsDialog.show()
            settingsDialog.raise()
            settingsDialog.requestActivate()
        }

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: settingsDialog.visible
        onActivated: settingsDialog.close()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Kirigami.Heading {
                level: 2
                text: qsTr("Application Settings")
                color: Kirigami.Theme.textColor
                Layout.fillWidth: true
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                text: "−"
                implicitWidth: 32
                font.bold: true
                enabled: deviceController.uiTextScale > 75
                onClicked: root.zoomOut()
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Zoom Out (Ctrl+-)")
            }

            QQC2.Label {
                text: qsTr("Zoom: %1%").arg(deviceController.uiTextScale)
                font.family: "monospace"
                font.bold: true
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.textColor
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                text: "+"
                implicitWidth: 32
                font.bold: true
                enabled: deviceController.uiTextScale < 160
                onClicked: root.zoomIn()
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Zoom In (Ctrl++ / Ctrl+=)")
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "edit-reset"
                text: qsTr("Reset Zoom (100%)")
                enabled: deviceController.uiTextScale !== 100
                onClicked: root.resetZoom()
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Reset Zoom to 100% (Ctrl+0)")
            }
        }

        Flickable {
            id: settingsFlickable
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            contentWidth: width
            contentHeight: settingsContentCol.implicitHeight + 12
            boundsBehavior: Flickable.StopAtBounds
            flickableDirection: Flickable.VerticalFlick

            QQC2.ScrollBar.vertical: HighContrastScrollBar {
                id: settingsVerticalScrollBar
                parent: settingsFlickable
                anchors.top: settingsFlickable.top
                anchors.right: settingsFlickable.right
                anchors.bottom: settingsFlickable.bottom
                policy: settingsFlickable.contentHeight > settingsFlickable.height
                    ? QQC2.ScrollBar.AlwaysOn
                    : QQC2.ScrollBar.AsNeeded
            }

            ColumnLayout {
                id: settingsContentCol
                width: Math.max(
                    480,
                    settingsFlickable.width - (settingsFlickable.contentHeight > settingsFlickable.height ? 18 : 2)
                )
                spacing: 12

                    RestApiSettings {
                        id: restApiSettingsSection
                        Layout.fillWidth: true
                        controller: settingsDialog.controller
                        fontScale: settingsDialog.fontScale
                        highContrastPositiveText: settingsDialog.highContrastPositiveText
                        highContrastWarningText: settingsDialog.highContrastWarningText
                    }

                // Section 3: Appearance & System Integration
                Rectangle {
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    Layout.fillWidth: true
                    implicitHeight: themeCol.implicitHeight + 20
                    radius: 8
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1

                    ColumnLayout {
                        id: themeCol
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 10

                        QQC2.Label {
                            text: qsTr("APPEARANCE & SYSTEM INTEGRATION")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }

                        GridLayout {
                            Layout.fillWidth: true
                            columns: 2
                            rowSpacing: 8
                            columnSpacing: 12

                            QQC2.Label {
                                text: qsTr("Application Theme:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.textColor
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("◐ System")
                                    checkable: true
                                    checked: deviceController.appThemeMode === "system"
                                    highlighted: checked
                                    onClicked: deviceController.applyAppThemeMode("system")
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("☀ Light")
                                    checkable: true
                                    checked: deviceController.appThemeMode === "light"
                                    highlighted: checked
                                    onClicked: deviceController.applyAppThemeMode("light")
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("🌙 Dark")
                                    checkable: true
                                    checked: deviceController.appThemeMode === "dark"
                                    highlighted: checked
                                    onClicked: deviceController.applyAppThemeMode("dark")
                                }

                                Item { Layout.fillWidth: true }
                            }

                            QQC2.Label {
                                text: qsTr("Taskbar Tray Icon:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.textColor
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("◐ System")
                                    checkable: true
                                    checked: deviceController.trayIconMode === "auto"
                                    highlighted: checked
                                    onClicked: deviceController.applyTrayIconMode("auto")
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("☀ Light Panel")
                                    checkable: true
                                    checked: deviceController.trayIconMode === "light"
                                    highlighted: checked
                                    onClicked: deviceController.applyTrayIconMode("light")
                                }

                                QQC2.Button {
                                    font.pointSize: 10 * root.fontScale
                                    text: qsTr("🌙 Dark Panel")
                                    checkable: true
                                    checked: deviceController.trayIconMode === "dark"
                                    highlighted: checked
                                    onClicked: deviceController.applyTrayIconMode("dark")
                                }

                                Item { Layout.fillWidth: true }
                            }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            height: 1
                            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            HighContrastCheckBox {
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                text: qsTr("Start on KDE System Login (Autostart, Minimized to Tray)")
                                checked: deviceController.autostartEnabled
                                onToggled: deviceController.applyAutostartEnabled(checked)
                            }

                            HighContrastCheckBox {
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                text: qsTr("Keep Running in Taskbar on Window Close")
                                checked: deviceController.closeToTray
                                onToggled: deviceController.setCloseToTrayEnabled(checked)
                            }
                        }
                    }
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Item { Layout.fillWidth: true }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "window-close"
                text: qsTr("Close")
                highlighted: true
                onClicked: settingsDialog.close()
            }
        }
    }
}
