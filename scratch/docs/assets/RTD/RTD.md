* Nickel-based resistance temperature detectors tend to have a lower maximum sensing temperature than their platinum counterparts.
* Platinum detectors are capable of measuring well beyond the melting point of solder, so you tend to find that for high-temperature applications, they are fitted on a cable using crimped connections or built into a probe device, rather than just as a surface-mounted component.
*  Many resistance temperature detectors can also work quite well at the lower end of the range, with a significant quantity of options on the market for operating temperatures well below what you will generally find in the natural environment.
*  Surface mount RTD components will typically only have an operating temperature range similar to most other surface mounted parts (around -55°C to 175°C). However, lead mounted RTD components can operate within a range of -200°C to 850°C.
* RTDs have less resistance w.r.t thermistors therefore these are implemented using a voltage divider.
* Unlike thermistors, which define the resistance at 25°C as their specification resistance, RTDs use the resistance at 0°C as their specification resistance. 

# RTD Implementation: Voltage Divider



* For 100 ohm RTD the voltage divider approach is not good.
* It is suitable for thermistor based designs.

# RTD Implementation: Basic Wheatstone Bridge

* One of the most accurate ways to measure resistance is to use a Wheatstone bridge. A Wheatstone bridge uses two balanced legs in a bridge circuit to measure the unknown resistance of one resistor in one of the four legs. If this unknown resistance is a device such as an RTD, we can make an extremely accurate measurement of the resistance of that device. This circuit provides a voltage change as the resistance changes, allowing a microcontroller or other monitoring device to measure the resistance of the unknown element - the RTD in this case.
* If the resolution of the output is sufficient for the application, and an ADC with differential inputs is available, this is a simple implementation. With the addition of an operational amplifier or instrumentation amplifier, the differential voltage output of the Wheatstone bridge can be increased, providing a more usable voltage that is more suitable for the typical resolution of ADCs, and compatible with ADCs that don’t have differential inputs.
* We’ll need to use high precision, low temp coeff (0.1% tolerance, 25ppm/degC) resistors to achieve this.
* Bridge is balanced at 0degC and and at 150degC we will expect a potential difference of 0.344V, at min sensing temp we should see a potential diff of -0.106V
* This change in potential can be increased if we use low resistance on the top of the bridge.
* NXP Kinetis can deliver around 0.02°C steps for the ADC resolution.
* It is noticed that this board has a different order to the analog channels;
*  the other resistive elements of the bridge are placed on the other side of the thermal break on the board

# RTD Implementation: Operational Amplifier with Error Compensation

* if uC doesn't have a differential ADC, or perhaps it doesn’t even have a high-resolution ADC, prefer to use a 24-bit or better analog to digital converter with a programmable gain amplifier built-in. 
* used TPS60403 to generate negative voltage

# RTD Implementation: Instrumentation Amplifier with Linear Compensation

While the circuit above is an excellent option for implementing a linearized RTD at a low cost, we can take it a step further with just a small additional expense. By changing the operational amplifier to an instrumentation amplifier, we can buffer the input cheaper than if we added a buffer amplifier to the operational amplifier. The instrumentation amplifier has a very high input impedance so that it won’t bias the sensor’s measurement in any quantifiable way.



# RTD Implementation: Digitized Wheatstone Bridge

* ADS1220 

# Other Options: RTD Converter IC

In addition to measuring temperature by reading a voltage from a voltage divider or a Wheatstone bridge, we can also use a temperature sensor amplifier like the ones we will look at for use with thermocouples. These ICs will provide you with a digital temperature output rather than a voltage level, and typically incorporate all the amplification and compensation circuitry that you need to provide the most accurate temperature measurement that the sensor can deliver. The cost of this option can be a significant factor, but so is the cost of using a PGA-ADC, as discussed above. Using a PGA-ADC provides a better learning experience and demonstration for this article so that we won’t be looking in detail at an RTD converter IC.

