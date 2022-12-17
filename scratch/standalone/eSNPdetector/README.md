# eSNP Detector v1

We present an eSNP detection instrument. This instrument uses electrochemical sensing in order to detect conjugation of target ssDNA with predefined molecular probes.

## Objective
- Interface with an [EmStat Pico](https://www.palmsens.com/product/oem-emstat-pico-module/) to conduct predefined Electrochemical sensing protocols.
- Provide a simple end-user facing UI for conducting SNP detection tests

## UI for HMI
<img width="400" alt="" src="https://user-images.githubusercontent.com/33483920/173353425-01667f5f-60c8-48a5-a5b9-8f860af272af.mov">

## TODOs
- [ ] v1: single SNP detection using electrochemical sensing
- [ ] v2: Add **Melting Curve Analysis** (MCA) to the electrochemical sensing
- [ ] v3: Add 16-way multiplexing
- [ ] v4: Add 16-way multiplexing along with 16 MCA channels

# Working Principle
<img width="830" alt="Electrochimicial DNA sensing" src="https://user-images.githubusercontent.com/33483920/173180289-01205983-66f9-4da9-8b32-e53d9eb5be9a.PNG">


# Build Instrunctions

> It is recommended to `pipenv` to manage dependencies and creation of a virtual environment for the project. Useful for developemnt.

```bash
$ pipenv run python main.py

Or,

$ pipenv shell
$ python main.py
```

## Dependencies
### Hardware 
- Raspberry Pi Zero
- Hyperpixel 4.0
- EmStat Pico Module
- 3 terminal Slider Switch (Optional)
- Green LED (Optional)
- A rate limiting resistor for LED (Optional)
- A DC Power Jack (Optional)

### Software
- python 3.8
- pipenv
- `pyqt5`
- [`palmsens (internal)`](https://github.com/PalmSens/MethodSCRIPT_Examples/tree/master/MethodSCRIPTExample_Python/MethodSCRIPTExample_Python/palmsens)


# System Components Diagram
```mermaid
flowchart TD

subgraph platform
Pl[Hardware + Firmware]
end

subgraph Hardware
H[Hyperpixel 4.0 Display] --> | Header HAT | Pi
Pi[Raspberry Pi Zero]
E[EmStat Pico] --> | UART / USB | Pi
p[5V, 1A] --> sw[Slider Switch]
sw --> | resistor | l[Power LED]
end

Hardware --> platform
Firmware --> platform
E <-.-> | Communicate over Serial | P

subgraph Firmware
P[/Python UI/] --> | Setup.py | R
M([Method Script]) --> P
R[/Raspbian OS/]
Conf([Config.txt]) --> | Set OS Params | R
sw --> | power | Pi
end
```

## Development Environment [Not Implemented]
```mermaid
flowchart TD

E[EmStat Pico] --> | USB | Comp[Development Computer]

subgraph Firmware
P[/Python UI/] --> | Execute: main.py | Linux
M([Method Script]) --> P
Linux[Any General Purpose OS]
end

E <-.-> | Communicate over Serial | P
```

## Test Environment [Not Implemented]
```mermaid
flowchart TD

E[/EmStat Pico tty Emulator/] --> | Pseudoterminal | Fake[Fake Serial Device]

subgraph Firmware
P[/Python UI/] --> | Execute: main.py | Linux
M([Method Script]) --> P
Linux[Linux OS]
end

Fake <-.-> | Communicate over Serial | P
```

## Production Environment [Not Implemented]
```mermaid
flowchart TD

subgraph platform
Pl[Hardware + Firmware]
end

subgraph Hardware
H[Hyperpixel 4.0 Display] --> | Header HAT | Pi
Pi[Raspberry Pi Zero]
E[EmStat Pico] --> | UART / USB | Pi
p[5V, 1A] --> sw[Slider Switch]
sw --> | resistor | l[Power LED]
end

Hardware --> platform
Firmware --> platform
E <-.-> | Communicate over Serial | P

subgraph Firmware

subgraph meta-eSNP
subgraph recipie-ui
P[/Python UI/] --- | Setup.py | PR[recipie-ui]
M([Method Script]) --- PR
init[init file] --> PR
end

subgraph recipie-boot
Conf([Config.txt]) --> | Set OS Params | Appe[Append File]
end
end

meta-rpi --> Y
meta-eSNP --> Y[Yocto Image]
M --> P
sw --> | power | Pi
end
```

## EmStat Communcation Diagram
![MethodSCRIPT-Generic](https://user-images.githubusercontent.com/33483920/173180292-e87789e0-e0a4-407e-b505-6f4d2169696e.gif)
