

# Containers Co-Ordinate [Opentron Code VS. Standard]

## Objective:-

- To determine and understand the position and geometric co-ordinate of the modules (containers) used in Opentron OT-2 robotic fluid handling system.
- To verify and compare the modules (containers) used in Opentron OT-2 robotic fluid handling system with the standard modules (containers) along with the Geometric coordinate codes. 



## How the Co-ordinate system works for Opentron OT-2 robotic fluid handling system :

For example, lets take a simple geometric co-ordinate code of 6 well plate.

###### Dimension of the plate -

- Depth of well : 17.4mm
- Diameter: 22.5mm
- Total working liquid volume: 16800ul
- Centre distance between one well to another well in X direction is **39.12mm** as well as in Y direction also is **39.12mm**.

Now the code For reaching the pipette each and every well -  

```json
"6-well-plate": {
               "origin-offset": {                          
                    "x": 23.16,
                    "y": 24.76
               },
               "locations": {
                    "A1": {
                         "x": 0,
                         "y": 0,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    },
                    "B1": {
                         "x": 39.12,
                         "y": 0,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    },
                    "A2": {
                         "x": 0,
                         "y": 39.12,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    },
                    "B2": {
                         "x": 39.12,
                         "y": 39.12,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    },
                    "A3": {
                         "x": 0,
                         "y": 78.24,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    },
                    "B3": {
                         "x": 39.12,
                         "y": 78.24,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    }
               }
          }
```

The Opentron OT-2 robotic fluid handling system has 3 Axis (XYZ) where the Electronic pipette can travel.

- **X-axis** for linier traveling the pipette in X direction.
- **Y-axis** for linier traveling the pipette in Y direction.
- **Z-axis** for linier traveling the pipette in Z direction.

Each well named for simplify and defining the code. The 6 well plate have 6 wells. 2 column and 3 rows. The column named as **A** and **B** and the rows named as **1**, **2** & **3**.

#### Location Image
![explaining_working_of_coordinate_6_well_opentron](https://user-images.githubusercontent.com/51780510/169030417-593de07e-a352-4ebc-a4e2-fb7187a533f6.JPG)
#### Cad file image
![explaining_working_of_coordinate_6_well_opentron_CAD](https://user-images.githubusercontent.com/51780510/169030469-83acd471-e986-4984-b802-ef0fcf5ccc75.JPG)



```json origin offset for 6 well plate
"origin-offset": {                         
                    "x": 23.16,
                    "y": 24.76
                 }
```

That line of code define the safe origin-offset position of the pipette.


```json move location to A1 well
"A1": {
                         "x": 0,
                         "y": 0,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    }
```

- From the safe origin-offset position the Pipette move **23.16mm** in **X** direction and **24.76mm** in **Y** direction to reach **A1** position.
- The coordinate of the **A1** position is X=0mm, Y=0mm & Z=0mm.
- **"depth": 17.4** means the pipette can travel **17.4mm** in **Z** direction to reach the bottom of the plate in short the height of the well.
- **"diameter": 22.5** means the maximum diameter of the wells is **22.5mm**.
- **"total-liquid-volume": 16800** means the maximum volume of liquid store in one well is **16800ul**.


Similarly for reaching the position at **B1** the following code will be follows.

```json move location to B1 well
"B1": {
                         "x": 39.12,
                         "y": 0,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                   }
```

- From the **A1** position the Pipette move **39.12mm** in **X** direction and **0.0mm** in **Y** direction and **0.0mm** in **Z** direction to reach **B1** position.
- The coordinate of the **B1** position is X=39.12mm, Y=0mm & Z=0mm.
- Similarly **"depth": 17.4** means the pipette can travel **17.4mm** in **Z** direction to reach the bottom of the plate in short the height of the well.
- Similarly **"diameter": 22.5** means the maximum diameter of the wells is **22.5mm**.
- Similarly **"total-liquid-volume": 16800** means the maximum volume of liquid store in one well is **16800ul**.

Folloded by reaching the **B3** position the following code will be follows.

```json move location to B3 well
"B3": {
                         "x": 39.12,
                         "y": 78.24,
                         "z": 0,
                         "depth": 17.4,
                         "diameter": 22.5,
                         "total-liquid-volume": 16800
                    }
```
- From the **A1** position the Pipette move **39.12mm** in **X** direction and **78.24mm** in **Y** direction and **0.0mm** in **Z** direction to reach **B3** position.
- The coordinate of the **B3** position is X=39.12mm, Y=78.24mm & Z=0mm.
- **"depth": 17.4** means the pipette can travel **17.4mm** in **Z** direction to reach the bottom of the plate in short the height of the well.
- **"diameter": 22.5** means the maximum diameter of the wells is **22.5mm**.
- **"total-liquid-volume": 16800** means the maximum volume of liquid store in one well is **16800ul**.


## Comparison of modules used in Opentron OT-2 fluid handeling system with standard module :

| Module                      | Description & Match and Mis-match Details                                       |
| --------------------------- | ------------------------------------------------------------ |
| 6 well plate                | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/6_well_plate/README.md) |
| 96 well plate Biorad        | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/96_well_plate_biorad/README.md) |
| T-25 Flask                  | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/T25_flask/README.md) |
| T-75 Flask                  | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/T75_flask/README.md) |
| 1000ul tip rack             | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/Tip_rack_1000ul/README.md) |
| 10ul tip rack               | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/Tip_rack_10ul/README.md) |
| Standard 96 well plate Flat | [URL LINK](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/Standard_module_compare_config_files/standard_96_well_plate/README.md) |


## Referance :
- Opentron OT-2 Code - [Link](https://github.com/TechnocultureResearch/Genotyper-Firmware/blob/dev/docs/Orchestrator/ContainerCoordinateSystem/default-containers.json)






