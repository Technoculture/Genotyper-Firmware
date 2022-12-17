# Objective

* To detect different DNA concentration samples using photodiode.
* PSOC internal Transimpedance amplifier will be used to convert the current from the Photodiode to Voltage

# Equipment used


* PSOC5LP                                           
* USB-A CABLE Female to male                                        
* DNA Quantification Setup
* (Photo Diode & UV LED )
* PSOC Creator software                                    
* LT3092 current Regulator Breakout Board
* DC Supply 
* Resistors - R_set=22k, R_out= 1 Ohm for (100 to 160mA) current, 5 Ohm for (20 to 40 ) mA    


 # DNA concentration
| Sample  | Actual(ng/uL) |  Calculated(ng/uL) |
| ----  | ----  | ----  |
| Sample 1  | 86.343  |  93.4   | 
| Sample 2  | 10.625  |  13.34   |  
| Sample 3  | 0.603  |   1.9  |  
| Sample 4  | -7.43  |   0.27  |  
| Sample 5  |  -3.682 |   0.03  |  

![dna_conc](https://user-images.githubusercontent.com/86110190/147212137-442aa5e9-2c90-497d-8451-5d884cc8340a.jpeg)
![dna_conc_actual](https://user-images.githubusercontent.com/86110190/147212142-d78acf58-cf7c-4c57-9b1d-965ff2e9a5ea.jpeg)

 # 1st Setup with 275nm UV
 * Photodidoe - [PC10-2-TO5](https://www.mouser.in/ProductDetail/First-Sensor/PC10-2-TO5?qs=IxDs%252B9hDEuBXeJgE5fsD8A%3D%3D)
 * Active area - 10 mm square
 * UV LED - [XST-3535-UV-A60-CE275-00](https://www.mouser.in/ProductDetail/Luminus-Devices/XST-3535-UV-A60-CE275-00?qs=XeJtXLiO41SVe8YDB0XxlA%3D%3D)
 * Wavelength - 275nm
 * Current- 160mA
 * R_out -1 Ohm
 * ![dna_detection_setup](https://user-images.githubusercontent.com/86110190/147203541-05c1fa1a-aa9a-4da5-b43d-23d88b4d639c.jpeg)
 * ![setup](https://user-images.githubusercontent.com/86110190/147203549-c154c46d-8c18-405e-a902-14642da40ad0.jpeg)

## PSOC Configuration to sense photodiode current using internal Transimpedance Amplifier

![dna_detection_psoc_config](https://user-images.githubusercontent.com/86110190/147202938-74631f4c-c5a3-43ff-9c4f-2ae2b5b16dc2.png)

![dna_detection_pinout](https://user-images.githubusercontent.com/86110190/147202949-4d8439d4-6c3c-4e98-a0fe-d96e7ecbe15c.png)
![dna_detection_main_cpp](https://user-images.githubusercontent.com/86110190/147202954-29bee2fb-778e-445d-bea6-a64ae92b22c1.png)
 

## Test Results
| Reading (Volts)  | Buffer Sample  | Sample 1  | Sample 2  | Sample 3  | Sample 4  | Sample 5  |
|  ---- | ----  | ----  | ----  | ----  | ----  | ----  |
| 1st Reading   | 1.86  | 1.38  |  1.68 | 2.02  | 2  | 2.02  |
| 2nd Reading  |  2.14 |  1.4 |  1.82 |  2.06 | 1.96  | 2  |
| 3rd Reading  |  1.96 | 1.38  |  1.8 | 2.12  | 1.96  | 2  |
| 4th Reading  |  2.03 |  1.38 |  1.8 | 2.12  | 1.96  |  1.97 |
| 5th Reading  |  2.13 |  1.36 |  1.72 | 2.02  | 2  | 1.9  |

# Graphical Representation of results

![Response_with_high_power_275nm_uv](https://user-images.githubusercontent.com/86110190/147440095-c6292c80-fb65-4e03-8a5e-12383adf9518.png)

## PSOC Configuration to sense photodiode current using internal Transimpedance Amplifier for 2nd, 3rd, 4th setup

![dna_detection_psoc_setup](https://user-images.githubusercontent.com/86110190/147325197-6f1f3240-d1d0-4f90-99c6-ab502ca509b1.png)

![dna_detection_main_c](https://user-images.githubusercontent.com/86110190/147325211-16570d98-1f1d-410e-a02d-fa887c4402b8.png)



# 2nd Setup with 265nm UV

 * Photodidoe - [PC10-2-TO5](https://www.mouser.in/ProductDetail/First-Sensor/PC10-2-TO5?qs=IxDs%252B9hDEuBXeJgE5fsD8A%3D%3D)
 * Active area - 10 mm square
 * UV LED - [QBHP684E-UV265](https://www.digikey.in/en/products/detail/qt-brightek-qtb/QBHP684E-UV265N/13278846)
 * Wavelength - 265nm
 * Current- 20mA
 * R_out -5 Ohm

| Reading (Volts)  | Buffer Sample  | Sample 1  | Sample 2  | Sample 3  | Sample 4  | Sample 5  |
|  ---- | ----  | ----  | ----  | ----  | ----  | ----  |
| 1st Reading   | 0.400  | 0.230  | 0.400  | 0.540  | 0.420  | 0.380  |
| 2nd Reading  | 0.400  | 0.230  |  0.400 | 0.420  | 0.420  | 0.390  |
| 3rd Reading  |  0.400 | 0.230  | 0.400 |  0.400 | 0.410  | 0.360  |
| 4th Reading  |  0.400 | 0.230  |  0.380 | 0.440  |  0.440 | 0.400 |
| 5th Reading  |  0.400 |  0.220 | 0.380  | 0.400  | 0.400  | 0.360  |

# Graphical Representation of results
![Response_with_265nm_uv](https://user-images.githubusercontent.com/86110190/147323938-649daa26-7469-4551-9d09-0840c193ec7d.png)


# 3rd Setup with 280nm UV

 * Photodidoe - [PC10-2-TO5](https://www.mouser.in/ProductDetail/First-Sensor/PC10-2-TO5?qs=IxDs%252B9hDEuBXeJgE5fsD8A%3D%3D)
 * Active area - 10 mm square
 * UV LED - [ELUC3535NUB-P7085Q05075020-S21Q](https://www.digikey.in/en/products/detail/everlight-electronics-co-ltd/ELUC3535NUB-P7085Q05075020-S21Q/12177237)
 * Wavelength - [270~285]nm
 * Current- 20mA
 * R_out -5 Ohm


| Reading (Volts)  | Buffer Sample  | Sample 1  | Sample 2  | Sample 3  | Sample 4  | Sample 5  |
|  ---- | ----  | ----  | ----  | ----  | ----  | ----  |
| 1st Reading   | 0.680  | 0.540  |  0.620 | 0.760  | 0.740  | 0.690  |
| 2nd Reading  |  0.700 |  0.550 | 0.660  |  0.740 | 0.730  | 0.730  |
| 3rd Reading  | 0.720  | 0.500  | 0.640 | 0.680  | 0.700  |  0.660 |
| 4th Reading  | 0.720  |  0.500 | 0.680  | 0.740  | 0.740  | 0.640 |
| 5th Reading  |  0.730 |  0.520 | 0.680  |  0.700 | 0.700  |  0.640 |

# Graphical Representation of results
![Response_with_280nm_uv](https://user-images.githubusercontent.com/86110190/147323953-acc90e36-30e6-495b-87f9-2025b70c0826.png)


# 4th Setup with 275nm UV

 * Photodidoe - [PC10-2-TO5](https://www.mouser.in/ProductDetail/First-Sensor/PC10-2-TO5?qs=IxDs%252B9hDEuBXeJgE5fsD8A%3D%3D)
 * Active area - 10 mm square
 * UV LED - [VLMU35CB20-275-120](https://www.mouser.in/ProductDetail/Vishay-Semiconductors/VLMU35CB20-275-120?qs=sGAEpiMZZMv0NwlthflBi%2F1oddQbB4RJ4YAEAUJRlIw%3D)
 * Wavelength - 275nm
 * Current- 110mA
 * R_out -1 Ohm


| Reading (Volts)  | Buffer Sample  | Sample 1  | Sample 2  | Sample 3  | Sample 4  | Sample 5  |
|  ---- | ----  | ----  | ----  | ----  | ----  | ----  |
| 1st Reading   | 1.420  | 0.920  | 1.260  |  1.440 | 1.400  | 1.420  |
| 2nd Reading  | 1.440  | 0.900  |  1.260 | 1.480  | 1.480  | 1.420  |
| 3rd Reading  | 1.440  | 0.880  | 1.280 | 1.420  | 1.380  | 1.320  |
| 4th Reading  | 1.440  | 0.940  | 1.340  | 1.440  | 1.400  | 1.400 |
| 5th Reading  | 1.420  | 0.900  |  1.280 |  1.440 | 1.480  |  1.420 |

# Graphical Representation of results

![Response_with_275nm_uv](https://user-images.githubusercontent.com/86110190/147323957-1ceb7d0f-f63c-487a-8c3f-e9d07cc046e0.png)


# 5th Setup with 255nm UV

 * Photodidoe - [PC10-2-TO5](https://www.mouser.in/ProductDetail/First-Sensor/PC10-2-TO5?qs=IxDs%252B9hDEuBXeJgE5fsD8A%3D%3D)
 * Active area - 10 mm square
 * UV LED - [CUD5GF1B](https://www.digikey.in/en/products/detail/seti-seoul-viosys/CUD5GF1B/11616835?s=N4IgTCBcDaIMIFUAiBWA4gMQIwCEQF0BfIA)
 * Wavelength - 255nm
 * Current- 130mA
 * R_out -1 Ohm


| Reading (Volts)  | Buffer Sample  | Sample 1  | Sample 2  | Sample 3  | Sample 4  | Sample 5  |
|  ---- | ----  | ----  | ----  | ----  | ----  | ----  |
| 1st Reading   |   |   |   |   |   |   |
| 2nd Reading  |   |   |   |   |   |   |
| 3rd Reading  |   |   |  |   |   |   |
| 4th Reading  |   |   |   |   |   |  |
| 5th Reading  |   |   |   |   |   |   |

# Graphical Representation of results




