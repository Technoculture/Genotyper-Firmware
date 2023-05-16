"""
Objectives:
    - Given a set of nodes (action, selector and sequence nodes), generate a tree of nodes
    - Each action, selector and sequence should be modelled as a tool
    - Each tool should have a pre-condition
    - Preconditions should be checked before executing the tool
    - The tree should be traversed in a depth-first manner
    - Visualize the generated tree
    - We will be using the reactive control nodes system to generate the behavior tree
Task:
    - Given a task in english, generate a correct behavior tree
"""

import os
import json
from datetime import datetime
import asyncio
import random

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
from cred import API_KEY, MyApiKEY


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

llm = ChatOpenAI(temperature=0.0, model_name="gpt-3.5-turbo", openai_api_key=MyApiKEY)

# prefix = """Given a task description, generate a sequence of actions that can be performed to complete the task input.
# Only use the available tools to complete the task. A tool is a collection of information that you will use to generate the actionable steps to complete the task. A tool consists of a node with a 'node name' that defines one of the sub-task needed to complete the main given task,
# a 'node type' is the node's type that defines what a tool's node is, a list of 'zenoh modules' that is used to create actionable steps by taking each firmware module as a physical tool, 'minimum reply' that decides how many actionable steps you can generate per 'node_name', 'description' that is telling what the particular tool is doing.
# You have access to the following tools: """

prefix = """Act as a task planner. Given a task description, you have to complete the task by planning a sequence of actionable steps
for the modules present in each tool. The list variable module in each tool is a list of names of physical tools. 
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
# output = llm_chain({"input": q})


# Define a function to generate the behavior tree response
class NodeText:
    def __init__(self, text, children = []):
        self.text = text
        self.children = children


    def tip_avail(self, answer):
        children = []
        if answer:
            children.append('is_tip_available_in_tray')
        else:
            children.append('tip_stock_error')
        return children

    def tip_stock(self):
        children = []
        children.append('is_tip_available')
        return children
    
    def tip_tray(self, answer):
        children = []
        if answer:
            children.append('is_already_in_position')
        else:
            children.append('is_tip_available_in_tray')
        return children

    def slider_pos(self, answer):
        children = []
        if answer:
            children.append('move_tip_slider_to_position')
            children.append('is_slider_position_reached')
        else:
            children.append('move_slider_to_load_position')
            children.append('move_tip_slider_to_position')
            children.append('is_slider_position_reached')
        return children

    def pos_reach(self, answer):
        children = []
        if answer:
            children.append('pick_up_tip_using_gantry')
            children.append('is_caught_tip_firm_and_oriented')
        else:
            children.append('move_slider_to_load_position')
            children.append('move_tip_slider_to_position')
        return children

    def caught_tip(self, answer):
        children = []
        if answer:
            children.append('goto_discard_position')
            children.append('prepare_to_discard')
            children.append('eject_tip')
            children.append('is_discard_tip_successful')
        else:
            children.append('is_tip_available_in_tray')
            children.append('goto_discard_position')
            children.append('prepare_to_discard')
            children.append('eject_tip')
            children.append('is_discard_tip_successful')
        return children

    def discard_tip(self, answer):
        children = []
        if answer:
            children.append('is_discard_success')
        else:
            children.append('is_discard_tip_successful')
        return children

    def discard_success(self, answer, tip_tray_info):
        children = []
        if tip_tray_info == True:
            if answer:
                children.append('load_next_tray')
                children.append('is_load_new_tray_successful')
            else:
                children.append('is_discard_success')
        else:
            children.append('is_tip_available_in_tray')
        return children


    def load_tray_success(self, answer):
        children = []
        if answer:
            children.append('load_next_tray')
            children.append('is_load_new_tray_successful')
        else:
            children.append('load_new_tray_maintainance_error')
        return children
    
    def tray_maintenance(self):
        children = []
        children.append('load_next_tray')
        return children
    
#     if getattr(Commands(fn_name=node.text,tools=tools), node.text):
#         func_name = getattr(Commands(fn_name=node.text,tools=tools), node.text)
        
#     def generate_response_tip(self, text):

#         if 


#         return steps

# NodeText('is_tip_available', children=['tip_stock_error', 'is_tip_available_in_tray'])


# if functions[0] == 'is_tip_available':
# if getattr(Commands(fn_name=node.text,tools=tools), node.text):
#     func_name = getattr(Commands(fn_name=node.text,tools=tools), node.text)

    

functions = ['is_tip_available',

    'tip_stock_error',

    'is_tip_available_in_tray',

    'is_discard_success',

    'is_discard_tip_successful',

    'move_slider_to_load_position',

    'load_next_tray',

    'load_new_tray_maintainance_error',

    'is_already_in_position',

    'move_tip_slider_to_position',

    'is_slider_position_reached',

    'is_load_new_tray_successful',

    'tray_maintainance_error',

    'pick_up_tip_using_gantry',

    'is_caught_tip_firm_and_oriented',

    'goto_discard_position',

    'prepare_to_discard',

    'eject_tip',]

def generate_behavior_tree(root):

    tree = ['Root']

    # root = NodeText(functions[0])
    node_children = []
    tip_tray_info = []

    def traverse(node, level):
        # tip = input("Enter the input tip: ")
        if getattr(Commands(fn_name=node.text,tools=tools), node.text):
            func_name = getattr(Commands(fn_name=node.text,tools=tools), node.text)

            if node.text == 'is_tip_available_in_tray':
                tip = input("Enter the input tip: ")
                tree.append('\t'*level + '- ' + func_name(input=tip)[0])
                tip_tray_info.insert(0,func_name(input=tip)[1])
            elif node.text not in ['is_tip_available', 'tip_stock_error', 'load_new_tray_maintainance_error']:
                # tip_tray_info = tip_tray_info[0]
                tree.append('\t'*level + '- ' + func_name(tip_tray_info[0])[0])
            else:
                tree.append('\t'*level + '- ' + func_name()[0])
            # tree.append("Place the sample in the PCR.")

            if node.text == 'is_tip_available':
                # x = NodeText(text=node.text, children = NodeText(node.text).tip_avail(func_name()[1]))
                # print(x.children)
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).tip_avail(func_name()[1])))
            elif node.text == 'tip_stock_error':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).tip_stock()))
            elif node.text == 'is_tip_available_in_tray':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).tip_tray(tip_tray_info[0])))
            elif node.text == 'is_already_in_position':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).slider_pos(func_name(tip_tray_info)[1])))
            elif node.text == 'is_slider_position_reached':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).pos_reach(func_name(tip_tray_info)[1])))
            elif node.text == 'is_caught_tip_firm_and_oriented':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).caught_tip(func_name(tip_tray_info)[1])))
            elif node.text == 'is_discard_tip_successful':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).discard_tip(func_name(tip_tray_info)[1])))
            elif node.text == 'is_discard_success':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).discard_success(func_name(tip_tray_info)[1], tip_tray_info)))
            elif node.text == 'is_load_new_tray_successful':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).load_tray_success(func_name(tip_tray_info)[1])))
            elif node.text == 'load_new_tray_maintainance_error':
                node_children.insert(0, NodeText(text=node.text, children = NodeText(node.text).tray_maintenance()))

            node = node_children[0]
            for child in node.children:
                child_node = NodeText(text=child)
                # print(child)
                traverse(child_node, level + 1)

    traverse(root, 1)

    return '\n'.join(tree)

# def generate_behavior_tree(root):
#     tree = ['Root']

#     def traverse(node, level):
#         tree.append('\t'*level + '- ' + node.text)
#         for child in node.children:
#             traverse(child, level + 1)

#     traverse(root, 1)
#     return '\n'.join(tree)

# rootsteps = generate_response()  # list of Node objects
# root_node = rootsteps[0]  # select the first node as the root of the behavior tree
# behavior_tree = generate_behavior_tree(root_node)
# print(behavior_tree)

behavior_tree = generate_behavior_tree(NodeText(text=functions[0]))

output = llm_chain({"input": q}, behavior_tree)
print(behavior_tree)


# langchain_visualizer.visualize(generate_behavior_tree(root))
# async def llmtree():
#     q = "Pick up the sample and place it in the PCR."
#     output = {}
#     output["log"] = llm_chain({"input": q})
#     return output['log']


# langchain_visualizer.visualize(llmtree)
