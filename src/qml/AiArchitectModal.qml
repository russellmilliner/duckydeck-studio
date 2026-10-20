import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Window {
    id: aiDialog
    title: qsTr("Macro Template Generator (experimental) — DuckyDeck Studio")
    width: 600
    height: 440
    minimumWidth: 500
    minimumHeight: 360
    visible: false
    transientParent: root
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    property var root: null
    property var controller: null
    readonly property bool closableWithEscape: true
    readonly property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0

    property var scrapedShortcuts: []

    function refreshScrapedShortcuts() {
        if (!aiDialog.controller || !scrapeToggle.checked) {
            aiDialog.scrapedShortcuts = []
            return
        }
        try {
            const raw = aiDialog.controller.scrapeAppShortcutsJson(appInput.text)
            const arr = JSON.parse(raw || "[]")
            aiDialog.scrapedShortcuts = Array.isArray(arr) ? arr : []
        } catch (e) {
            aiDialog.scrapedShortcuts = []
        }
    }

    function open() {
        aiDialog.refreshScrapedShortcuts()
        aiDialog.show()
        aiDialog.raise()
        aiDialog.requestActivate()
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

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: aiDialog.visible
        onActivated: aiDialog.close()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        QQC2.Label {
            Layout.fillWidth: true
            font.bold: true
            font.pointSize: 12 * aiDialog.fontScale
            text: qsTr("Macro Template Generator (experimental)")
            color: Kirigami.Theme.textColor
            elide: Text.ElideRight
        }

        Flickable {
            id: aiFlickable
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            contentWidth: width
            contentHeight: aiFormCol.implicitHeight + 8
            boundsBehavior: Flickable.StopAtBounds
            flickableDirection: Flickable.VerticalFlick

            QQC2.ScrollBar.vertical: HighContrastScrollBar {
                parent: aiFlickable
                anchors.top: parent.top
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                policy: aiFlickable.contentHeight > aiFlickable.height
                    ? QQC2.ScrollBar.AlwaysOn
                    : QQC2.ScrollBar.AsNeeded
            }

            ColumnLayout {
                id: aiFormCol
                width: Math.max(
                    200,
                    aiFlickable.width - (aiFlickable.contentHeight > aiFlickable.height ? 16 : 2)
                )
                spacing: 10

                QQC2.Label {
                    Layout.fillWidth: true
                    font.bold: true
                    font.pointSize: 10 * aiDialog.fontScale
                    text: qsTr("Target Application / Game:")
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: appInput
                    Layout.fillWidth: true
                    font.pointSize: 10 * aiDialog.fontScale
                    text: "Kdenlive"
                    placeholderText: qsTr("e.g., Spectacle, Kdenlive, Dolphin, KWin")
                    onTextChanged: aiDialog.refreshScrapedShortcuts()
                }

                HighContrastCheckBox {
                    id: scrapeToggle
                    Layout.fillWidth: true
                    font.pointSize: 10 * aiDialog.fontScale
                    checked: true
                    text: qsTr("Scrape ~/.config/kglobalshortcutsrc for local KDE keybindings")
                    onToggled: aiDialog.refreshScrapedShortcuts()
                }

                QQC2.Label {
                    visible: scrapeToggle.checked
                    Layout.fillWidth: true
                    font.bold: true
                    font.pointSize: 10 * aiDialog.fontScale
                    text: aiDialog.scrapedShortcuts.length > 0
                        ? qsTr("Matching KDE Shortcuts (%1 found — select one):").arg(aiDialog.scrapedShortcuts.length)
                        : qsTr("Matching KDE Shortcuts (none found for filter; default template will be used):")
                    color: Kirigami.Theme.textColor
                }

                QQC2.ComboBox {
                    id: shortcutCombo
                    visible: scrapeToggle.checked && aiDialog.scrapedShortcuts.length > 0
                    Layout.fillWidth: true
                    font.pointSize: 10 * aiDialog.fontScale
                    model: {
                        const list = aiDialog.scrapedShortcuts || []
                        const out = []
                        for (let i = 0; i < list.length; ++i) {
                            const it = list[i]
                            out.push(it.action + "  [" + it.shortcut + "  →  " + it.duckyscript + "]")
                        }
                        return out
                    }
                }

                QQC2.Label {
                    Layout.fillWidth: true
                    font.bold: true
                    font.pointSize: 10 * aiDialog.fontScale
                    text: qsTr("Macro / Layout Goal:")
                    color: Kirigami.Theme.textColor
                }

                T.TextArea {
                    id: promptInput
                    Layout.fillWidth: true
                    implicitHeight: 90
                    padding: 6
                    font.pointSize: 10 * aiDialog.fontScale
                    wrapMode: TextEdit.Wrap
                    selectByMouse: true
                    color: Kirigami.Theme.textColor
                    selectionColor: Kirigami.Theme.highlightColor
                    selectedTextColor: Kirigami.Theme.highlightedTextColor
                    background: Rectangle {
                        radius: 4
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.width: 1
                        border.color: promptInput.activeFocus
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, 0.25)
                    }
                    text: "Create a timeline razor cut & ripple delete macro with OLED HUD confirmation."
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Item { Layout.fillWidth: true }

            QQC2.Button {
                icon.name: "dialog-cancel"
                text: qsTr("Cancel")
                font.pointSize: 10 * aiDialog.fontScale
                onClicked: aiDialog.close()
            }

            QQC2.Button {
                icon.name: "dialog-ok-apply"
                text: qsTr("Generate Macro")
                highlighted: true
                font.bold: true
                font.pointSize: 10 * aiDialog.fontScale
                onClicked: {
                    if (aiDialog.controller) {
                        let effectivePrompt = promptInput.text
                        if (scrapeToggle.checked && aiDialog.scrapedShortcuts.length > 0) {
                            const idx = Math.max(0, Math.min(shortcutCombo.currentIndex, aiDialog.scrapedShortcuts.length - 1))
                            const picked = aiDialog.scrapedShortcuts[idx]
                            if (picked && picked.action && picked.shortcut) {
                                effectivePrompt = "[SHORTCUT:" + picked.action + "|" + picked.shortcut + "]\n" + promptInput.text
                            }
                        }
                        aiDialog.controller.generateAiMacro(appInput.text, effectivePrompt)
                    }
                    aiDialog.close()
                }
            }
        }
    }
}
