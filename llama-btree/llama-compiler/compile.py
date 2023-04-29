# Step1: Deserialize the nodes.lib.yaml file into python objects using Pydantic
'''
Module to deserialize the library trees input file nodes.lib.yaml
'''

import yaml
from typing import List
from pydantic import BaseModel
from typing import Optional


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
with open('nodes.lib.yaml', 'r') as f:
    nodes_dict = yaml.safe_load(f)

# deserializing
deserialized_nodes = [(nodes,Node.parse_obj(node_dict)) for nodes,node_dict in nodes_dict['content'].items()]

# output: deserialized contents as python objects
for node_name,node in deserialized_nodes:

    # node name
    print(node_name)

    # type
    print(node.type)

    # zenoh
    if node.zenoh is not None:

        # modules
        print(node.zenoh.modules)
        # min_reply
        print(node.zenoh.min_reply)
    else:
        print(None)
        print(None)

    # description
    print(node.description)
