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

#define SPI_CS_PIN      10
#define SERVO_0_PIN     2  // SERVO_0
#define SERVO_1_PIN     3 // SERVO_1
#define MOTOR_PIN0      11
#define MOTOR_PIN1      12

#define LIMIT_SW_BOTTOM 6
#define LIMIT_SW_TOP    7
#define BAUDRATE        115200

#define HOME_POS                servo_set_pos(90,90)
#define DELAY_MS                1000

#define DNA_EXT_CONTROL_ID      00
#define DNA_EXT_STATUS_ID       01

#define START  1
#define STOP   0 


#define X1 10
#define X2 20
#define X3 30
#define X4 40 
#define X5 50
#define X6 60

#define Y1 10
#define Y2 20
#define Y3 30
#define Y4 40
#define Y5 50
#define Y6 60

uint8_t servo0_pos[6] = {X1, X2, X3, X4, X5, X6};
uint8_t servo1_pos[6] = {Y1, Y2, Y3, Y4, Y5, Y6};



enum status {
  IDLE = 0,
  TASK_RUNNING,
  TASK_COMPLETE
};
enum z_direction{
  DOWN=0, UP=1
};

/* functions*/

void check_can_msg();
void move_z(bool dir);
void servo_set_pos(const uint8_t  &servo0pos, const uint8_t &servo1pos);
void stop_motor_isr();
#endif