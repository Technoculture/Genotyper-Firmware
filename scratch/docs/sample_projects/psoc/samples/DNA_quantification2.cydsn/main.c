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
//    opamp_Start();
     TIA_1_Start();
     TIA_1_SetResFB(TIA_1_RES_FEEDBACK_1000K);

    /* Set the capacitive feedback to 3.3pF */
     TIA_1_SetCapFB(TIA_1_CAP_FEEDBACK_1_3PF);

    


    for(;;)
    {
        /* Place your application code here. */
    }
}

/* [] END OF FILE */
