#include "dna_extractor.h"


const int SPI_CS_PIN = 10;              // CANBed V1
uint32_t current_time = 0;
uint32_t prev_time = 0;
uint32_t interval =5000;
uint8_t dna_ext_status = 0;
unsigned char dna_st[1] = {IDLE};

MCP_CAN CAN(SPI_CS_PIN);                                    // Set CS pin

inline void toggle_dna_ext_status_isr(){
    DEBUG("CAN transmit callback triggered.");
  if(dna_ext_status == IDLE)    
  {
   dna_st[0] = TASK_RUNNING;
   CAN.sendMsgBuf(DNA_EXT_CONTROL_ID, 0, sizeof(dna_st), dna_st); 
   DEBUG("dna extractor start");
  }  
  else if(dna_ext_status == TASK_RUNNING)
  {
    dna_st[0] = IDLE;
    CAN.sendMsgBuf(DNA_EXT_CONTROL_ID, 0, sizeof(dna_st), dna_st); 
    DEBUG("dna extractor stop");
  }
}  

void setup()
{
    Serial.begin(BAUDRATE);
    while(!Serial);
    
    while (CAN_OK != CAN.begin(CAN_500KBPS))    // init can bus : baudrate = 500k
    {
        Serial.println("CAN BUS FAIL ON DNA EXTRACTOR!");
        delay(100);
    }
    Serial.println("CAN BUS OK ON DNA EXTRACTOR!");

  pinMode(DNA_EXT_CTRL_PIN,INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(DNA_EXT_CTRL_PIN),toggle_dna_ext_status_isr,FALLING);

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
          case DNA_EXT_STATUS_ID : CAN.readMsgBuf(&len, buf);    // read data,  len: data length, buf: data buf
                                dna_ext_status= buf[0];
                                DEBUG(dna_ext_status); 
                                break;
         default : break;
        }
      Serial.print("msg from can id  ");
        DEBUG(canId);
    }
  
    prev_time = current_time;
   } 
//DEBUG(dna_ext_status);  
}
  