// This program is to test the PID contrallability of a peltier block using h bridge and a ntc thermistor
#include"Thermocycler.h"


//Define Variables we'll be connecting to
double Setpoint, Input, Output;

//Define the aggressive and conservative Tuning Parameters
//double aggKp=1, aggKi=0.5, aggKd=0.15;
//double consKp=2, consKi=0.05, consKd=0.5;
double consKp=2.0206, consKi=0.37236, consKd=2.6591 ; /* These parameters were calculated from matlab simulation*/

//Specify the links and initial tuning parameters
PID PeltierPID(&Input, &Output, &Setpoint, consKp, consKi, consKd, DIRECT);

void setup()
{
  Serial.begin(BAUDRATE);
  //initialize the variables we're linked to
  V0 = analogRead(PIN_THERMISTOR_INPUT);
  Input = (double)temp_reading(V0);
  Setpoint = 90;
 //turn the PID on
 PeltierPID.SetMode(AUTOMATIC);
  pinMode(PIN_THERMISTOR_INPUT,INPUT);
  pinMode(PELTIER_PIN1,OUTPUT);
  pinMode(PELTIER_PIN2,OUTPUT);

  pinMode(FAN_PIN1,OUTPUT);
  pinMode(FAN_PIN2,OUTPUT);

  pinMode(PIN_LED_INDICATOR,OUTPUT);
  pinMode(PIN_PELTIER_CONTROL_OUTPUT,OUTPUT);


  digitalWrite(PELTIER_PIN1,HIGH);
  digitalWrite(PELTIER_PIN2,LOW);
 PeltierPID.SetTunings(consKp, consKi, consKd);
}

void loop()
{
  V0 = analogRead(PIN_THERMISTOR_INPUT);
  Input = (double)temp_reading(V0);

  double gap = abs(Setpoint-Input); //distance away from setpoint
  // if (gap < 30)
  // {  //we're close to setpoint, use conservative tuning parameters
  //   PeltierPID.SetTunings(consKp, consKi, consKd);
  // }
  // else
  // {
  //    //we're far from setpoint, use aggressive tuning parameters
  //    PeltierPID.SetTunings(aggKp, aggKi, aggKd);
  // }

 PeltierPID.Compute();
  analogWrite(PIN_PELTIER_CONTROL_OUTPUT, Output);

  if (gap <5)
  digitalWrite(PIN_LED_INDICATOR, HIGH);
  else 
  digitalWrite(PIN_LED_INDICATOR,LOW);

  if(Input > (Setpoint))
   {
      digitalWrite(FAN_PIN1, HIGH);
      digitalWrite(FAN_PIN2,LOW);
   }
  else
  { 
    digitalWrite(FAN_PIN1,LOW);
    digitalWrite(FAN_PIN2,LOW);
  }

}

// Temperature reading
float temp_reading(int v)
{
  R2 = R1 * (1023.0 / (float)v - 1.0);
  logR2 = log(R2);
  T = (1.0 / (c1 + c2*logR2 + c3*logR2*logR2*logR2));
  TC = T - 273.15;
  Serial.print("Temp: "); 
  Serial.print(TC);
  Serial.print(" C  "); 
  Serial.println("");
return TC;
}
