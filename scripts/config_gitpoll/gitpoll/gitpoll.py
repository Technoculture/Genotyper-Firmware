#!/usr/bin/env python

import os, sys
import time
from datetime import timedelta
from dotenv import load_dotenv
from git.repo import Repo
from git.exc import GitCommandError

def configure_environment() -> str:
    """Load environment variables and create the base directory.
    
    Returns:
        str: The base directory path.
    """
    load_dotenv()
    
    if sys.platform.startswith("win"):
        tcr_base_dir = os.path.expanduser('~/tmp/tcr')
    else:
        tcr_base_dir = '/tmp/tcr'
    
    os.makedirs(tcr_base_dir, exist_ok=True)
    return tcr_base_dir

def configure_git_repo(tcr_base_dir: str) -> tuple:
    """Configure and set up the Git repository.
    
    Args:
        tcr_base_dir (str): The base directory path.

    Returns:
        tuple: A tuple containing the Repo object and the branch name.
    """
    access_token = os.environ['GITHUB_ACCESS_TOKEN']
    repo_url = f'https://{access_token}@github.com/TechnocultureResearch/genodatalib.git'
    repo_local_path = os.path.join(tcr_base_dir, 'genodatalib')
    branch_name = 'dev'

    if not os.path.exists(repo_local_path):
        repo = Repo.clone_from(repo_url, repo_local_path, branch=branch_name)
        repo.remotes.origin.fetch()
    else:
        repo = Repo(repo_local_path)

    if repo.active_branch.name != branch_name:
        repo.git.checkout(branch_name)

    return repo, branch_name

class GitPoll:
    def __init__(self) -> None:
        """Initialize the GitSync object."""
        tcr_base_dir = configure_environment()
        self.repo, self.branch_name = configure_git_repo(tcr_base_dir)
        self.counter = 0 # Counter for number of syncs
        self.log = {} # Dictionary to store sync success/failure with timestamp

    def poll(self) -> None:
        """Sync the Git repository with the local folder."""
        try:
            self.repo.remotes.origin.pull(self.branch_name)
            self.counter += 1
            self.log[time.time()] = True
        except GitCommandError as e:
            print(e)
            self.log[time.time()] = False
            exit(1)

    def __repr__(self) -> str:
        """Return the string representation of the GitSync object.
        If there are any failed syncs, it will show the last failed sync.
        Also shows how long the script has been running.
        """
        start_time = min(self.log.keys()) if self.log else time.time()
        duration = time.time() - start_time
        formatted_duration = str(timedelta(seconds=int(duration)))

        if False in self.log.values():
            last_failed_sync = max([k for k, v in self.log.items() if v == False])
            return f'Last sync failed at {time.ctime(last_failed_sync)}. Synced {self.counter} times. Running for {formatted_duration}.'
        else:
            return f'Synced {self.counter} times. Running for {formatted_duration}.'

if __name__ == "__main__":
    """Main function to run the script."""

    git_sync = GitPoll()
    print('Syncing Git repository with local folder...')
    git_sync.poll()
    print('Syncing complete.')
    print(git_sync)
