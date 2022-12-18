# [Transimpedance amplifier](https://www.cypress.com/documentation/code-examples/ce95383-transimpedance-amplifier-tia-psoc-35lp)

## Objective
* To implement transimpedance amplifier with PSOC5LP without using any physical analog circuit.

## Steps
* Create project inside the workspace created above.
* Add components to `TopDesign.cysch` from Components Catalog window.
* ![TIA_schema](https://user-images.githubusercontent.com/86110190/139803233-53e9fbb0-3455-4dbf-a1f1-3bf5dc0c1cf8.png)
* To implement we need three blocks TIA block, IDAC block for current source, VDAC block for reference voltage and one analog pin.
* Configure the blocks as shown below.
* ![configure_tia](https://user-images.githubusercontent.com/86110190/139803327-d9fe94b8-bd22-4941-86cc-c39092ff059e.png)
* ![configure_idac](https://user-images.githubusercontent.com/86110190/139803728-678160a4-0dbe-4cff-8193-5dcd076aee67.png)
* ![configure_vdac](https://user-images.githubusercontent.com/86110190/139803739-298c3a0f-118f-4ada-90e6-c494fd6112f7.png)
* Build the schematic then configure the hardware pins then build again.
* ![TIA-Pin_configure](https://user-images.githubusercontent.com/86110190/139804817-b156b535-d822-4763-ad96-930ff1495701.png)
* ![tia_main](https://user-images.githubusercontent.com/86110190/139805100-7749d5b9-adf4-4f97-86ae-4906cb7acecc.png)
* Inside the main program we have to enable the blocks we are using. Otherwise blocks wont work.
* If main.c builds successfully then program the device.
* No External input is required.
* Expected voltage on P0.4 is 1.55V we got 1.65V

* ![TIA](https://user-images.githubusercontent.com/86110190/139805185-a3cd9de8-0189-46bf-b2e0-4d1e4f133742.jpg)