#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use panic_probe as _;

mod tim;
mod rcc;
mod gpio;
mod adc;
mod cordic;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[rtic::app(device=stm32ral::stm32g4::stm32g474, dispatchers=[SAI])]
mod app {
    use stm32ral::{read_reg, modify_reg, write_reg};
    use stm32ral::{adc12_common, dma, dmamux};
    use crate::{
        rcc, tim, gpio, cordic, adc,
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
        tim1: tim::Tim,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {

        let clocks = rcc::setup(cx.device.RCC, cx.device.PWR, cx.device.FLASH);

        cx.core.SCB.enable_icache();

        let cordic = cordic::Cordic::new(cx.device.CORDIC);
        cordic.init();

        let pins = gpio::setup(cx.device.GPIOA, cx.device.GPIOB, cx.device.GPIOC,
                               cx.device.GPIOD, cx.device.GPIOE);

        let tim1 = tim::Tim::from_tim1(cx.device.TIM1);

        tim1.setup_bldc_pwm(8500);

        let mut adc1 = adc::Adc::new(cx.device.ADC1);
        let adc2 = adc::Adc::new(cx.device.ADC2);

        let adc12 = cx.device.ADC12_Common;

        modify_reg!(adc12_common, adc12, CCR, DUAL: DualRJ);

        defmt::println!("ADC1");
        adc1.setup_adc1(adc12);
        defmt::println!("ADC2");
        adc2.setup_adc2();
        defmt::println!("ADC init done");

        let dmamux = cx.device.DMAMUX;
        let mut dma1 = cx.device.DMA1;

        write_reg!(dma, dma1, CCR1, MSIZE: 1, PSIZE: 1, MINC: 1, CIRC: 1);
        write_reg!(dma, dma1, CCR2, MSIZE: 1, PSIZE: 1, MINC: 1, CIRC: 1);

        write_reg!(dma, dma1, CNDTR1, 8);
        write_reg!(dma, dma1, CNDTR2, 8);

        unsafe {
            defmt::println!("ADC1 ptr {}", ADC1BUF.as_ptr());
            defmt::println!("ADC2 ptr: {}", ADC2BUF.as_ptr());

            write_reg!(dma, dma1, CMAR1, ADC1BUF.as_ptr() as u32);
            write_reg!(dma, dma1, CMAR2, ADC2BUF.as_ptr() as u32);

            write_reg!(dma, dma1, CPAR1, adc1.dr());
            write_reg!(dma, dma1, CPAR2, adc2.dr());

            defmt::println!("ADC1 CMAR1 {:x}", read_reg!(dma, dma1, CMAR1));
            defmt::println!("ADC2 CMAR2: {:x}", read_reg!(dma, dma1, CMAR1));

            defmt::println!("ADC1 CPAR1 {:x}", read_reg!(dma, dma1, CPAR1));
            defmt::println!("ADC2 CPAR2: {:x}", read_reg!(dma, dma1, CPAR2));
        }

        write_reg!(dmamux, dmamux, C0CR, DMAREQ_ID: 5);
        write_reg!(dmamux, dmamux, C1CR, DMAREQ_ID: 36);

        modify_reg!(dma, dma1, CCR1, TCIE: 1, EN: 1);
        modify_reg!(dma, dma1, CCR2, TCIE: 1, EN: 1);

        adc1.start();
        adc2.start();

        tim1.motor_on();

        defmt::println!("Init done!");

        (Shared {
        },

         Local {
            cordic,
             adc1,
             tim1,
         },

         init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            defmt::println!("idle task. ADC:: {}", unsafe { ADC1BUF[0]});
        }
    }

    #[task(binds=ADC1_2, priority=5, local=[adc1])]
    fn adc_1_2(mut cx: adc_1_2::Context) {
        defmt::println!("ADC_1_2 interrupt!");
    }

    #[task(binds=DMA1_CH1, priority=4)]
    fn dma1_ch1(mut cx: dma1_ch1::Context) {
        //defmt::println!("ADC1 regular channels: {}", unsafe { ADC1BUF});
        //defmt::println!("DMA1_Ch1 interrupt!");
    }

    #[task(binds=DMA1_CH2, priority=4)]
    fn dma1_ch2(mut cx: dma1_ch2::Context) {
        //defmt::println!("DMA1_Ch2 interrupt!");
        //defmt::println!("ADC2 regular channels: {}", unsafe { ADC2BUF});
    }

    #[task(binds=TIM1_UP_TIM16, priority=3, local=[tim1])]
    fn bldc_pwm_int(mut cx: bldc_pwm_int::Context) {
        //defmt::println!("ADC1 regular channels: {}", unsafe { ADC1BUF});
    }
}
