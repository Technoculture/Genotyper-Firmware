#include<Arduino.h>
#include <PID_v1.h>

#ifndef THERMOCYCLER_H__
#define THERMOCYCLER_H__



#define PIN_THERMISTOR_INPUT A0
#define PIN_PELTIER_CONTROL_OUTPUT 3


#define BAUDRATE 115200

#define PIN_LED_INDICATOR 2

#define PELTIER_TERMINAL1 4
#define PELTIER_TERMINAL2 5
int V0=0;
const float R1 = 10000;
float logR2, R2, T, TC, Tf;
const float c1 = 1.009249522e-03, c2 = 2.378405444e-04, c3 = 2.019202697e-07;

float temp_reading(int);

#endif // THERMOCYCLER_H__