import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.CheckBox {
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
