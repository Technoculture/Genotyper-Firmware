# Some problems are specially important to IOT devices
- OTA Updates
- Remote Debugability (SSH, Logging, Streaming Camera Views, Remote Control (without VNC), VNC)
- Configuration Management Problem
- Status Management Problem
- Visibility Problem
- Network Problem

> 😢 **TLDR**: This thing is a nightmare.

# Daemons
Yes, we will probably need a bunch of daemon processes - for monitoring network status, debugability, Configuration, etc.

## Why this should be a multi-process application?
- One app with so many responsibility would be quite complex to write and maintain
- Failure in a single process architecture would be catastrophic
- Status and Configuration Management might involve polling of servers, heartbeats, and frequently carried out tasks

## Probable Solutions:
- Some sort of comprehensive service provider? TorizonCore? AWS IOT Core?

# Then there's the Cloud Side
- MQTT? HTTP? I mean whats the cloud plan?
- Is there an admin panel for monitoring all deployed devices? IOT Core? Graffana?
- Are there logics on the logs for triggering actions and alerts? AWS Cloudwatch takes care of this well.
- How do we manage and stream configurations? 
- How do we maintain status of our IOT nodes? AWS Something?
- Who runs and maintains the OTA server? Us or a server provider like Meander?
