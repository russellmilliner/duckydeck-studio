import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Window {
    id: drawerRoot
    title: qsTr("Auto-Switcher Rules")
    width: Math.min(840, Screen.desktopAvailableWidth >= 800 ? Screen.desktopAvailableWidth - 80 : 840)
    height: Math.min(780, Screen.desktopAvailableHeight >= 600 ? Screen.desktopAvailableHeight - 80 : 780)
    minimumWidth: 660
    minimumHeight: 540
    visible: false
    transientParent: root
    modality: Qt.NonModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

    property var root: null
    property var controller: null
    property var profileChoices: []
    property var profileNumbers: []
    property var fallbackChoices: []
    property var fallbackNumbers: []
    property int editingRuleIndex: -1
    property string ruleFilterText: ""
    property int totalRuleCount: 0
    property string previousSpecificClass: ""
    property int settleMsValue: controller ? controller.autoSwitchSettleMs : 250
    property int dwellMsValue: controller ? controller.autoSwitchDwellMs : 150
    property bool enableSectionExpanded: controller ? controller.autoswitchEnableSectionExpanded : true
    property bool ruleEditorSectionExpanded: controller ? controller.autoswitchRuleEditorExpanded : true
    property bool modifierSectionExpanded: controller ? controller.modifierSectionExpanded : true
    property int editingModifierRuleIndex: -1

    readonly property var modifierKeyOptions: [
        { code: "MENU", label: qsTr("Menu / App Key (KEY_COMPOSE)") },
        { code: "RCTRL", label: qsTr("Right Ctrl") },
        { code: "RALT", label: qsTr("Right Alt / AltGr") },
        { code: "RSHIFT", label: qsTr("Right Shift") },
        { code: "LCTRL", label: qsTr("Left Ctrl") },
        { code: "LALT", label: qsTr("Left Alt") },
        { code: "LSHIFT", label: qsTr("Left Shift") },
        { code: "SUPER", label: qsTr("Super / Windows Key") },
        { code: "CTRL_ALT", label: qsTr("Ctrl + Alt (Chord)") },
        { code: "CTRL_SUPER", label: qsTr("Ctrl + Super (Chord)") },
        { code: "CAPSLOCK", label: qsTr("Caps Lock") },
        { code: "SCROLLLOCK", label: qsTr("Scroll Lock") },
        { code: "PAUSE", label: qsTr("Pause / Break") },
        { code: "FN", label: qsTr("Fn (if exposed via USB HID)") },
        { code: "F13", label: qsTr("F13") },
        { code: "F14", label: qsTr("F14") },
        { code: "F15", label: qsTr("F15") },
        { code: "F16", label: qsTr("F16") },
        { code: "F17", label: qsTr("F17") },
        { code: "F18", label: qsTr("F18") },
        { code: "F19", label: qsTr("F19") },
        { code: "F20", label: qsTr("F20") },
        { code: "F21", label: qsTr("F21") },
        { code: "F22", label: qsTr("F22") },
        { code: "F23", label: qsTr("F23") },
        { code: "F24", label: qsTr("F24") }
    ]

    readonly property var modifierModeOptions: [
        { code: "hold", label: qsTr("Momentary Hold (while held)") },
        { code: "oneshot", label: qsTr("One-Shot Sticky (1 key or timeout)") },
        { code: "double_tap_hold", label: qsTr("Double-Tap + Hold") },
        { code: "toggle", label: qsTr("Toggle Lock (tap / 2× tap)") }
    ]

    onModifierSectionExpandedChanged: {
        if (drawerRoot.controller
            && drawerRoot.controller.modifierSectionExpanded !== drawerRoot.modifierSectionExpanded) {
            drawerRoot.controller.updateModifierSectionExpanded(drawerRoot.modifierSectionExpanded)
        }
    }

    function setSectionsExpanded(enableExp, ruleExp) {
        const nextEnable = !!enableExp
        const nextRule = !!ruleExp
        drawerRoot.enableSectionExpanded = nextEnable
        drawerRoot.ruleEditorSectionExpanded = nextRule
        if (drawerRoot.controller) {
            drawerRoot.controller.setAutoSwitchSectionsExpanded(nextEnable, nextRule)
        }
    }

    onEnableSectionExpandedChanged: {
        if (drawerRoot.controller
            && drawerRoot.controller.autoswitchEnableSectionExpanded !== drawerRoot.enableSectionExpanded) {
            drawerRoot.controller.setAutoSwitchSectionsExpanded(
                drawerRoot.enableSectionExpanded,
                drawerRoot.ruleEditorSectionExpanded
            )
        }
    }

    onRuleEditorSectionExpandedChanged: {
        if (drawerRoot.controller
            && drawerRoot.controller.autoswitchRuleEditorExpanded !== drawerRoot.ruleEditorSectionExpanded) {
            drawerRoot.controller.setAutoSwitchSectionsExpanded(
                drawerRoot.enableSectionExpanded,
                drawerRoot.ruleEditorSectionExpanded
            )
        }
    }

    property int contextMenuRuleIndex: -1
    property string contextMenuResourceClass: ""
    property string contextMenuTitleContains: ""
    property bool contextMenuTitleIsRegex: false
    property string contextMenuFaviconUrl: ""
    property int contextMenuTargetProfile: 1
    property bool contextMenuRuleEnabled: true
    property string titleRegexError: {
        if (!titleRegexCheck || !titleRegexCheck.checked || !drawerRoot.controller) {
            return ""
        }
        const pat = titleInput ? titleInput.text.trim() : ""
        if (pat.length === 0) {
            return ""
        }
        return drawerRoot.controller.validateTitleRegexPattern(pat)
    }

    readonly property bool closableWithEscape: true
    readonly property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor
    readonly property color subtleBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

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
        id: rulesModel
    }

    ListModel {
        id: modifierRulesModel
    }

    function loadModifierRuleIntoEditor(mIdx, trigKey, modeStr, holdDelay, oneshotTimeout, targetProf) {
        drawerRoot.modifierSectionExpanded = true
        drawerRoot.editingModifierRuleIndex = mIdx
        for (let i = 0; i < drawerRoot.modifierKeyOptions.length; ++i) {
            if (drawerRoot.modifierKeyOptions[i].code === trigKey) {
                modifierKeyCombo.currentIndex = i
                break
            }
        }
        for (let i = 0; i < drawerRoot.modifierModeOptions.length; ++i) {
            if (drawerRoot.modifierModeOptions[i].code === modeStr) {
                modifierModeCombo.currentIndex = i
                break
            }
        }
        modifierHoldDelayInput.text = String(holdDelay || 200)
        modifierOneshotTimeoutInput.text = String(oneshotTimeout || 6)
        const pIdx = drawerRoot.profileNumbers.indexOf(targetProf)
        if (pIdx >= 0) {
            modifierTargetProfileCombo.currentIndex = pIdx
        }
        drawerRoot.requestActivate()
        Qt.callLater(function() {
            if (mainFlickable && modifierLayerCard) {
                const maxScroll = Math.max(0, mainFlickable.contentHeight - mainFlickable.height)
                const targetY = Math.max(0, Math.min(maxScroll, modifierLayerCard.y - 8))
                mainFlickable.contentY = targetY
            }
            modifierKeyCombo.forceActiveFocus()
        })
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

    function loadRuleIntoEditor(rIdx, resClass, titleSubstr, favUrl, targetProf, isRegex) {
        drawerRoot.ruleEditorSectionExpanded = true
        drawerRoot.editingRuleIndex = rIdx
        classInput.text = resClass
        titleInput.text = titleSubstr
        titleRegexCheck.checked = !!isRegex
        faviconUrlInput.text = favUrl || ""
        const pIdx = drawerRoot.profileNumbers.indexOf(targetProf)
        if (pIdx >= 0) {
            targetProfileCombo.currentIndex = pIdx
        }
        drawerRoot.requestActivate()
        Qt.callLater(function() {
            if (mainFlickable && ruleEditorCard) {
                const maxScroll = Math.max(0, mainFlickable.contentHeight - mainFlickable.height)
                const targetY = Math.max(0, Math.min(maxScroll, ruleEditorCard.y - 8))
                mainFlickable.contentY = targetY
            }
            classInput.forceActiveFocus()
            classInput.selectAll()
        })
    }

    QQC2.Menu {
        font.pointSize: 10 * drawerRoot.fontScale
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
                    drawerRoot.contextMenuTargetProfile,
                    drawerRoot.contextMenuTitleIsRegex
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
        const savedScrollY = mainFlickable ? mainFlickable.contentY : 0

        drawerRoot.enableSectionExpanded = controller.autoswitchEnableSectionExpanded
        drawerRoot.ruleEditorSectionExpanded = controller.autoswitchRuleEditorExpanded
        drawerRoot.modifierSectionExpanded = controller.modifierSectionExpanded
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
            const fbChoices = []
            const fbNums = []

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
            if (modifierTargetProfileCombo.currentIndex < 0 && activeIdx >= 0) {
                modifierTargetProfileCombo.currentIndex = activeIdx
            }
        } catch (e) {}

        // 1b. Sync main-keyboard modifier layer rules
        try {
            const mArr = JSON.parse(controller.modifierRulesJson || "[]")
            modifierRulesModel.clear()
            for (let i = 0; i < mArr.length; ++i) {
                const m = mArr[i]
                modifierRulesModel.append({
                    mRuleIndex: m.index !== undefined ? m.index : i,
                    mRuleEnabled: !!m.enabled,
                    triggerKey: String(m.triggerKey || "RCTRL"),
                    triggerKeyLabel: String(m.triggerKeyLabel || m.triggerKey || "Right Ctrl"),
                    mode: String(m.mode || "hold"),
                    modeLabel: String(m.modeLabel || "Momentary Hold"),
                    holdDelayMs: m.holdDelayMs !== undefined ? Number(m.holdDelayMs) : 200,
                    oneshotTimeoutSec: m.oneshotTimeoutSec !== undefined ? Number(m.oneshotTimeoutSec) : 6,
                    targetProfile: m.targetProfile || 1,
                    targetProfileName: String(m.targetProfileName || "Default")
                })
            }
        } catch (e) {}

        // 2. Sync rules list in-place to preserve scroll position
        try {
            const rArr = JSON.parse(controller.autoSwitchRulesJson || "[]")
            drawerRoot.totalRuleCount = rArr.length
            if (drawerRoot.editingRuleIndex >= 0 && drawerRoot.editingRuleIndex < rArr.length) {
                const editedRule = rArr[drawerRoot.editingRuleIndex]
                if (editedRule && editedRule.targetProfile) {
                    const editedProfIdx = drawerRoot.profileNumbers.indexOf(editedRule.targetProfile)
                    if (editedProfIdx >= 0) {
                        targetProfileCombo.currentIndex = editedProfIdx
                    }
                }
            }
            const q = drawerRoot.ruleFilterText.trim().toLowerCase()
            const nextItems = []
            for (let i = 0; i < rArr.length; ++i) {
                const r = rArr[i]
                const rIdx = r.index !== undefined ? r.index : i
                const resClass = r.resourceClass || ""
                const titleSub = r.titleContains || ""
                const titleRegex = !!r.titleIsRegex
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
                        (titleRegex ? "regex " : "") +
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
                    titleIsRegex: titleRegex,
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

            if (mainFlickable && savedScrollY > 0) {
                const maxScroll = Math.max(0, mainFlickable.contentHeight - mainFlickable.height)
                mainFlickable.contentY = Math.min(savedScrollY, maxScroll)
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
        function onModifierRulesJsonChanged() {
            drawerRoot.syncFromController()
        }
        function onModifierSectionExpandedChanged() {
            if (drawerRoot.controller) {
                drawerRoot.modifierSectionExpanded = drawerRoot.controller.modifierSectionExpanded
            }
        }
        function onProfilesJsonChanged() {
            drawerRoot.syncFromController()
        }
        function onDefaultFallbackProfileChanged() {
            drawerRoot.syncFromController()
        }
        function onAutoswitchEnableSectionExpandedChanged() {
            if (drawerRoot.controller) {
                drawerRoot.enableSectionExpanded = drawerRoot.controller.autoswitchEnableSectionExpanded
            }
        }
        function onAutoswitchRuleEditorExpandedChanged() {
            if (drawerRoot.controller) {
                drawerRoot.ruleEditorSectionExpanded = drawerRoot.controller.autoswitchRuleEditorExpanded
            }
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
                drawerRoot.ruleEditorSectionExpanded = true
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
        width: drawerRoot.width
        height: drawerRoot.height
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: drawerRoot.visible
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
                        font.pointSize: 10 * drawerRoot.fontScale
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        radius: 4
                        color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.14)
                        border.color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                        border.width: 1
                        implicitHeight: autoSwitchHostLbl.implicitHeight + 8

                        QQC2.Label {
                            id: autoSwitchHostLbl
                            anchors.fill: parent
                            anchors.margins: 4
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * drawerRoot.fontScale
                            font.bold: true
                            color: Kirigami.Theme.textColor
                            text: qsTr("💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable) — Window tracking uses KDE KWin & USB HID. Without the host app, switch profiles manually on hardware via GOTO_PROFILE / NEXT_PROFILE / PREV_PROFILE or holding '-' on the keypad.")
                        }
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

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: mainFlickable
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    policy: mainFlickable.contentHeight > mainFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: mainBodyCol
                    width: Math.max(
                        300,
                        mainFlickable.width - (mainFlickable.contentHeight > mainFlickable.height ? 14 : 0)
                    )
                    height: implicitHeight
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
                                spacing: 6

                                QQC2.ToolButton {
                                    id: enableSectionTwistButton
                                    objectName: "enableSectionTwistButton"
                                    Layout.alignment: Qt.AlignVCenter
                                    implicitWidth: 26
                                    implicitHeight: 26
                                    width: 26
                                    height: 26
                                    padding: 0
                                    text: drawerRoot.enableSectionExpanded ? "▾" : "▸"
                                    font.bold: true
                                    font.pointSize: 11 * drawerRoot.fontScale
                                    onClicked: drawerRoot.enableSectionExpanded = !drawerRoot.enableSectionExpanded
                                    QQC2.ToolTip.delay: 500
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: drawerRoot.enableSectionExpanded
                                        ? qsTr("Minimize Auto-Switcher enable & timing settings")
                                        : qsTr("Expand Auto-Switcher enable & timing settings")
                                }

                                QQC2.Switch {
                                    id: masterEnableSwitch
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    checked: drawerRoot.controller ? drawerRoot.controller.autoSwitchEnabled : true
                                    text: qsTr("Enable Automatic Profile Switching on Window Focus")
                                    font.bold: true
                                    font.pointSize: 10.5 * drawerRoot.fontScale
                                    onToggled: {
                                        if (drawerRoot.controller) {
                                            const fbNum = drawerRoot.fallbackNumbers[fallbackCombo.currentIndex] || 0
                                            drawerRoot.controller.setAutoSwitchConfig(checked, fbNum)
                                        }
                                    }
                                }

                                QQC2.Label {
                                    visible: !drawerRoot.enableSectionExpanded
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("%1 / %2 ms").arg(drawerRoot.settleMsValue).arg(drawerRoot.dwellMsValue)
                                    font.family: "monospace"
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            ColumnLayout {
                                id: enableSectionDetailsCol
                                visible: drawerRoot.enableSectionExpanded
                                Layout.fillWidth: true
                                spacing: 8

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    QQC2.Label {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Fallback Profile (when no rule matches):")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.ComboBox {
                                        id: fallbackCombo
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 120
                                        Layout.preferredWidth: 1
                                        Layout.alignment: Qt.AlignVCenter
                                        font.pointSize: 10 * drawerRoot.fontScale
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
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        font.bold: true
                                        color: Kirigami.Theme.textColor
                                        QQC2.ToolTip.delay: 500
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
                                            font.pointSize: 11 * drawerRoot.fontScale
                                            enabled: drawerRoot.settleMsValue > 0
                                            onClicked: drawerRoot.applyTimings(Math.max(0, drawerRoot.settleMsValue - 50), drawerRoot.dwellMsValue)
                                            QQC2.ToolTip.delay: 500
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
                                            font.pointSize: 11 * drawerRoot.fontScale
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
                                            QQC2.ToolTip.delay: 500
                                            QQC2.ToolTip.visible: hovered
                                            QQC2.ToolTip.text: qsTr("Wait for the mouse cursor to stop moving for this duration before switching profiles (0–5000 ms)")
                                        }

                                        QQC2.Button {
                                            text: "+"
                                            implicitWidth: 28
                                            implicitHeight: 30
                                            font.bold: true
                                            font.pointSize: 11 * drawerRoot.fontScale
                                            enabled: drawerRoot.settleMsValue < 5000
                                            onClicked: drawerRoot.applyTimings(Math.min(5000, drawerRoot.settleMsValue + 50), drawerRoot.dwellMsValue)
                                            QQC2.ToolTip.delay: 500
                                            QQC2.ToolTip.visible: hovered
                                            QQC2.ToolTip.text: qsTr("Increase Mouse Settle by 50 ms")
                                        }

                                        QQC2.Label {
                                            text: qsTr("ms")
                                            font.pointSize: 10 * drawerRoot.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    Item { Layout.preferredWidth: 8 }

                                    QQC2.Label {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Window Dwell:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        font.bold: true
                                        color: Kirigami.Theme.textColor
                                        QQC2.ToolTip.delay: 500
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
                                            font.pointSize: 11 * drawerRoot.fontScale
                                            enabled: drawerRoot.dwellMsValue > 0
                                            onClicked: drawerRoot.applyTimings(drawerRoot.settleMsValue, Math.max(0, drawerRoot.dwellMsValue - 50))
                                            QQC2.ToolTip.delay: 500
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
                                            font.pointSize: 11 * drawerRoot.fontScale
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
                                            QQC2.ToolTip.delay: 500
                                            QQC2.ToolTip.visible: hovered
                                            QQC2.ToolTip.text: qsTr("Minimum time cursor/focus must stay in the window before switching profiles (0–5000 ms)")
                                        }

                                        QQC2.Button {
                                            text: "+"
                                            implicitWidth: 28
                                            implicitHeight: 30
                                            font.bold: true
                                            font.pointSize: 11 * drawerRoot.fontScale
                                            enabled: drawerRoot.dwellMsValue < 5000
                                            onClicked: drawerRoot.applyTimings(drawerRoot.settleMsValue, Math.min(5000, drawerRoot.dwellMsValue + 50))
                                            QQC2.ToolTip.delay: 500
                                            QQC2.ToolTip.visible: hovered
                                            QQC2.ToolTip.text: qsTr("Increase Window Dwell by 50 ms")
                                        }

                                        QQC2.Label {
                                            text: qsTr("ms")
                                            font.pointSize: 10 * drawerRoot.fontScale
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
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Instant (0 ms)")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        implicitHeight: 26
                                        highlighted: drawerRoot.settleMsValue === 0 && drawerRoot.dwellMsValue === 0
                                        onClicked: drawerRoot.applyTimings(0, 0)
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Switch immediately when entering a window (0 ms settle, 0 ms dwell)")
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Balanced (250 / 150 ms)")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        implicitHeight: 26
                                        highlighted: drawerRoot.settleMsValue === 250 && drawerRoot.dwellMsValue === 150
                                        onClicked: drawerRoot.applyTimings(250, 150)
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Default balanced settle timing (250 ms mouse settle, 150 ms window dwell)")
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Relaxed (500 / 300 ms)")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        implicitHeight: 26
                                        highlighted: drawerRoot.settleMsValue === 500 && drawerRoot.dwellMsValue === 300
                                        onClicked: drawerRoot.applyTimings(500, 300)
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Relaxed settle timing (500 ms mouse settle, 300 ms window dwell)")
                                    }

                                    Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }
                                }
                            }
                        }
                    }

                    // Main-Keyboard Modifier Layer Shift Card
                    Rectangle {
                        id: modifierLayerCard
                        objectName: "modifierLayerCard"
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: modifierLayerLayout.implicitHeight + 18
                        radius: 8
                        color: drawerRoot.controller && drawerRoot.controller.modifierLayerActive
                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.14))
                            : Kirigami.Theme.backgroundColor
                        border.color: drawerRoot.controller && drawerRoot.controller.modifierLayerActive
                            ? Kirigami.Theme.highlightColor
                            : drawerRoot.subtleBorder
                        border.width: drawerRoot.controller && drawerRoot.controller.modifierLayerActive ? 2 : 1

                        ColumnLayout {
                            id: modifierLayerLayout
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.ToolButton {
                                    id: modifierSectionTwistButton
                                    objectName: "modifierSectionTwistButton"
                                    Layout.alignment: Qt.AlignVCenter
                                    implicitWidth: 26
                                    implicitHeight: 26
                                    width: 26
                                    height: 26
                                    padding: 0
                                    text: drawerRoot.modifierSectionExpanded ? "▾" : "▸"
                                    font.bold: true
                                    font.pointSize: 11 * drawerRoot.fontScale
                                    onClicked: drawerRoot.modifierSectionExpanded = !drawerRoot.modifierSectionExpanded
                                    QQC2.ToolTip.delay: 500
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: drawerRoot.modifierSectionExpanded
                                        ? qsTr("Minimize Main-Keyboard Modifier Layer Shift section")
                                        : qsTr("Expand Main-Keyboard Modifier Layer Shift section")
                                }

                                QQC2.Switch {
                                    id: modifierEnableSwitch
                                    objectName: "modifierEnableSwitch"
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    checked: drawerRoot.controller ? drawerRoot.controller.modifierSwitchEnabled : true
                                    text: qsTr("⌨️ Main-Keyboard Modifier Layer Shift (Hold / One-Shot / Toggle)")
                                    font.bold: true
                                    font.pointSize: 10.5 * drawerRoot.fontScale
                                    onToggled: {
                                        if (drawerRoot.controller) {
                                            drawerRoot.controller.updateModifierSwitchEnabled(checked)
                                        }
                                    }
                                }

                                QQC2.Label {
                                    visible: drawerRoot.controller && drawerRoot.controller.modifierLayerActive
                                    Layout.alignment: Qt.AlignVCenter
                                    Layout.maximumWidth: 220
                                    text: drawerRoot.controller ? drawerRoot.controller.modifierLayerStatus : ""
                                    font.bold: true
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.textColor
                                    elide: Text.ElideRight
                                }

                                QQC2.Label {
                                    visible: !drawerRoot.modifierSectionExpanded
                                        && !(drawerRoot.controller && drawerRoot.controller.modifierLayerActive)
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("%1 layer(s)").arg(modifierRulesModel.count)
                                    font.family: "monospace"
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            ColumnLayout {
                                id: modifierSectionDetailsCol
                                visible: drawerRoot.modifierSectionExpanded
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    wrapMode: Text.WordWrap
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    text: qsTr("Hold or tap a modifier/dedicated key on your main PC keyboard to shift the duckyPad to a global desktop profile and restore your window's profile on release. Includes a chord guard so normal shortcuts (like Ctrl+C) never flicker the keypad.")
                                }

                                // Row 1: Trigger Key + Activation Mode
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    QQC2.Label {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Trigger Key:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.ComboBox {
                                        id: modifierKeyCombo
                                        objectName: "modifierKeyCombo"
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 110
                                        Layout.preferredWidth: 1
                                        Layout.alignment: Qt.AlignVCenter
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        textRole: "label"
                                        valueRole: "code"
                                        model: drawerRoot.modifierKeyOptions
                                        currentIndex: 1 // Default RCTRL
                                    }

                                    QQC2.Label {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Mode:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.ComboBox {
                                        id: modifierModeCombo
                                        objectName: "modifierModeCombo"
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 110
                                        Layout.preferredWidth: 1
                                        Layout.alignment: Qt.AlignVCenter
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        textRole: "label"
                                        valueRole: "code"
                                        model: drawerRoot.modifierModeOptions
                                        currentIndex: 0 // Default Hold
                                    }
                                }

                                // Row 2: Hold Delay / One-Shot Timeout + Target Profile + Save Button
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    QQC2.Label {
                                        visible: modifierModeCombo.currentIndex !== 1
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Hold Delay:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.TextField {
                                        id: modifierHoldDelayInput
                                        visible: modifierModeCombo.currentIndex !== 1
                                        Layout.preferredWidth: 64
                                        Layout.minimumWidth: 56
                                        implicitHeight: 30
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        selectByMouse: true
                                        text: "200"
                                        validator: IntValidator { bottom: 0; top: 2000 }
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Chord-guard hold delay in milliseconds before shifting (0–2000 ms, default 200 ms)")
                                    }

                                    QQC2.Label {
                                        visible: modifierModeCombo.currentIndex !== 1
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("ms")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.Label {
                                        visible: modifierModeCombo.currentIndex === 1
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Timeout:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.TextField {
                                        id: modifierOneshotTimeoutInput
                                        visible: modifierModeCombo.currentIndex === 1
                                        Layout.preferredWidth: 56
                                        Layout.minimumWidth: 48
                                        implicitHeight: 30
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        selectByMouse: true
                                        text: "6"
                                        validator: IntValidator { bottom: 1; top: 60 }
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("One-Shot auto-restore timeout in seconds if no duckyPad key is pressed (1–60 s, default 6 s)")
                                    }

                                    QQC2.Label {
                                        visible: modifierModeCombo.currentIndex === 1
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("sec")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.Label {
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr("Profile:")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.ComboBox {
                                        id: modifierTargetProfileCombo
                                        objectName: "modifierTargetProfileCombo"
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 100
                                        Layout.preferredWidth: 1
                                        Layout.alignment: Qt.AlignVCenter
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        model: drawerRoot.profileChoices
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        visible: drawerRoot.editingModifierRuleIndex >= 0
                                        text: qsTr("Cancel")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        onClicked: {
                                            drawerRoot.editingModifierRuleIndex = -1
                                        }
                                    }

                                    QQC2.Button {
                                        id: saveModifierRuleBtn
                                        objectName: "saveModifierRuleButton"
                                        Layout.alignment: Qt.AlignVCenter
                                        icon.name: drawerRoot.editingModifierRuleIndex >= 0 ? "document-save" : "list-add"
                                        text: drawerRoot.editingModifierRuleIndex >= 0 ? qsTr("Update Layer") : qsTr("Add Layer")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        highlighted: true
                                        enabled: modifierTargetProfileCombo.currentIndex >= 0
                                        onClicked: {
                                            if (!drawerRoot.controller) return
                                            const keyOpt = drawerRoot.modifierKeyOptions[Math.max(0, modifierKeyCombo.currentIndex)]
                                            const modeOpt = drawerRoot.modifierModeOptions[Math.max(0, modifierModeCombo.currentIndex)]
                                            const parsedDelay = parseInt(modifierHoldDelayInput.text, 10)
                                            const parsedTimeout = parseInt(modifierOneshotTimeoutInput.text, 10)
                                            const profNum = drawerRoot.profileNumbers[modifierTargetProfileCombo.currentIndex] || 1
                                            drawerRoot.controller.saveModifierRule(
                                                drawerRoot.editingModifierRuleIndex,
                                                true,
                                                keyOpt ? keyOpt.code : "RCTRL",
                                                modeOpt ? modeOpt.code : "hold",
                                                isNaN(parsedDelay) ? 200 : parsedDelay,
                                                isNaN(parsedTimeout) ? 6 : parsedTimeout,
                                                profNum
                                            )
                                            drawerRoot.editingModifierRuleIndex = -1
                                        }
                                    }
                                }

                                // Configured Modifier Layer Rules List
                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 4
                                    visible: modifierRulesModel.count > 0

                                    Repeater {
                                        model: modifierRulesModel
                                        delegate: Rectangle {
                                            id: modRuleRow
                                            required property int index
                                            required property int mRuleIndex
                                            required property bool mRuleEnabled
                                            required property string triggerKey
                                            required property string triggerKeyLabel
                                            required property string mode
                                            required property string modeLabel
                                            required property int holdDelayMs
                                            required property int oneshotTimeoutSec
                                            required property int targetProfile
                                            required property string targetProfileName

                                            Layout.fillWidth: true
                                            implicitHeight: modRuleInnerRow.implicitHeight + 10
                                            radius: 6
                                            color: Kirigami.Theme.alternateBackgroundColor
                                            border.color: drawerRoot.editingModifierRuleIndex === mRuleIndex
                                                ? Kirigami.Theme.highlightColor
                                                : drawerRoot.subtleBorder
                                            border.width: drawerRoot.editingModifierRuleIndex === mRuleIndex ? 2 : 1

                                            MouseArea {
                                                anchors.fill: parent
                                                acceptedButtons: Qt.LeftButton
                                                onDoubleClicked: {
                                                    drawerRoot.loadModifierRuleIntoEditor(
                                                        modRuleRow.mRuleIndex,
                                                        modRuleRow.triggerKey,
                                                        modRuleRow.mode,
                                                        modRuleRow.holdDelayMs,
                                                        modRuleRow.oneshotTimeoutSec,
                                                        modRuleRow.targetProfile
                                                    )
                                                }
                                            }

                                            RowLayout {
                                                id: modRuleInnerRow
                                                anchors.fill: parent
                                                anchors.leftMargin: 8
                                                anchors.rightMargin: 8
                                                anchors.topMargin: 5
                                                anchors.bottomMargin: 5
                                                spacing: 8

                                                HighContrastCheckBox {
                                                    Layout.alignment: Qt.AlignVCenter
                                                    checked: modRuleRow.mRuleEnabled
                                                    onToggled: {
                                                        if (drawerRoot.controller) {
                                                            drawerRoot.controller.toggleModifierRule(modRuleRow.mRuleIndex, checked)
                                                        }
                                                    }
                                                    QQC2.ToolTip.delay: 500
                                                    QQC2.ToolTip.visible: hovered
                                                    QQC2.ToolTip.text: qsTr("Enable or disable this modifier layer rule")
                                                }

                                                QQC2.Label {
                                                    Layout.fillWidth: true
                                                    Layout.minimumWidth: 60
                                                    Layout.preferredWidth: 1
                                                    Layout.alignment: Qt.AlignVCenter
                                                    text: {
                                                        const timingDesc = modRuleRow.mode === "oneshot"
                                                            ? qsTr("%1 s timeout").arg(modRuleRow.oneshotTimeoutSec)
                                                            : (modRuleRow.mode === "toggle"
                                                                ? qsTr("lock on/off")
                                                                : qsTr("%1 ms hold").arg(modRuleRow.holdDelayMs))
                                                        return qsTr("⌨️ %1  •  %2 (%3)  →  Profile #%4 (%5)")
                                                            .arg(modRuleRow.triggerKeyLabel)
                                                            .arg(modRuleRow.modeLabel)
                                                            .arg(timingDesc)
                                                            .arg(modRuleRow.targetProfile)
                                                            .arg(modRuleRow.targetProfileName)
                                                    }
                                                    font.bold: true
                                                    font.pointSize: 10 * drawerRoot.fontScale
                                                    elide: Text.ElideRight
                                                    color: modRuleRow.mRuleEnabled
                                                        ? Kirigami.Theme.textColor
                                                        : Kirigami.Theme.disabledTextColor
                                                }

                                                QQC2.Button {
                                                    Layout.alignment: Qt.AlignVCenter
                                                    implicitHeight: 26
                                                    icon.name: "document-edit"
                                                    text: qsTr("Edit")
                                                    font.pointSize: 10 * drawerRoot.fontScale
                                                    onClicked: {
                                                        drawerRoot.loadModifierRuleIntoEditor(
                                                            modRuleRow.mRuleIndex,
                                                            modRuleRow.triggerKey,
                                                            modRuleRow.mode,
                                                            modRuleRow.holdDelayMs,
                                                            modRuleRow.oneshotTimeoutSec,
                                                            modRuleRow.targetProfile
                                                        )
                                                    }
                                                }

                                                QQC2.Button {
                                                    Layout.alignment: Qt.AlignVCenter
                                                    implicitWidth: 28
                                                    implicitHeight: 26
                                                    text: "✕"
                                                    font.bold: true
                                                    font.pointSize: 10 * drawerRoot.fontScale
                                                    onClicked: {
                                                        if (drawerRoot.editingModifierRuleIndex === modRuleRow.mRuleIndex) {
                                                            drawerRoot.editingModifierRuleIndex = -1
                                                        }
                                                        if (drawerRoot.controller) {
                                                            drawerRoot.controller.removeModifierRule(modRuleRow.mRuleIndex)
                                                        }
                                                    }
                                                    QQC2.ToolTip.delay: 500
                                                    QQC2.ToolTip.visible: hovered
                                                    QQC2.ToolTip.text: qsTr("Remove modifier layer rule")
                                                }
                                            }
                                        }
                                    }
                                }
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
                                    font.pointSize: 11 * drawerRoot.fontScale
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
                                    font.pointSize: 10 * drawerRoot.fontScale
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
                                font.pointSize: 10 * drawerRoot.fontScale
                                highlighted: drawerRoot.controller && drawerRoot.controller.windowPickerArmed
                                onClicked: {
                                    drawerRoot.ruleEditorSectionExpanded = true
                                    if (drawerRoot.controller) {
                                        drawerRoot.controller.armWindowPicker(false)
                                    }
                                }
                                QQC2.ToolTip.delay: 500
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Click this button, then click any open window to fill its class and title below")
                            }

                            QQC2.Button {
                                Layout.alignment: Qt.AlignVCenter
                                icon.name: "edit-paste"
                                text: qsTr("Use Current")
                                font.pointSize: 10 * drawerRoot.fontScale
                                enabled: drawerRoot.controller
                                    && drawerRoot.controller.activeWindowClass.length > 0
                                    && drawerRoot.controller.activeWindowClass !== "Listening for KWin..."
                                onClicked: {
                                    drawerRoot.ruleEditorSectionExpanded = true
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
                                QQC2.ToolTip.delay: 500
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Copy the last detected window class and title into the rule editor below")
                            }
                        }
                    }

                    // Add / Edit Rule Form Card
                    Rectangle {
                        id: ruleEditorCard
                        objectName: "ruleEditorCard"
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: ruleFormCol.implicitHeight + 18
                        radius: 8
                        color: Kirigami.Theme.backgroundColor
                        border.color: drawerRoot.editingRuleIndex >= 0
                            ? Kirigami.Theme.highlightColor
                            : drawerRoot.subtleBorder
                        border.width: drawerRoot.editingRuleIndex >= 0 ? 2 : 1

                        ColumnLayout {
                            id: ruleFormCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.ToolButton {
                                    id: ruleEditorTwistButton
                                    objectName: "ruleEditorTwistButton"
                                    Layout.alignment: Qt.AlignVCenter
                                    implicitWidth: 26
                                    implicitHeight: 26
                                    width: 26
                                    height: 26
                                    padding: 0
                                    text: drawerRoot.ruleEditorSectionExpanded ? "▾" : "▸"
                                    font.bold: true
                                    font.pointSize: 11 * drawerRoot.fontScale
                                    onClicked: drawerRoot.ruleEditorSectionExpanded = !drawerRoot.ruleEditorSectionExpanded
                                    QQC2.ToolTip.delay: 500
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: drawerRoot.ruleEditorSectionExpanded
                                        ? qsTr("Minimize Add / Edit Window Rule section")
                                        : qsTr("Expand Add / Edit Window Rule section")
                                }

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.alignment: Qt.AlignVCenter
                                    text: drawerRoot.editingRuleIndex >= 0
                                        ? qsTr("Edit Rule #%1").arg(drawerRoot.editingRuleIndex + 1)
                                        : qsTr("Add New Window Rule")
                                    font.bold: true
                                    font.pointSize: 11 * drawerRoot.fontScale
                                    color: Kirigami.Theme.textColor
                                    elide: Text.ElideRight

                                    MouseArea {
                                        anchors.fill: parent
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: drawerRoot.ruleEditorSectionExpanded = !drawerRoot.ruleEditorSectionExpanded
                                    }
                                }
                            }

                            GridLayout {
                                id: ruleEditorGrid
                                visible: drawerRoot.ruleEditorSectionExpanded
                                Layout.fillWidth: true
                                columns: 2
                                rowSpacing: 6
                                columnSpacing: 8

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Window Class:")
                                    font.pointSize: 10 * drawerRoot.fontScale
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
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        selectByMouse: true
                                        onTextChanged: {
                                            const trimmed = text.trim()
                                            if (trimmed.length > 0 && trimmed.toLowerCase() !== "*browser*") {
                                                drawerRoot.previousSpecificClass = trimmed
                                            }
                                        }
                                    }

                                    HighContrastCheckBox {
                                        id: matchAllBrowsersCheck
                                        Layout.alignment: Qt.AlignVCenter
                                        checked: classInput.text.trim().toLowerCase() === "*browser*"
                                        text: qsTr("🌐 Match All Browsers")
                                        font.pointSize: 10 * drawerRoot.fontScale
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
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Match all web browser window classes (Chrome, Zen, Firefox, Edge, Brave, Opera, Vivaldi, Chromium, LibreWolf, etc.)")
                                    }
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Title Substring:")
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    spacing: 8

                                    QQC2.TextField {
                                        id: titleInput
                                        objectName: "titleSubstringInput"
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 80
                                        Layout.preferredWidth: 1
                                        placeholderText: titleRegexCheck.checked
                                            ? qsTr("Regular expression (e.g. YouTube|Twitch, ^Gmail.*Inbox, /pattern/)")
                                            : qsTr("Optional substring or regex (e.g. YouTube, Gmail, YouTube|Twitch)")
                                        font.family: titleRegexCheck.checked ? "monospace" : Kirigami.Theme.defaultFont.family
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        selectByMouse: true
                                        QQC2.ToolTip.visible: drawerRoot.titleRegexError.length > 0
                                        QQC2.ToolTip.text: qsTr("Invalid regex: %1").arg(drawerRoot.titleRegexError)
                                    }

                                    HighContrastCheckBox {
                                        id: titleRegexCheck
                                        objectName: "titleRegexCheckBox"
                                        Layout.alignment: Qt.AlignVCenter
                                        text: qsTr(".* Regex")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        QQC2.ToolTip.delay: 500
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Match window title using a regular expression (case-insensitive by default; supports |, ^, $, .*, [a-z], etc.)")
                                    }
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Favicon / Site URL:")
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.TextField {
                                    id: faviconUrlInput
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("Optional URL or domain for rule icon (e.g. https://news.ycombinator.com or http://192.168.1.50:8123)")
                                    font.family: "monospace"
                                    font.pointSize: 10 * drawerRoot.fontScale
                                    selectByMouse: true
                                }

                                QQC2.Label {
                                    Layout.alignment: Qt.AlignVCenter
                                    text: qsTr("Target Profile:")
                                    font.pointSize: 10 * drawerRoot.fontScale
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
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        model: drawerRoot.profileChoices
                                    }

                                    QQC2.Button {
                                        Layout.alignment: Qt.AlignVCenter
                                        visible: drawerRoot.editingRuleIndex >= 0
                                        text: qsTr("Cancel")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        onClicked: {
                                            drawerRoot.editingRuleIndex = -1
                                            classInput.text = ""
                                            titleInput.text = ""
                                            titleRegexCheck.checked = false
                                            faviconUrlInput.text = ""
                                        }
                                    }

                                    QQC2.Button {
                                        id: saveRuleBtn
                                        objectName: "saveAutoSwitchRuleButton"
                                        Layout.alignment: Qt.AlignVCenter
                                        icon.name: drawerRoot.editingRuleIndex >= 0 ? "document-save" : "list-add"
                                        text: drawerRoot.editingRuleIndex >= 0 ? qsTr("Update Rule") : qsTr("Save Rule")
                                        font.pointSize: 10 * drawerRoot.fontScale
                                        highlighted: true
                                        enabled: classInput.text.trim().length > 0
                                            && targetProfileCombo.currentIndex >= 0
                                            && drawerRoot.titleRegexError.length === 0
                                        onClicked: {
                                            if (!drawerRoot.controller) return
                                            const profNum = drawerRoot.profileNumbers[targetProfileCombo.currentIndex] || 1
                                            drawerRoot.controller.saveAutoSwitchRuleFull(
                                                drawerRoot.editingRuleIndex,
                                                true,
                                                classInput.text.trim(),
                                                titleInput.text.trim(),
                                                titleRegexCheck.checked,
                                                faviconUrlInput.text.trim(),
                                                profNum
                                            )
                                            drawerRoot.editingRuleIndex = -1
                                            classInput.text = ""
                                            titleInput.text = ""
                                            titleRegexCheck.checked = false
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
                            id: configuredRulesHeaderLabel
                            Layout.alignment: Qt.AlignVCenter
                            text: drawerRoot.ruleFilterText.trim().length > 0
                                ? qsTr("Configured Rules (%1 of %2)").arg(rulesModel.count).arg(drawerRoot.totalRuleCount)
                                : qsTr("Configured Rules (%1)").arg(drawerRoot.totalRuleCount)
                            font.bold: true
                            font.pointSize: 11 * drawerRoot.fontScale
                            color: Kirigami.Theme.textColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: rulesHeaderHover.hovered
                            QQC2.ToolTip.text: rulesListView.canReorder
                                ? qsTr("Drag to Reorder • Double-click to Edit • Right-click for Options")
                                : qsTr("Double-click to Edit • Right-click for Options")
                            HoverHandler { id: rulesHeaderHover }
                        }

                        QQC2.TextField {
                            id: ruleFilterField
                            objectName: "ruleFilterField"
                            Layout.minimumWidth: 160
                            Layout.preferredWidth: 250
                            Layout.alignment: Qt.AlignVCenter
                            placeholderText: qsTr("Filter rules by class, title, URL, or profile...")
                            selectByMouse: true
                            font.pointSize: 10 * drawerRoot.fontScale
                            rightPadding: clearRuleFilterBtn.visible ? 28 : 8
                            onTextChanged: {
                                drawerRoot.ruleFilterText = text
                                drawerRoot.syncFromController()
                            }

                            QQC2.ToolButton {
                                id: clearRuleFilterBtn
                                objectName: "clearRuleFilterButton"
                                visible: ruleFilterField.text.length > 0
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
                                font.pointSize: 9 * drawerRoot.fontScale
                                onClicked: {
                                    ruleFilterField.text = ""
                                    ruleFilterField.forceActiveFocus()
                                }
                                QQC2.ToolTip.delay: 500
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Clear filter")
                            }
                        }
                    }

                    // Fully Expanded Rules List Container with Drag-and-Drop Reordering & Right-Click Context Menu
                    Rectangle {
                        id: rulesListContainer
                        Layout.fillWidth: true
                        readonly property int fullListHeight: rulesModel.count > 0
                            ? (rulesModel.count * 66 + 6)
                            : 72
                        implicitHeight: fullListHeight
                        Layout.minimumHeight: fullListHeight
                        Layout.preferredHeight: fullListHeight
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
                            interactive: false
                            cacheBuffer: 100000
                            model: rulesModel
                            spacing: 6
                            boundsBehavior: Flickable.StopAtBounds
                            highlightFollowsCurrentItem: false

                            property int draggedRuleIndex: -1
                            property int dropTargetRuleIndex: -1
                            readonly property bool canReorder: drawerRoot.ruleFilterText.trim().length === 0

                            QQC2.Label {
                                anchors.centerIn: parent
                                visible: rulesListView.count === 0
                                text: drawerRoot.totalRuleCount > 0
                                    ? qsTr("No configured rules match \"%1\"").arg(drawerRoot.ruleFilterText.trim())
                                    : qsTr("No auto-switcher rules configured yet.")
                                font.pointSize: 11 * drawerRoot.fontScale
                                color: Kirigami.Theme.disabledTextColor
                            }

                            delegate: Item {
                                id: ruleDelegate
                                objectName: "autoSwitchRuleDelegate"
                                required property int index
                                required property int ruleIndex
                                required property bool ruleEnabled
                                required property string resourceClass
                                required property string titleContains
                                required property bool titleIsRegex
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
                                width: Math.max(200, rulesListView.width - 4)
                                height: 60
                                z: isBeingDragged ? 100 : 1

                                QQC2.ToolTip.visible: ruleDragMouse.containsMouse && !ruleDragMouse.dragging && !ruleEnableCheck.hovered
                                QQC2.ToolTip.delay: 300
                                QQC2.ToolTip.text: rulesListView.canReorder
                                    ? qsTr("Drag to Reorder • Double-click to Edit • Right-click for Options")
                                    : qsTr("Double-click to Edit • Right-click for Options")

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
                                            drawerRoot.contextMenuTitleIsRegex = ruleDelegate.titleIsRegex
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
                                                ruleDelegate.targetProfile,
                                                ruleDelegate.titleIsRegex
                                            )
                                        }
                                    }

                                    onPositionChanged: (mouse) => {
                                        if (!pressed || !(mouse.buttons & Qt.LeftButton) || !rulesListView.canReorder) return
                                        if (!dragging && Math.abs(mouse.y - pressY) > 5) {
                                            dragging = true
                                            rulesListView.draggedRuleIndex = ruleDelegate.index
                                            rulesListView.dropTargetRuleIndex = ruleDelegate.index
                                        }
                                        if (dragging) {
                                            const posInFlick = ruleDragMouse.mapToItem(mainFlickable, mouse.x, mouse.y)
                                            if (posInFlick.y < 32 && mainFlickable.contentY > 0) {
                                                mainFlickable.contentY = Math.max(0, mainFlickable.contentY - 12)
                                            } else if (posInFlick.y > mainFlickable.height - 32) {
                                                const maxScroll = Math.max(0, mainFlickable.contentHeight - mainFlickable.height)
                                                mainFlickable.contentY = Math.min(maxScroll, mainFlickable.contentY + 12)
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
                                            if (drawerRoot.controller && fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                if (drawerRoot.editingRuleIndex === fromIdx) {
                                                    drawerRoot.editingRuleIndex = toIdx
                                                }
                                                const savedY = mainFlickable.contentY
                                                drawerRoot.controller.reorderAutoSwitchRule(fromIdx, toIdx)
                                                mainFlickable.contentY = savedY
                                            }
                                        }
                                    }

                                    onCanceled: {
                                        dragging = false
                                        dragOffsetX = 0
                                        dragOffsetY = 0
                                        rulesListView.draggedRuleIndex = -1
                                        rulesListView.dropTargetRuleIndex = -1
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
                                                font.pointSize: 13 * drawerRoot.fontScale
                                                color: ruleDragMouse.containsMouse || ruleDelegate.isBeingDragged
                                                    ? Kirigami.Theme.textColor
                                                    : Kirigami.Theme.disabledTextColor
                                            }
                                        }

                                        HighContrastCheckBox {
                                            id: ruleEnableCheck
                                            font.pointSize: 10 * drawerRoot.fontScale
                                            Layout.alignment: Qt.AlignVCenter
                                            checked: ruleEnabled
                                            onToggled: {
                                                if (drawerRoot.controller) {
                                                    drawerRoot.controller.toggleAutoSwitchRule(ruleIndex, checked)
                                                }
                                            }
                                            QQC2.ToolTip.delay: 500
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
                                                    font.pointSize: 11 * drawerRoot.fontScale
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
                                                        text: titleIsRegex
                                                            ? qsTr("regex: /%1/").arg(titleContains)
                                                            : qsTr("title: \"%1\"").arg(titleContains)
                                                        font.family: titleIsRegex ? "monospace" : ""
                                                        font.pointSize: 10 * drawerRoot.fontScale
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
                                                font.pointSize: 10 * drawerRoot.fontScale
                                                font.weight: Font.Medium
                                                elide: Text.ElideRight
                                                color: Kirigami.Theme.textColor
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
                    font.pointSize: 10 * drawerRoot.fontScale
                    icon.name: "window-close"
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: drawerRoot.close()
                }
            }
        }
    }
}
