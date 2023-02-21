#![no_main]
#![no_std]
#![deny(unsafe_code)]
//#![deny(warnings)]

// use core::panic::PanicInfo;
//use cortex_m::asm;

use panic_halt as _;

//#[inline(never)]
//#[panic_handler]
//fn panic(info: &PanicInfo) -> ! {
//    rtt_target::rprintln!("{}", info);
//
//    loop {}
//}

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use core::borrow::Borrow;
    use rtt_target::{rprintln, rtt_init_print};
    use stepper::{
        drivers::drv8825::DRV8825, fugit, motion_control, ramp_maker, Direction, Stepper,
    };

    use stm32f4xx_hal::{gpio::Edge, prelude::*};

    // Define a Numeric Type
    // type Num = fixed::FixedU64<fixed::types::U32F32>;
    //use ramp_maker::trapezoidal::DefaultNum as Num;
    type Num = fixed::FixedU128<typenum::consts::U32>;

    // Resources shared between tasks
    #[shared]
    struct Shared {
        delayval: u32,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        //button: gpio::PC13<Input>,
        //led: gpio::PA5<Output<PushPull>>,
        //delay: timer::DelayMs<TIM1>,
    }

    use num_traits::cast::ToPrimitive;
    pub struct DelayToTicks;
    impl<const TIMER_HZ: u32> motion_control::DelayToTicks<Num, TIMER_HZ> for DelayToTicks {
        type Error = core::convert::Infallible;

        fn delay_to_ticks(
            &self,
            delay: Num,
        ) -> Result<fugit::TimerDurationU32<TIMER_HZ>, Self::Error> {
            Ok(fugit::TimerDurationU32::<TIMER_HZ>::from_ticks(
                Num::to_u32(&delay).expect("the delay to convert"),
            ))
        }
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        //let cp = cortex_m::Peripherals::take().unwrap();
        let mut dp = ctx.device;

        // Configure and obtain handle for delay abstraction
        // 1) Promote RCC structure to HAL to be able to configure clocks
        let rcc = dp.RCC.constrain();
        // 2) Configure the system clocks
        // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
        // 3) Create delay handle
        let delay_timer = dp.TIM1.borrow();
        //let delay = dp.TIM1.delay_ms(&clocks);

        rprintln!("Hello");
        // Configure the LED pin as a push pull ouput and obtain handle
        // On the Nucleo FR401 theres an on-board LED connected to pin PA5
        // 1) Promote the GPIOA PAC struct
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        // 2) Configure Pin and Obtain Handle
        let led = gpioa.pa5.into_push_pull_output();

        // Configure the button pin as input and obtain handle
        // On the Nucleo FR401 there is a button connected to pin PC13
        // 1) Promote the GPIOC PAC struct
        let gpioc = dp.GPIOC.split();
        // 2) Configure Pin and Obtain Handle
        let mut button = gpioc.pc13;

        // Configure Button Pin for Interrupts
        // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
        let mut syscfg = dp.SYSCFG.constrain();
        // 2) Make button an interrupt source
        button.make_interrupt_source(&mut syscfg);
        // 3) Make button an interrupt source
        button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        // 4) Enable gpio interrupt for button
        button.enable_interrupt(&mut dp.EXTI);

        //// Configure the stepper motor
        //// 1) Promote the GPIOB PAC struct
        //let gpiod = dp.GPIOD.split();
        //// 2) Configure Pins and Obtain Handle
        let mut dir = gpioa.pa9.into_push_pull_output().internal_pull_down(true);
        let mut step = gpioa.pa8.into_push_pull_output().internal_pull_down(true);
        let mut enable = gpiob.pb10.into_push_pull_output().internal_pull_down(true);

        let max_speed = Num::from_num(1000.0);
        let target_accel = Num::from_num(10.0); // steps per (unit of time)^2
        let profile = ramp_maker::Trapezoidal::new(target_accel);

        let mut timer = dp.TIM1.counter_us(&clocks);
        // 3) Create stepper motor
        let mut stepper = Stepper::from_driver(DRV8825::new())
            // Enable direction control
            .enable_direction_control(dir, Direction::Forward, &mut timer)
            .unwrap()
            // Enable step control
            .enable_step_control(step)
            .enable_motion_control((timer, profile, DelayToTicks));

        //4) Move stepper motor
        let target_steps = 1000;
        //enable.set_high();
        rprintln!("Moving {} steps", target_steps);
        let result = stepper
            .move_to_position(max_speed, target_steps)
            .wait()
            .unwrap();
        rprintln!("result: {:?}", result);

        (
            // Initialization of shared resources
            Shared { delayval: 2000_u32 },
            // Initialization of task local resources
            Local {},
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }

    // // Background task, runs whenever no other tasks are running
    // #[idle(local = [led, delay], shared = [delayval])]
    // fn idle(mut ctx: idle::Context) -> ! {
    //     let led = ctx.local.led;
    //     let delay = ctx.local.delay;
    //     loop {
    //         // Turn On LED
    //         led.set_high();
    //         // Obtain shared delay variable and delay
    //         delay.delay_ms(ctx.shared.delayval.lock(|del| *del));
    //         // Turn off LED
    //         led.set_low();
    //         // Obtain shared delay variable and delay
    //         delay.delay_ms(ctx.shared.delayval.lock(|del| *del));
    //     }
    // }

    // #[task(binds = EXTI15_10, local = [button], shared=[delayval])]
    // fn button_pressed(mut ctx: button_pressed::Context) {
    //     ctx.shared.delayval.lock(|del| {
    //         *del = *del - 500_u32;
    //         if *del < 500_u32 {
    //             *del = 2000_u32;
    //         }
    //         *del
    //     });
    //     // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
    //     ctx.local.button.clear_interrupt_pending_bit();
    // }
}
