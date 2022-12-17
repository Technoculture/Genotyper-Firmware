from typing import List, Optional
from pydantic import BaseModel, validator
from json import load
# TODO: Is this seperation needed?
from genotyper.configuration.type import InvalidBaudRateError
# from enum import Enum, IntEnum


# class ComputeModule(str, Enum):
#     RPI4: "rpi4"
#     IMX8M: "imx8m"
#     DESKTOP: "desktop"


# class Architecture(str, Enum):
#     YOCTO: "yocto"
#     LINUX: "linux"
#     MACOS: "macos"
#     WINDOWS: "windows"


class WhoMe(BaseModel):
    name: str
    # compute_module: Optional[ComputeModule]
    # architecture: Optional[Architecture]

    # class Config:
    #     """https://stackoverflow.com/questions/65209934/pydantic-enum-field-does-not-get-converted-to-string"""
    #     use_enum_values = True


class Identity(BaseModel):
    model: str
    version: float
    firmware_version: float


# class BaudRate(IntEnum):
#     BAUD_9600: 9600
#     BAUD_115200: 115200


class PortAddr(BaseModel):
    __root__: str


class SerialNumber(BaseModel):
    __root__: str


class Serial(BaseModel):
    emulated_port_addr: PortAddr
    serial_number: SerialNumber
    baud_rate: int

    @validator('baud_rate')
    def baud_rate_is_valid(cls, rate: int) -> int:
        _valid_baud_rates = [9600, 115200]
        _valid_baud_rates_str = ", ".join(
            [str(rate) for rate in _valid_baud_rates])

        if rate not in _valid_baud_rates:
            raise InvalidBaudRateError(
                message="value must be a valid serial baud rate: {}".format(_valid_baud_rates_str))
        return rate

    @validator('serial_number')
    def serial_number_must_be_valid(cls, sl: str) -> str:

        return sl


class Orchestrator(BaseModel):
    identity: Identity
    serial: Serial


class ChipManager(BaseModel):
    identity: Identity
    serial: Serial


class SwabManager(BaseModel):
    identity: Identity
    serial: Serial


class InputQueueManager(BaseModel):
    identity: Identity
    serial: Serial


class SolutionManager(BaseModel):
    identity: Identity
    serial: Serial
    reagent_id: str


class ResourceManagers(BaseModel):
    chip_manager: ChipManager
    swab_manager: SwabManager
    input_queue_manager: InputQueueManager
    solution_managers: List[SolutionManager]


class ChipCentrifuge(BaseModel):
    identity: Identity
    serial: Serial


class Instruments(BaseModel):
    chip_centrifuges: List[ChipCentrifuge]
    # standalone_na_amplifiers: List[]


class Modules(BaseModel):
    orchestrator: Orchestrator
    resource_managers: ResourceManagers
    instruments: Instruments


class Offset(BaseModel):
    x: int
    y: int
    z: int


class LayoutItem(BaseModel):
    name: str
    serial_number: Optional[str] = None
    offset: Optional[Offset] = None
    rotation: Optional[int] = None


class RobotConfig(BaseModel):
    whome: WhoMe
    modules: Modules
    layout: List[LayoutItem]


def get_robot_config(config_file: str) -> RobotConfig:
    with open(file=config_file) as f:
        data = load(f)
        return RobotConfig(**data)
