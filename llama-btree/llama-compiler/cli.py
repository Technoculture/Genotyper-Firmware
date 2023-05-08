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


# from langchain.llms import OpenAI
from langchain.chat_models import ChatOpenAI
from langchain import PromptTemplate
from langchain.agents import initialize_agent
from langchain.agents import AgentType
import langchain_visualizer
from graphviz import Digraph
import asyncio

from compile import *
from cred import API_KEY

template = """
Question: {q}
"""

tools = [
    is_tip_available,
    tip_stock_error,
    is_tip_available_in_tray,
    is_discard_success,
    tray_maintainance_error,
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

llm = ChatOpenAI(temperature=0.0, model_name="gpt-3.5-turbo", openai_api_key=API_KEY)


prompt = PromptTemplate(
    input_variables=["q"],
    template=template
)


agent = initialize_agent(
    tools, llm, agent=AgentType.ZERO_SHOT_REACT_DESCRIPTION, verbose=True)


async def llmtree_agent():
    q = "Task: Pick up the sample and place it in the PCR."
    return agent.run(
        q
    )


langchain_visualizer.visualize(llmtree_agent)
