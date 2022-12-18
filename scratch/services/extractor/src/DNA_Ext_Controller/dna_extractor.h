#ifndef DNA_EXTRACTOR_H
#define DNA_EXTRACTOR_H

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
#define DNA_EXT_CTRL_PIN        5

#define BAUDRATE                115200

#define HOME_POS                servo_set_pos(90,90)

#define DELAY_MS                1000

#define DNA_EXT_CONTROL_ID      00
#define DNA_EXT_STATUS_ID       01


enum status {
  IDLE = 00,
  TASK_RUNNING,
  TASK_COMPLETE
};




#endif // DNA_EXTRACTOR_H