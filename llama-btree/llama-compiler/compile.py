'''
Module to deserialize the library trees input file nodes.lib.yaml. 
'''

import yaml
from pydantic import BaseModel#, Field
from typing import Callable, List, Optional#, Union
# from enum import Enum
# from abc import ABC, abstractmethod
import re
from langchain.agents import Tool


class ZenohConfig(BaseModel):
    modules: Optional[list[str]]
    min_reply: Optional[str]

class Node(BaseModel):
    type: str
    description: str
    zenoh: Optional[ZenohConfig]

    # data types and object type
    class Config:
        arbitrary_types_allowed = True
        allow_mutation = False


# extracting contents
with open('\\tmp\\tcr\\genodatalib\\library\\nodes.lib.yaml', "r") as f:
    nodes_dict = yaml.safe_load(f)

# deserializing
# deserialized_nodes = [(nodes,Node.parse_obj(node_dict)) for nodes,node_dict in nodes_dict['content'].items()]
deserialized_nodes = [Node.parse_obj(node_dict) for node_dict in nodes_dict['content'].values()]
# print(deserialized_nodes)

class NodeInput(BaseModel):
    name: str
    mode: str
    modules: Optional[list[str]]
    min_reply: Optional[str]
    description: str
    executor: Callable
    preconditions: Optional[List[str]] = None
    needs_tool: Optional[str] = None
    input_format: Optional[str] = None

    @classmethod
    def from_docstring(cls, fn: Callable):
        docstring = fn.__doc__
        lines = docstring.strip().split("\n")
        fields = {}

        for line in lines:
            # colon is the separator
            match = re.match(r"(.*):(.*)", line.strip())
            if match:
                key, value = match.groups()
                key, value = key.strip(), value.strip()  # remove whitespace
                fields[key] = value.split(
                    ",") if key == "preconditions" else value
        fields["executor"] = fn
        fields["name"] = fn.__name__

        # output: deserialized contents as python objects
        for node in deserialized_nodes:
            # type
            fields["mode"] = node.type

            # zenoh
            if node.zenoh is not None:
                # modules
                fields["modules"] = node.zenoh.modules
                # min_reply
                fields["min_reply"] = node.zenoh.min_reply
            else:
                fields["modules"] = None
                fields["min_reply"] = None

        return cls(**fields)


def node_to_tool(node: NodeInput) -> Tool:
    '''
    A function that takes node as a collection of class NodeInput objects and returns a Tool that takes a tool function as input.
    '''

    need_tool_str = f"NEEDS TOOL = {node.needs_tool}" if node.needs_tool else ""
    preconditions_str = f"!!! PRECONDITIONS = {', '.join(node.preconditions)}" if node.preconditions else "No preconditions"

    return Tool(
        f"Executing {node.name}; Fetching each zenoh module for use in the list {node.modules} one by one; Collective final response allowed for all available zenoh modules can be {node.min_reply} number of times",
        node.executor,
        f"{node.description}; [{preconditions_str}; {need_tool_str}]; Accepts inputs of the form: {node.input_format}",
    )


def node_tool(fn: Callable) -> Tool:
    '''
    Function decorates a tool function's returned string response with the string inputs used as Tool class instance
    '''
    node = NodeInput.from_docstring(fn)
    return node_to_tool(node)


def send(address, payload):
    """
    Uses zenoh endpoint (address) to send payload
    """
    ...


async def send_and_await_response(address, payload) -> str:
    """
    Uses zenoh endpoint (address) to send payload and waits for a response
    """
    
    return "Response"


@node_tool
async def is_tip_available(input: str):
    """
    mode: condition
    description: Verify if a tip is available for use
    preconditions: The pipette is attached to the system and selected as the active tool
    needs_tool: A Pipette
    input_format: Either of 10ul, 100ul or 1000ul ONLY
    """
    if input not in ["10ul", "100ul", "1000ul"]:
        return f"Tip of size {input} is not available"

    response = await send_and_await_response("pipette/tip_slider/position", "0")
    return response


@node_tool
def tip_stock_error(input: str):
    """
    mode: action
    description: Address an error related to the tip stock
    """
    return "Tip stock error"


@node_tool
def is_tip_available_in_tray(input: str):
    """
    mode: condition
    description: Check if the tip is available in the tray
    """
    return "Tip is available in tray"


@node_tool
def is_discard_success(input: str):
    """
    mode: condition
    description: Check if the discard was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Discard was successful"


@node_tool
def tray_maintainance_error(input: str):
    """
    mode: action
    description: Handle tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tray maintainance error"


@node_tool
def is_load_new_tray_successful(input: str):
    """
    mode: condition
    description: Check if the load new tray was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray was successful"


@node_tool
def load_new_tray_maintenance_error(input: str):
    """
    mode: action
    description: Handle load new tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray maintainance error"


@node_tool
def is_already_in_position(input: str):
    """
    mode: condition
    description: Check if the tip slider is already in position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider is already in position"


@node_tool
def is_slider_position_reached(input: str):
    """
    mode: condition
    description: Check if the tip slider position was reached
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider position was reached"


@node_tool
def is_caught_tip_firm_and_oriented(input: str):
    """
    mode: condition
    description: Check if the caught tip is firm and oriented
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Caught tip is firm and oriented"


@node_tool
def is_pick_up_success(input: str):
    """
    mode: condition
    description: Check if the pick up was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Pick up was successful"


@node_tool
def handle_pickup_failure(input: str):
    """
    mode: action
    description: Handle pick up failure
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Pick up failure"


@node_tool
def move_slider_to_load_position(input: str):
    """
    mode: action
    description: Move the tip slider to load position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Moved tip slider to load position"


@node_tool
def load_next_tray(input: str):
    """
    mode: action
    description: Load the next tray
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Loaded next tray"


@node_tool
def move_tip_slider_to_position(input: str):
    """
    mode: action
    description: Move the tip slider to position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Moved tip slider to position"


@node_tool
def pick_up_tip_using_gantry(input: str):
    """
    mode: action
    description: Pick up tip using gantry
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Picked up tip using gantry"


@node_tool
def goto_discard_position(input: str):
    """
    mode: action
    description: Go to discard position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Went to discard position"


@node_tool
def prepare_to_discard(input: str):
    """
    mode: action
    description: Prepare to discard
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Prepared to discard"


@node_tool
def eject_tip(input: str):
    """
    mode: action
    description: Eject tip
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Ejected tip"


@node_tool
def is_discard_tip_successful(input: str):
    """
    mode: condition
    description: Check if the discard tip was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    needs_tool: A Pipette
    """
    return "Discard tip was successful"
