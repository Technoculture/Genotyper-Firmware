# Objective

* To detect different DNA concentration samples using photodiode
* Internal Delta sigma ADC block is being used to detect photodiode input.
* According to photodiode input signal it is converted to PWM signal 

# Equipment used


* PSOC5LP                                           
* MicroUSB to USB-A CABLE                                         
* DNA Quantification Setup(Photo Diode UV LED Setup)
* PSOC Creator software     
* LT3092 current source
* DC Supply 7.5V, 160mA
* Resistors 22k, 1Ohm 
* Digital storage oscilloscope

# PSOC Configuration
## Schematic

![photodiode_pwm_detection](https://user-images.githubusercontent.com/86110190/147477492-8a9c70e8-9d28-482c-97eb-fb14cc743b02.png)
## Pinouts
![Photodiode_pwm_pins](https://user-images.githubusercontent.com/86110190/147477505-2dadaae3-112b-4a34-9d73-502e3d0c212f.png)
## Code
![photodiode_main_cpp](https://user-images.githubusercontent.com/86110190/147477512-e478f11e-4418-452d-aeb8-571e9108c4d4.png)

# Results
* We will get Pulse width modulated signal according to photodiode input signal.
* Below results represents output with different photodiode used with same UV LED setup of DNA quantification. 
![pwmoutput1](https://user-images.githubusercontent.com/86110190/147478333-30075519-3cad-4a1d-b031-279fd722ca62.jpeg)
![pwmoutput2](https://user-images.githubusercontent.com/86110190/147478340-0e65e1b3-91c5-4a2d-aa59-5820e4104fea.jpeg)
![pwmoutput3](https://user-images.githubusercontent.com/86110190/147478344-c83b5bd3-24cd-4881-9f96-50f34333d010.jpeg)
![pwmoutput4](https://user-images.githubusercontent.com/86110190/147478348-de501f9a-33a5-415a-b6bd-87dcfc9604ed.jpeg)
