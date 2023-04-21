#!/usr/bin/env python

import time
import logging
from daemonize import Daemonize
from gitpoll import gitpoll

PID = "/tmp/tcr/gitpoll_daemon.pid"
LOGFILE = "/tmp/tcr/gitpoll_daemon.log"

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
    logger.debug("Starting daemon")
    gs = gitpoll.GitPoll()
    while(True):
        gs.poll()
        logger.debug(gs)
        time.sleep(6)

if __name__ == "__main__":
    logger, keep_fds = init_logger()

    logger.debug(f"Logging to {LOGFILE}")

    daemon = Daemonize(app="gitpoll_daemon", pid=PID, action=main, keep_fds=keep_fds, logger=logger)
    print(f"Run `cat {daemon.pid}` to get the pid of this daemon")
    daemon.start()

