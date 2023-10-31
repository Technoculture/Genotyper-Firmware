from pydantic import BaseModel, Field
from typing import List, Optional
from enum import Enum
import yaml

class NodeType(str, Enum):
    SEQUENCE = "sequence"
    ACTION = "action"
    CONDITION = "condition"
    ERROR = "error"


class ErrorType(Enum):
    FATAL = 1
    INVENTORY_ERROR = 2
    MAINTAINABLE = 3
    INSPECTABLE = 4
    INVENTORY_WARN = 5
    RECOVERABLE = 6


class Node(BaseModel):
    name: str = Field(..., description="Name of the node")
    type: NodeType = Field(..., description="Type of the node")
    children: Optional[List["Node"]] = Field(default=[], description="List of child nodes")


class BTree(BaseModel):
    name: str = Field(..., description="Name of the BTree")
    nodes: List[Node] = Field(..., description="List of nodes in the BTree")


class Step(BaseModel):
    name: str = Field(..., description="Name of the step")
    btree: BTree = Field(..., description="BTree of the step")


class Workflow(BaseModel):
    name: str = Field(..., description="Name of the workflow")
    steps: List[Step] = Field(..., description="List of steps in the workflow")


def run_workflow(workflow: Workflow):
    print(workflow.json(indent=2))


def read_workflow(file_path: str) -> Workflow:
    with open(file_path, 'r') as stream:
        config = yaml.safe_load(stream)
    return Workflow(**config)

if __name__ == "__main__":
    print("hello")

    w = read_workflow("sample.yaml")
    print(w)
    run_workflow(w)

