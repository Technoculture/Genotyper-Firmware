import os
from git.repo import Repo
from gitpoll import configure_environment, configure_git_repo, GitPoll

from unittest.mock import patch, MagicMock

def test_configure_environment():
    with patch('os.makedirs') as mock_makedirs:
        tcr_base_dir = configure_environment()
        assert os.path.exists(tcr_base_dir)
        assert isinstance(tcr_base_dir, str)
        mock_makedirs.assert_called_once_with(tcr_base_dir, exist_ok=True)

def test_configure_git_repo():
    tcr_base_dir = configure_environment()
    repo, branch_name = configure_git_repo(tcr_base_dir)
    assert isinstance(repo, Repo) == True
    assert repo.active_branch.name == branch_name

def test_git_poll_init():
    git_poll = GitPoll()
    assert isinstance(git_poll.repo, Repo) == True
    assert isinstance(git_poll.branch_name, str) == True
    assert git_poll.counter == 0
    assert isinstance(git_poll.log, dict) == True

def test_git_poll_poll():
    git_poll = GitPoll()
    initial_count = git_poll.counter
    initial_log = git_poll.log.copy()
    git_poll.poll()
    assert git_poll.counter == initial_count + 1
    assert len(git_poll.log) == len(initial_log) + 1
    assert list(git_poll.log.values())[-1] in [True, False]

def test_git_poll_repr():
    git_poll = GitPoll()
    repr_string = git_poll.__repr__()
    assert isinstance(repr_string, str) == True

if __name__ == "__main__":
    print("Running test_sync.py")
    test_configure_environment()
    test_configure_git_repo()
    test_git_poll_init()
    test_git_poll_poll()
    test_git_poll_repr()
    print("All tests passed!")

