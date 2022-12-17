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

int main(void)
{
    CyGlobalIntEnable; /* Enable global interrupts. */
    PWM_Start();
    /* Place your initialization/startup code here (e.g. MyInst_Start()) */
    ADC_Start(); 
    for(;;)
    {
        ADC_StartConvert();
        ADC_IsEndConversion( ADC_WAIT_FOR_RESULT);
        int val=ADC_GetResult32();
        ADC_StopConvert();
        PWM_WriteCompare(val);
        CyDelay(50);   /* Place your application code here. */
    
    }
}

/* [] END OF FILE */
