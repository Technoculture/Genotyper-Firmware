# Project- [OP-AMP](https://www.youtube.com/watch?v=7FBw8-PBIb0)
## Objective
* To implement OP-AMP functionality on PSoC5LP without using the physical opamp.
## PSoC Creator Home Page
![PSoC_Creator_home](https://user-images.githubusercontent.com/86110190/139788838-112bf60b-8f7d-4d02-8875-c5277457f569.png)
* First we have to create the schematic in `TopDesign.cysch`
* ![op_amp_topdesign_schema](https://user-images.githubusercontent.com/86110190/139789355-764bec85-cf6f-46ad-a0f7-c377ea5889be.png)
* Components can be searched from the search bar inside Component Catalog window on right side of the screen.
* All pins are analogPin.
* OP-AMP will be implemented inside the chip but resistors we have to connect externally therefore these components are found in Off-chip section search
* When all pins are connected and configured then build the program. After builds successfully then we have to configure the pins of the IC.
* Now we have to configure the hardware pins, default pins are already selected but if we want to change them then it can be done by changing the port column and then build again.
* ![pins](https://user-images.githubusercontent.com/86110190/139791618-e1be6a6e-1ed7-4814-bcef-f3207a537021.png)
* Next inside source files open `main.c` and enable opamp_0 function inside the main.
* ![opamp_main_c](https://user-images.githubusercontent.com/86110190/139791972-311bee6f-39e8-49e5-a50e-93881a7ba7cb.png)
* Build the program ,connect board to system then click on program ![program](https://user-images.githubusercontent.com/86110190/139792147-f3dedee6-0a80-4359-843a-6b568c6ad221.png) to flash the progra into the PSoC board.
* ![opam_setup](https://user-images.githubusercontent.com/86110190/139793213-65bddcd2-0374-49e2-90a5-3c0a7ac30048.jpeg)
* External resistors are connected as designed in schematic then Input voltage applied 1V output expected 2V and measured 1.99V