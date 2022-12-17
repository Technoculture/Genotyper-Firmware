# Simple Goals
- Business Logic should be a seperate entity and completely under unit tests
- Dependency Management should set up dev environment, use docker if its not enough
- Use a Configuration management libray over Parameters spread over codebase
- Do not depend on hardware to be connected for Firmware developemnt (linux)

# Goals: 12 Factor Application?

[Source](https://12factor.net)

1. **Codebase : One codebase tracked in revision control, many deploys**
    > Monorepo?
2. **Dependencies : Explicitly declare and isolate dependencies**
    > Use a dependency manager? Do not depend on local environment.
3. **Config : Store config in the environment**
    > IOT further complicates Configuration Management
4. Backing services : Treat backing services as attached resources
    > What are the repercussions of this?
5. Build, release, run : Strictly separate build and run stages 
    > what does this even mean?
6. Processes : Execute the app as one or more stateless processes
    > Then where do the state machines go?
7. Port binding : Export services via port binding
    > What does this entail for embedded systems? (Linux)
8. Concurrency : Scale out via the process model
    > So ZeroMQ like messaging system over direct connection with serial ports?
9. Disposability : Maximize robustness with fast startup and graceful shutdown
    > This requires stateless processes, with state in an external cache-db like process?
10. Dev/prod parity : Keep development, staging, and production as similar as possible
11. **Logs : Treat logs as event streams**
12. Admin processes : Run admin/management tasks as one-off processes

---

## The big confusions with the 12 factor model
- How does 12 factor model apply to IOT?
- How is 12 factor approach apply to connected embedded devices / robotics?
