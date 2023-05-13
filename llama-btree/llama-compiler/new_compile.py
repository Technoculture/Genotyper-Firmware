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
tools = {}

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
    tools.update({f'{node_name}':[node.type, modules, min_reply, node.description]})
# print(node_library_docs)
# print(tools)

# Define a function to check if a tip is available
class Commands:

    def __init__(self, fn_name, tools):

        self.name = fn_name
        self.key_list = tools.get(self.name.strip())


    def is_tip_available(self):

        # name = is_tip_available.__name__
        # key_list = tools.get(name.strip())
        tipRM = ["10ml", "100ml", "1000ml"]
        if self.key_list[2] == "any":
            if "TipRM" in self.key_list[1] and tipRM:
                answer = f"This node is {self.key_list[0]}.\
                    The pipette must be attached to the system before picking up the tip.\
                    {self.key_list[3]}.\
                        The zenoh module TipRM is available.\
                            Proceed to choose and pick-up a tip."
            else:
                answer = "The zenoh TipRM isn't available. Please arrange tip stock in TipRM. Waiting for TipRM to be filled..."
        else:
            answer = "The pipette is attached to the system but the zenoh TipRM isn't available."

        return answer

    # print(is_tip_available(tools))


    def tip_stock_error(self):

        if self.key_list[2] == "any":
            answer = f"This node is {self.key_list[0]}.\
            {self.key_list[3]}.\
            Got a Tip stock error."
        else:
            answer = "Tip stock error"

        return answer


    def is_tip_available_in_tray(self, input:str):

        tipRM_tray = ["10ml", "100ml", "1000ml"]
        if "TipRM" in self.key_list[1] and input in tipRM_tray:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    {self.key_list[3]}.\
                        Tip {input} from zenoh module TipRM is available in the tray."
            else:
                answer = f"{input} Tip is available in the tray."
        else:
            answer = f"TipRM {input} tip is not available in the tray.\
                Please place the {input} tip in the tray."

        return answer

    def move_slider_to_load_position(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Moved tip slider to load position."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Moved tip slider to load position."
        else:
            answer = f"{self.key_list[3]}. Moved tip slider to load position."

        return answer

    def is_already_in_position(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Tip slider is already in position."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Tip slider is already in position."
        else:
            answer = f"{self.key_list[3]}."

        return answer

    def move_tip_slider_to_position(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Moved tip slider to load position."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Moved tip slider to position."
        else:
            answer = f"{self.key_list[3]}."

        return answer


    def is_slider_position_reached(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Tip slider position was reached."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Tip slider position was reached."
        else:
            answer = f"{self.key_list[3]}."

        return answer


    def pick_up_tip_using_gantry(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Picked up tip using gantry."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Picked up tip using gantry."
        else:
            answer = f"{self.key_list[3]}."

        return answer
    
    
    def is_caught_tip_firm_and_oriented(self, tip_tray_info):

        if tip_tray_info and "Pipette" in self.key_list[1]:
            if self.key_list[2] == "any":
                answer = f"This node is {self.key_list[0]}.\
                    Pipette is present, Pipette is the actively mounted tool.\
                        {self.key_list[3]}.\
                            zenoh tip is present in the tray of {self.key_list[1]}.\
                                Caught tip is firm and oriented."
            else:
                answer = "Pipette is present, Pipette is the actively mounted tool. Caught tip is firm and oriented."
        else:
            answer = f"{self.key_list[3]}."

        return answer


# # for fn_name in tools.keys():
# if  "is_tip_available" in tools.keys():
#     fn_name = "is_tip_available"
#     if getattr(Commands(fn_name=fn_name,tools=tools), fn_name):
#         func_name = getattr(Commands(fn_name=fn_name,tools=tools), fn_name)
#         print(func_name())
