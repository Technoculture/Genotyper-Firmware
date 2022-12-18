cmake_minimum_required(VERSION 3.16)

project(hmi LANGUAGES CXX)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt5 
    REQUIRED COMPONENTS 
    Core 
    Gui
    Qml
    Quick 
    SerialPort
)

set(PROJECT_SOURCES
    main.cpp
    qml.qrc
)
add_executable(hmi ${PROJECT_SOURCES})

# What does this even do?
target_compile_definitions(hmi
     PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>
)

target_link_libraries(hmi
    PUBLIC
    Qt5::Core 
    Qt5::Gui
    Qt5::Qml
    Qt5::Quick
    Qt5::SerialPort
)

# Compile Commands in Debug Builds
if(CMAKE_BUILD_TYPE STREQUAL Debug)
    message("Debug build: Generate compile commands database")
    set(CMAKE_EXPORT_COMPILE_COMMANDS true)
endif()

# set_target_properties(hmi PROPERTIES OUTPUT_NAME hmi-Debug)

# TODO: No idea what this does
install(
    TARGETS hmi
    RUNTIME
        DESTINATION bin
        COMPONENT runtime
)
