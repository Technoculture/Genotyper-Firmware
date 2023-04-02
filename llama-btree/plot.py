from graphviz import Digraph
import json

steps = []

with open('output.json', 'r') as f:
    data = json.load(f)
    entries = []
    for entry in data:
        entries.append(entry["log"]["intermediate_steps"])
    for entry in entries:
        step_names = []
        for key in entry:
            step_names.append(key[0][0])
        steps.append(step_names)
    # print(len(steps))
    print(steps)


def draw(steps):
    dot = Digraph()
    for index, step in enumerate(steps):
        for i, node in enumerate(step):
            shape = 'box' if not node.startswith('is_') else 'ellipse'
            dot.node(str(i) + f" {index}", node + f" {index}", shape=shape)
        for i in range(len(step) - 1):
            dot.edge(str(i) + f" {index}", str(i+1) + f" {index}")

    # Save the behavior tree as an image file
    dot.render('assets/behavior_tree', format='png')


draw(steps=steps)
