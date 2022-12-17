# [LED brightness control using pwm according to ADC input]()
## Objective
* To become familiar with use of ADC input and PWM output using the PSoC 5LP.
## Steps
* Create the project inside the workspace.
* Add components to `TopDesign.cysch` from components catalog.
* Block 1 -PWM Block 2- ADC_SAR
* ![adc_pwm_schema](https://user-images.githubusercontent.com/86110190/139797825-bd704d1b-bb6b-4539-8e6b-0aabda074c2e.png)

* Add clock to clock input and an analog pin for led to pwm pin of the PWM block.
* Configure the pwm block as Resolution 8bit UDB, PWM Mode ONE-OUTPUT
* ![configure_pwm](https://user-images.githubusercontent.com/86110190/139794744-36903bf0-dc53-4971-87be-d34320df77db.png)
* ![configure_adc](https://user-images.githubusercontent.com/86110190/139795632-cce06103-5290-4d08-956b-84b28882bed9.png)
* Configure the ADC block as shown above, connect an analog pin as POT.
* ![adc_pwm_pin](https://user-images.githubusercontent.com/86110190/139796044-21e8f89a-0b4b-4e75-bf96-b2a36da30ccb.png)
* Configure the hardware then build.
* ![adc_pwm_main](https://user-images.githubusercontent.com/86110190/139796258-1e11f916-7018-4482-a22f-2c3b51b2d339.png)
* LED connected between P2.1 and ground.
* POT connected between P2.3 and ground.
* ![adc_pwm_test](https://user-images.githubusercontent.com/86110190/139797113-715cb584-0d3e-4404-9ece-c9712b7b5168.jpeg)
* Brightness of LED is being controlled with respect to potentiometer.