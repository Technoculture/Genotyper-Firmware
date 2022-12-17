#include"dna_extractor.h"

Servo  servo[2]; 

MCP_CAN CAN(SPI_CS_PIN); 


uint32_t current_time = 0;
uint32_t prev_time = 0;
uint32_t interval =5000;
unsigned char dna_st[1] = {00};
uint8_t dna_ext_status=0;
bool STOP_MOTOR = false;
uint8_t pos = 0;
bool motor_position = UP;


uint32_t time_current = 0;
uint32_t time_prev =0;
uint32_t waiting_interval = 1000;


/* setup */
void setup()
{
Serial.begin(BAUDRATE);
while(!Serial);
   
while (CAN_OK != CAN.begin(CAN_500KBPS))    // init can bus : baudrate = 500k
   {
      DEBUG("CAN BUS FAIL ON DNA EXTRACTOR!");
      delay(100);
   }
DEBUG("CAN BUS OK ON DNA EXTRACTOR!");

/* digital pins for motor*/

pinMode(MOTOR_PIN0, OUTPUT);
pinMode(MOTOR_PIN1, OUTPUT);

/* interrupt enabled optical limit switches*/

pinMode(LIMIT_SW_BOTTOM,INPUT);
attachInterrupt(digitalPinToInterrupt(LIMIT_SW_BOTTOM),stop_motor_isr,RISING);
pinMode(LIMIT_SW_TOP,INPUT);
attachInterrupt(digitalPinToInterrupt(LIMIT_SW_TOP),stop_motor_isr,RISING);

/* initial homing : go down*/
DEBUG("homing");
while(digitalRead(LIMIT_SW_BOTTOM)==LOW)
 {
   digitalWrite(MOTOR_PIN0,HIGH);
   digitalWrite(MOTOR_PIN1,LOW);
 }

DEBUG("reached home");
dna_ext_status = TASK_RUNNING;
HOME_POS;
}
/* Setup ends here */

/* Super loop starts here*/
void loop()
{
//TODO: check can messages on regular interval
//   if(CAN_MSGAVAIL == CAN.checkReceive())            // check if data coming
//   {
//     check_can_msg(); 
//   }
// //TODO: send status of the system on regular interval
// current_time = millis();
// if(current_time - prev_time >= interval)
// {
//   dna_st[0] = dna_ext_status;
//   CAN.sendMsgBuf(DNA_EXT_STATUS_ID, 0, sizeof(dna_st), dna_st);
//   DEBUG("DNA Extractor  status  sent");
//   prev_time = current_time;
// }

//TODO: position the scara hand on certain chamber
if(dna_ext_status == TASK_RUNNING)
  {
    if(pos<=5)
    {
      servo_set_pos(servo0_pos[pos],servo1_pos[pos]);
      time_current = millis();
      if(time_current- time_prev >= waiting_interval)
      {

      if(motor_position == DOWN)
        {
  //TODO: move to top untill limit switch is hit          
          move_z(UP);
          DEBUG("moving up");
        }
  //TODO: wait for certain time  
  //TODO: move to bottom untill limit switch is hit
        else  if(motor_position == UP)
        {
          move_z(DOWN);
          DEBUG("moving down");
        }
      time_prev = time_current ;
      }
    }
    else 
    {
      HOME_POS;
      dna_ext_status = IDLE;
      DEBUG("HOME POSITION REACHED");
    }
  }   
//TODO: repeat the process

}
  /* Super Loop ends here*/

inline void stop_motor_isr()
{
STOP_MOTOR = true;
digitalWrite(MOTOR_PIN0,LOW);
digitalWrite(MOTOR_PIN1,LOW);
if(motor_position == UP){ motor_position = DOWN; pos++; }
else motor_position = UP;
}

// void check_can_msg()
// {
//   DEBUG("CAN receive callback triggered.");
//   unsigned char len = 0;
//   unsigned char buf[1];
//   uint32_t canId = CAN.getCanId();
//   switch(canId)
//     {
//       case DNA_EXT_CONTROL_ID: 
//         CAN.readMsgBuf(&len, buf);    // read data,  len: data length, buf: data buf 
//         if (buf[0] == START) 
//           {
//             dna_ext_status = TASK_RUNNING;
//           }
//         else if (buf[0] == STOP) 
//           { 
//             dna_ext_status = IDLE;
//           }
//         break;
//       default : DEBUG("invalid can id ");
//                 break;
//     }
// Serial.print("msg from can id ");
// DEBUG(canId);
// }

void move_z(bool dir)
{
  if(dir == UP)
  {
    digitalWrite(MOTOR_PIN1,HIGH);
    digitalWrite(MOTOR_PIN0,LOW);
  }
  else
  {
    digitalWrite(MOTOR_PIN0,HIGH);
    digitalWrite(MOTOR_PIN1,LOW);
  }
}

inline void servo_set_pos(const uint8_t  &servo0pos, const uint8_t &servo1pos){
  /*
   * pos: [0 - 180] indicates angle in degrees
  */
  servo[0].write(servo0pos);
  servo[1].write(servo1pos);
}

