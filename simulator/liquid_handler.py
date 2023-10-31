import zenoh, time

from gensi.formats import Message, Sample

def listener(sample):
    msg_str = sample.payload.decode('utf-8')
    received_msg = Sample.parse_raw(msg_str)
    print(f"Received {sample.kind} ('{sample.key_expr}': '{received_msg}')")

if __name__ == "__main__":
    session = zenoh.open()
    sub = session.declare_subscriber('incomming_swab/push', listener)
    time.sleep(60)
