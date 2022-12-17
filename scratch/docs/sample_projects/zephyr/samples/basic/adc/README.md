# Objective

* Test the sample code for ADC_VREF on the nRF52840 Development Kit.

# Requirements

* Zephyr setup on local system (Linux/macOS/Windows). Kindly follow the [Getting Started](https://docs.zephyrproject.org/latest/getting_started/index.html) page of [Zephyr Project](https://zephyrproject.org/) documentation.

## Pin connections

* No external connections required.

# Procedure

* Navigate to ZephyrPlayground directory and run the following commands.

  ```west build -p auto -b nrf52840dk_nrf52811 adc ```

* If successfully built then run

  ```west flash --erase```

* Use any serial monitor of your choice to view the serial data (ADC data in mV).

![adc](https://user-images.githubusercontent.com/86110190/133202847-c58561f0-2c19-4528-8ef3-bb167a6ea0ba.JPG)
