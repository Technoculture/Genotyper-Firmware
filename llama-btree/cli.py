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
import langchain_visualizer
from langchain.llms import OpenAI, HuggingFacePipeline
from langchain import LLMChain
from langchain.agents import ZeroShotAgent, AgentExecutor
from transformers import AutoTokenizer, AutoModelForCausalLM, pipeline
from nodelib import *

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

tool_names = [tool.name for tool in tools]


def get_llm(*, vendor: str = "openai", model_name: str = ""):
    if vendor == "openai":
        model_name = "gpt-3.5-turbo" if model_name == "" else model_name
        llm = OpenAI(temperature=0, model_name=model_name, verbose=True)
        return llm
    elif vendor == "hf":
        model_name = "cerebras/Cerebras-GPT-111M" if model_name == "" else model_name
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModelForCausalLM.from_pretrained(model_name)
        pipe = pipeline(
            "text-generation", model=model, tokenizer=tokenizer,
            max_new_tokens=100, early_stopping=True, no_repeat_ngram_size=2
        )
        llm = HuggingFacePipeline(pipeline=pipe, verbose=True)
        return llm
    else:
        raise ValueError("Invalid vendor")


llm = get_llm()


prefix = """Given a task description, generate a sequence of actions that can be performed to complete the task.
Only use the available tools to complete the task.
You have access to the following tools:"""
suffix = """
DO NOT REQUEST ACTIONS THAT ARE NOT IN THE TOOL LIST.
BE VERY CAREFUL WITH THIS ONE.

Task: {input}
{agent_scratchpad}"""

prompt = ZeroShotAgent.create_prompt(
    tools=tools,
    prefix=prefix,
    suffix=suffix,
    input_variables=["input", "agent_scratchpad"],
)


llm_chain = LLMChain(llm=llm, prompt=prompt, verbose=True)
agent = ZeroShotAgent(llm_chain=llm_chain, allowed_tools=tool_names)

agent_executor = AgentExecutor.from_agent_and_tools(
    agent=agent,
    tools=tools,
    return_intermediate_steps=True,
)


async def llmtree_agent():
    q = "Pick up the sample and place it in the PCR."
    output = {}
    output["datetime"] = datetime.now().strftime("%d/%m/%Y %H:%M:%S")
    output["log"] = agent_executor({"input": q, "agent_scratchpad": ""})

    file_path = "output.json"
    if os.path.exists(file_path) and os.path.getsize(file_path) > 0:
        with open(file_path, "r") as jsonfile:
            data = json.load(jsonfile)
    else:
        data = []
    data.append(output)
    with open(file_path, "w") as jsonfile:
        json.dump(data, jsonfile, indent=2)

    return output

langchain_visualizer.visualize(llmtree_agent)
