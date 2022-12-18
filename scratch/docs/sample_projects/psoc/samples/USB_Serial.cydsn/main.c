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
    ADC_SAR_Start();
    ADC_SAR_StartConvert();
   // const int thermocouple_input = A1;	/* AD595 O/P pin */
    /* Place your initialization/startup code here (e.g. MyInst_Start()) */

    for(;;)
    {   
        int val= ADC_SAR_GetResult16();
        char send[100];
        sprintf(send,"%d\r\n",val);
        
        USBUART_1_PutString(send);
        CyDelay(100);
        }
      
}

/* [] END OF FILE */
  