import QtQuick 2.0
import QtQuick.Layouts 1.3

Item {
    property string btn_text: "Button"
    property color light_green_low: "#C4FFCE"
    property color light_green_high: "#A8FFB7"
    property color light_green_highlight: "#61F479"
    property color dark_green_low: "#0B9821"
    property color dark_green_high: "#1A8502"

    height: 80

    Rectangle {
        id: rectangle
        anchors.fill: parent
        radius: 20
        border.color: light_green_highlight

        gradient: Gradient {
            id: light_green_gradient
            GradientStop { position: 0.0; color: light_green_low }
            GradientStop { position: 1.0; color: light_green_high }
        }

        RowLayout {
            spacing: 40
            anchors.verticalCenter: parent.verticalCenter

            anchors.horizontalCenter: parent.horizontalCenter

            Text {
                id: text_field
                text: btn_text

                font.pixelSize: 40
                horizontalAlignment: Text.AlignLeft
                verticalAlignment: Text.AlignTop

                font.weight: Font.Light
                font.family: "SF Pro"
                color: dark_green_high
            }

            Image {
                source: "/icons/ArrowCircleRight.png"
            }
        }
    }
}
