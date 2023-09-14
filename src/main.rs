#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;

use stm32_hal2::{
    self,
    clocks::{Clocks},
    gpio::{Pin, PinMode, Port},
    pac,
};

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = pac, dispatchers = [SAI])]
mod app {

    use stm32_hal2::{
        self,
        clocks::Clocks,
        gpio::{Pin, PinMode, Port},
        timer::{Timer, TimerConfig, Alignment, CaptureCompareDma, UpdateReqSrc, CountDir},
        pac,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let clock_cfg = Clocks::default();
        clock_cfg.setup().unwrap();
        let mut rcc = cx.device.RCC;

        defmt::println!("Setup:GPIO");
        let _pwm_pin_u_h = Pin::new(Port::A, 8, PinMode::Alt(6));
        let _pwm_pin_v_h = Pin::new(Port::A, 9, PinMode::Alt(6));
        let _pwm_pin_w_h = Pin::new(Port::A, 10, PinMode::Alt(6));
        let _pwm_pin_u_l = Pin::new(Port::A, 11, PinMode::Alt(6));
        let _pwm_pin_v_l = Pin::new(Port::A, 12, PinMode::Alt(6));
        let _pwm_pin_w_l = Pin::new(Port::F, 0, PinMode::Alt(6));

        defmt::println!("Setup:PWM Timer 1");
        let mut pwm_timer = Timer::new_tim1(
            cx.device.TIM1,
            10_000.,
            TimerConfig {
                auto_reload_preload: true,
                alignment: Alignment::Center1,
                capture_compare_dma: CaptureCompareDma::Ccx,
                one_pulse_mode: false,
                update_request_source: UpdateReqSrc::Any,
                direction: CountDir::Up,
            },
            &clock_cfg,
        );
        defmt::println!("Timer 1 max duty{}", pwm_timer.get_max_duty());
        

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            //defmt::println!("idle");
        }
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}