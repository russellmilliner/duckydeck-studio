import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.Label {
    property int level: 1
    font.pointSize: level === 1 ? 15 : (level === 2 ? 13 : (level === 3 ? 11.5 : (level === 4 ? 10.5 : 10)))
    font.weight: Font.DemiBold
    color: Kirigami.Theme.textColor
}
