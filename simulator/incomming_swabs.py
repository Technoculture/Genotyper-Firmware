import zenoh
import random
import time
import math
import typer

from gensi.formats import Message, Sample

app = typer.Typer()

@app.command()
def simulate_incomming_samples(median: int = 300, sigma: float = 1.2, min_time: int = 60, speedup_factor: int = 300):
    random.seed()
    SIM_SPEEDUP_FACTOR = speedup_factor

    def get_sample_at_random(median=median, sigma=sigma, min_time=min_time):
        median = median / SIM_SPEEDUP_FACTOR
        min_time = min_time / SIM_SPEEDUP_FACTOR
        mu = math.log(median)

        sleep_time = random.lognormvariate(mu, sigma)

        # Ensure sleep_time is not less than min_time
        while sleep_time < min_time:
            sleep_time = random.lognormvariate(mu, sigma)

        print(f"Idle for {(sleep_time * SIM_SPEEDUP_FACTOR) / 60:.2f}min")
        time.sleep(sleep_time)
        return "DNA Test"

    session = zenoh.open()
    key = 'incomming_swab/push'
    pub = session.declare_publisher(key)

    while True:
        t = get_sample_at_random()
        sample = Sample()
        buf = sample.model_dump_json()
        print(f"Putting Data ('{key}': '{buf}')")
        pub.put(buf)

if __name__ == "__main__":
    print("Waiting for other apps to get started.")
    time.sleep(3)
    print("App Started.")

    app()
