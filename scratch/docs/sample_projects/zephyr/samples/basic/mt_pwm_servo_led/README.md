# Objective

* Test the sample code for PWM driven micro-servo motor and PWM LED demonstrating multi-tasking.

# Requirements

* Zephyr setup on local system (Linux/macOS/Windows). Kindly follow the [Getting Started](https://docs.zephyrproject.org/latest/getting_started/index.html) guide of [Zephyr Project](https://zephyrproject.org/) documentation for more information.

# External Pin Connections

| Board pin | Servo Pin |
| --------- | --------- |
| 5V        | VCC       |
| GND       | GND       |
| P0.13     | SIG       |


# Procedure

* Navigate to ZephyrPlayground directory and run the following commands.

  ```
  west build -p auto -b nrf52840dk_nrf52811 servo_motor
  ```

* If successfully built then run

  ```
  west flash --erase
  ```
  
# Observation
  
* The micro-servo starts rotating and PWM driven LED starts blinking at two different rates of time interval, which can be altered in the source code.
