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
    ADC_SAR_Start();
    ADC_SAR_StartConvert();

    /* Place your initialization/startup code here (e.g. MyInst_Start()) */

    for(;;)
    {
     int val = ADC_SAR_GetResult8();
    PWM_WriteCompare(val);
    CyDelay(50);/* Place your application code here. */
    }
}

/* [] END OF FILE */
