import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.ScrollBar {
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
