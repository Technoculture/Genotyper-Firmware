"""
Objectives:
    - Given a set of nodes (action, selector and sequence nodes), generate a tree of nodes
    - Each action, selector and sequence should be modelled as a tool
    - Each tool should have a pre-condition
    - Preconditions should be checked before executing the tool
    - The tree should be traversed in a depth-first manner
    - Visualize the generated tree

Task:
    - Given a task in english, generate a correct behavior tree
"""

import langchain_visualizer
import asyncio
from dataclasses import dataclass
import re
from pydantic import BaseModel, Field

from langchain.llms import OpenAI
from langchain import PromptTemplate
from langchain.agents import tool, Tool
from langchain.agents import initialize_agent

from typing import Callable, List, Optional, Union
from enum import Enum
from abc import ABC, abstractmethod

from graphviz import Digraph

template = """
Question: {q}
"""


class NodeMode(str, Enum):
    ACTION = "action"
    CONDITION = "condition"


class Node(BaseModel):
    name: str
    mode: NodeMode
    description: str
    executor: Callable
    preconditions: List[str]

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

        return cls(**fields)


def node_to_tool(node: Node) -> Tool:
    preconditions_str = ""
    for precond in node.preconditions:
        preconditions_str += f"- {precond.strip()}\n"
    print(preconditions_str.strip())

    return Tool(
        node.name,
        lambda msg: f"Executing {node.name}",
        f"""{node.description}
PRECONDITIONS:
{preconditions_str}
        """,
    )


def node_tool(fn: Callable) -> Tool:
    node = Node.from_docstring(fn)
    return node_to_tool(node)


@node_tool
def is_tip_available():
    """
    mode: condition
    description: Check if the tip is available
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip is available"


@node_tool
def tip_stock_error():
    """
    mode: action
    description: Handle tip stock error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip stock error"


@node_tool
def is_tip_available_in_tray():
    """
    mode: condition
    description: Check if the tip is available in the tray
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip is available in tray"


@node_tool
def is_discard_success():
    """
    mode: condition
    description: Check if the discard was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Discard was successful"


@node_tool
def tray_maintainance_error():
    """
    mode: action
    description: Handle tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tray maintainance error"


@node_tool
def is_load_new_tray_successful():
    """
    mode: condition
    description: Check if the load new tray was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray was successful"


@node_tool
def load_new_tray_maintenance_error():
    """
    mode: action
    description: Handle load new tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray maintainance error"


@node_tool
def is_already_in_position():
    """
    mode: condition
    description: Check if the tip slider is already in position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider is already in position"


@node_tool
def is_slider_position_reached():
    """
    mode: condition
    description: Check if the tip slider position was reached
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider position was reached"


@node_tool
def is_caught_tip_firm_and_oriented():
    """
    mode: condition
    description: Check if the caught tip is firm and oriented
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Caught tip is firm and oriented"


@node_tool
def is_pick_up_success():
    """
    mode: condition
    description: Check if the pick up was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Pick up was successful"


@node_tool
def handle_pickup_failure():
    """
    mode: action
    description: Handle pick up failure
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Pick up failure"


@node_tool
def move_slider_to_load_position():
    """
    mode: action
    description: Move the tip slider to load position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Moved tip slider to load position"


@node_tool
def load_next_tray():
    """
    mode: action
    description: Load the next tray
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Loaded next tray"


@node_tool
def move_tip_slider_to_position():
    """
    mode: action
    description: Move the tip slider to position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Moved tip slider to position"


@node_tool
def pick_up_tip_using_gantry():
    """
    mode: action
    description: Pick up tip using gantry
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Picked up tip using gantry"


@node_tool
def goto_discard_position():
    """
    mode: action
    description: Go to discard position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Went to discard position"


@node_tool
def prepare_to_discard():
    """
    mode: action
    description: Prepare to discard
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Prepared to discard"


@node_tool
def eject_tip():
    """
    mode: action
    description: Eject tip
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Ejected tip"


@node_tool
def is_discard_tip_successful():
    """
    mode: condition
    description: Check if the discard tip was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Discard tip was successful"


tools = [
    is_tip_available,
    tip_stock_error,
    is_tip_available_in_tray,
    is_discard_success,
    move_slider_to_load_position,
    load_next_tray,
    is_load_new_tray_successful,
    load_new_tray_maintenance_error,
    is_already_in_position,
    move_tip_slider_to_position,
    is_slider_position_reached,
    tray_maintainance_error,
    pick_up_tip_using_gantry,
    is_caught_tip_firm_and_oriented,
    goto_discard_position,
    prepare_to_discard,
    eject_tip,
    is_discard_tip_successful,
    is_pick_up_success,
    handle_pickup_failure
]

llm = OpenAI(temperature=0.0, model_name="gpt-3.5-turbo")


prompt = PromptTemplate(
    input_variables=["q"],
    template=template
)


agent = initialize_agent(
    tools, llm, agent="zero-shot-react-description", verbose=True)


async def llmtree_agent():
    q = "Task: Pick up the sample and place it in the PCR."
    return agent.run(
        q
    )


langchain_visualizer.visualize(llmtree_agent)
