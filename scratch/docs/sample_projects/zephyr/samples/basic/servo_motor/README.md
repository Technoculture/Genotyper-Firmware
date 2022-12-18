# Objective

* Test the sample code for micro-servo motor.

# Requirements

* Zephyr setup on local system (Linux/macOS/Windows). Kindly follow the [Getting Started](https://docs.zephyrproject.org/latest/getting_started/index.html) guide of [Zephyr Project](https://zephyrproject.org/) documentation for more information.

# Pin connections

| Board pin | Servo Pin |
| --------- | --------- |
| 5V        | VCC       |
| GND       | GND       |
| P0.13     | SIG       |


# Procedure

* Navigate to ZephyrPlayground directory and run the following commands.

  ```west build -p auto -b nrf52840dk_nrf52811 servo_motor ```

* If successfully built then run

  ```west flash --erase```
  
# Observation
  
* The micro-servo starts rotating with a step-size of time interval which can be altered in the source code.
