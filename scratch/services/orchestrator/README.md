# Objective 
Interprets g-code into motion

| Op Code | Meaning                                                               | Example                                                            |
|---------|-----------------------------------------------------------------------|--------------------------------------------------------------------|
| `G1`    | Linear movement commands                                              | `G1 X30 Y15 Z20 E5`, where E refers to the pipette linear actuator |
| `G28`   | Home Position                                                         | `G28` ?                                                            |
| `G90`   | Set absolute position                                                 | `G90 X100 Y90 Z78 E4` ?                                            |
| `M701`  | Load pipette tip                                                      |                                                                    |
| `M1001` | Check tip presence and alignment with laser system (optional feature) |                                                                    |
