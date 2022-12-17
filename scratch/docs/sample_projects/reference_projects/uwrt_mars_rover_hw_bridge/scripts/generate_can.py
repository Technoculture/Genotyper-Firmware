#!/usr/bin/env python3
'''
Script that generates the DBC file and C source code given the yaml CAN description.
This script should be called in the root directory of the uwrt_mars_rover_hw_bridge repository.
'''

import subprocess
import os
import cantools
import yaml
import shutil
import glob
import re

import generate_can_enums
import generate_can_wrapper

supported_years = ['2021', '2022']

HWBRIDGE_ROOT_PATH = os.getcwd()

ROVER_CAN_NAME = 'uwrt_mars_rover_can'

DBC_FILE_NAME = ROVER_CAN_NAME + '.dbc'
DBC_DUMP_FILE_NAME = ROVER_CAN_NAME + '_dump.txt'
CAN_ENUMS_HEADER_FILE_NAME = ROVER_CAN_NAME + '_enums.h'
CAN_WRAPPER_HEADER_FILE_NAME = ROVER_CAN_NAME + '_wrapper.h'
CAN_WRAPPER_SOURCE_FILE_NAME = ROVER_CAN_NAME + '_wrapper.cpp'

SCRIPT_FOLDER_PATH = os.path.join(HWBRIDGE_ROOT_PATH, 'scripts')
CAN_FOLDER_PATH = os.path.join(HWBRIDGE_ROOT_PATH, 'can')

CLANG_FORMAT_CMD = 'clang-format-11'

for year in supported_years:

    YAML_FILE_NAME = ROVER_CAN_NAME + '_' + year + '.yaml'
    GENERATED_FOLDER_PATH = os.path.join(CAN_FOLDER_PATH, 'generated_' + year)
    YAML_FILE_PATH = os.path.join(CAN_FOLDER_PATH, YAML_FILE_NAME)


    AUTOGEN_message_enums = {}
    AUTOGEN_signal_enums = []
    AUTOGEN_signal_enum_choices = {}
    AUTOGEN_msg_map = {}
    AUTOGEN_original_msg_names = {}


    # https://github.com/eerimoq/cantools/blob/871581b57785fdbd79c118878f9b9c148984963c/cantools/database/can/c_source.py#L762
    def camel_to_snake_case(value):
        value = re.sub(r'(.)([A-Z][a-z]+)', r'\1_\2', value)
        value = re.sub(r'(_+)', '_', value)
        value = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', value).lower()
        value = re.sub(r'[^a-zA-Z0-9]', '_', value)
        return value


    # load yaml file
    with open(YAML_FILE_PATH) as file:
        can_yaml = yaml.load(file, Loader=yaml.FullLoader)

    bus_name = list(can_yaml.keys())[0]
    bus_frequency = can_yaml[bus_name]['bus_frequency']
    canid_filter_mask = can_yaml[bus_name]['canid_filter_mask']
    canid_filters = can_yaml[bus_name]['canid_filters']

    # sort nodes in alphabetical order
    can_yaml[bus_name]['nodes'].sort()

    roboteq_enums = can_yaml[bus_name]['roboteq_canids']

    # extract CAN nodes
    nodes = []
    for node in can_yaml[bus_name]['nodes']:
        nodes.append(
            cantools.database.can.Node(name=node, comment=None)
        )

    # if there are CAN messages defined
    if can_yaml[bus_name]['messages']:

        # sort messages by id
        can_yaml[bus_name]['messages'].sort(key=lambda x: x[list(x.keys())[0]]['id'])

        # extract CAN messages
        messages = []
        for message in can_yaml[bus_name]['messages']:
            message_name = list(message.keys())[0]
            message = message[message_name]

            # add to autogen dicts
            msg_snake_upper = camel_to_snake_case(message_name).upper()
            AUTOGEN_message_enums[msg_snake_upper] = message['id']
            AUTOGEN_msg_map[msg_snake_upper] = []
            AUTOGEN_original_msg_names[msg_snake_upper] = message_name

            # sort senders and receivers
            message['senders'].sort()
            message['receivers'].sort()

            # track start bit for each signal
            startbit = 0

            # extract message signals
            signals = []
            for signal in message['signals']:
                signal_name = list(signal.keys())[0]
                signal = signal[signal_name]

                # add to autogen signal enums list and msg map dict
                signal_snake_upper = camel_to_snake_case(signal_name).upper()
                if 'values' in signal and signal['values'] is not None:
                    AUTOGEN_signal_enum_choices[signal_snake_upper] = signal['values']
                AUTOGEN_signal_enums.append(signal_snake_upper)
                AUTOGEN_msg_map[msg_snake_upper].append(signal_snake_upper)

                length = signal['length']
                is_signed = signal['is_signed']

                # NOTE: it is assumed that min/max bounds do not cover SNA values

                # if given both scale/offset and min/max
                if 'scale' in signal and 'offset' in signal and 'min' in signal and 'max' in signal:
                    scale = signal['scale']
                    offset = signal['offset']
                    sig_min = signal['min']
                    sig_max = signal['max']

                # else if only given scale/offset (no min/max)
                elif 'scale' in signal and 'offset' in signal:
                    scale = signal['scale']
                    offset = signal['offset']

                    # calculate min/max based on scale/offset and length
                    sig_min = - (2**(length-1)-1) * scale + \
                        offset if is_signed else offset
                    sig_max = (2**(length-1)-1) * scale + \
                        offset if is_signed else (2**length-2) * scale + offset

                # else if only given min/max (no scale/offset)
                elif 'min' in signal and 'max' in signal:
                    sig_min = signal['min']
                    sig_max = signal['max']

                    # calculate scale/offset based on min/max and length
                    scale = (sig_max - sig_min) / (2**length - 2)
                    offset = sig_min + (2**(length-1)-1) * \
                        scale if is_signed else sig_min

                else:
                    print('Missing fields for signal "' + signal_name +
                        '" in message "' + message_name + '"!')
                    print('Supply either signal scale/offset or signal min/max')
                    print('Script terminated.')
                    exit()

                # add these missing fields to the yaml
                signal['scale'] = scale
                signal['offset'] = offset
                signal['min'] = sig_min
                signal['max'] = sig_max

                signals.append(
                    cantools.database.can.Signal(
                        name=signal_name,
                        start=startbit,
                        length=signal['length'],
                        byte_order='little_endian',
                        is_signed=signal['is_signed'],
                        scale=signal['scale'],
                        offset=signal['offset'],
                        minimum=signal['min'],
                        maximum=signal['max'],
                        unit=signal['unit'],
                        choices=signal['values'] if 'values' in signal else None,
                        comment=signal['comment'],
                        receivers=message['receivers']
                    )
                )

                startbit += signal['length']

            messages.append(
                cantools.database.can.Message(
                    frame_id=message['id'],
                    name=message_name,
                    length=message['size'],
                    signals=signals,
                    comment=message['comment'],
                    senders=message['senders'],
                    cycle_time=(None if message['frequency']
                                == 0 else 1/message['frequency']),
                )
            )

    can_db = cantools.database.can.Database(
        messages=messages,
        nodes=nodes
    )

    # create "generated" folder
    if not os.path.exists(GENERATED_FOLDER_PATH):
        os.mkdir(GENERATED_FOLDER_PATH)

    # generate normalized yaml file with all fields
    TEMP_YAML_PATH = os.path.join(GENERATED_FOLDER_PATH, 'temp.yaml')
    try:
        with open(TEMP_YAML_PATH, 'w') as file:
            yaml.dump(can_yaml, file, sort_keys=False,
                    indent=2)

        shutil.copyfile(TEMP_YAML_PATH, YAML_FILE_PATH)
        print('Successfully updated', YAML_FILE_NAME)
    except:
        print('ERROR - yaml file not updated!')
    finally:
        if os.path.exists(TEMP_YAML_PATH):
            os.remove(TEMP_YAML_PATH)

    # cd into generated folder
    os.chdir(GENERATED_FOLDER_PATH)

    # generate dbc file
    cantools.database.dump_file(can_db, DBC_FILE_NAME)
    print('Successfully generated', DBC_FILE_NAME)

    # generate human-readable dbc dump
    subprocess.run(
        ['python3 -m cantools dump ' + DBC_FILE_NAME + ' > ' + DBC_DUMP_FILE_NAME], shell=True)
    print('Successfully generated', DBC_DUMP_FILE_NAME)

    # generate c source from dbc file
    subprocess.run(
        ['python3 -m cantools generate_c_source ' + DBC_FILE_NAME], shell=True)

    # generate CAN enums header
    vars = {
        'canbus_frequency_value': bus_frequency,
        'canid_filter_mask': canid_filter_mask,
        'canid_filters': canid_filters,
        'roboteq_enums': roboteq_enums,
        'msg_enums': AUTOGEN_message_enums,
        'cansignal_enums': AUTOGEN_signal_enums,
        'cansignal_enum_choices': AUTOGEN_signal_enum_choices,
    }
    generate_can_enums.generate(CAN_ENUMS_HEADER_FILE_NAME, vars)
    print('Successfully generated', CAN_ENUMS_HEADER_FILE_NAME)

    # generate CAN wrapper header and cpp source
    vars = {
        'rover_can_name': ROVER_CAN_NAME,
        'enums_header_file_name': CAN_ENUMS_HEADER_FILE_NAME,
    }
    generate_can_wrapper.generate_header(CAN_WRAPPER_HEADER_FILE_NAME, vars)

    vars = {
        'rover_can_name': ROVER_CAN_NAME,
        'can_wrapper_header_file_name': CAN_WRAPPER_HEADER_FILE_NAME,
        'msg_map': AUTOGEN_msg_map,
        'original_msg_names': AUTOGEN_original_msg_names,
    }
    generate_can_wrapper.generate_source(CAN_WRAPPER_SOURCE_FILE_NAME, vars)

    print('Successfully generated', CAN_WRAPPER_HEADER_FILE_NAME,
        'and', CAN_WRAPPER_SOURCE_FILE_NAME)

    # format generated files
    files = []
    extensions = ['*.c', '*.cpp', '*.h']
    for extension in extensions:
        files.extend(glob.glob(os.path.join(GENERATED_FOLDER_PATH, extension)))
    for f in files:
        subprocess.run([CLANG_FORMAT_CMD + ' -i ' + f], shell=True)

# delete pycache folder
os.chdir(HWBRIDGE_ROOT_PATH)
PYCACHE_FOLDER_PATH = os.path.join(SCRIPT_FOLDER_PATH, '__pycache__')
if os.path.exists(PYCACHE_FOLDER_PATH):
    shutil.rmtree(PYCACHE_FOLDER_PATH)
