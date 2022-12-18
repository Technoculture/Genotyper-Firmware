# Socat ([man file](https://linux.die.net/man/1/socat))

- **Scenario**: An app (Qt app, or Some command line util) talks to an embedded system over UART. The communication maybe a plain ASCII over Serial or a more efficient Serialization Protocol (Protobuf) over Serial.

- **Problem**: How do we develop an app like this without harware present? How could we run integration tests wihtout the need of the hardware (**Note**: This would be great for CI/CD).

- **The big question**: Can socat save us?

## TLDR
> ✅
> Just Use This: 
> [Virtual serial line (RS232) on linux with socat and pyserial](https://gist.github.com/sutyum/03e78373dadbcbd0c961c43561606aab)

## These guys think Yay
> Tiny Note: These are mostly tutorials on **How to** do this and don't really make for or against claims.

- [Virtual Serial Ports Using Socat](https://jamesthom.as/2021/01/virtual-serial-ports-using-socat/)
- [TTY-Redirector/Pseudo-TTYs under Linux](https://www.wut.de/e-58www-29-apus-000.php)
- [HowTo: Virtual Serial Ports on Linux using socat, and more](https://justcheckingonall.wordpress.com/2009/06/09/howto-vsp-socat/)
- [create virtual serial port with socat in linux](https://www.youtube.com/watch?v=iFmD-CeB96A)
- [Virtual Serial Port in C++/Linux](https://github.com/cymait/virtual-serial-port-example)

### Demo Time!
![Socat Demo Video](https://github.com/TechnocultureResearch/Genotyper-Firmware/raw/dev/docs/QuestionLog/socat_demo.mov)

### Random aside: A socat snippet from ablog above
> this bit is okay not super useful

Here is the magic command you need:

`socat -d -d -v pty,rawer,link=<PORT_NAME> EXEC:<COMMAND>,pty,rawer`
It took me a while to find the correct combination of options to make this work. Here’s what I’ve used…

The first pty options creates the virtual serial port. It turns out terminal echo using rawer. The port is created using the file id given by the link option. The EXEC option will run the executable (given by the COMMAND value) and connect to the virtual serial port using stdin & stdout.

The `-d -d -v` options are used to print virtual port stream values to the console as well as the target.

## These guys have doubts
- [Emulating a serial port on UNIX: Didn't understand well the reason for not using socat](https://www.itdev.co.uk/blog/emulating-serial-port-unix)

### Socat is cool though
- [Serialport-Server: Expose a local serial port on a network](https://hub.docker.com/r/akshmakov/serialport-server)

---

# Counter Voice: Forget About Socat - Go **Zmq**
- [Serial2ZMQ](https://github.com/wookiesh/serial2zmq)
- Exposing a serial port to a ZMQ bus would allow it to participate in more complex communication topologies beyond the one-to -one communication supported by Serial
    > 🤔 **Wonder**:
    > - Serial exposed to ZMQ could allow for separate logging, caching and queing of incomming messages
    > - Serial messages need not be ephimeral
    > - Serial messages may be used by multiple processes simultaneously. This may be useful for event triggered functionalities, if any.
