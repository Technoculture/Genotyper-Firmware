/* ========================================
 *
 * Copyright YOUR COMPANY, THE YEAR
 * All Rights Reserved
 * UNPUBLISHED, LICENSED SOFTWARE.
 *
 * CONFIDENTIAL AND PROPRIETARY INFORMATION
 * WHICH IS THE PROPERTY OF your company.
 *
 * ========================================
*/
#include "project.h"
#include<stdio.h>

int main(void) 
{
    CyGlobalIntEnable; 
     float temperature;
    int temp;
    COM_port_Start(0,COM_port_5V_OPERATION);
    char send[100];
    while(COM_port_GetConfiguration()==0){}
     ADC_Start(); 
    for(;;)
    {
       
        ADC_StartConvert();
        ADC_IsEndConversion( ADC_WAIT_FOR_RESULT);
        int adc_val=ADC_GetResult32();
        ADC_StopConvert();
        temperature = (( ((float)adc_val * 4.88) - 0.0027 ) / 10.0 );
        temp =(int) temperature/100;
        sprintf(send,"%d\r\n",temp);
        COM_port_PutString(send);
        CyDelay(100);
    }
}

/* [] END OF FILE */
