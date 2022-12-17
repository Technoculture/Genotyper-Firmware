from pydantic import BaseModel
from typing import Dict
import os
from json import dump
# TODO: is this seperation needed?
from gero.configuration.type import ConfigFilenameError


def _deserialize_smoothie_config(config_path: str) -> Dict[str, str]:
    """Reads a `config.txt` file for a Smoothieware board

    Args:
        config_path (str): path to a `config.txt` file for the Smoothieware board

    Returns:
        dict[str, str]: return a python dictionary, serialized form of the config file
    """

    # Dictionary to story the keys and values from `config.txt`
    myvars: Dict[str, str] = {}
    (counter_lines, counter_skipped_lines) = 0, 0

    with open(config_path) as myfile:
        for line in myfile:
            counter_lines += 1

            # split code and comments for each of the line
            _code, __comment = line.partition("#")[::2]

            # skip empty lines
            if _code.strip() == "":
                counter_skipped_lines += 1
                continue

            # partition key value pair using a <space> character
            name, var = _code.partition(" ")[::2]

            # strip leading and trailing whitespace from the key and value strings
            name, var = name.strip(), str(var.strip())
            name = name.replace('.', '_')
            myvars[name] = var

        # debug(f"line skipped: {skipped_line_count} out of {line_count}")

        return myvars


def _check_config_file(config_path: str) -> None:
    """Check if the path is named `config.txt` and that the path exists.

    Args:
        config_path (str): a path to a valid `config.txt` file

    Raises:
        FileNotFoundError: If a file is not found at the path provided
        ValueError: If the file name is not `config.txt`
    """

    _filename: str = os.path.basename(config_path)

    if not os.path.exists(config_path):
        raise FileNotFoundError(
            "The path must point to a file named config.txt")

    if _filename != "config.txt":
        raise ConfigFilenameError()


def _dump_dict_to_json(dictionary: Dict[str, str], path: str = "config.json") -> None:
    """Takes a python dictionary and saves a json file with the contents of the python dict. The file name(or path) can be provided as one of the inputs.

    Args:
        dictionary (dict[str, str]): takes a deserialized form of the `config.txt`
        path (str, optional): path to the `config.txt` file. Defaults to "config.json".
    """

    with open(path, 'w+') as file:
        dump(dictionary, file)


def smoothie_config_to_dict(config_file: str) -> Dict[str, str]:
    """Converts the `config.txt` file for smoothieware to a python dictionary object

    Args:
        config_file (str): path to a `config.txt` file
    """

    _check_config_file(config_file)
    return _deserialize_smoothie_config(config_file)


def smoothie_config_to_json(config_file: str, output_file: str):
    """Converts the `config.txt` file for smoothieware to a json file

    Args:
        config_file (str): path to a `config.txt` file
    """

    _smoothie_dict = smoothie_config_to_dict(config_file)
    _dump_dict_to_json(_smoothie_dict, output_file)


class SmoothieConfig(BaseModel):
    """
    This object parses and deserializes relavant sections of the config file from its .json form. 
    Config file is assumed to have valid values against each of the keys. These values are not validated for their correctness by this model.
    """

    mm_per_arc_segment: float
    mm_max_arc_error: float
    alpha_steps_per_mm: float
    beta_steps_per_mm: float
    gamma_steps_per_mm: float
    extruder_hotend_steps_per_mm: float
    junction_deviation: float
    x_axis_max_speed: int
    y_axis_max_speed: int
    z_axis_max_speed: int
    extruder_hotend_max_speed: float
    temperature_control_hotend_designator: str

    temperature_control_hotend_set_m_code: str
    temperature_control_hotend_set_and_wait_m_code: str
    temperature_control_bed_set_m_code: str
    temperature_control_bed_set_and_wait_m_code: str

    switch_fan_input_on_command: str
    switch_fan_input_off_command: str
    switch_servo_input_on_command: str
    switch_servo_input_off_command: str

    switch_fan_output_type: str
    switch_servo_output_type: str
    switch_servo_pwm_period_ms: float
    corexy_homing: bool

    alpha_min_endstop: str
    alpha_max_endstop: str
    alpha_homing_direction: str
    alpha_min: float
    alpha_max: float
    beta_min: float
    beta_max: float
    beta_min_endstop: str
    beta_max_endstop: str
    beta_homing_direction: str
    gamma_min_endstop: str
    gamma_homing_direction: str
    gamma_min: float
    gamma_max: float
    alpha_max_travel: float
    beta_max_travel: float
    gamma_max_travel: float

    alpha_fast_homing_rate_mm_s: float
    alpha_slow_homing_rate_mm_s: float
    beta_fast_homing_rate_mm_s: float
    beta_slow_homing_rate_mm_s: float
    gamma_fast_homing_rate_mm_s: float
    gamma_slow_homing_rate_mm_s: float

    alpha_homing_retract_mm: float
    beta_homing_retract_mm: float
    gamma_homing_retract_mm: float

    homing_order: str
    zprobe_probe_height: float
    uart0_baud_rate: int


def get_smoothie_config(config_file: str) -> SmoothieConfig:
    config_dict = smoothie_config_to_dict(config_file)
    return SmoothieConfig(**config_dict)
