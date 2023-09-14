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
mod opamp;
mod ma734;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[rtic::app(device = stm32g4xx_hal::stm32, peripherals = true, dispatchers=[SAI])]
mod app {
    use cortex_m::asm::delay;
    use crate::{
        rcc, tim, gpio, cordic, adc, dma, opamp, ma734,
    };
    use stm32g4xx_hal::{
      rcc::{Config},
    };
    use stm32g4xx_hal::adc::ClockSource;
    use stm32g4xx_hal::prelude::*;

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
        encoder: ma734::MA734,
        tim1: tim::Tim,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {

        defmt::println!("Start");

        let rcc = ctx.device.RCC.constrain();
        let mut rcc = rcc.freeze(Config::hsi());

        ctx.core.SCB.enable_icache();

        let cordic = cordic::Cordic::new(ctx.device.CORDIC);
        cordic.init();

        let pins = gpio::setup(ctx.device.GPIOA, ctx.device.GPIOB,
                                     ctx.device.GPIOF, ctx.device.GPIOG);

        let opamp  = opamp::Opamp::new(ctx.device.OPAMP);
        opamp.init();

        let tim1 = tim::Tim::from_tim1(ctx.device.TIM1);

        tim1.setup_bldc_pwm(8500);

        let encoder = ma734::MA734::new(ctx.device.SPI1, pins.encoder_nss);

        encoder.init();

        defmt::println!("enc: {}", encoder.read_angle());

        //let mut adc1 = adc::Adc::new(ctx.device.ADC1);
        //let adc2 = adc::Adc::new(ctx.device.ADC2);

        //let adc12 = ctx.device.ADC12_Common;
        let (adc1, adc2) = adc::adc12(
            ctx.device.ADC1,
            ctx.device.ADC2,
            4.MHz(),
            &mut delay,
        );

        //adc1.enable_temperature();???

        //modify_reg!(adc12_common, adc12, CCR, DUAL: DualRJ);


        //adc1.setup_adc1(adc12);
        //adc2.setup_adc2();

        //let zero1 = adc1.get_avg_reading(13);
        //let zero2 = adc1.get_avg_reading(16);

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
             encoder,
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

    #[task(binds=ADC1_2, priority=5, local=[adc1, adc2, encoder])];
    fn adc_1_2(cx: adc_1_2::Context) {
        //defmt::println!("inj: {}, {}", cx.local.adc1.get_inj_data() - cx.shared.zero1, cx.local.adc2.get_inj_data() - cx.shared.zero2);
        //defmt::println!("inj: {}, {}", cx.shared.zero1, cx.shared.zero2);
        if cx.local.adc1.read_jeos() {
            cx.local.adc1.clear_jeos();
            defmt::println!("{}", cx.local.encoder.read_angle());
        }
    }

    #[task(binds=DMA1_CH1, priority=4, local=[adc1_dma])]
    fn dma1_ch1(cx: dma1_ch1::Context) {
        cx.local.adc1_dma.clear_tcif();
    }

    #[task(binds=DMA1_CH2, priority=3, local=[adc2_dma])]
    fn dma1_ch2(cx: dma1_ch2::Context) {
        cx.local.adc2_dma.clear_tcif();
    //    defmt::println!("ADC2 regular channels: {}", unsafe { ADC1BUF});
    }

    //#[task(binds=TIM1_UP_TIM16, priority=3, local=[tim1])]
    //fn bldc_pwm_int(mut cx: bldc_pwm_int::Context) {
    //    //defmt::println!("ADC1 regular channels: {}", unsafe { ADC1BUF});
    //    defmt::println!("Tim1");
    //}
}
