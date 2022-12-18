# Objective
To implement a working GATT design for Genotyper

## Final designs
These designs could be used as final GATT designs. 

| File| Description |
| --- | --- |
| [Final_Setup_1](./Final_Setup_1.ncs) | It is based on the design that seemed memory efficient |
| [Final_Setup_2](./Final_Setup_2.ncs) | It is similar to first design with some minor changes so that design becomes understandable  |
| [Final_Setup_3](./Final_Setup_3.ncs) | Here, the characteristics and services are designed on the basis of advertising interval. For instance, those characteristic whose value needs to be updated frequently are combined in same service   |
| [Final_Setup_4](./Final_Setup_4.ncs) | Simialar to previous design, just some reshuffling of characteristics is done |

> ❗Detailed Design Document: [**PDF**](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/NRF_Connect/Genotyper_GATT_Design.pdf)

## Services Designs
These designs were created to get a clear understanding of the memory taken by each service. Each of them include GAP service

| File | Description |
| --- | --- |
| [DFU_Battery_setup](./DFU_Battery_setup.ncs) | Includes battery and DFU service |
| [DFU_Batt_temp_setup](./DFU_Batt_temp_setup.ncs) | Includes battery , DFU  and temperature service |
| [DFU_Batt_Tem_Det_setup](./DFU_Batt_Tem_Det_setup.ncs) | Includes battery, DFU, temperature and Detection service |
| [DFU_Batt_Tem_Det_SP_setup](./DFU_Batt_Tem_Det_SP_setup.ncs) | Includes battery, DFU, temperature, Detection and one other service |
| [DFU_Batt_Tem_Det_SP_R2](./DFU_Batt_Tem_Det_SP_R2.ncs) | Similar to DFU_Batt_Tem_Det_SP_setup with some changes in characteristics   |

## Complete Designs
These designs were based on the previous GATT design. These designs simply group the characteristics in the services based on their simiarity of the data they convey.

| File | Description |
| --- | --- |
| [GATT_Setup_1](./GATT_Setup_1.ncs) | It contains all the services |
| [GATT_Setup_2](./GATT_Setup_2.ncs) | Similar to GATT_Setup_1 with a few changes |

## Test Designs
Their objective was to understand the memory allocation better and to govern the maximum number of services that could be uploaded on dongle.


| File | Description |
| --- | --- |
| [Reduced_setup_1](./Reduced_setup_1.ncs) | Custom UUIDs are used for characteristics and services |
| [Reduced_Setup_2](./Reduced_Setup_2.ncs) | Similar to Reduced_setup_1 with a few changes in grouping and Custom UUIDs |
| [Reduced_Setup_3](./Reduced_Setup_3.ncs) | Similar to Reduced_setup_1 with a few changes in grouping and Custom UUIDs  |
| [Reduced_design_1](./Reduced_design_1.ncs) | It was another design but it didn't get uploaded on the hardware because of memory limitation |
| [Reduced_design_2](./Reduced_design_2) |  It was another design but it didn't get uploaded on the hardware because of memory limitation  |
| [Reduced_R3](./Reduced_R3.ncs) | It was another test design but it also memory error  |
| [Reduced_value_size](./Reduced_value_size.ncs) | It was  done in order tosee the changes if variable length is fixed at a lower value |
| [reduced_setup_2_values](./reduced_setup_2_values.ncs) |It was similar to Reduced_value_size with a few changes   |
| [reduced_test](./reduced_test.ncs) |It was another reduced design but it also showed memory error  |
| [Reduced_Services_1](./Reduced_Services_1.ncs) | Less number of services were used to remove the error  |
| [Complete_Default_setup](./Complete_Default_setup.ncs) | This is a complete design that was based on the earlier grouping of characteristics |

## Important points 
  * Reshuffling of the characteristics and services can be done if it seems necessary
  * Use of SIG-adopted services and characteristics is advisable if the NO_MEM error occurs
  * Descriptors (strings) for defining the characteristic name is not necessary as the app will identify the characteristic on the basis of its UUID. Hence the additional descriptor used for defining the string can be removed 
  * Client Characterisitic Configuration Descriptor contains information about whether notification or indication is enabled or disabled
  * Have a clear understanding about the descriptors and the permissions needed to be enabled with each characteristics
  
