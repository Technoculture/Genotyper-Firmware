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
    CyGlobalIntEnable; /* Enable global interrupts. */
    USBUART_1_Start(0,USBUART_1_5V_OPERATION);
    while(USBUART_1_GetConfiguration()==0){}
    /* Place your initialization/startup code here (e.g. MyInst_Start()) */
    ADC_Start(); 
    for(;;)
    {
        ADC_StartConvert();
        ADC_IsEndConversion( ADC_WAIT_FOR_RESULT);
        int val=ADC_GetResult32();
        ADC_StopConvert();
         char send[100];
        sprintf(send,"%d\r\n",val);
        
        USBUART_1_PutString(send);
        
        CyDelay(50);   /* Place your application code here. */
    
    }
}

/* [] END OF FILE */
