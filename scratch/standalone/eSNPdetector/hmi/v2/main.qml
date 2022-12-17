import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Layouts 1.3

Window {
    id: root_window

    property int screen_width: 720
    property int screen_height: 720

    width: screen_width
    height: screen_height

    visible: true
    title: qsTr("eSNP Detector")

    //    flags: Qt.FramelessWindowHint

    property color screen_background: "white"
    color: screen_background

    Rectangle {
        id: root_background

        anchors.fill: parent
        anchors {
            rightMargin: 80
            leftMargin: 80
            bottomMargin: 80
            topMargin: 80
        }

        clip: true

        ColumnLayout {
            //            anchors.left: parent.left
            //            anchors.top: parent.top
            clip: false
            anchors.fill: parent
            spacing: 20

            ColumnLayout {

                Text {
                    text: "ADCY8"
                    font.pixelSize: 100
                    verticalAlignment: Text.AlignVCenter
                    lineHeight: 90
                    lineHeightMode: Text.FixedHeight
                    Layout.fillWidth: true
                    Layout.rowSpan: 1
                    Layout.columnSpan: 2
                    font.styleName: "Ultralight"
                    font.weight: Font.ExtraLight
                    font.family: "SF Pro Rounded"
                    font.capitalization: Font.AllUppercase
                }

                Text {
                    text: "Absolute\nPitch"
                    elide: Text.ElideNone
                    font.pixelSize: 100
                    verticalAlignment: Text.AlignVCenter
                    lineHeight: 90
                    lineHeightMode: Text.FixedHeight
                    clip: false
                    Layout.fillWidth: true
                    Layout.columnSpan: 2
                    font.capitalization: Font.Capitalize
                    font.styleName: "Regular"
                    font.weight: Font.ExtraLight
                }
            }

            RowLayout {
                spacing: 40
                //anchors.fill: parent
                Layout.minimumHeight: 240
                Layout.minimumWidth: 560

                //                Layout.fillHeight: true
                //                Layout.fillWidth: true

                ColumnLayout {
                    Layout.maximumWidth: 240

                    Text {
                        text: "(noun)"
                        font.pixelSize: 24
                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        Layout.columnSpan: 1
                        font.family: "SF Pro Rounded"
                        font.weight: Font.ExtraLight
                        font.capitalization: Font.MixedCase
                    }

                    Text {
                        width: 240
                        height: 240
                        text: "It is the ability to instantaneously identify and label tones with their musical note names, without the aid of an external reference tone."
                        elide: Text.ElideNone
                        font.pixelSize: 24
                        wrapMode: Text.Wrap
                        textFormat: Text.AutoText
                        Layout.fillHeight: false
                        Layout.fillWidth: true
                        font.family: "SF Pro Rounded"
                        font.weight: Font.Light
                        font.capitalization: Font.MixedCase
                    }
                }

                ColumnLayout {
                    Layout.alignment: Qt.AlignLeft | Qt.AlignBottom
                    FitButton {
                        Layout.rowSpan: 2
                        Layout.fillWidth: true

                        btn_text: "Start"
                    }
                }
            }

        }
    }

}
