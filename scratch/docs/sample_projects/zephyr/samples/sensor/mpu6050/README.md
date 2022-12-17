# Objective

* Test the sample code for MPU6050 3-axis gyro-accelerometer for the nRF52840 DK.
* Observe the readings of the gyro-accelerometer on serial monitor.

# Requirements

* Zephyr setup on local system (Linux/macOS/Windows). Kindly follow the [Getting Started](https://docs.zephyrproject.org/latest/getting_started/index.html) guide of [Zephyr Project](https://zephyrproject.org/) documentation for more information.

# Pin connections

| Board pin | Sensor Pin |
| --------- | ---------- |
| 5V        | VCC        |
| GND       | GND        |
| P0.26     | SDA        |
| P0.27     | SCL        |
| P1.11     | INT        |

# Procedure

* Navigate to ZephyrPlayground directory and run the following commands.

  ```west build -p auto -b nrf52840dk_nrf52811 mpu6050 ```

* If successfully built then run

  ```west flash --erase```
  
# Observation

* The real-time gyro-acceleroscope readings are displayed on the serial monitor when the sensor is either moved or rotated.

![mpu6050](https://user-images.githubusercontent.com/86110190/133202956-3ec19d0b-b225-4d3e-84c7-b2473afa9522.JPG)
