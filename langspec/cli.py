import os
import sys
import yaml

from langchain.llms import OpenAI
from langchain import PromptTemplate

template = """
Given the spec, generate the entire program. BE VERY CAREFUL WITH THIS ONE.

{spec}
"""


def main():
    llm = OpenAI(temperature=0, model_name="gpt-3.5-turbo")

    with open(os.path.join(sys.path[0], "spec.yaml"), "r") as f:
        spec = yaml.load(f, Loader=yaml.FullLoader)
        print(spec)

    prompt = PromptTemplate(
        input_variables=["spec"],
        template=template
    )

    output = llm(prompt.format(spec=spec))

    print(output)


if __name__ == "__main__":
    main()
