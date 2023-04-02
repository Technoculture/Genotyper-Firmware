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
from langchain.llms import OpenAI
from langchain import LLMChain
from langchain.agents import ZeroShotAgent, AgentExecutor
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

llm = OpenAI(temperature=0.0, model_name="gpt-3.5-turbo")

prefix = """Given a task description, generate a sequence of actions that can be performed to complete the task.
You have access to the following tools:"""
suffix = """\n\nQ: {q}\nA:"""

prompt = ZeroShotAgent.create_prompt(
    tools=tools,
    prefix=prefix,
    suffix=suffix,
    input_variables=["q"],
)

print(prompt.template)

llm_chain = LLMChain(llm=llm, prompt=prompt)
agent = ZeroShotAgent(llm_chain=llm_chain, allowed_tools=[
                      tool.name for tool in tools])

agent_executor = AgentExecutor.from_agent_and_tools(agent=agent, tools=tools)


async def llmtree_agent():
    q = "Task: Pick up the sample and place it in the PCR."
    return agent_executor.run(q)


langchain_visualizer.visualize(llmtree_agent)

"""
Tests and metrics:
 - Effect of temperature on the agent's performance
 - Accuracy compared to human performance
 - Hallucination rate
 - Precondition disregard count
"""
