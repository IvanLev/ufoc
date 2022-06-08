#![allow(dead_code)]

use stm32ral::{gpio, write_reg};

type Gpio = gpio::Instance;

/// Pins container.
///
/// Contains the results of setting up the GPIOs,
/// including access to switches, the HUB75E interface, and the GPIOs.
#[allow(clippy::manual_non_exhaustive)]
pub struct Pins {
    pub encoder_nss: OutputPin,
    _private: (),
}

//
//OPAMPS:
//OPAMP1- I_A ADC1_13
//OPAMP1- I_B ADC2_16

pub fn setup(gpioa: Gpio, gpiob: Gpio, gpiof: Gpio, gpiog: Gpio) -> Pins {
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
    write_reg!(gpio, gpioa, ODR, 0);
    write_reg!(gpio, gpioa, MODER, MODER0: Analog, MODER1: Analog, MODER2: Analog, MODER3: Analog,
                                   MODER4: Input, MODER5: Analog, MODER6: Input,
                                   MODER7: Analog, MODER8: Alternate, MODER9: Alternate,
                                   MODER10: Alternate, MODER11: Alternate, MODER12: Alternate,
                                   MODER13: Alternate, MODER14: Alternate, MODER15: Alternate );
    write_reg!(gpio, gpioa, PUPDR, PUPDR13: PullUp, PUPDR14: PullDown);

    write_reg!(gpio, gpioa, AFRH, AFRH8: 6, AFRH9: 6, AFRH10: 6, AFRH11: 6, AFRH12: 6, AFRH13: 0,
                                  AFRH14: 0);

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
    write_reg!(gpio, gpiob, ODR, 0);
    write_reg!(gpio, gpiob, MODER, MODER0: Input, MODER3: Alternate,
                                   MODER5: Alternate, MODER8: Output);

    write_reg!(gpio, gpiob, AFRL, AFRL3: 5, AFRL5: 5);

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
    write_reg!(gpio, gpiof, ODR, 0);
    write_reg!(gpio, gpiof, MODER, MODER0: Alternate, MODER1: Input);

    write_reg!(gpio, gpiof, AFRL, AFRL0: 6);

    // GPIOG
    // PE0-9: Not on chip
    // PE10: SW UART?
    // PE11-15: Not on chip
    write_reg!(gpio, gpiog, ODR, 0);

    Pins {
        encoder_nss:  OutputPin::new(&gpiob, 8),
        _private: (),
    }
}

/// Pin for runtime control of outputs.
pub struct OutputPin {
    bsrr: u32,
    pin: u32,
}

impl OutputPin {
    /// Construct a new OutputPin from a given GPIO instance and pin number.
    fn new(port: &gpio::Instance, pin: u32) -> OutputPin {
        OutputPin {
            bsrr: &port.BSRR as *const _ as u32, pin
        }
    }

    /// Set pin low if `level` is 0, otherwise set it high.
    pub fn set(&self, level: u32) {
        // NOTE(unsafe): Write into a write-only atomic register.
        unsafe {
            if level == 0 {
                core::ptr::write_volatile(self.bsrr as *mut u32, 1 << (self.pin + 16));
            } else {
                core::ptr::write_volatile(self.bsrr as *mut u32, 1 << self.pin);
            }
        }
    }

    /// Set pin high.
    pub fn set_high(&self) {
        self.set(1);
    }

    /// Set pin low.
    pub fn set_low(&self) {
        self.set(0);
    }
}

/// Pin for runtime reading of inputs.
pub struct InputPin {
    idr: u32,
    pin: u32,
}

impl InputPin {
    /// Construct a new InputPin from a given GPIO instance and pin number.
    fn new(port: &gpio::Instance, pin: u32) -> Self {
        InputPin {
            idr: &port.IDR as *const _ as u32, pin
        }
    }

    /// Reads current pin state.
    ///
    /// Returns true for a high level and false for a low level.
    pub fn get(&self) -> bool {
        // NOTE(unsafe): Read from a read-only register.
        unsafe {
            (core::ptr::read_volatile(self.idr as *const u32) >> self.pin) & 1 == 1
        }
    }
}

