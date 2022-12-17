# Objective

* To drive the SG90 servo motor with smoothieboard.
* To test the pipette holding movement with sg90 servo assembled with end effector.

# Setup

* Orchestrator wiring is kept same as earlier.
* End effector assembled with SG90 servo.
* Signal pin connected to P3.25
* GND connected to GND
* Power pin connected with external 5V power supply.



# Config file

# Switch module for servo control using S/W PWM

switch.servo.enable                            	true             # Enable this module

switch.servo.input_on_command        M280             # M280 S7.5 would be midway

switch.servo.input_off_command        M281             # Same as M280 S0 0% duty cycle, effectively off

switch.servo.output_pin                        3.25             # May be any spare pin

switch.servo.output_type                      swpwm            # Software pwm output settable with S parameter in the input_on_command

switch.servo.pwm_period_ms             20               # set period to 20ms (50Hz) default is 50Hz



# Commands to execute

* `M280 S5 to M280 S10` for free servo motor.
* `M280 S7.5` to hold the pipette and `M280 S5` to  release the pipette.



# Steps followed

* Set the power supply 12V 1A for motors, and use another channel for servo motor and set 5V 0.5A.
* Turn on both the channels.
* Connect USB cable. Wait for the board to get ready(follow LED blinking pattern)

# [Video Link](https://youtu.be/HdRmGCZ7Fhc)