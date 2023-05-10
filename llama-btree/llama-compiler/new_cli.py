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

import os
import json
from datetime import datetime
import asyncio

# from langchain.llms import OpenAI
from langchain.chat_models import ChatOpenAI
from langchain import PromptTemplate
from langchain.llms import HuggingFacePipeline
from langchain import LLMChain
from langchain.agents import ZeroShotAgent, AgentExecutor
from transformers import AutoTokenizer, AutoModelForCausalLM, pipeline
from langchain.agents import initialize_agent
from langchain.agents import AgentType
import langchain_visualizer
from graphviz import Digraph

from new_compile import *
from cred import API_KEY


# def get_llm(*, vendor: str = "openai", model_name: str = ""):
#     if vendor == "openai":
#         model_name = "gpt-3.5-turbo" if model_name == "" else model_name
#         llm = ChatOpenAI(temperature=0, model_name=model_name, openai_api_key=API_KEY, verbose=True)
#         # llm = OpenAI(temperature=0, model_name=model_name, openai_api_key=API_KEY, verbose=True)
#         return llm
#     elif vendor == "hf":
#         model_name = "cerebras/Cerebras-GPT-111M" if model_name == "" else model_name
#         tokenizer = AutoTokenizer.from_pretrained(model_name)
#         model = AutoModelForCausalLM.from_pretrained(model_name)
#         pipe = pipeline(
#             "text-generation", model=model, tokenizer=tokenizer,
#             max_new_tokens=100, early_stopping=True, no_repeat_ngram_size=2
#         )
#         llm = HuggingFacePipeline(pipeline=pipe, verbose=True)
#         return llm
#     else:
#         raise ValueError("Invalid vendor")

# llm = get_llm()

llm = ChatOpenAI(temperature=0.0, model_name="gpt-3.5-turbo", openai_api_key=API_KEY)

# prefix = """Given a task description, generate a sequence of actions that can be performed to complete the task input.
# Only use the available tools to complete the task. A tool is a collection of information that you will use to generate the actionable steps to complete the task. A tool consists of a node with a 'node name' that defines one of the sub-task needed to complete the main given task,
# a 'node type' is the node's type that defines what a tool's node is, a list of 'zenoh modules' that is used to create actionable steps by taking each firmware module as a physical tool, 'minimum reply' that decides how many actionable steps you can generate per 'node_name', 'description' that is telling what the particular tool is doing.
# You have access to the following tools: """

prefix = """Act as a robotic task planner. Given a task description, you have to complete the task by planning and generating a sequence of actionable steps.
Only use the available tools to complete the task. Check the validity of each tool description. Use a tool only once.
You have access to the following tools:
{tools} """

suffix = """
DO NOT REQUEST A TOOL THAT ARE NOT IN THE TOOL LIST.
BE VERY CAREFUL WITH THIS ONE.

Task: {input}"""

# prompt_template = prefix + node_library_docs + suffix
prompt_template = prefix.format(tools=node_library_docs) + suffix

prompt = PromptTemplate(input_variables=["input"],
                        template=prompt_template)

llm_chain = LLMChain(llm=llm, prompt=prompt, verbose=True)

q = "Pick up the sample and place it in the PCR."
output = llm_chain({"input": q})

import random

# Define a function to check if a tip is available
def is_tip_available():
    return random.choice([True, False])

# Define a function to check for errors related to tip stock/inventory
def tip_stock_error():
    return random.choice([True, False])

# Define a function to load the next tray
def load_next_tray():
    return "Loading next tray..."

# Define a function to pick up a tip using gantry
def pick_up_tip_using_gantry():
    return "Picking up the tip using gantry..."

# Define a function to check if the tip is caught firmly and oriented
def is_caught_tip_firm_and_oriented():
    return random.choice([True, False])

# Define a function to move the slider to the load position
def move_slider_to_load_position():
    return "Moving slider to load position..."

# Define a function to move the tip slider to the required position
def move_tip_slider_to_position():
    return "Moving tip slider to position..."

# Define a function to check if the slider position is reached
def is_slider_position_reached():
    return random.choice([True, False])

# Define a function to check if the tip is available in the tray
def is_tip_available_in_tray():
    return random.choice([True, False])

# Define a function to prepare to discard the used tip
def prepare_to_discard():
    return "Preparing to discard the used tip..."

# Define a function to move to the discard position
def goto_discard_position():
    return "Moving to the discard position..."

# Define a function to eject the used tip
def eject_tip():
    return "Ejecting the used tip..."

# Define a function to check if the discard of the tip was successful
def is_discard_tip_successful():
    return random.choice([True, False])

# # Define a function to load a new tray
# def load_next_tray_using_tool():
#     return "Loading next tray using tool..."

# Define a function to handle errors related to stock/inventory
def handle_stock_error():
    # Check for errors related to stock/inventory using Tool 2: tip_stock_error
    if tip_stock_error():
        return "Error: Tip stock/inventory issue detected. Please resolve the issue and try again."
    # Load the next tray using Tool 7: load_next_tray
    else:
        return load_next_tray()

# Define a function to handle errors related to tray maintenance
def handle_tray_error():
    # Load the next tray using Tool 7: load_next_tray
    return load_next_tray()

# Define a function to generate the behavior tree response
class NodeText:
    def __init__(self, text, children=[]):
        self.text = text
        self.children = children

tools = {
    'is_tip_available': 'Tool 1',
    'tip_stock_error': 'Tool 2',
    'load_next_tray': 'Tool 7',
    'pick_up_tip_using_gantry': 'Tool 14',
    'is_caught_tip_firm_and_oriented': 'Tool 15',
    'move_slider_to_load_position': 'Tool 6',
    'move_tip_slider_to_position': 'Tool 10',
    'is_slider_position_reached': 'Tool 11',
    'is_tip_available_in_tray': 'Tool 3',
    'prepare_to_discard': 'Tool 17',
    'goto_discard_position': 'Tool 16',
    'eject_tip': 'Tool 18',}

def generate_response(tools):
    steps = [
        NodeText("Check if a tip is available using Tool 1: is_tip_available."),
        NodeText("If there is a tip available, move to step 2. If not, check for errors related to stock/inventory using Tool 2: tip_stock_error, and then load the next tray using Tool 7: load_next_tray."),
        NodeText("Pick up the tip using Tool 14: pick_up_tip_using_gantry."),
        NodeText("Check if the tip is caught firmly and oriented using Tool 15: is_caught_tip_firm_and_oriented."),
        NodeText("If the tip is caught properly, move to step 4. If not, go back to step 2 and repeat the process with a new tip."),
        NodeText("Move the slider to the load position using Tool 6: move_slider_to_load_position, if necessary."),
        NodeText("Move the tip slider to the position needed using Tool 10: move_tip_slider_to_position."),
        NodeText("Check if the slider position is reached using Tool 11: is_slider_position_reached."),
        NodeText("If the position is reached, move to step 7. If not, go back to step 5 and repeat the process."),
        NodeText("Check if the tip is available in the tray using Tool 3: is_tip_available_in_tray."),
        NodeText("If the tip is still available, move to step 8. If not, go back to step 1 and repeat the process."),
        NodeText("Prepare to discard the used tip using Tool 17: prepare_to_discard."),
        NodeText("Move to the discard position using Tool 16: goto_discard_position."),
        NodeText("Eject the used tip using Tool 18: eject_tip."),
        NodeText("Check if the discard of the tip was successful using Tool 5: is_discard_tip_successful."),
        NodeText("If the discard was successful, move to step 12. If not, go back to step 8 and repeat the process."),
        NodeText("Load a new tray if necessary using Tool 7: load_next_tray."),
        NodeText("Place the sample in the PCR.")
    ]

    return steps


def generate_behavior_tree(root):
    tree = ['Root']

    def traverse(node, level):
        tree.append('\t'*level + '- ' + node.text)
        for child in node.children:
            traverse(child, level + 1)

    traverse(root, 1)
    return '\n'.join(tree)

rootsteps = generate_response(tools)  # list of Node objects
root_node = rootsteps[0]  # select the first node as the root of the behavior tree
behavior_tree = generate_behavior_tree(root_node)
print(behavior_tree)
               

# langchain_visualizer.visualize(generate_behavior_tree(root))
# async def llmtree():
#     q = "Pick up the sample and place it in the PCR."
#     output = {}
#     output["log"] = llm_chain({"input": q})
#     return output['log']


# langchain_visualizer.visualize(llmtree)
