import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Kirigami.Dialog {
    id: aiDialog
    title: qsTr("AI Profile Architect & Shortcut Scraper")
    preferredWidth: 560
    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel

    property var controller: null

    onAccepted: {
        if (controller) {
            controller.generateAiMacro(appInput.text, promptInput.text)
        }
    }

    ColumnLayout {
        spacing: Kirigami.Units.largeSpacing
        width: parent.width

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: appInput
                Kirigami.FormData.label: qsTr("Target Application / Game:")
                text: "Kdenlive"
                placeholderText: qsTr("e.g., Blender, Kdenlive, Helldivers 2, FreeCAD")
            }

            QQC2.CheckBox {
                id: scrapeToggle
                Kirigami.FormData.label: qsTr("Grounding:")
                checked: true
                text: qsTr("Scrape ~/.config/kglobalshortcutsrc for local KDE keybindings")
            }

            T.TextArea {
                id: promptInput
                Kirigami.FormData.label: qsTr("Macro / Layout Goal:")
                Layout.fillWidth: true
                implicitHeight: 90
                padding: 6
                wrapMode: TextEdit.Wrap
                selectByMouse: true
                color: Kirigami.Theme.textColor
                selectionColor: Kirigami.Theme.highlightColor
                selectedTextColor: Kirigami.Theme.highlightedTextColor
                background: Rectangle {
                    radius: 4
                    color: Kirigami.Theme.backgroundColor
                    border.width: 1
                    border.color: promptInput.activeFocus
                        ? Kirigami.Theme.highlightColor
                        : Qt.alpha(Kirigami.Theme.textColor, 0.25)
                }
                text: "Create a timeline razor cut & ripple delete macro with OLED HUD confirmation."
            }
        }
    }
}
