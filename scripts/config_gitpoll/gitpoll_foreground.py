#!/usr/bin/env python

import time
import logging
from gitpoll import gitpoll

PID = "/tmp/gitpoll_foreground.pid"
LOGFILE = "/tmp/gitpoll_foreground.log"

def init_logger():
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.DEBUG)
    logger.propagate = False
    fh = logging.FileHandler(LOGFILE, "w")
    fh.setLevel(logging.DEBUG)
    logger.addHandler(fh)
    formatter = logging.Formatter("%(asctime)s [%(levelname)s]: %(message)s")
    fh.setFormatter(formatter)
    keep_fds = [fh.stream.fileno()]
    return logger, keep_fds

def main():
    logger.debug("Starting foreground process")
    gs = gitpoll.GitPoll()
    while(True):
        gs.poll()
        logger.debug(gs)
        time.sleep(6)

if __name__ == "__main__":
    logger, keep_fds = init_logger()

    logger.debug(f"Logging to {LOGFILE}")
    main()

