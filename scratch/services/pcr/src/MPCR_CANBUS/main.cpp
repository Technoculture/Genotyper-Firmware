#include "mpcr.h"

// Globals
const int SPI_CS_PIN = 10;  
uint8_t chamber_1_temp =  95;
uint8_t chamber_2_temp =  45;  
uint8_t cycle_count = 0;
float temperature=0;
int adc_val;
uint8_t pcr_status = 0;
uint8_t swab_position = 0;
uint32_t current_time = 0;
uint32_t prev_time = 0;
uint32_t interval =5000;
unsigned char pcr_st[1] = {00};


Servo  servo[2];
MCP_CAN CAN(SPI_CS_PIN);  

inline void servo_set_pos(const uint8_t  &pos)
{
  /*
   * pos: [0 - 180] indicates angle in degrees
  */
  servo[0].write(pos);
  servo[1].write(pos);
}

// inline void toggle_pcr_status_isr()
// {
//   if(pcr_status == IDLE)               pcr_status = TASK_RUNNING;
//   else if(pcr_status == TASK_RUNNING)  pcr_status = TASK_COMPLETE;
// }

float adc_to_degC(const uint32_t& adc_val) { return ( (((float)adc_val * 3.33) - 0.0027 ) / 10.0 ); }


void check_can_msg()
{
  DEBUG("CAN receive callback triggered.");
  unsigned char len = 0;
  unsigned char buf[1];
  unsigned long canId = CAN.getCanId();
  switch(canId)
    {
      case PCR_CONTROL_ID: CAN.readMsgBuf(&len, buf);    // read data,  len: data length, buf: data buf 
                          if (buf[0] == START) 
                            {
                              pcr_status = TASK_RUNNING;
                              pcr_st[0] = TASK_RUNNING; 
                            }
                          else if (buf[0] == STOP) 
                            { 
                              pcr_status = IDLE;
                              pcr_st[0] = IDLE;
                            }
                          break;
      default : DEBUG("invalid can id ");
                break;
    }
        Serial.print("msg from can id  ");
        DEBUG(canId);
}

void setup(){
  // Setup Peripherals
  Serial.begin(BAUDRATE);
  while(!Serial){}

  servo[0].attach(SERVO_0_PIN);
  servo[1].attach(SERVO_1_PIN);

  pinMode(THERMOCOUPLE_PIN,INPUT);

  // pinMode(PCR_CONTROL_PIN,INPUT_PULLUP);
  // attachInterrupt(digitalPinToInterrupt(PCR_CONTROL_PIN),toggle_pcr_status_isr,FALLING);

  while (CAN_OK != CAN.begin(CAN_500KBPS))    // init can bus : baudrate = 500k
    {
        DEBUG("CAN BUS FAIL ON MPCR!");
        delay(100);
    }
    DEBUG("CAN BUS OK ON MPCR!");

         
  // Initialize Global Variables
  pcr_status = IDLE;
  swab_position = SWAB_POS_CONTAINER1;
  pcr_st[0] = IDLE;
  CAN.sendMsgBuf(PCR_STATUS_ID, 0, sizeof(pcr_st), pcr_st);
  delay(DELAY_MS);
}

void loop()
{
current_time = millis();
if(current_time - prev_time >= interval)
{
  CAN.sendMsgBuf(PCR_STATUS_ID, 0, sizeof(pcr_st), pcr_st);
  DEBUG("pcr                  status             sent");
   prev_time = current_time;
}
// TODO: check for can messages then perform accordingly
  
if(CAN_MSGAVAIL == CAN.checkReceive())            // check if data coming
    {
      check_can_msg(); 
    }


 // home position for initial case
if(pcr_status == IDLE)
  {
     pcr_st[0] = IDLE;
    servo_set_pos(HOME_POS);
    temperature =  adc_to_degC(analogRead(THERMOCOUPLE_PIN));
    DEBUG("IDLE");
    DEBUG(temperature);
    delay(50);
  }
else if(pcr_status == TASK_RUNNING)
    {
     temperature =  adc_to_degC(analogRead(THERMOCOUPLE_PIN));
      if(cycle_count <= PCR_CYCLES)
        {
      // TODO: wait to reach a predefined temperature
          if( (swab_position == SWAB_POS_CONTAINER1) && ( temperature <= chamber_1_temp ))
          { 
            servo_set_pos(CHAMBER_1_POS);
            swab_position = SWAB_POS_CONTAINER1;
            DEBUG("inside chamber 1  ");
            DEBUG(temperature);
            return;
          }

        // TODO: goto right container

          swab_position = SWAB_POS_CONTAINER2;
       // TODO: wait to reach a predefined temperature   
          if((swab_position == SWAB_POS_CONTAINER2) && (temperature >= chamber_2_temp)) 
          {
            servo_set_pos(CHAMBER_2_POS);
            DEBUG("inside chamber 2  ");
            DEBUG(temperature);
            return;
          }
  
          DEBUG(cycle_count++);
          swab_position = SWAB_POS_CONTAINER1;
          return;
        }
      else if(cycle_count >= PCR_CYCLES) 
        {
          pcr_status = TASK_COMPLETE;
          swab_position = SWAB_POS_CONTAINER1;
          cycle_count=0;
          DEBUG("PCR COMPLETE");
        }
  }
else if(pcr_status == TASK_COMPLETE) {
    pcr_status = IDLE;
     pcr_st[0] = IDLE;
    cycle_count=0;
  }
else
 {
   DEBUG("Error [Illegitimate pcr_status]: Else block of superloop reached.");
 }

 DEBUG(pcr_status);
 DEBUG(swab_position);

}