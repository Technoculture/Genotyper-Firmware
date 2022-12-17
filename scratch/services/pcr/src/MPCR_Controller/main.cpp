#include "mpcr.h"


const int SPI_CS_PIN = 10;              // CANBed V1
uint32_t current_time = 0;
uint32_t prev_time = 0;
uint32_t interval =5000;
uint8_t pcr_status = 0;
unsigned char pcr_st[1] = {IDLE};

MCP_CAN CAN(SPI_CS_PIN);                                    // Set CS pin

inline void toggle_pcr_status_isr(){
    DEBUG("CAN transmit callback triggered.");
  if(pcr_status == IDLE)    
  {
   pcr_st[0] = TASK_RUNNING;
   CAN.sendMsgBuf(PCR_CONTROL_ID, 0, sizeof(pcr_st), pcr_st); 
   DEBUG("pcr start command passed");
  }  
  else if(pcr_status == TASK_RUNNING)
  {
    pcr_st[0] = IDLE;
    CAN.sendMsgBuf(PCR_CONTROL_ID, 0, sizeof(pcr_st), pcr_st); 
    DEBUG("pcr stop command passed");
  }
}  

void setup()
{
    Serial.begin(115200);
    while(!Serial);
    
    while (CAN_OK != CAN.begin(CAN_500KBPS))    // init can bus : baudrate = 500k
    {
        Serial.println("CAN BUS FAIL ON PCR CONTROLLER!");
        delay(100);
    }
    Serial.println("CAN BUS OK ON PCR CONTROLLER!");

  pinMode(PCR_CONTROL_PIN,INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(PCR_CONTROL_PIN),toggle_pcr_status_isr,FALLING);

}

void loop()
{
    //TODO: Check the status of mpcr in every 10 seconds
    
 current_time = millis();
 if(current_time - prev_time >= interval)
  {
   unsigned char len = 0;
    unsigned char buf[1];

    if(CAN_MSGAVAIL == CAN.checkReceive())            // check if data coming
    {
      // read data,  len: data length, buf: data buf

        unsigned long canId = CAN.getCanId();
        switch(canId)
        {
          case PCR_STATUS_ID : CAN.readMsgBuf(&len, buf);    // read data,  len: data length, buf: data buf
                                pcr_status= buf[0];
                                DEBUG(pcr_status); 
                                break;
         default : break;
        }
      Serial.print("msg from can id  ");
        DEBUG(canId);
    }
  
    prev_time = current_time;
   } 
//DEBUG(pcr_status);  
}
  