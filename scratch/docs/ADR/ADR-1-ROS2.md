# ROS2 as the primary middleware

* Status: proposed
* Deciders: Satyam
* Date: 2022-01-12

Technical Story: Genotyper is complex and hence requires various modules, and processes working in parallel. They need to communicate with each other in a systematic manner.

## Context and Problem Statement

[Describe the context and problem statement, e.g., in free form using two to three sentences. You may want to articulate the problem in form of a question.]

## Decision Drivers 

* [driver 1, e.g., a force, facing concern, …]
* [driver 2, e.g., a force, facing concern, …]

## Considered Options

* [option 1]
* [option 2]
* [option 3]

## Decision Outcome

Chosen option: "[option 1]", because [justification. e.g., only option, which meets k.o. criterion decision driver | which resolves force force | … | comes out best (see below)].

### Positive Consequences 

* [e.g., improvement of quality attribute satisfaction, follow-up decisions required, …]
* …

### Negative Consequences 

* [e.g., compromising quality attribute, follow-up decisions required, …]
* …

## Pros and Cons of the Options

### CAN Bus, Zeromq, UART, protobuf
* Good, because is used widely within automotive applications
* Bad, because it is difficult to use in server-client mode (Reference?)
* Bad, beacuse its key use case is not event driven systems (Reference?)
* Bad, because may cause message loss problems as CAN networks value latest messages (Reference?)

### UART, protobuf, Zeromq
* Good, because of large body of literature avaible on using UART and Protobuf
* Good, because easy and quick to set up
* bad, because requires building up a layer of software to support Reliability, Recoverability, Heartbeat, etc.
* bad, because zeromq does not support persistance of pubsub data ([Stackoverflow QA](https://stackoverflow.com/questions/4059706/zeromq-persistence-patterns))

### ROS, DDS, Micro-ROS
* Risky, because sparse availability of samples for micro-ros ([Example using ZephyrRTOS](https://micro.ros.org/docs/tutorials/core/first_application_rtos/zephyr/))

## Links 

* Architecturally Significant Concerns discussed in [[ASC-1]](./ASC-1-Characteristics.md).
