#![no_main]
#![no_std]

mod tim;
mod dma;

use defmt_rtt as _; // global logger
use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[rtic::app(device = stm32g4xx_hal::stm32, peripherals = true, dispatchers=[SAI])]
mod app {
    use stm32g4xx_hal::rcc::Config;
    use stm32g4xx_hal::prelude::*;
    use crate::tim::PwmTim;

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::println!("Start");

        let rcc = ctx.device.RCC.constrain();
        let mut rcc = rcc.freeze(Config::pll());

        ctx.core.SCB.enable_icache();
        ctx.core.SCB.enable_dcache(&mut ctx.core.CPUID);

        //TODO:Enable and init cordic

        defmt::println!("Setup Gpio");
        let gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpiob = ctx.device.GPIOB.split(&mut rcc);
        let gpiof = ctx.device.GPIOF.split(&mut rcc);
        let gpiog = ctx.device.GPIOG.split(&mut rcc);

        //TODO: Figure out what to do with gpio pins

        //TODO: Setup OpAmp. To do it with HAL?

        defmt::println!("Setup TIM1PWM");
        let t1 = ctx.device.TIM1;
        let pwmTimer = PwmTim::new(t1);
        pwmTimer.setup_bldc_pwm(8500);
        pwmTimer.set_bldc_pwm(0, 0, 0);

        //TODO: Init and enable encoder

        //TODO: Init and enable ADC

        //TODO: Init and enable DMA

        pwmTimer.motor_on();

        //TODO: Start ADC1 and ADC2

        defmt::println!("Init done!");

        (Shared {
        },

         Local {
         },

         init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            //defmt::println!("idle");
        }
    }

    #[task(binds=ADC1_2, priority=5)]//, local=[adc1, adc2, encoder])]
    fn adc_1_2(cx: adc_1_2::Context) {
        //defmt::println!("inj: {}, {}", cx.local.adc1.get_inj_data() - cx.shared.zero1, cx.local.adc2.get_inj_data() - cx.shared.zero2);
        //defmt::println!("inj: {}, {}", cx.shared.zero1, cx.shared.zero2);
        //if cx.local.adc1.read_jeos() {
        //    cx.local.adc1.clear_jeos();
        //    defmt::println!("{}", cx.local.encoder.read_angle());
        //}
    }

    #[task(binds=DMA1_CH1, priority=4)]//, local=[adc1_dma])]
    fn dma1_ch1(cx: dma1_ch1::Context) {
        //cx.local.adc1_dma.clear_tcif();
    }

    #[task(binds=DMA1_CH2, priority=3)]//, local=[adc2_dma])]
    fn dma1_ch2(cx: dma1_ch2::Context) {
        //cx.local.adc2_dma.clear_tcif();
        //    defmt::println!("ADC2 regular channels: {}", unsafe { ADC1BUF});
    }

    //#[task(binds=TIM1_UP_TIM16, priority=3, local=[tim1])]
    //fn bldc_pwm_int(mut cx: bldc_pwm_int::Context) {
    //    //defmt::println!("ADC1 regular channels: {}", unsafe { ADC1BUF});
    //    defmt::println!("Tim1");
    //}
}
