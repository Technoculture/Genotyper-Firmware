This is a continuation of [Is `socat` awesome](./IsSocatAwesome.md).

# is ZMQ awesome for hardware firmware seperation?

- [Serial2ZMQ](https://github.com/wookiesh/serial2zmq)
- Exposing a serial port to a ZMQ bus would allow it to participate in more complex communication topologies beyond the one-to-one communication supported by Serial
    > 🤔 **Wonder**:
    > - Serial exposed to ZMQ could allow for separate logging, caching and queing of incomming messages
    > - Serial messages need not be ephimeral
    > - Serial messages may be used by multiple processes simultaneously. This may be useful for event triggered functionalities, if any.
