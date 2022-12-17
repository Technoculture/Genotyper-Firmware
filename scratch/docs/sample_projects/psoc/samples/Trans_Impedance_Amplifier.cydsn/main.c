/*******************************************************************************
* File: main.c
*
* Version : 1.2
*
* Description: 
*   This is a source code for example project of TIA.
*
********************************************************************************
* Copyright 2012, Cypress Semiconductor Corporation. All rights reserved.
* This software is owned by Cypress Semiconductor Corporation and is protected
* by and subject to worldwide patent and copyright laws and treaties.
* Therefore, you may use this software only as provided in the license agreement
* accompanying the software package from which you obtained this software.
* CYPRESS AND ITS SUPPLIERS MAKE NO WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
* WITH REGARD TO THIS SOFTWARE, INCLUDING, BUT NOT LIMITED TO, NONINFRINGEMENT,
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
*******************************************************************************/

#include <project.h>

/*******************************************************************************
* Function Name: main
********************************************************************************
* Summary:
*  Main function performs following functions:
*   1. Start TIA, IDAC and VDAC
*   2. TIA Resistive feedback is set to 40k ohms
*   3. TIA Capacitive feedback is set to 3.3 pF
*
* Parameters:
*  None.
*
* Return:
*  None.
*
*******************************************************************************/
int main()
{
 
    /* Start the TIA component */
    TIA_1_Start();

    /* Start the IDAC component */
    IDAC8_1_Start();

    /* Start the VDAC component */
    VDAC8_1_Start();

    /* Set the Resistive feedback to 40k ohms */
    TIA_1_SetResFB(TIA_1_RES_FEEDBACK_40K);

    /* Set the capacitive feedback to 3.3pF */
    TIA_1_SetCapFB(TIA_1_CAP_FEEDBACK_3_3PF);

    
    for(;;)
    {
    }
}


/* [] END OF FILE */
