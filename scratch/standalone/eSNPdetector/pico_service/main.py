# TODO: Add pipenv for dependency management
# TODO: Set up a simple Python Qt/QML application
# TODO: Set up methodscript communication
# TODO: Setup some form State Machine Logic for Navigation State
# TODO: Setup setup.py file for installation

import logging
import sys
from palmsens.instrument import Instrument

logging.basicConfig(
    stream=sys.stdout,
    # encoding='utf-8', # not sure whats the issue with this
    level=logging.DEBUG,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logging.getLogger('eSNP')

if __name__ == "__main__":
    logging.debug("init")
