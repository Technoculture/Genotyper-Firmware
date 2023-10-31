from typing import Protocol


class Command(Protocol):
    """Protocol for defining a command"""

    def execute(self) -> None:
        """method to call in order to execute the Command."""
        ...


def init() -> None:
    """initialize function"""
    print("hello")
