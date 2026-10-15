import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Kirigami.InlineMessage {
    id: udevBanner
    property var controller: null

    Layout.fillWidth: true
    type: Kirigami.MessageType.Warning
    visible: controller ? controller.udevRuleMissing : false
    text: qsTr("duckyPad Pro detected, but Linux USB permissions (udev rules) are not installed yet.")

    actions: [
        Kirigami.Action {
            icon.name: "security-high"
            text: qsTr("Install Rules Automatically (Polkit)")
            onTriggered: {
                if (udevBanner.controller) {
                    udevBanner.controller.installUdevRules()
                }
            }
        }
    ]
}
