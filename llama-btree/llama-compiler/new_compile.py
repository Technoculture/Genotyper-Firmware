'''
Module to deserialize the library trees input file nodes.lib.yaml. 
'''

import yaml
from pydantic import BaseModel#, Field
from typing import Callable, List, Optional#, Union
import re


class ZenohConfig(BaseModel):
    modules: Optional[List[str]]
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
with open('C:\\Users\\mishr\\OneDrive\\Documents\\tmp\\tcr\\genodatalib\\library\\nodes.lib.yaml', "r") as f:
# with open('library\\nodes.lib.yaml', "r") as f:
    nodes_dict = yaml.safe_load(f)

# deserializing
deserialized_nodes = [(nodes,Node.parse_obj(node_dict)) for nodes,node_dict in nodes_dict['content'].items()]
# deserialized_nodes = [Node.parse_obj(node_dict) for node_dict in nodes_dict['content'].values()]
# print(deserialized_nodes)

# class NodeInput(BaseModel):
#     name: str
#     mode: str
#     modules: Optional[list[str]]
#     min_reply: Optional[str]
#     description: str

#     @classmethod
#     def docstring(cls, node_name, node):

#         fields = {}
#         # node name
#         fields["name"] = node_name
#         # type
#         fields["mode"] = node.type

#         # zenoh
#         if node.zenoh is not None:
#             # modules
#             fields["modules"] = node.zenoh.modules
#             # min_reply
#             fields["min_reply"] = node.zenoh.min_reply
#         else:
#             fields["modules"] = None
#             fields["min_reply"] = None

#         # description
#         fields["description"] = node.description
        
#         return cls(**fields)


# def node_contents(node: NodeInput) ->str:
#     return NodeInput.from_docstring()

# print(NodeInput.docstring(deserialized_nodes))

# output: deserialized contents as python objects
# node_library = [NodeInput.docstring(node_name, node) for node_name,node in deserialized_nodes]

# print(node_library)
# for node in node_library:
#     x = node.name
#     print(x)

node_library_docs = ""
# serial = 0
serial = 1

for node_name,node in deserialized_nodes:
    if node.zenoh is not None:
        # modules
        modules = node.zenoh.modules
        # min_reply
        min_reply = node.zenoh.min_reply
    else:
        modules = None
        min_reply = None
    
    # print(node_name, node.type, modules, min_reply, node.description)

    # serial += 1
    # # print(serial)
    # node_library_docs += node_library_docs + f'''Tool {serial}:{node_name},
    # node type:{node.type},
    # zenoh modules:{modules},
    # minimum reply:{min_reply},
    # description:{node.description} \n'''

    node_library_docs += f"Tool {serial}: {node_name}\n" \
             f"node type: {node.type}\n" \
             f"zenoh modules: {modules}\n" \
             f"minimum reply: {min_reply}\n" \
             f"description: {node.description}\n\n"
    
    serial += 1

# print(node_library_docs)
