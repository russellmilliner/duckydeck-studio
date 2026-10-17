import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Window {
    id: aiDialog
    title: qsTr("AI Profile Architect & Shortcut Scraper — DuckyDeck Studio")
    width: 600
    height: 400
    minimumWidth: 500
    minimumHeight: 340
    visible: false
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    property var controller: null
    readonly property bool closableWithEscape: true
    readonly property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0

    function open() {
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

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: aiDialog.visible
        onActivated: aiDialog.close()
        onAmbiguouslyActivated: aiDialog.close()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        QQC2.Label {
            Layout.fillWidth: true
            font.bold: true
            font.pointSize: 12 * aiDialog.fontScale
            text: qsTr("AI Profile Architect & Shortcut Scraper")
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
                    placeholderText: qsTr("e.g., Blender, Kdenlive, Helldivers 2, FreeCAD")
                }

                QQC2.CheckBox {
                    id: scrapeToggle
                    Layout.fillWidth: true
                    font.pointSize: 10 * aiDialog.fontScale
                    checked: true
                    text: qsTr("Scrape ~/.config/kglobalshortcutsrc for local KDE keybindings")
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
                        aiDialog.controller.generateAiMacro(appInput.text, promptInput.text)
                    }
                    aiDialog.close()
                }
            }
        }
    }
}
