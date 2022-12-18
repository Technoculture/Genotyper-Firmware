# AsyncIO

- 🔥 [Developing with AsyncIO](https://docs.python.org/3/library/asyncio-dev.html)

> An event loop runs in a thread (typically the main thread) and executes all callbacks and Tasks in its thread. While a Task is running in the event loop, no other Tasks can run in the same thread. When a Task executes an await expression, the running Task gets suspended, and the event loop executes the next Task.

# AsyncIO And Threads
> The `loop.run_in_executor()` method can be used with a concurrent.futures.`ThreadPoolExecutor` to execute blocking code in a different OS thread without blocking the OS thread that the event loop runs in.

Blocking (CPU-bound) code should not be called directly. For example, if a function performs a CPU-intensive calculation for 1 second, all concurrent asyncio Tasks and IO operations would be delayed by 1 second.

An executor can be used to run a task in a different thread or even in a different process to avoid blocking the OS thread with the event loop. See the `loop.run_in_executor()` method for more details.

- [How to speed up I/O-intensive tasks with multithreading and asyncio](https://kimmosaaskilahti.fi/blog/2021-01-03-asyncio-workers/)
- [A better way for asynchronous programming: asyncio over multi-threading](https://towardsdatascience.com/a-better-way-for-asynchronous-programming-asyncio-over-multi-threading-3457d82b3295)
- [AsyncIO with Thread Executors](https://pymotw.com/3/asyncio/executors.html)

## ThreadExecutors
> There is currently no way to schedule coroutines or callbacks directly from a different process (such as one started with multiprocessing). The Event Loop Methods section lists APIs that can read from pipes and watch file descriptors without blocking the event loop. In addition, asyncio’s Subprocess APIs provide a way to start a process and communicate with it from the event loop. Lastly, the aforementioned loop.run_in_executor() method can also be used with a concurrent.futures.ProcessPoolExecutor to execute code in a different process.

- [ThreadPoolExecutor in Python: The complete guide](https://superfastpython.com/threadpoolexecutor-in-python/)
