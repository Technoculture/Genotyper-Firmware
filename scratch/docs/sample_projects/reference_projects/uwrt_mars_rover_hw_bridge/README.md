# UWRT Mars Rover HW Bridge
This repo contains a library that's shared by [ROS software](https://github.com/uwrobotics/uwrt_mars_rover) and 
[firmware](https://github.com/uwrobotics/MarsRover2020-firmware) repositories. The library contains payload structs and 
convenience functions to facilitate communication between our ROS system and our embedded boards.k

## Making Changes to HW Bridge
When additions or changes to this repo are made, they need to be run through CI and merged into 
[ROS software](https://github.com/uwrobotics/uwrt_mars_rover) and [firmware](https://github.com/uwrobotics/MarsRover2020-firmware).
Since this repo does not have its own CI until [this issue](https://github.com/uwrobotics/uwrt_mars_rover_hw_bridge/issues/13)
is addressed, we must manually test changes.

The process is as follows:
1. Make change to software repo on a branch and create a PR
2. Create a draft PR in the SW Repo. You should point the rosinstall file to your HW Bridge pr branch. Example:
```
- git:
    uri: https://github.com/uwrobotics/uwrt_mars_rover_hw_bridge.git
    local-name: uwrt_mars_rover/uwrt_mars_rover_hw_bridge/lib/uwrt_mars_rover_hw_bridge/
    version: fix-ros-build # Modify THIS line
```
3. Create a draft PR in the FW repo. You should point the git submodule to your HW Bridge pr branch. Example:
```
[submodule "uwrt-mars-rover-hw-bridge"]
    path = uwrt-mars-rover-hw-bridge
    url = ../uwrt_mars_rover_hw_bridge.git
    branch = fix-ros-build # Add THIS line
```
4. Ensure all pipelines pass and link the passing pipelines in your bridge PR.
5. Once approved, ask one of the leads to merge it in (Only Github admins can force this merge in).
6. Change the SW repo to use the **latest commit hash** of HW Bridge repo.
7. Convert your SW PR to non-draft. 
8. Change the FW repo to use the **latest commit hash** of HW Bridge repo.
9. Convert your FW PR to non-draft. 
10. Get approvals and merge in SW and FW PRs!
