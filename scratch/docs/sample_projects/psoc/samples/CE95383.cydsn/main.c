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

#include <device.h>

/*******************************************************************************
* Function Name: main
********************************************************************************
* Summary:
*  Main function performs following functions:
*   1. Initializes the LCD and clears the display
*   2. Start TIA, IDAC and VDAC
*   3. TIA Resistive feedback is set to 40k ohms
*   4. TIA Capacitive feedback is set to 3.3 pF
*   5. Print test name and TIA resistive and capacitive feedback values on the 
*      LCD
*
* Parameters:
*  None.
*
* Return:
*  None.
*
*******************************************************************************/
void main()
{
    /* Start the LCD component */
    LCD_Char_1_Start();

    /* Clear LCD Display */
    LCD_Char_1_ClearDisplay();

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

    LCD_Char_1_Position(0u, 0u);
    LCD_Char_1_PrintString("    TIA DEMO");

    LCD_Char_1_Position(1u ,0u);
    LCD_Char_1_PrintString("RF=40k  CF=3.3pF");
    
    for(;;)
    {
    }
}


/* [] END OF FILE */
