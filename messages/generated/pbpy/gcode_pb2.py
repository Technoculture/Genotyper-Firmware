# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: gcode.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='gcode.proto',
  package='gcode',
  syntax='proto2',
  serialized_options=None,
  serialized_pb=_b('\n\x0bgcode.proto\x12\x05gcode\"\x95\x02\n\x07\x43ommand\x12!\n\x02op\x18\x01 \x02(\x0e\x32\x15.gcode.Command.Opcode\x12\x1f\n\x03\x61rg\x18\x02 \x01(\x0b\x32\x12.gcode.Command.Arg\x1aH\n\x03\x41rg\x12&\n\x03pos\x18\x01 \x01(\x0b\x32\x17.gcode.Command.PositionH\x00\x12\r\n\x03num\x18\x02 \x01(\rH\x00\x42\n\n\x08\x61rgtypes\x1a \n\x08Position\x12\t\n\x01x\x18\x01 \x02(\x11\x12\t\n\x01y\x18\x02 \x02(\x11\"Z\n\x06Opcode\x12\x08\n\x04HOME\x10\x00\x12\x08\n\x04MOVE\x10\x01\x12\x15\n\x11\x41\x42SOLUTE_POSITION\x10\x02\x12\x0c\n\x08LOAD_TIP\x10\x03\x12\x17\n\x13\x43HECK_TIP_PRESSENCE\x10\x04')
)



_COMMAND_OPCODE = _descriptor.EnumDescriptor(
  name='Opcode',
  full_name='gcode.Command.Opcode',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='HOME', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MOVE', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='ABSOLUTE_POSITION', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='LOAD_TIP', index=3, number=3,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='CHECK_TIP_PRESSENCE', index=4, number=4,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=210,
  serialized_end=300,
)
_sym_db.RegisterEnumDescriptor(_COMMAND_OPCODE)


_COMMAND_ARG = _descriptor.Descriptor(
  name='Arg',
  full_name='gcode.Command.Arg',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='pos', full_name='gcode.Command.Arg.pos', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='num', full_name='gcode.Command.Arg.num', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
    _descriptor.OneofDescriptor(
      name='argtypes', full_name='gcode.Command.Arg.argtypes',
      index=0, containing_type=None, fields=[]),
  ],
  serialized_start=102,
  serialized_end=174,
)

_COMMAND_POSITION = _descriptor.Descriptor(
  name='Position',
  full_name='gcode.Command.Position',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='x', full_name='gcode.Command.Position.x', index=0,
      number=1, type=17, cpp_type=1, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='y', full_name='gcode.Command.Position.y', index=1,
      number=2, type=17, cpp_type=1, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=176,
  serialized_end=208,
)

_COMMAND = _descriptor.Descriptor(
  name='Command',
  full_name='gcode.Command',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='op', full_name='gcode.Command.op', index=0,
      number=1, type=14, cpp_type=8, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='arg', full_name='gcode.Command.arg', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[_COMMAND_ARG, _COMMAND_POSITION, ],
  enum_types=[
    _COMMAND_OPCODE,
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=23,
  serialized_end=300,
)

_COMMAND_ARG.fields_by_name['pos'].message_type = _COMMAND_POSITION
_COMMAND_ARG.containing_type = _COMMAND
_COMMAND_ARG.oneofs_by_name['argtypes'].fields.append(
  _COMMAND_ARG.fields_by_name['pos'])
_COMMAND_ARG.fields_by_name['pos'].containing_oneof = _COMMAND_ARG.oneofs_by_name['argtypes']
_COMMAND_ARG.oneofs_by_name['argtypes'].fields.append(
  _COMMAND_ARG.fields_by_name['num'])
_COMMAND_ARG.fields_by_name['num'].containing_oneof = _COMMAND_ARG.oneofs_by_name['argtypes']
_COMMAND_POSITION.containing_type = _COMMAND
_COMMAND.fields_by_name['op'].enum_type = _COMMAND_OPCODE
_COMMAND.fields_by_name['arg'].message_type = _COMMAND_ARG
_COMMAND_OPCODE.containing_type = _COMMAND
DESCRIPTOR.message_types_by_name['Command'] = _COMMAND
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Command = _reflection.GeneratedProtocolMessageType('Command', (_message.Message,), dict(

  Arg = _reflection.GeneratedProtocolMessageType('Arg', (_message.Message,), dict(
    DESCRIPTOR = _COMMAND_ARG,
    __module__ = 'gcode_pb2'
    # @@protoc_insertion_point(class_scope:gcode.Command.Arg)
    ))
  ,

  Position = _reflection.GeneratedProtocolMessageType('Position', (_message.Message,), dict(
    DESCRIPTOR = _COMMAND_POSITION,
    __module__ = 'gcode_pb2'
    # @@protoc_insertion_point(class_scope:gcode.Command.Position)
    ))
  ,
  DESCRIPTOR = _COMMAND,
  __module__ = 'gcode_pb2'
  # @@protoc_insertion_point(class_scope:gcode.Command)
  ))
_sym_db.RegisterMessage(Command)
_sym_db.RegisterMessage(Command.Arg)
_sym_db.RegisterMessage(Command.Position)


# @@protoc_insertion_point(module_scope)