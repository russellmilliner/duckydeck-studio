import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Window {
    id: drawerRoot
    title: qsTr("Auto-Switcher Rules")
    width: Math.min(840, Screen.desktopAvailableWidth - 80)
    height: Math.min(780, Screen.desktopAvailableHeight - 80)
    minimumWidth: 660
    minimumHeight: 540
    visible: false
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

    property var controller: null
    property var profileChoices: []
    property var profileNumbers: []
    property var fallbackChoices: [qsTr("None (Stay on active profile)")]
    property var fallbackNumbers: [0]
    property int editingRuleIndex: -1
    property string ruleFilterText: ""
    property int totalRuleCount: 0
    property string previousSpecificClass: ""
    property int settleMsValue: controller ? controller.autoSwitchSettleMs : 250
    property int dwellMsValue: controller ? controller.autoSwitchDwellMs : 150

    property int contextMenuRuleIndex: -1
    property string contextMenuResourceClass: ""
    property string contextMenuTitleContains: ""
    property string contextMenuFaviconUrl: ""
    property int contextMenuTargetProfile: 1
    property bool contextMenuRuleEnabled: true

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor
    readonly property color subtleBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

    ListModel {
        id: rulesModel
    }

    function applyTimings(newSettleMs, newDwellMs) {
        const clampedSettle = Math.max(0, Math.min(5000, Math.round(newSettleMs)))
        const clampedDwell = Math.max(0, Math.min(5000, Math.round(newDwellMs)))
        drawerRoot.settleMsValue = clampedSettle
        drawerRoot.dwellMsValue = clampedDwell
        if (mouseSettleInput.text !== String(clampedSettle)) {
            mouseSettleInput.text = String(clampedSettle)
        }
        if (windowDwellInput.text !== String(clampedDwell)) {
            windowDwellInput.text = String(clampedDwell)
        }
        if (drawerRoot.controller) {
            drawerRoot.controller.setAutoSwitchTimings(clampedSettle, clampedDwell)
        }
    }

    function loadRuleIntoEditor(rIdx, resClass, titleSubstr, favUrl, targetProf) {
        drawerRoot.editingRuleIndex = rIdx
        classInput.text = resClass
        titleInput.text = titleSubstr
        faviconUrlInput.text = favUrl || ""
        const pIdx = drawerRoot.profileNumbers.indexOf(targetProf)
        if (pIdx >= 0) {
            targetProfileCombo.currentIndex = pIdx
        }
        classInput.forceActiveFocus()
    }

    QQC2.Menu {
        id: ruleContextMenu
        modal: true

        QQC2.MenuItem {
            enabled: false
            text: drawerRoot.contextMenuResourceClass.toLowerCase() === "*browser*"
                ? qsTr("Rule #%1 — All Web Browsers").arg(drawerRoot.contextMenuRuleIndex + 1)
                : qsTr("Rule #%1 — %2").arg(drawerRoot.contextMenuRuleIndex + 1).arg(drawerRoot.contextMenuResourceClass)
            font.bold: true
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-edit"
            text: qsTr("Edit Rule")
            onTriggered: {
                drawerRoot.loadRuleIntoEditor(
                    drawerRoot.contextMenuRuleIndex,
                    drawerRoot.contextMenuResourceClass,
                    drawerRoot.contextMenuTitleContains,
                    drawerRoot.contextMenuFaviconUrl,
                    drawerRoot.contextMenuTargetProfile
                )
            }
        }

        QQC2.MenuItem {
            icon.name: drawerRoot.contextMenuRuleEnabled ? "view-hidden" : "view-visible"
            text: drawerRoot.contextMenuRuleEnabled ? qsTr("Disable Rule") : qsTr("Enable Rule")
            onTriggered: {
                if (drawerRoot.controller && drawerRoot.contextMenuRuleIndex >= 0) {
                    drawerRoot.controller.toggleAutoSwitchRule(
                        drawerRoot.contextMenuRuleIndex,
                        !drawerRoot.contextMenuRuleEnabled
                    )
                }
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "edit-delete"
            text: qsTr("Delete Rule")
            onTriggered: {
                if (drawerRoot.controller && drawerRoot.contextMenuRuleIndex >= 0) {
                    if (drawerRoot.editingRuleIndex === drawerRoot.contextMenuRuleIndex) {
                        drawerRoot.editingRuleIndex = -1
                    }
                    drawerRoot.controller.removeAutoSwitchRule(drawerRoot.contextMenuRuleIndex)
                }
            }
        }
    }

    function open() {
        syncFromController()
        drawerRoot.show()
        drawerRoot.raise()
        drawerRoot.requestActivate()
    }

    function syncFromController() {
        if (!controller) return
        const savedScrollY = rulesListView ? rulesListView.contentY : 0

        drawerRoot.settleMsValue = controller.autoSwitchSettleMs
        drawerRoot.dwellMsValue = controller.autoSwitchDwellMs
        if (!mouseSettleInput.activeFocus) {
            mouseSettleInput.text = String(drawerRoot.settleMsValue)
        }
        if (!windowDwellInput.activeFocus) {
            windowDwellInput.text = String(drawerRoot.dwellMsValue)
        }

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
            drawerRoot.totalRuleCount = rArr.length
            const q = drawerRoot.ruleFilterText.trim().toLowerCase()
            const nextItems = []
            for (let i = 0; i < rArr.length; ++i) {
                const r = rArr[i]
                const rIdx = r.index !== undefined ? r.index : i
                const resClass = r.resourceClass || ""
                const titleSub = r.titleContains || ""
                const favUrl = r.faviconUrl || ""
                const tProf = r.targetProfile || 1
                const tProfName = r.targetProfileName || "Default"

                if (q.length > 0) {
                    const browserKeywords = resClass.toLowerCase() === "*browser*"
                        ? "all web browsers chrome zen firefox edge brave opera vivaldi chromium"
                        : ""
                    const haystack = (
                        "#" + (rIdx + 1) + " " +
                        resClass + " " +
                        browserKeywords + " " +
                        titleSub + " " +
                        favUrl + " " +
                        "p" + tProf + " #" + tProf + " " + tProf + " " +
                        tProfName
                    ).toLowerCase()
                    if (haystack.indexOf(q) === -1) {
                        continue
                    }
                }

                nextItems.push({
                    ruleIndex: rIdx,
                    ruleEnabled: !!r.enabled,
                    resourceClass: resClass,
                    titleContains: titleSub,
                    faviconUrl: favUrl,
                    targetProfile: tProf,
                    targetProfileName: tProfName,
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
        function onAutoSwitchSettleMsChanged() {
            if (drawerRoot.controller) {
                drawerRoot.settleMsValue = drawerRoot.controller.autoSwitchSettleMs
                if (!mouseSettleInput.activeFocus) {
                    mouseSettleInput.text = String(drawerRoot.settleMsValue)
                }
            }
        }
        function onAutoSwitchDwellMsChanged() {
            if (drawerRoot.controller) {
                drawerRoot.dwellMsValue = drawerRoot.controller.autoSwitchDwellMs
                if (!windowDwellInput.activeFocus) {
                    windowDwellInput.text = String(drawerRoot.dwellMsValue)
                }
            }
        }
        function onLastPickedWindowClassChanged() {
            if (drawerRoot.controller && drawerRoot.controller.lastPickedWindowClass.length > 0) {
                classInput.text = drawerRoot.controller.lastPickedWindowClass
                titleInput.text = drawerRoot.controller.lastPickedWindowTitle
            }
        }
        function onLastPickedWindowTitleChanged() {
            if (drawerRoot.controller && drawerRoot.controller.lastPickedWindowClass.length > 0) {
                titleInput.text = drawerRoot.controller.lastPickedWindowTitle
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
            anchors.margins: 14
            spacing: 10

            // Header
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2

                    Kirigami.Heading {
                        text: qsTr("Auto-Switcher Rules")
                        level: 2
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        wrapMode: Text.WordWrap
                        text: qsTr("Automatically switches duckyPad profiles when application windows focus or after the mouse settles")
                        color: Kirigami.Theme.disabledTextColor
                        font.pixelSize: 12
                    }
                }
            }

            // Bounded Scrollable Body so nothing ever overflows vertically or horizontally
            Flickable {
                id: mainFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 120
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: Math.max(height, mainBodyCol.implicitHeight)
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick
                interactive: rulesListView.draggedRuleIndex === -1

                QQC2.ScrollBar.vertical: QQC2.ScrollBar {
                    policy: mainFlickable.contentHeight > mainFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                    implicitWidth: 10
                }

                ColumnLayout {
                    id: mainBodyCol
                    width: Math.max(
                        300,
                        mainFlickable.width - (mainFlickable.contentHeight > mainFlickable.height ? 14 : 0)
                    )
                    height: Math.max(implicitHeight, mainFlickable.height)
                    spacing: 10

                    // Global Enable + Fallback Desktop Profile + Settle Timings Card
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: globalCfgLayout.implicitHeight + 18
                        radius: 8
                        color: Kirigami.Theme.backgroundColor
                        border.color: drawerRoot.subtleBorder
                        border.width: 1

                        ColumnLayout {
                            id: globalCfgLayout
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Switch {
                                    id: masterEnableSwitch
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    checked: drawerRoot.controller ? drawerRoot.controller.autoSwitchEnabled : true
                                    text: qsTr("Enable Automatic Profile Switching on Window Focus")
                                    font.bold: true
                                    font.pixelSize: 13
                                    onToggled: {
                                        if (drawerRoot.controller) {
                                            const fbNum = drawerRoot.fallbackNumbers[fallbackCombo.currentIndex] || 0
                                            drawerRoot.controller.setAutoSwitchConfig(checked, fbNum)
                                        }
                                    }
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Fallback Profile (when no rule matches):")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.ComboBox {
                                    id: fallbackCombo
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    Layout.alignment: Qt.AlignVCenter
                                    font.pixelSize: 12
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

                            Rectangle {
                                Layout.fillWidth: true
                                height: 1
                                color: drawerRoot.subtleBorder
                            }

                            // Row 3a: Mouse Settle & Window Dwell Steppers (wide & readable)
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Mouse Settle:")
                                    font.pixelSize: 12
                                    font.bold: true
                                    color: Kirigami.Theme.textColor
                                    QQC2.ToolTip.visible: settleLabelHover.hovered
                                    QQC2.ToolTip.text: qsTr("How long the mouse cursor must stop moving inside a window before switching profiles")
                                    HoverHandler { id: settleLabelHover }
                                }

                                RowLayout {
                                    spacing: 4
                                    Layout.alignment: Qt.AlignVCenter

                                    QQC2.Button {
                                        text: "−"
                                        implicitWidth: 28
                                        implicitHeight: 30
                                        font.bold: true
                                        font.pixelSize: 14
                                        enabled: drawerRoot.settleMsValue > 0
                                        onClicked: drawerRoot.applyTimings(Math.max(0, drawerRoot.settleMsValue - 50), drawerRoot.dwellMsValue)
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Decrease Mouse Settle by 50 ms")
                                    }

                                    QQC2.TextField {
                                        id: mouseSettleInput
                                        Layout.preferredWidth: 78
                                        Layout.minimumWidth: 78
                                        implicitHeight: 30
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: 13
                                        selectByMouse: true
                                        text: String(drawerRoot.settleMsValue)
                                        validator: IntValidator { bottom: 0; top: 5000 }
                                        onEditingFinished: {
                                            const parsed = parseInt(text, 10)
                                            drawerRoot.applyTimings(isNaN(parsed) ? 0 : parsed, drawerRoot.dwellMsValue)
                                        }
                                        WheelHandler {
                                            onWheel: (event) => {
                                                const step = event.angleDelta.y > 0 ? 50 : -50
                                                drawerRoot.applyTimings(drawerRoot.settleMsValue + step, drawerRoot.dwellMsValue)
                                                event.accepted = true
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Wait for the mouse cursor to stop moving for this duration before switching profiles (0–5000 ms)")
                                    }

                                    QQC2.Button {
                                        text: "+"
                                        implicitWidth: 28
                                        implicitHeight: 30
                                        font.bold: true
                                        font.pixelSize: 14
                                        enabled: drawerRoot.settleMsValue < 5000
                                        onClicked: drawerRoot.applyTimings(Math.min(5000, drawerRoot.settleMsValue + 50), drawerRoot.dwellMsValue)
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Increase Mouse Settle by 50 ms")
                                    }

                                    QQC2.Label {
                                        text: qsTr("ms")
                                        font.pixelSize: 12
                                        color: Kirigami.Theme.disabledTextColor
                                    }
                                }

                                Item { Layout.preferredWidth: 8 }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Window Dwell:")
                                    font.pixelSize: 12
                                    font.bold: true
                                    color: Kirigami.Theme.textColor
                                    QQC2.ToolTip.visible: dwellLabelHover.hovered
                                    QQC2.ToolTip.text: qsTr("Minimum time a window must stay hovered or focused before switching profiles")
                                    HoverHandler { id: dwellLabelHover }
                                }

                                RowLayout {
                                    spacing: 4
                                    Layout.alignment: Qt.AlignVCenter

                                    QQC2.Button {
                                        text: "−"
                                        implicitWidth: 28
                                        implicitHeight: 30
                                        font.bold: true
                                        font.pixelSize: 14
                                        enabled: drawerRoot.dwellMsValue > 0
                                        onClicked: drawerRoot.applyTimings(drawerRoot.settleMsValue, Math.max(0, drawerRoot.dwellMsValue - 50))
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Decrease Window Dwell by 50 ms")
                                    }

                                    QQC2.TextField {
                                        id: windowDwellInput
                                        Layout.preferredWidth: 78
                                        Layout.minimumWidth: 78
                                        implicitHeight: 30
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: 13
                                        selectByMouse: true
                                        text: String(drawerRoot.dwellMsValue)
                                        validator: IntValidator { bottom: 0; top: 5000 }
                                        onEditingFinished: {
                                            const parsed = parseInt(text, 10)
                                            drawerRoot.applyTimings(drawerRoot.settleMsValue, isNaN(parsed) ? 0 : parsed)
                                        }
                                        WheelHandler {
                                            onWheel: (event) => {
                                                const step = event.angleDelta.y > 0 ? 50 : -50
                                                drawerRoot.applyTimings(drawerRoot.settleMsValue, drawerRoot.dwellMsValue + step)
                                                event.accepted = true
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Minimum time cursor/focus must stay in the window before switching profiles (0–5000 ms)")
                                    }

                                    QQC2.Button {
                                        text: "+"
                                        implicitWidth: 28
                                        implicitHeight: 30
                                        font.bold: true
                                        font.pixelSize: 14
                                        enabled: drawerRoot.dwellMsValue < 5000
                                        onClicked: drawerRoot.applyTimings(drawerRoot.settleMsValue, Math.min(5000, drawerRoot.dwellMsValue + 50))
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Increase Window Dwell by 50 ms")
                                    }

                                    QQC2.Label {
                                        text: qsTr("ms")
                                        font.pixelSize: 12
                                        color: Kirigami.Theme.disabledTextColor
                                    }
                                }

                                Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }
                            }

                            // Row 3b: Timing Presets Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Timing Presets:")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Instant (0 ms)")
                                    font.pixelSize: 11
                                    implicitHeight: 26
                                    highlighted: drawerRoot.settleMsValue === 0 && drawerRoot.dwellMsValue === 0
                                    onClicked: drawerRoot.applyTimings(0, 0)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Switch immediately when entering a window (0 ms settle, 0 ms dwell)")
                                }

                                QQC2.Button {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Balanced (250 / 150 ms)")
                                    font.pixelSize: 11
                                    implicitHeight: 26
                                    highlighted: drawerRoot.settleMsValue === 250 && drawerRoot.dwellMsValue === 150
                                    onClicked: drawerRoot.applyTimings(250, 150)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Default balanced settle timing (250 ms mouse settle, 150 ms window dwell)")
                                }

                                QQC2.Button {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Relaxed (500 / 300 ms)")
                                    font.pixelSize: 11
                                    implicitHeight: 26
                                    highlighted: drawerRoot.settleMsValue === 500 && drawerRoot.dwellMsValue === 300
                                    onClicked: drawerRoot.applyTimings(500, 300)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Relaxed settle timing (500 ms mouse settle, 300 ms window dwell)")
                                }

                                Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }
                            }
                        }
                    }

                    // Live Active Window Monitor + Picker Card
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: liveWinRow.implicitHeight + 16
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
                            anchors.margins: 8
                            spacing: 8

                            ColumnLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                Layout.alignment: Qt.AlignVCenter
                                spacing: 2

                                QQC2.Label {
                                    text: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                        ? qsTr("🎯 Click any open window to capture its class and title...")
                                        : qsTr("Detected Window: %1").arg(
                                            drawerRoot.controller ? drawerRoot.controller.activeWindowClass : "-"
                                          )
                                    font.bold: true
                                    font.family: "monospace"
                                    font.pixelSize: 13
                                    color: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                        ? Kirigami.Theme.highlightColor
                                        : Kirigami.Theme.textColor
                                    elide: Text.ElideRight
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                }

                                QQC2.Label {
                                    text: drawerRoot.controller && drawerRoot.controller.activeWindowTitle.length > 0
                                        ? qsTr("Title: %1").arg(drawerRoot.controller.activeWindowTitle)
                                        : qsTr("Switch focus to another app window or click 'Pick Window' to grab its class and title")
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.disabledTextColor
                                    elide: Text.ElideRight
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                }
                            }

                            QQC2.Button {
                                Layout.alignment: Qt.AlignVCenter
                                icon.name: "crosshairs"
                                text: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                    ? qsTr("Armed...")
                                    : qsTr("🎯 Pick Window")
                                font.pixelSize: 12
                                highlighted: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                onClicked: {
                                    if (drawerRoot.controller) {
                                        drawerRoot.controller.armWindowPicker(false)
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Click this button, then click any open window to fill its class and title below")
                            }

                            QQC2.Button {
                                Layout.alignment: Qt.AlignVCenter
                                icon.name: "edit-paste"
                                text: qsTr("Use Current")
                                font.pixelSize: 12
                                enabled: drawerRoot.controller
                                    && drawerRoot.controller.activeWindowClass.length > 0
                                    && drawerRoot.controller.activeWindowClass !== "Listening for KWin..."
                                onClicked: {
                                    if (drawerRoot.controller) {
                                        const cls = drawerRoot.controller.activeWindowClass
                                        const rawTitle = drawerRoot.controller.activeWindowTitle
                                        const isBrowser = drawerRoot.controller.isBrowserWindowClass(cls)
                                        if (matchAllBrowsersCheck.checked && isBrowser) {
                                            drawerRoot.previousSpecificClass = cls
                                            classInput.text = "*browser*"
                                        } else {
                                            classInput.text = cls
                                        }
                                        titleInput.text = isBrowser
                                            ? drawerRoot.controller.cleanBrowserWindowTitle(rawTitle)
                                            : rawTitle
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Copy the last detected window class and title into the rule editor below")
                            }
                        }
                    }

                    // Add / Edit Rule Form Card
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: ruleFormCol.implicitHeight + 18
                        radius: 8
                        color: Kirigami.Theme.backgroundColor
                        border.color: drawerRoot.subtleBorder
                        border.width: 1

                        ColumnLayout {
                            id: ruleFormCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            QQC2.Label {
                                text: drawerRoot.editingRuleIndex >= 0
                                    ? qsTr("Edit Rule #%1").arg(drawerRoot.editingRuleIndex + 1)
                                    : qsTr("Add New Window Rule")
                                font.bold: true
                                font.pixelSize: 13
                                color: Kirigami.Theme.textColor
                            }

                            GridLayout {
                                Layout.fillWidth: true
                                columns: 2
                                rowSpacing: 6
                                columnSpacing: 8

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Window Class:")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    spacing: 8

                                    QQC2.TextField {
                                        id: classInput
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 80
                                        Layout.preferredWidth: 1
                                        placeholderText: qsTr("e.g. firefox, chrome, zen, code, or *browser*")
                                        font.family: "monospace"
                                        font.pixelSize: 12
                                        selectByMouse: true
                                        onTextChanged: {
                                            const trimmed = text.trim()
                                            if (trimmed.length > 0 && trimmed.toLowerCase() !== "*browser*") {
                                                drawerRoot.previousSpecificClass = trimmed
                                            }
                                        }
                                    }

                                    QQC2.CheckBox {
                                        id: matchAllBrowsersCheck
                                        Layout.alignment: Qt.AlignVCenter
                                        checked: classInput.text.trim().toLowerCase() === "*browser*"
                                        text: qsTr("🌐 Match All Browsers")
                                        font.pixelSize: 11
                                        onToggled: {
                                            if (checked) {
                                                const cur = classInput.text.trim()
                                                if (cur.length > 0 && cur.toLowerCase() !== "*browser*") {
                                                    drawerRoot.previousSpecificClass = cur
                                                }
                                                classInput.text = "*browser*"
                                                if (drawerRoot.controller && titleInput.text.trim().length > 0) {
                                                    titleInput.text = drawerRoot.controller.cleanBrowserWindowTitle(titleInput.text)
                                                }
                                            } else {
                                                if (drawerRoot.previousSpecificClass.length > 0
                                                    && drawerRoot.previousSpecificClass.toLowerCase() !== "*browser*") {
                                                    classInput.text = drawerRoot.previousSpecificClass
                                                } else if (drawerRoot.controller
                                                    && drawerRoot.controller.isBrowserWindowClass(drawerRoot.controller.activeWindowClass)) {
                                                    classInput.text = drawerRoot.controller.activeWindowClass
                                                } else {
                                                    classInput.text = ""
                                                }
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Match all web browser window classes (Chrome, Zen, Firefox, Edge, Brave, Opera, Vivaldi, Chromium, LibreWolf, etc.)")
                                    }
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Title Substring:")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.TextField {
                                    id: titleInput
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("Optional (e.g. YouTube, Gmail, Twitch — matched against window title)")
                                    font.pixelSize: 12
                                    selectByMouse: true
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Favicon / Site URL:")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.TextField {
                                    id: faviconUrlInput
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("Optional URL or domain for rule icon (e.g. https://news.ycombinator.com or http://192.168.1.50:8123)")
                                    font.family: "monospace"
                                    font.pixelSize: 12
                                    selectByMouse: true
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Target Profile:")
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    spacing: 6

                                    QQC2.ComboBox {
                                        id: targetProfileCombo
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 120
                                        Layout.preferredWidth: 1
                                        Layout.alignment: Qt.AlignVCenter
                                        font.pixelSize: 12
                                        model: drawerRoot.profileChoices
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        visible: drawerRoot.editingRuleIndex >= 0
                                        text: qsTr("Cancel")
                                        font.pixelSize: 12
                                        onClicked: {
                                            drawerRoot.editingRuleIndex = -1
                                            classInput.text = ""
                                            titleInput.text = ""
                                            faviconUrlInput.text = ""
                                        }
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        icon.name: drawerRoot.editingRuleIndex >= 0 ? "document-save" : "list-add"
                                        text: drawerRoot.editingRuleIndex >= 0 ? qsTr("Update Rule") : qsTr("Save Rule")
                                        font.pixelSize: 12
                                        highlighted: true
                                        enabled: classInput.text.trim().length > 0 && targetProfileCombo.currentIndex >= 0
                                        onClicked: {
                                            if (!drawerRoot.controller) return
                                            const profNum = drawerRoot.profileNumbers[targetProfileCombo.currentIndex] || 1
                                            drawerRoot.controller.saveAutoSwitchRuleWithFavicon(
                                                drawerRoot.editingRuleIndex,
                                                true,
                                                classInput.text.trim(),
                                                titleInput.text.trim(),
                                                faviconUrlInput.text.trim(),
                                                profNum
                                            )
                                            drawerRoot.editingRuleIndex = -1
                                            classInput.text = ""
                                            titleInput.text = ""
                                            faviconUrlInput.text = ""
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Rules List Header + Filter
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            Layout.alignment: Qt.AlignVCenter
                            text: drawerRoot.ruleFilterText.trim().length > 0
                                ? qsTr("Configured Rules (%1 of %2) — Double-click to Edit • Right-click for Options").arg(rulesModel.count).arg(drawerRoot.totalRuleCount)
                                : qsTr("Configured Rules (%1) — Drag to Reorder • Double-click to Edit • Right-click for Options").arg(drawerRoot.totalRuleCount)
                            font.bold: true
                            font.pixelSize: 13
                            color: Kirigami.Theme.textColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }

                        QQC2.TextField {
                            id: ruleFilterField
                            Layout.minimumWidth: 140
                            Layout.preferredWidth: 220
                            Layout.alignment: Qt.AlignVCenter
                            placeholderText: qsTr("Filter rules by class, title, URL, or profile...")
                            selectByMouse: true
                            font.pixelSize: 12
                            onTextChanged: {
                                drawerRoot.ruleFilterText = text
                                drawerRoot.syncFromController()
                            }
                        }

                        QQC2.ToolButton {
                            visible: ruleFilterField.text.length > 0
                            Layout.alignment: Qt.AlignVCenter
                            text: "✕"
                            implicitWidth: 26
                            implicitHeight: 26
                            font.pixelSize: 11
                            onClicked: {
                                ruleFilterField.text = ""
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Clear filter")
                        }
                    }

                    // Scrollable Rules List Container with Drag-and-Drop Reordering & Right-Click Context Menu
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.minimumHeight: 160
                        Layout.preferredHeight: 1
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: drawerRoot.subtleBorder
                        border.width: 1
                        clip: true

                        ListView {
                            id: rulesListView
                            anchors.fill: parent
                            anchors.margins: 6
                            clip: true
                            model: rulesModel
                            spacing: 6
                            boundsBehavior: Flickable.StopAtBounds
                            highlightFollowsCurrentItem: false

                            property int draggedRuleIndex: -1
                            property int dropTargetRuleIndex: -1
                            readonly property bool canReorder: drawerRoot.ruleFilterText.trim().length === 0

                            QQC2.ScrollBar.vertical: QQC2.ScrollBar {
                                policy: rulesListView.contentHeight > rulesListView.height
                                    ? QQC2.ScrollBar.AlwaysOn
                                    : QQC2.ScrollBar.AsNeeded
                                minimumSize: 0.14
                                implicitWidth: 10
                            }

                            QQC2.Label {
                                anchors.centerIn: parent
                                visible: rulesListView.count === 0
                                text: drawerRoot.totalRuleCount > 0
                                    ? qsTr("No configured rules match \"%1\"").arg(drawerRoot.ruleFilterText.trim())
                                    : qsTr("No auto-switcher rules configured yet.")
                                font.pixelSize: 13
                                color: Kirigami.Theme.disabledTextColor
                            }

                            delegate: Item {
                                id: ruleDelegate
                                required property int index
                                required property int ruleIndex
                                required property bool ruleEnabled
                                required property string resourceClass
                                required property string titleContains
                                required property string faviconUrl
                                required property int targetProfile
                                required property string targetProfileName
                                required property string ruleIcon

                                readonly property bool isBeingDragged: rulesListView.draggedRuleIndex === index
                                readonly property bool isDropTarget: rulesListView.draggedRuleIndex !== -1
                                    && rulesListView.dropTargetRuleIndex === index
                                    && rulesListView.draggedRuleIndex !== index
                                readonly property real rowStride: height + rulesListView.spacing
                                readonly property real shiftOffsetY: {
                                    const fromIdx = rulesListView.draggedRuleIndex
                                    const toIdx = rulesListView.dropTargetRuleIndex
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

                                Kirigami.Theme.colorSet: Kirigami.Theme.View
                                Kirigami.Theme.inherit: false
                                width: Math.max(
                                    200,
                                    rulesListView.width - (rulesListView.contentHeight > rulesListView.height ? 14 : 0)
                                )
                                height: 60
                                z: isBeingDragged ? 100 : 1

                                // Drop target slot outline shown in the target gap while dragging
                                Rectangle {
                                    visible: rulesListView.draggedRuleIndex !== -1
                                        && rulesListView.dropTargetRuleIndex === ruleDelegate.index
                                    anchors.fill: parent
                                    radius: 6
                                    color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.10)
                                    border.width: 1.5
                                    border.color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.55)
                                }

                                MouseArea {
                                    id: ruleDragMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                                    cursorShape: ruleDragMouse.dragging
                                        ? Qt.ClosedHandCursor
                                        : Qt.ArrowCursor

                                    property real pressX: 0
                                    property real pressY: 0
                                    property real dragOffsetX: 0
                                    property real dragOffsetY: 0
                                    property bool dragging: false

                                    onPressed: (mouse) => {
                                        if (mouse.button === Qt.RightButton) {
                                            drawerRoot.contextMenuRuleIndex = ruleDelegate.ruleIndex
                                            drawerRoot.contextMenuResourceClass = ruleDelegate.resourceClass
                                            drawerRoot.contextMenuTitleContains = ruleDelegate.titleContains
                                            drawerRoot.contextMenuFaviconUrl = ruleDelegate.faviconUrl
                                            drawerRoot.contextMenuTargetProfile = ruleDelegate.targetProfile
                                            drawerRoot.contextMenuRuleEnabled = ruleDelegate.ruleEnabled
                                            ruleContextMenu.popup()
                                            return
                                        }
                                        pressX = mouse.x
                                        pressY = mouse.y
                                        dragOffsetX = 0
                                        dragOffsetY = 0
                                        dragging = false
                                    }

                                    onDoubleClicked: (mouse) => {
                                        if (mouse.button === Qt.LeftButton) {
                                            drawerRoot.loadRuleIntoEditor(
                                                ruleDelegate.ruleIndex,
                                                ruleDelegate.resourceClass,
                                                ruleDelegate.titleContains,
                                                ruleDelegate.faviconUrl,
                                                ruleDelegate.targetProfile
                                            )
                                        }
                                    }

                                    onPositionChanged: (mouse) => {
                                        if (!pressed || !(mouse.buttons & Qt.LeftButton) || !rulesListView.canReorder) return
                                        if (!dragging && Math.abs(mouse.y - pressY) > 5) {
                                            dragging = true
                                            rulesListView.interactive = false
                                            rulesListView.draggedRuleIndex = ruleDelegate.index
                                            rulesListView.dropTargetRuleIndex = ruleDelegate.index
                                        }
                                        if (dragging) {
                                            const posInView = ruleDragMouse.mapToItem(rulesListView, mouse.x, mouse.y)
                                            if (posInView.y < 24 && rulesListView.contentY > 0) {
                                                rulesListView.contentY = Math.max(0, rulesListView.contentY - 10)
                                            } else if (posInView.y > rulesListView.height - 24) {
                                                const maxScroll = Math.max(0, rulesListView.contentHeight - rulesListView.height)
                                                rulesListView.contentY = Math.min(maxScroll, rulesListView.contentY + 10)
                                            }

                                            dragOffsetX = Math.max(-10, Math.min(10, mouse.x - pressX))
                                            dragOffsetY = mouse.y - pressY

                                            const draggedCenterY = ruleDelegate.y + dragOffsetY + ruleDelegate.height * 0.5
                                            const targetIdx = Math.max(
                                                0,
                                                Math.min(
                                                    Math.max(0, rulesListView.count - 1),
                                                    Math.floor(draggedCenterY / ruleDelegate.rowStride)
                                                )
                                            )
                                            rulesListView.dropTargetRuleIndex = targetIdx
                                        }
                                    }

                                    onReleased: {
                                        if (dragging) {
                                            const fromIdx = rulesListView.draggedRuleIndex
                                            const toIdx = rulesListView.dropTargetRuleIndex
                                            dragging = false
                                            dragOffsetX = 0
                                            dragOffsetY = 0
                                            rulesListView.draggedRuleIndex = -1
                                            rulesListView.dropTargetRuleIndex = -1
                                            rulesListView.interactive = true
                                            if (drawerRoot.controller && fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                if (drawerRoot.editingRuleIndex === fromIdx) {
                                                    drawerRoot.editingRuleIndex = toIdx
                                                }
                                                const savedY = rulesListView.contentY
                                                drawerRoot.controller.reorderAutoSwitchRule(fromIdx, toIdx)
                                                rulesListView.contentY = savedY
                                            }
                                        }
                                    }

                                    onCanceled: {
                                        dragging = false
                                        dragOffsetX = 0
                                        dragOffsetY = 0
                                        rulesListView.draggedRuleIndex = -1
                                        rulesListView.dropTargetRuleIndex = -1
                                        rulesListView.interactive = true
                                    }
                                }

                                Rectangle {
                                    id: ruleCard
                                    width: parent.width
                                    height: parent.height
                                    x: ruleDelegate.isBeingDragged ? ruleDragMouse.dragOffsetX : 0
                                    y: ruleDelegate.isBeingDragged ? ruleDragMouse.dragOffsetY : ruleDelegate.shiftOffsetY
                                    scale: ruleDelegate.isBeingDragged ? 1.015 : 1.0
                                    radius: 6
                                    opacity: ruleDelegate.isBeingDragged ? 0.96 : 1.0
                                    color: ruleDelegate.isBeingDragged
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.24))
                                        : Kirigami.Theme.backgroundColor
                                    border.color: (drawerRoot.editingRuleIndex === ruleIndex || ruleDelegate.isBeingDragged)
                                        ? Kirigami.Theme.highlightColor
                                        : (ruleDragMouse.containsMouse ? Kirigami.Theme.focusColor : drawerRoot.subtleBorder)
                                    border.width: (drawerRoot.editingRuleIndex === ruleIndex || ruleDelegate.isBeingDragged) ? 2 : 1

                                    Behavior on y {
                                        enabled: rulesListView.draggedRuleIndex !== -1 && !ruleDelegate.isBeingDragged
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
                                        anchors.leftMargin: 10
                                        anchors.rightMargin: 10
                                        anchors.topMargin: 6
                                        anchors.bottomMargin: 6
                                        spacing: 8

                                        // Drag Handle Grip Indicator
                                        Item {
                                            visible: rulesListView.canReorder
                                            Layout.preferredWidth: 18
                                            Layout.fillHeight: true
                                            Layout.alignment: Qt.AlignVCenter

                                            QQC2.Label {
                                                anchors.centerIn: parent
                                                text: "⠿"
                                                font.pixelSize: 17
                                                color: ruleDragMouse.containsMouse || ruleDelegate.isBeingDragged
                                                    ? Kirigami.Theme.textColor
                                                    : Kirigami.Theme.disabledTextColor
                                            }
                                        }

                                        QQC2.CheckBox {
                                            Layout.alignment: Qt.AlignVCenter
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
                                            Layout.alignment: Qt.AlignVCenter
                                            source: ruleIcon.length > 0 ? ruleIcon : "application-x-executable"
                                            Layout.preferredWidth: 26
                                            Layout.preferredHeight: 26
                                            isMask: false
                                            opacity: ruleEnabled ? 1.0 : 0.45
                                        }

                                        ColumnLayout {
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 0
                                            Layout.preferredWidth: 1
                                            Layout.alignment: Qt.AlignVCenter
                                            clip: true
                                            spacing: 2

                                            RowLayout {
                                                Layout.fillWidth: true
                                                Layout.minimumWidth: 0
                                                spacing: 8

                                                QQC2.Label {
                                                    id: classLabel
                                                    Layout.fillWidth: true
                                                    Layout.minimumWidth: 60
                                                    Layout.preferredWidth: 1
                                                    text: "#" + (ruleIndex + 1) + "  " + (
                                                        resourceClass.toLowerCase() === "*browser*"
                                                            ? qsTr("All Web Browsers (*browser*)")
                                                            : resourceClass
                                                    )
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pixelSize: 14
                                                    elide: Text.ElideRight
                                                    color: ruleEnabled ? Kirigami.Theme.textColor : Kirigami.Theme.disabledTextColor
                                                }

                                                Rectangle {
                                                    id: titleBadge
                                                    visible: titleContains.length > 0
                                                    Layout.maximumWidth: Math.max(100, ruleCard.width * 0.45)
                                                    implicitWidth: Math.min(titleTag.implicitWidth + 14, Math.max(100, ruleCard.width * 0.45))
                                                    implicitHeight: 20
                                                    radius: 4
                                                    color: Kirigami.Theme.alternateBackgroundColor
                                                    border.color: drawerRoot.subtleBorder
                                                    border.width: 1

                                                    QQC2.Label {
                                                        id: titleTag
                                                        anchors.fill: parent
                                                        anchors.leftMargin: 6
                                                        anchors.rightMargin: 6
                                                        verticalAlignment: Text.AlignVCenter
                                                        text: qsTr("title: \"%1\"").arg(titleContains)
                                                        font.pixelSize: 11
                                                        elide: Text.ElideRight
                                                        color: Kirigami.Theme.disabledTextColor
                                                    }
                                                }
                                            }

                                            QQC2.Label {
                                                Layout.fillWidth: true
                                                Layout.minimumWidth: 0
                                                text: faviconUrl.length > 0
                                                    ? qsTr("Switches to Profile #%1 (%2)  •  🔗 %3").arg(targetProfile).arg(targetProfileName).arg(faviconUrl)
                                                    : qsTr("Switches to Profile #%1 (%2)").arg(targetProfile).arg(targetProfileName)
                                                font.pixelSize: 12
                                                font.weight: Font.Medium
                                                elide: Text.ElideRight
                                                color: Kirigami.Theme.highlightColor
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Bottom Footer Bar (always visible)
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: drawerRoot.close()
                }
            }
        }
    }
}
