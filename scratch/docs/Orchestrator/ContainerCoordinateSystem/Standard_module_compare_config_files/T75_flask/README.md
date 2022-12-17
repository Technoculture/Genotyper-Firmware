# From Opentron code :

- depth: 163 mm
- diameter: 25 mm
- total-liquid-volume: 75000 ul

# For standard T-75 flask :

- depth:  150 mm
- diameter: 25 mm
- total-liquid-volume: 75000 ul

```json T-25_flask code
"T75-flask": {
               "origin-offset": {
                    "x": 42.75,
                    "y": 63.875
               },
               "locations": {
                    "A1": {
                         "x": 0,
                         "y": 0,
                         "z": 0,
                         "depth": 163,
                         "diameter": 25,
                         "total-liquid-volume": 75000
                    }
               }
          }
```



###### Image - 1 standard T-75 flask

![T75 FLASK](https://user-images.githubusercontent.com/51780510/169042643-a5610073-9748-43d5-a68d-1dd3de17dc91.jpeg)



###### Image - 2 standard T-75 flask cad image

![T75_cad_image](https://user-images.githubusercontent.com/51780510/169042693-27bc7d3e-0ce9-41a9-adba-8ac7322ac046.JPG)


###### CAD of standard T-75 flask

 [T75_flask_cad.dwg](T75_flask_cad.dwg) 



###### Table-1 : Standard T-75 flask product description

|          | Description | URL  |
| -------- | ----------- | ---- |
| Material |  USP class VI virgin polystyrene           |  -    |
| Supplier |    Nest Scientific         |  ([Links Url](https://www.stellarscientific.com/t-75-75cm2-polystyrene-cell-culture-flask-vent-cap-non-treated-surface-rnase-and-dnase-free-sterile-5-pk-100-cs/))    |





###### Table-2 : Measurement techniques and specification of standard T-75 flask

| Standard 6 well plate | Measurement technique                                        | Value    |
| --------------------- | ------------------------------------------------------------ | -------- |
| depth                 | Use Vernier caliper, taking reading from top opening cap of the T-75 flask through the bottom surface | 150 mm   |
| diameter              | Use Vernier caliper to measure the diameter of the top opening cap | 25 mm    |
| total liquid volume   | Simply calculate after determination of the height and diameter | 75000 ul |



###### Table-3 : T-75 flask comparison table

|                         | FROM OPENTRON CODE T-75 FLASK | STANDARD T-75 FLASK | RESULT    |
| ----------------------- | ----------------------------- | ------------------- | --------- |
| **Depth**               | 163 mm                        | 150 mm              | Not Match |
| **Diameter**            | 25 mm                         | 25 mm               | Match     |
| **Total Liquid Volume** | 75000 ul                      | 75000 ul            | Match     |
| **Geometric position**  | -                             | -                   | -         |
