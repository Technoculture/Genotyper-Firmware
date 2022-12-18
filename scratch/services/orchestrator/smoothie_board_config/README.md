# Objective 

* To configure and test the smoothie board as per the orchestrator.
* To test two axes movement with smoothie board
* To calibrate the smoothie board configurations properly.
* To test SG90 servo motor with smoothieboard.
## Steps to prepare the smoothie board for test run

* Connect the board using USB cable.
* SD card mounted on the smoothie board comes with config file and firmware already flashed. It can also work fine without any modification.
* In some cases it may not work.
* In that case it is recommended to upgrade the firmware.
* To upgrade the firmware follow the below links.
* Download the configuration file from [here](https://smoothieware.org/configuring-smoothie) .
* Download the firmware from [here](https://github.com/Smoothieware/Smoothieware/blob/edge/FirmwareBin/firmware.bin) .
* **File name must not be altered.**
* Alternately config.txt as well firmware file is added in the folder here.
* Paste the config.txt and firmware.bin file into the sd card then eject(not physically from the board, only from the menu in the system).
* RESET the board , then flashing process begins.
* Four onboard leds will start counting up as the firmware gets flashed.
* If everything works fine then led 1 & 4 will be ON and led 2,3 will blink together.
* Also the firmware.bin will become FIRMWARE.CUR file.
* If there is a problem then format the sd card then try again the above process.
* For better understanding the configuration parameters check thid [link](https://smoothieware.org/configuration-options)

## Putting configuration parameters correctly
* Before giving the power supply it is mandatory to configure properly the parameters.


| Parameters | Value |
| ----                    | ----     |
| default_feed_rate       | 5000     |
| alpha_current           | 0.5      |
|  beta_current           |  0.5     |
|  gamma_current          |  0.5     |
|  alpha_steps_per_mm     | 200.250  |
|  beta_steps_per_mm      |  507.937 |
| gamma_steps_per_mmm     | 2000     |
| z_acceleration          | 500      |
|  extruder.hotend.enable |  false   |
|  laser_module_enable    | false    |
|  temperature_control.hotend.enable |  false |
| temperature_control.bed.enable     |  false |
| switch.fan.enable                  | false  |
|  endstops_enable                   | true   |
| corexy_homing                      | true   |
|  alpha_homing_direction  | home_to_min  |
|  beta_homing_direction   | home_to_max  |
|  gamma_homing_direction  | home_to_min  |
|  alpha_max               |  531   |
|  beta_max                |  125   |
|  gamma_max               |  100   |
| alpha_max_travel   | 575 |
| beta_max_travel    | 135 |
| alpha_limit_enable | true |
| beta_limit_enable  | true |
| gamma_limt_enable  | true |
| alpha_homing_retract_mm | 3 |
| beta_homing_retract_mm | 3 |
| gamma_homing_retract_mm | 10 |

* Just save the configuration and then reset the board to make the changes work.
* Test the motors without installing the, on any axes, if everything works then install motors on axes.


**Z axis motor pins, Step pin, dir pin & EN pin changed as it was not working**
**all pins of Z axis swapped with extruder 1 motor pins**





## Wiring Up the Motors with board
* check the coils of stepper motor by doing the continuity test of wires.
* There are 2 coils of which 4 wires we will get.
* ![steppers](https://user-images.githubusercontent.com/86110190/158013114-ec2de02a-608c-4662-99d1-ff5d87f916a9.png)
* ![motor_wiring](https://user-images.githubusercontent.com/86110190/158013123-ae2517c7-8257-420f-8162-8395218194a8.png)

## Troubleshooting steps
* Troubleshooting steps can be found [here](https://smoothieware.org/troubleshooting)
* LED blinking patterns are well described here to get better understanding of the ststus of the board.

# Software
* If flashing done successfully then it comes to the software through which we will pass the commands.
* Pronterface [download](https://github.com/kliment/Printrun/releases/download/printrun-2.0.0rc8/pronterface-windows-x64_3.8-2.0.0rc8.zip)
* ![pronterface](https://user-images.githubusercontent.com/86110190/158013761-131b2d72-7603-4896-92b2-9434489eaf96.png)
* Just select the COM port and select any baudrate then click connect, board will be connected over the serial port.

## [Endstops](https://smoothieware.org/endstops)
* When motor testing is done without attaching the motor to the axes.
* Now its time to add mototrs to axes and add limit switches to extreme limits of the axes.
* ![endstops](https://user-images.githubusercontent.com/86110190/158014671-a67502fa-5777-4e2b-9cab-3fc07cd44212.png)
* connect the common pin to signal pin(green), and NC pin to GND pin(blue). Repeat for each endstops.
* C : Common
* NO : Normally Open, meaning it is not connected to C when the interrupter is not pressed, and connected to C when the interrupter is pressed.
* NC : Normally Closed, meaning it is connected to C when the interrupter is not pressed, and connected not to C when the interrupter is pressed.
* ![limit](https://user-images.githubusercontent.com/86110190/158015088-41de165f-31b5-4d2d-8990-de7a447d8f63.png)
* Pinouts may vary for different limit switch, verify before connecting.




## Board Pinouts

 ![smoothieboard_pinout](https://user-images.githubusercontent.com/86110190/158008764-7accd6e5-6192-4624-8ebb-2cff66955d03.png)
 ![pin_capabilities](https://user-images.githubusercontent.com/86110190/158008773-bb2deb76-e5c1-4c3e-be9c-d15b97a0d687.png)

## Calibration
steps_per_mm = (full steps per rotation * microstep per step)/mm per rotation


in general full steps per rotation =200, microsteps per step for smoothieboard= 32


which becomes (200 * 32) / mm per rotation.


mm per rotation is the distance traveled in any particular axis when stepper rotates one revolution and can be calculated by rotating the stepper one rotation manually or using the pronterface.
# Servo Parameters
| Parameters | Value |
| ---- | ---- |
| switch.servo.enable   | true |
| switch.servo.input_on_command  |   M280  |
| switch.servo.input_off_command  |   M281    |
| switch.servo.output_pin |  3.25    |
| switch.servo.output_type | swpwm    |
| switch.servo.pwm_period_ms    |  20   |
| #switch.servo.startup_state |  false  |
| #switch.servo.startup_value  |    7.43  |
| #switch.servo.default_on_value  | 3.3  |

* Connect the servo signal pin to a PWM enabled pin, here it is connected to pin 3.25
* Coonect 5V,GND to servo 5V, GND pins
* Paste the above parameters in the config. file 
* Eject the sd card from the system(not physically)
* Reset the board manually
* Connect using pronterface.
* Pass commands using pronterface serial terminal
* `M280 S5`, `M280 S10`, `M280 S7.5` Tested
* Config file with servo parameters [config.txt](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/services/orchestrator/smothhie_board%20config/config.txt)

# Playing Gcode Files
1. From Serial Terminal
 * Enter `@play /sd/gcodefile.gcode` in terminal then press `enter`
 * File must be stored in the sd card for above command to work.

2. By Loading any file from system
 *  Click on `Load File` --> Click `print`, then it will start executing commands from that specific gcode file.
 * File may be stored anywhere in the system or sd card.
# How to Setup smoothieboard [Video](https://youtu.be/WFVUPwUXx9Q)
## System testing [video](https://youtu.be/HWVqq3cTOIQ)
