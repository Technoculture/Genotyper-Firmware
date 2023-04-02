import os
import sys
import yaml

from langchain.llms import OpenAI
from langchain import PromptTemplate
from langchain.agents import tool, Tool
from langchain.agents import initialize_agent

from typing import Callable, List, Optional
from enum import Enum

template = """
Given the a question, answer it. If one of tools appear to be useful, always use them.

Question: {q}
"""


class NodeType(str, Enum):
    ACTION = "action"
    SEQUENCE = "sequence"
    SELECTOR = "selector"


class Node:
    def __init__(self,
                 name: str,
                 description: str,
                 func: Callable = lambda: None,
                 *, is_selector=False):
        self.children = []
        self.name = name
        self.description = description
        self.func = func
        self.is_selector = is_selector

    def __repr__(self) -> str:
        return f"Node('{self.name}', '{self.description}', type={self.node_type}, children={len(self.children)})"

    @property
    def node_type(self) -> NodeType:
        if len(self.children) == 0:
            return NodeType.ACTION
        elif self.is_selector:
            return NodeType.SELECTOR
        else:
            return NodeType.SEQUENCE

    def append(self, node):
        self.children.append(node)

    @property
    def is_leaf(self):
        return len(self.children) == 0

    def execute(self, whiteboard):
        if self.node_type == NodeType.ACTION:
            print("executing {}".format(self))
        elif self.node_type == NodeType.SELECTOR:
            print("selecting {}".format(self))
        elif self.node_type == NodeType.SEQUENCE:
            print("sequencing {}".format(self))

    def to_tools(self) -> List[Tool]:
        tools = [Tool(self.name, self.func, self.description)]
        if not self.is_leaf:
            for child in self.children:
                tools += child.to_tools()
        return tools


def main():
    book_a_ticket = Node(
        "Book a ticket", "Given a location, a start date, and a budget, this tool automatically books a ticket")
    plan_a_sequence_of_flights = Node(
        "Plan a sequence of flights", "Given a vague idea for travel, produces a sequence of flights proposed")
    plan_a_trip = Node(
        "Plan a trip", "Given a vague idea for travel, tries to help you plan a trip")

    plan_a_trip.append(book_a_ticket)
    plan_a_trip.append(plan_a_sequence_of_flights)

    tools = plan_a_trip.to_tools()

    [print(t.name) for t in tools]

    llm = OpenAI(temperature=0, model_name="gpt-3.5-turbo")

    prompt = PromptTemplate(
        input_variables=["q"],
        template=template
    )

    agent = initialize_agent(
        tools, llm, agent="zero-shot-react-description", verbose=True)

    q = input("Question: ")
    output = agent.run(q)

    print(output)


if __name__ == "__main__":
    main()
