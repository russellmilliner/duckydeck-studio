import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Rectangle {
    id: msgRoot
    property int type: 0
    property string text: ""
    property list<QtObject> actions

    implicitHeight: visible ? contentRow.implicitHeight + 16 : 0
    radius: 6
    color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.14)
    border.width: 1
    border.color: Kirigami.Theme.neutralTextColor

    RowLayout {
        id: contentRow
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        QQC2.Label {
            Layout.fillWidth: true
            text: msgRoot.text
            color: Kirigami.Theme.textColor
            wrapMode: Text.WordWrap
        }

        Repeater {
            model: msgRoot.actions
            delegate: QQC2.Button {
                required property QtObject modelData
                action: modelData
            }
        }
    }
}
