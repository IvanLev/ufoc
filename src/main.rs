#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use panic_probe as _;

mod tim;
mod rcc;
mod gpio;
mod adc;
mod cordic;
mod dma;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[rtic::app(device=stm32ral::stm32g4::stm32g474, dispatchers=[SAI])]
mod app {
    use stm32ral::{read_reg, modify_reg, write_reg};
    use stm32ral::adc12_common;
    use crate::{
        rcc, tim, gpio, cordic, adc, dma,
    };

    static mut ADC1BUF: [u16; 8] = [0u16; 8];

    static mut ADC2BUF: [u16; 8] = [0u16; 8];

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
        cordic: cordic::Cordic,
        adc1: adc::Adc,
        adc2: adc::Adc,
        adc1_dma: dma::DMAChannel,
        adc2_dma: dma::DMAChannel,
        tim1: tim::Tim,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {

        defmt::println!("Start");

        let clocks = rcc::setup(cx.device.RCC, cx.device.PWR, cx.device.FLASH);

        cx.core.SCB.enable_icache();

        let cordic = cordic::Cordic::new(cx.device.CORDIC);
        cordic.init();

        let pins = gpio::setup(cx.device.GPIOA, cx.device.GPIOB, cx.device.GPIOF,
                               cx.device.GPIOG);

        let tim1 = tim::Tim::from_tim1(cx.device.TIM1);

        tim1.setup_bldc_pwm(8500);

        let mut adc1 = adc::Adc::new(cx.device.ADC1);
        let mut adc2 = adc::Adc::new(cx.device.ADC2);

        let mut adc12 = cx.device.ADC12_Common;

        modify_reg!(adc12_common, adc12, CCR, DUAL: DualRJ);

        adc1.setup_adc1(adc12);
        adc2.setup_adc2();
        defmt::println!("ADC init done");

        let dmamux = dma::DMAMux::new(cx.device.DMAMUX);
        let dma1 = dma::DMA::new(cx.device.DMA1);

        dmamux.set(0, 5);
        dmamux.set(1, 36);

        dma1.c1.setup_adc_circ(adc1.dr());
        dma1.c2.setup_adc_circ(adc2.dr());

        dma1.c1.start_adc_rx(unsafe { &mut ADC1BUF[..]});
        dma1.c2.start_adc_rx(unsafe { &mut ADC2BUF[..]});

        tim1.motor_on();

        adc1.start();
        adc2.start();

        defmt::println!("Init done!");

        (Shared {
        },

         Local {
            cordic,
             adc1,
             adc2,
             adc1_dma: dma1.c1,
             adc2_dma: dma1.c2,
             tim1,
         },

         init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            //defmt::println!("idle");
        }
    }

    #[task(binds=ADC1_2, priority=5, local=[adc1, adc2])]
    fn adc_1_2(mut cx: adc_1_2::Context) {
        cx.local.adc1.clear_jeos();
    }

    #[task(binds=DMA1_CH1, priority=4, local=[adc1_dma])]
    fn dma1_ch1(mut cx: dma1_ch1::Context) {
        cx.local.adc1_dma.clear_tcif();
    }

    #[task(binds=DMA1_CH2, priority=3, local=[adc2_dma])]
    fn dma1_ch2(mut cx: dma1_ch2::Context) {
        cx.local.adc2_dma.clear_tcif();
    }

    //#[task(binds=TIM1_UP_TIM16, priority=3, local=[tim1])]
    //fn bldc_pwm_int(mut cx: bldc_pwm_int::Context) {
    //    //defmt::println!("ADC1 regular channels: {}", unsafe { ADC1BUF});
    //    defmt::println!("Tim1");
    //}
}
