#  Copyright (c) Technoculture Research, 2022. All rights reserved.

class Labware:
    pass


class Instrument:
    pass


def load_labware(name: str, deck: int) -> Labware:
    ...


def load_instrument(model: str, mount: int, tip_racks: int) -> Instrument:
    ...
