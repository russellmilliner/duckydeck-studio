import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Item {
    id: iconRoot
    property var source: ""
    property string fallback: ""
    property bool isMask: false
    property color color: Kirigami.Theme.textColor

    implicitWidth: 22
    implicitHeight: 22

    readonly property string srcStr: source ? source.toString() : ""
    readonly property bool isImageSource: srcStr.startsWith("http:")
        || srcStr.startsWith("https:")
        || srcStr.startsWith("qrc:")
        || srcStr.startsWith("file:")
        || srcStr.startsWith(":/")
        || srcStr.indexOf("/") !== -1
        || srcStr.indexOf("\\") !== -1

    Image {
        id: img
        anchors.fill: parent
        visible: iconRoot.isImageSource && status === Image.Ready
        source: iconRoot.isImageSource
            ? (iconRoot.srcStr.startsWith(":/") ? ("qrc" + iconRoot.srcStr) : iconRoot.srcStr)
            : ""
        fillMode: Image.PreserveAspectFit
        smooth: true
        mipmap: true
    }

    Rectangle {
        anchors.fill: parent
        radius: Math.max(4, Math.round(Math.min(width, height) * 0.22))
        visible: !img.visible
        color: iconRoot.srcStr === "input-keyboard" || iconRoot.srcStr === "network-wired"
            ? "transparent"
            : Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
        border.width: iconRoot.srcStr === "input-keyboard" || iconRoot.srcStr === "network-wired" ? 0 : 1
        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.38)

        QQC2.Label {
            anchors.centerIn: parent
            color: iconRoot.color
            font.pixelSize: Math.max(10, Math.round(Math.min(iconRoot.width, iconRoot.height) * 0.62))
            font.weight: Font.Bold
            text: {
                if (iconRoot.srcStr === "input-keyboard") return "⌨"
                if (iconRoot.srcStr === "network-wired") return "🔌"
                if (iconRoot.srcStr === "security-high") return "🛡"
                const clean = iconRoot.srcStr.replace(/^.*\./, "")
                if (clean.length > 0 && clean !== "application-x-executable") {
                    return clean.slice(0, 1).toUpperCase()
                }
                return "⚙"
            }
        }
    }
}
