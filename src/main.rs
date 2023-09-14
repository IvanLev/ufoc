#![no_main]
#![no_std]

mod tim;
mod dma;
mod opamp;
mod gpio;
mod adc;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[rtic::app(device = stm32g4xx_hal::stm32, peripherals = true, dispatchers=[SAI])]
mod app {
    use stm32g4xx_hal::gpio::gpiob::{PB3, PB4, PB5, PB8};
    use stm32g4xx_hal::gpio::{AF5};
    use stm32g4xx_hal::gpio::{Alternate, Output, PushPull};
    use stm32g4xx_hal::rcc::Config;
    use stm32g4xx_hal::prelude::*;
    use stm32g4xx_hal::spi::Spi;
    use stm32g4xx_hal::stm32::SPI1;
    use stm32g4xx_hal::spi;
    use ma734;
    use crate::tim::PwmTim;
    use crate::opamp::Opamp;
    use crate::gpio;
    use crate::adc::{Adc1, Adc2};

    type SCK = PB3<Alternate<AF5>>;
    type MISO = PB4<Alternate<AF5>>;
    type MOSI = PB5<Alternate<AF5>>;

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
        encoder: ma734::MA734<Spi<SPI1, (SCK, MISO, MOSI)>, PB8<Output<PushPull>>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::println!("Start");

        let dp = ctx.device;
        let rcc = dp.RCC.constrain();
        let mut rcc = rcc.freeze(Config::pll());
        //let mut rcc = rcc.freeze(Config::hsi());

        ctx.core.SCB.enable_icache();
        ctx.core.SCB.enable_dcache(&mut ctx.core.CPUID);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);
        let gpiof = dp.GPIOF.split(&mut rcc);
        //let gpiog = dp.GPIOG.split(&mut rcc);

        let mut nss_pin = gpiob.pb8.into_push_pull_output();
        nss_pin.set_high().unwrap();
        let spi1_sck: PB3<Alternate<AF5>> = gpiob.pb3.into_alternate();
        let spi1_miso: PB4<Alternate<AF5>> = gpiob.pb4.into_alternate();
        let spi1_mosi: PB5<Alternate<AF5>> = gpiob.pb5.into_alternate();

        let spi1: spi::Spi<_, _> = dp.SPI1.spi(
            (spi1_sck, spi1_miso, spi1_mosi),
            stm32g4xx_hal::spi::MODE_0,
            12.mhz(),
            &mut rcc,
        );

        //TODO:Enable and init cordic

        defmt::println!("Setup Gpio");
        unsafe {
            let rcc_ptr = &(*stm32g4xx_hal::stm32::RCC::ptr());
            rcc_ptr.cfgr.modify(|_, w| w.ppre2().variant(stm32g4xx_hal::stm32::rcc::cfgr::PPRE2_A::Div1));
            rcc_ptr.ahb2enr.modify(|_, w| w.gpioaen().set_bit()
                .gpioben().set_bit().gpiofen().set_bit());
            rcc_ptr.apb2enr.modify(|_, w| w.tim1en().set_bit().syscfgen().set_bit());
        }

        //Take all needed pins before giving gpio blocks
        gpio::setup(gpioa, gpiof);

        let opamp = Opamp::new(dp.OPAMP);
        opamp.init();

        defmt::println!("Setup TIM1PWM");
        let t1 = dp.TIM1;
        let pwmTimer = PwmTim::new(t1);
        pwmTimer.setup_bldc_pwm(8500);
        pwmTimer.set_bldc_pwm(0, 0, 0);
        defmt::println!("Setup TIM1PWM Done");

        let mut encoder = ma734::MA734::new(spi1, nss_pin);
        defmt::println!("Encoder created");
        let angle = encoder.read_angle().unwrap();
        defmt::println!("Angle: {}", angle);


        let mut adc1 = Adc1::new(dp.ADC1);
        let adc2 = Adc2::new(dp.ADC2);
        defmt::println!("ADC taken");
        adc1.setup(dp.ADC12_COMMON);
        defmt::println!("ADC1 setup done");
        adc2.setup();
        defmt::println!("ADC2 setup done");

        //let _zero1 = adc1.get_avg_reading(13);
        //let _zero2 = adc2.get_avg_reading(16);

        //TODO: Init and enable DMA

        pwmTimer.motor_on();

        adc1.start();
        adc2.start();

        defmt::println!("Init done!");

        (Shared {
        },

         Local {
             encoder,
         },

         init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            //defmt::println!("idle");
        }
    }

    #[task(binds=ADC1_2, priority=5, local=[encoder])]
    fn adc_1_2(cx: adc_1_2::Context) {
        //defmt::println!("inj: {}, {}", cx.local.adc1.get_inj_data() - cx.shared.zero1, cx.local.adc2.get_inj_data() - cx.shared.zero2);
        //defmt::println!("inj: {}, {}", cx.shared.zero1, cx.shared.zero2);
        //if cx.local.adc1.read_jeos() {
        //    cx.local.adc1.clear_jeos();
        //    defmt::println!("{}", cx.local.encoder.read_angle());
        //}
        //defmt::println!("ADC1_2!");
        let angle = cx.local.encoder.read_angle().unwrap();
        defmt::println!("Angle: {}", angle);
    }

    //#[task(binds=DMA1_CH1, priority=4)]//, local=[adc1_dma])]
    //fn dma1_ch1(_cx: dma1_ch1::Context) {
        //cx.local.adc1_dma.clear_tcif();
    //}

    //#[task(binds=DMA1_CH2, priority=3)]//, local=[adc2_dma])]
    //fn dma1_ch2(_cx: dma1_ch2::Context) {
        //cx.local.adc2_dma.clear_tcif();
        //    defmt::println!("ADC2 regular channels: {}", unsafe { ADC1BUF});
    //}

    //#[task(binds=TIM1_UP_TIM16, priority=1, local=[encoder])]
    //fn bldc_pwm_int(_cx: bldc_pwm_int::Context) {
    //    defmt::println!("Timer!");
        //let angle = cx.local.encoder.read_angle().unwrap();
        //defmt::println!("Angle: {}", angle);
    //}
}
