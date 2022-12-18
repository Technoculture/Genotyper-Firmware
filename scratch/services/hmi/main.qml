import QtQuick 2.15
import QtQuick.Window 2.15

Window {
    id: main_window

    property int height: 1920
    property int width: 1024

    maximumHeight: main_window.height
    minimumHeight: main_window.height
    maximumWidth: main_window.width
    minimumWidth: main_window.width

    visible: true
    title: qsTr("Geny")

//    Rectangle {
//        height: parent.fill
//        width: parent.fill
//        color: "red"

        Text {
            id: geny_title_text
            text: qsTr("Hi!, I am Geny.")
            font.family: "Tahoma"
            font.pointSize: 50
//            horizontalAlignment: Text.AlignHCenter
//            verticalAlignment: Text.AlignVCenter
//            wrapMode: Text.WordWrap
        }
//    }
}
