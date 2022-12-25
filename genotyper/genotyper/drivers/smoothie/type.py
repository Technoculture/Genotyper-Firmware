from asyncio import PriorityQueue
import asyncio
import time
from rich import print
from dataclasses import dataclass, field
from enum import Enum, auto, StrEnum
from typing import (  # type: ignore
    Optional,
    Dict,
    Self,
    Generic,
    TypeVar,
    List,
    Iterator,
    NamedTuple,
    Protocol,
    Tuple,
)


class EmitsGcode(Protocol):
    def gcode(self) -> str:
        ...


class CommandType(Enum):
    """
    tests:
    >>> CommandType.GO_HOME.value == 1
    """

    GO_HOME = auto()
    GO_XZ = auto()


class CommandInfo(NamedTuple):
    """
    tests:
    >>> CommandInfo("G28", [], "Go to home position").gcode == "G28"
    >>> CommandInfo("G1", ["x", "z"], "Go to specified XZ position").fields == ["x", "z"]
    """

    gcode: str
    fields: List[str]
    description: Optional[str] = None


_command_to_info: Dict[CommandType, CommandInfo] = {
    CommandType.GO_HOME: CommandInfo("G28", [], "Go to home position"),
    CommandType.GO_XZ: CommandInfo("G1", ["x", "z"], "Go to specified XZ position"),
}


class Coordinate(NamedTuple):
    """
    tests:
    >>> Coordinate(1, 2, 3).gcode() == "1 2 3"
    >>> Coordinate(1, 2, None).gcode() == "1 2 0"
    >>> Coordinate(1, None, 3).gcode() == "1 0 3"
    >>> Coordinate(1, None, None).gcode() == "1 0 0"
    """

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __len__(self) -> int:
        return len(self._fields)

    def gcode(self) -> str:
        return " ".join([f"{v}" for _, v in self._asdict().items() if v is not None])


class Coordinate2D(Coordinate):
    """
    tests:
    >>> Coordinate2D(1, 2).gcode() == "1 0 2"
    >>> Coordinate2D(1, 2).y == 0.0
    >>> Coordinate2D(1, 2).z == 2.0
    """

    def __new__(cls, x: float, z: float) -> "Coordinate2D":
        return super().__new__(cls, x, 0.0, z)


_hardcoded_locations: Dict[str, Coordinate2D] = {
    "tiprack": Coordinate2D(20, 100),
    "trash": Coordinate2D(0, 0),
    "home": Coordinate2D(0, 0),
}


class NamedLocation(StrEnum):  # type: ignore
    """
    tests:
    >>> NamedLocation.TIPRACK == "tiprack"
    """

    TIPRACK = "tiprack"
    TRASH = "trash"
    HOME = "home"


def HardcodedLocation(name: str) -> Coordinate2D:
    """
    tests:
    >>> HardcodedLocation("home") == Coordinate2D(0, 0)
    """
    return _hardcoded_locations[name]


@dataclass
class GcodeCommand(EmitsGcode):
    """
    tests:
    >>> GcodeCommand(CommandType.GO_HOME).gcode() == "G28"
    >>> GcodeCommand(CommandType.GO_HOME, Coordinate2D(1, 2)).gcode() == "G28"
    >>> GcodeCommand(CommandType.GO_XY, Coordinate2D(1, 2)).gcode() == "G1 1 2"
    """

    type: CommandType
    arg: Optional[Coordinate] = None
    info: Optional[CommandInfo] = field(init=False)

    def __post_init__(self) -> None:
        self.info = _command_to_info[self.type]
        arg_count: int = len(_command_to_info[self.type].fields)

        _arg_count: int = 0
        match self.arg:
            case None:
                _arg_count = 0
            case Coordinate2D(x, z):
                _arg_count = 2
            case Coordinate(x, y, z):
                _arg_count = 3

        if self.arg != _arg_count:
            raise ValueError(
                f"Command {self.type}: Expected {arg_count}, got {_arg_count}"
            )

    def gcode(self) -> str:
        return f"{_command_to_info[self.type].gcode} {self.arg.gcode() if self.arg else ''}"


@dataclass
class CommandSequence(EmitsGcode):
    """
    tests:
    >>> CommandSequence([GcodeCommand(CommandType.GO_HOME)]).gcode() == "G28"
    >>> CommandSequence([GcodeCommand(CommandType.GO_HOME), GcodeCommand(CommandType.GO_HOME)]).gcode() == "G28\nG28"
    >>> CommandSequence([GcodeCommand(CommandType.GO_HOME), GcodeCommand(CommandType.GO_HOME)]).name == ""
    >>> CommandSequence([GcodeCommand(CommandType.GO_HOME), GcodeCommand(CommandType.GO_HOME)], name="Pick Tip").name == "Pick Tip"
    >>> CommandSequence() + CommandSequence([GcodeCommand(CommandType.GO_HOME)]) == CommandSequence([GcodeCommand(CommandType.GO_HOME)])
    """

    seq: list[GcodeCommand] = field(default_factory=list[GcodeCommand])
    name: str = ""

    def __len__(self) -> int:
        return len(self.seq)

    def __add__(self, gc: GcodeCommand) -> Self:
        self.seq.append(gc)
        return self

    def __iter__(self) -> Iterator[GcodeCommand]:
        return iter(self.seq)

    def gcode(self) -> str:
        return "\n".join([gc.gcode() for gc in self.seq])


go_home = CommandSequence([GcodeCommand(type=CommandType.GO_HOME)], name="Go Home")
pick_tip = CommandSequence(
    [
        GcodeCommand(
            type=CommandType.GO_XZ, arg=HardcodedLocation(NamedLocation.TIPRACK)
        ),
        GcodeCommand(type=CommandType.GO_XZ, arg=HardcodedLocation(NamedLocation.HOME)),
        GcodeCommand(type=CommandType.GO_HOME),
    ],
    name="Pick Tip",
)


def run_through_serial_till_complete(gcode: str) -> None:
    ...


@dataclass
class TaskStats:
    """
    tests:
    >>> ts = TaskStats()
    >>> ts.start()
    >>> ts.complete()
    >>> ts.duration > 0
    """

    created_at: float = field(default_factory=time.time, init=False, repr=False)
    started_at: float = field(default=0.0, init=False, repr=False)
    completed_at: float = field(default=0.0, init=False, repr=False)
    duration: float = field(default=0.0, init=False, repr=False)
    elapsed: float = field(default=0.0, init=False, repr=False)

    def start(self) -> None:
        self.started_at = time.time()

    def complete(self) -> None:
        self.completed_at = time.time()
        self.duration = self.completed_at - self.started_at
        self.elapsed = self.completed_at - self.created_at

    def __str__(self) -> str:
        msg: str = f"Duration: {self.duration:.2f}s"
        if self.elapsed - self.duration > 0.5:
            msg += f", Waited For: +{self.elapsed - self.duration:.2f}s"
        return msg


@dataclass
class GcodeTask:
    """
    tests:
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).priority == 7
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)]), priority=5).priority == 5

    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.duration == 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.elapsed == 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.started_at == 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.completed_at == 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.created_at > 0

    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.start()
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.started_at > 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.complete()

    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.completed_at > 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.duration > 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.elapsed > 0
    >>> GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.elapsed > GcodeTask(CommandSequence([GcodeCommand(CommandType.GO_HOME)])).stats.duration
    """

    seq: CommandSequence
    priority: int = field(default=7)
    stats: TaskStats = field(default_factory=TaskStats)

    def __lt__(self, other: Self) -> bool:
        return bool(self.priority < other.priority)


def execute_seq(cmd_seq: CommandSequence, *, verbose: bool = False) -> None:
    print("⏳ Execution Start") if verbose else None

    try:
        for index, gc in enumerate(cmd_seq):
            if verbose:
                msg: str = f"   ↓ Executing #{index+1}: {gc.gcode():12}"
                if gc.info is not None:
                    msg += f"({gc.info.description})"
                print(msg)

            time.sleep(0.1)
        print("✅ Execution Complete") if verbose else None
    except (KeyboardInterrupt) as e:
        print(f"Execution Failed\n{e}")
    finally:
        print("---") if verbose else None


def execute_task(task: GcodeTask, *, verbose: bool = False) -> None:
    print(f"Task: priority={task.priority}, steps={len(task.seq)}")

    task.stats.start()
    execute_seq(task.seq, verbose=verbose)
    task.stats.complete()

    print(f"↳{task.seq.name}")
    print(f" ↳{task.stats}")


async def CmdExecute(taskQueue: PriorityQueue[Tuple[int, GcodeTask]]) -> None:
    while True:
        _: int
        task: GcodeTask

        _, task = await taskQueue.get()
        execute_task(task)
        # taskQueue.task_done()


async def CmdStream(taskQueue: PriorityQueue[Tuple[int, GcodeTask]]) -> None:
    gtasklist: List[GcodeTask] = [
        GcodeTask(seq=pick_tip),
        GcodeTask(seq=pick_tip, priority=4),
        GcodeTask(seq=go_home, priority=3),
        GcodeTask(seq=pick_tip),
        GcodeTask(seq=pick_tip, priority=6),
        GcodeTask(seq=go_home, priority=2),
        GcodeTask(seq=pick_tip),
        GcodeTask(seq=go_home, priority=2),
        GcodeTask(seq=pick_tip, priority=1),
    ]

    for task in gtasklist:
        await taskQueue.put((task.priority, task))
        # await asyncio.sleep(0.5)
    # await taskQueue.join()


async def main() -> None:
    async with asyncio.TaskGroup() as tg:  # type: ignore
        tg.create_task(CmdStream(GcodeTaskQueue))
        tg.create_task(CmdExecute(GcodeTaskQueue))
    print("Done")


if __name__ == "__main__":
    GcodeTaskQueue: PriorityQueue[Tuple[int, GcodeTask]] = PriorityQueue()
    try:
        asyncio.run(main())
    except (KeyboardInterrupt, asyncio.CancelledError):
        print(f"Exited with {GcodeTaskQueue.qsize()} tasks remaining")
        print(f"Pending Tasks Queue: {GcodeTaskQueue._queue}")  # type: ignore
