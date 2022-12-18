#include <zephyr.h>
#include <device.h>
#include <drivers/gpio.h>
#include <sys/printk.h>
#include <sys/__assert.h>
#include <string.h>
/*PWM Header is newly added*/
#include <drivers/pwm.h>

/* size of stack area used by each thread */
#define STACKSIZE 1024

/* scheduling priority used by each thread */
#define PRIORITY 7

/*Newly added*/
#define PERIOD_USEC	(20U * USEC_PER_MSEC)
#define STEP_USEC	100
#define MIN_PULSE_USEC	700
#define MAX_PULSE_USEC	2300

#define LED1_NODE DT_ALIAS(led1)

/*Newly added*/
#define PWM_NODE DT_ALIAS(pwm_servo)

#if !DT_NODE_HAS_STATUS(LED1_NODE, okay)
#error "Unsupported board: led1 devicetree alias is not defined"
#endif

/*Newly added*/
#if !DT_NODE_HAS_STATUS(PWM_NODE, okay)
#error "Unsupported board: pwm devicetree alias is not defined"
#endif

enum direction {
	DOWN,
	UP,
};

struct pwm_data_t {
    void *fifo_reserved;
    uint32_t count;
};

struct printk_data_t {
	void *fifo_reserved; /* 1st word reserved for use by fifo */
	uint32_t led;
	uint32_t cnt;
};

K_FIFO_DEFINE(printk_fifo);

struct led {
	struct gpio_dt_spec spec;
	const char *gpio_pin_name;
};

static const struct led led1 = {
	.spec = GPIO_DT_SPEC_GET_OR(LED1_NODE, gpios, 0),
	.gpio_pin_name = DT_PROP_OR(LED1_NODE, label, ""),
};

void servo(uint32_t sleep_ms)
{
	const struct device *pwm;
	uint32_t pulse_width = MIN_PULSE_USEC;
	enum direction dir = UP;
    int count = 0;
	int ret;

	/*printk("Servomotor control\n");*/

	pwm = DEVICE_DT_GET(PWM_NODE);

	if (!device_is_ready(pwm)) {
		printk("Error: PWM device %s is not ready\n", pwm->name);
		return;
	}

	while (1) {

        struct pwm_data_t data = { .count = count };

        size_t size = sizeof(data);
        char *ptr = k_malloc(size);
        __ASSERT_NO_MSG(ptr != 0);

        memcpy(ptr, &data, size);

        k_fifo_put(&printk_fifo, ptr);

        count++;

		ret = pwm_pin_set_usec(pwm, DT_PROP(DT_NODELABEL(pwm0), ch0_pin), PERIOD_USEC, pulse_width, 0);

		if (ret < 0) {
			printk("Error %d: failed to set pulse width\n", ret);
			return;
		}

		if (dir == DOWN) {
			if (pulse_width <= MIN_PULSE_USEC) {
				dir = UP;
				pulse_width = MIN_PULSE_USEC;
			} else {
				pulse_width -= STEP_USEC;
			}
		} else {
			pulse_width += STEP_USEC;

			if (pulse_width >= MAX_PULSE_USEC) {
				dir = DOWN;
				pulse_width = MAX_PULSE_USEC;
			}
		}

        k_msleep(sleep_ms);
	}
}

void blink(const struct led *led, uint32_t sleep_ms, uint32_t id)
{
	const struct gpio_dt_spec *spec = &led->spec;
    int cnt = 0;
	int ret;

	if (!device_is_ready(spec->port)) {
		printk("Error: %s device is not ready\n", spec->port->name);
		return;
	}

	ret = gpio_pin_configure_dt(spec, GPIO_OUTPUT);
	if (ret != 0) {
		printk("Error %d: failed to configure pin %d (LED '%s')\n",
			ret, spec->pin, led->gpio_pin_name);
		return;
	}

    while (1) {
        gpio_pin_set(spec->port, spec->pin, cnt % 2);

        struct printk_data_t tx_data = { .led = id, .cnt = cnt };

        size_t size = sizeof(struct printk_data_t);
        char *mem_ptr = k_malloc(size);
        __ASSERT_NO_MSG(mem_ptr != 0);

        memcpy(mem_ptr, &tx_data, size);

        k_fifo_put(&printk_fifo, mem_ptr);

        k_msleep(sleep_ms);
        cnt++;
    }
}

void blink1(void)
{
	blink(&led1, 2000, 0);
}

void servo0(void)
{
    servo(50);
}

void uart_out(void)
{
    while (1) {
        struct printk_data_t *rx_data0 = k_fifo_get(&printk_fifo,
                               K_FOREVER);
        struct pwm_data_t *rx_data1 = k_fifo_get(&printk_fifo,
                               K_FOREVER);
        printk("Toggled led%d; counter=%d\n; pwm counter=%d\n",
               rx_data0->led, rx_data0->cnt, rx_data1->count);
        k_free(rx_data0);
        k_free(rx_data1);
    }
}

K_THREAD_DEFINE(blink1_id, STACKSIZE, blink1, NULL, NULL, NULL,
		PRIORITY, 0, 0);
K_THREAD_DEFINE(uart_out_id, STACKSIZE, uart_out, NULL, NULL, NULL,
        PRIORITY, 0, 0);
K_THREAD_DEFINE(servo0_id, STACKSIZE, servo0, NULL, NULL, NULL,
		PRIORITY, 0, 0);
