#pragma once

// CAN includes
#include "CANMsgMap.h"
#include "uwrt_mars_rover_can.h"
#include "uwrt_mars_rover_can_enums.h"
#include "uwrt_mars_rover_can_wrapper.h"

namespace HWBRIDGE {

namespace CONTROL {
enum class Mode { OPEN_LOOP, POSITION, VELOCITY, CURRENT };
}  // namespace CONTROL

namespace LEDMATRIX {
enum class LEDMatrixState { SOLID_RED, SOLID_BLUE, SOLID_GREEN, FLASHING_RED, FLASHING_BLUE, FLASHING_GREEN, OFF };
}  // namespace LEDMATRIX
}  // namespace HWBRIDGE
