Dependencies
---
- Zenoh
- Pipenv
- Python 3.10
- tmux

Run the Program
---
> tmuxp load robot.yaml

Some tips:
- Use Ctrl-B & to kill all the processes in the simulation along with the tmux window
- If using tmux already for development (like I am), You will see an option on where to open this new tmux session - the append option works well
- Use Ctrl-B <window-number> to navigate to windows opened by the sim


Components
---
- Incomming Swabs (Source)
- Liquid Handling System
- Sample Prep
- Detection System
- Results Dispatcher (Sink)
