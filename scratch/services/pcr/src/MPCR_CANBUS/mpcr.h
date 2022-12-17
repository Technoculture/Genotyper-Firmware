#ifndef MPCR_H__
#define MPCR_H__

#include<Arduino.h>
#include <Servo.h>
#include <mcp_can.h>
#include <SPI.h>


#define DEBUG_MODE

#ifdef DEBUG_MODE
#define DEBUG(x) Serial.println(x);
#endif

#define SERVO_0_PIN             2  // TOP SERVO
#define SERVO_1_PIN             3 // BOTTOM SERVO
#define PCR_CONTROL_PIN         5
#define THERMOCOUPLE_PIN        A2
#define BAUDRATE                115200

#define HOME_POS                90
#define CHAMBER_1_POS           180
#define CHAMBER_2_POS           0

#define PCR_CYCLES              10
#define DELAY_MS                1000

#define PCR_CONTROL_ID          00
#define PCR_STATUS_ID           01

enum status {
  IDLE = 0,
  TASK_RUNNING,
  TASK_COMPLETE
};

enum position{
  SWAB_POS_CONTAINER1, 
  SWAB_POS_CONTAINER2,

};
enum pcr_control {
  START =01,
  STOP
};

#endif // MPCR_H__