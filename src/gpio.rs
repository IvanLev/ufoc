#![allow(dead_code)]

use stm32g4xx_hal::gpio::Alternate;
use stm32g4xx_hal::gpio::{AF5, AF6};
use stm32g4xx_hal::gpio::gpioa::{PA8, PA9, PA10, PA11, PA12, PA13, PA14, Parts as GPIOA};
use stm32g4xx_hal::gpio::gpiob::{Parts as GPIOB, PB3, PB5};
use stm32g4xx_hal::gpio::gpiof::{Parts as GPIOF, PF0};

//type Gpio = gpio::Instance;

//
//OPAMPS:
//OPAMP1- I_A ADC1_13
//OPAMP1- I_B ADC2_16

pub fn setup(gpioa: GPIOA, gpiob: GPIOB, gpiof: GPIOF) {
    // GPIOA
    // PA0:  Analog Input voltage divider ADC2_IN1
    // PA1:  Analog OPAMP1_VINP (Input voltage divider ADC2_IN2)
    // PA2:  Analog PCB/FET temperature sensor ADC1_IN3
    // PA3:  Analog OPAMP1_VINM0
    // PA4:  GPIO DIR
    // PA5:  Analog OPAMP2_VINM0
    // PA6:  GPIO STEP
    // PA7:  Analog OPAMP2_VINP
    // PA8:  AF6 PWM BLDC U_H
    // PA9:  AF6 PWM BLDC V_H
    // PA10: AF6 PWM BLDC W_H
    // PA11: AF6 PWM BLDC U_L
    // PA12: AF6 PWM BLDC V_L
    // PA13: AF0 SWDIO pulled up
    // PA14: AF0 SWCLK pulled down
    // PA15: Encoder something
    /*write_reg!(gpio, gpioa, MODER, MODER0: Analog, MODER1: Analog, MODER2: Analog, MODER3: Analog,
                                   MODER4: Input, MODER5: Analog, MODER6: Input,
                                   MODER7: Analog, MODER8: Alternate, MODER9: Alternate,
                                   MODER10: Alternate, MODER11: Alternate, MODER12: Alternate,
                                   MODER13: Alternate, MODER14: Alternate, MODER15: Alternate );
    write_reg!(gpio, gpioa, PUPDR, PUPDR13: PullUp, PUPDR14: PullDown);

    write_reg!(gpio, gpioa, AFRH, AFRH8: 6, AFRH9: 6, AFRH10: 6, AFRH11: 6, AFRH12: 6, AFRH13: 0,
                                  AFRH14: 0);*/
    let _pa0 = gpioa.pa0.into_analog();
    let _pa1 = gpioa.pa1.into_analog();
    let _pa2 = gpioa.pa2.into_analog();
    let _pa3 = gpioa.pa3.into_analog();
    let _pa4 = gpioa.pa4.into_pull_down_input();
    let _pa5 = gpioa.pa5.into_analog();
    let _pa6 = gpioa.pa6.into_pull_down_input();
    let _pa7 = gpioa.pa7.into_analog();
    let _pa8: PA8<Alternate<AF6>> = gpioa.pa8.into_alternate();
    let _pa9: PA9<Alternate<AF6>> = gpioa.pa9.into_alternate();
    let _pa10: PA10<Alternate<AF6>> = gpioa.pa10.into_alternate();
    let _pa11: PA11<Alternate<AF6>> = gpioa.pa11.into_alternate();
    let _pa12: PA12<Alternate<AF6>> = gpioa.pa12.into_alternate();
    //let _pa13: PA13<Alternate<AF0>> = gpioa.pa13.into_alternate();
    //let _pa14: PA14<Alternate<AF0>> = gpioa.pa14.into_alternate();
    let _pa15 = gpioa.pa15.into_pull_down_input();


    // GPIOB
    // PB0:  GPIO EN
    // PB1-2: Not on chip
    // PB3:  Encoder SCK
    // PB4:  Encoder something
    // PB5:  Encoder something
    // PB6:  Encoder something
    // PB7:  Encoder something
    // PB8:  Encoder NSS
    // PB9-15: Not on chip
    /*write_reg!(gpio, gpiob, ODR, 0);
    write_reg!(gpio, gpiob, MODER, MODER0: Input, MODER3: Alternate,
                                   MODER5: Alternate, MODER8: Output);

    write_reg!(gpio, gpiob, AFRL, AFRL3: 5, AFRL5: 5);*/
    let _pb0 = gpiob.pb0.into_pull_down_input();
    let _pb3: PB3<Alternate<AF5>> = gpiob.pb3.into_alternate();
    let _pb5: PB5<Alternate<AF5>> = gpiob.pb5.into_alternate();
    let _pb8 = gpiob.pb8.into_push_pull_output();

    // GPIOC
    // PC0-15: Not on chip

    // GPIOD
    // PD0-15: Not on chip

    // GPIOE
    // PE0-15: Not on chip

    // GPIOF
    // PF0: AF6 PWM BLDC W_L
    // PF1: GPIO
    // PF2-15: Not on chip
    /*write_reg!(gpio, gpiof, ODR, 0);
    write_reg!(gpio, gpiof, MODER, MODER0: Alternate, MODER1: Input);

    write_reg!(gpio, gpiof, AFRL, AFRL0: 6);*/
    let _pf0: PF0<Alternate<AF6>> = gpiof.pf0.into_alternate();

    // GPIOG
    // PE0-9: Not on chip
    // PE10: SW UART?
    // PE11-15: Not on chip
    //write_reg!(gpio, gpiog, ODR, 0);
}
