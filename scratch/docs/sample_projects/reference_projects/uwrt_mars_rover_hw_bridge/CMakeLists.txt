if (DEFINED CATKIN_DEVEL_PREFIX) # Detects if being built in a catkin context
    set(CATKIN ON)
endif ()

if ("$ENV{TARGET}" MATCHES "2021$")
set(YEAR 2021)
elseif ("ENV{TARGET}" MATCHES "2022$")
set(YEAR 2022)
else ()
set(YEAR 2021) # default year to 2021
endif ()

add_library(uwrt-mars-rover-hw-bridge)

if (YEAR EQUAL 2021)
target_sources(uwrt-mars-rover-hw-bridge 
    PRIVATE 
    src/hw_bridge.cpp 
    can/CANMsgMap.cpp 
    can/generated_2021/uwrt_mars_rover_can_wrapper.cpp
    can/generated_2021/uwrt_mars_rover_can.c
    )
elseif (YEAR EQUAL 2022)
target_sources(uwrt-mars-rover-hw-bridge 
    PRIVATE 
    src/hw_bridge.cpp 
    can/CANMsgMap.cpp 
    can/generated_2022/uwrt_mars_rover_can_wrapper.cpp
    can/generated_2022/uwrt_mars_rover_can.c
    )
endif ()

if (CATKIN)
    if (YEAR EQUAL 2021)
    target_include_directories(uwrt-mars-rover-hw-bridge 
        PUBLIC 
        include 
        can 
        can/generated_2021
        )
    elseif (YEAR EQUAL 2022)
    target_include_directories(uwrt-mars-rover-hw-bridge 
        PUBLIC 
        include 
        can 
        can/generated_2022
        )
    endif ()
    target_compile_definitions(uwrt-mars-rover-hw-bridge PRIVATE CATKIN)
else () #catkin_lint: skip (This else block isn't executed by catkin)
    if (YEAR EQUAL 2021)
    target_include_directories(uwrt-mars-rover-hw-bridge 
        PUBLIC 
        include/uwrt_mars_rover_utils 
        can 
        can/generated_2021
        )
    elseif (YEAR EQUAL 2022)
    target_include_directories(uwrt-mars-rover-hw-bridge 
        PUBLIC 
        include/uwrt_mars_rover_utils 
        can 
        can/generated_2022
        )
    endif ()
endif ()

# This needs to be here instead of in uwrt_mars_rover_utils ROS pkg because of older version of CMake included with Ubuntu 18.04
if (CATKIN)
    ## Mark libraries for installation
    install(TARGETS
            uwrt-mars-rover-hw-bridge
            ARCHIVE DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
            LIBRARY DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
            RUNTIME DESTINATION ${CATKIN_GLOBAL_BIN_DESTINATION}
            )

    ## Mark cpp header files for installation
    install(DIRECTORY include/${PROJECT_NAME}/
            DESTINATION ${CATKIN_PACKAGE_INCLUDE_DESTINATION}
            FILES_MATCHING
            PATTERN "*.h"
            PATTERN "*.hpp"
            PATTERN ".svn" EXCLUDE
            )
endif ()

