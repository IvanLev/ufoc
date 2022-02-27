#![allow(dead_code)]

use stm32ral::{gpio, write_reg};

type Gpio = gpio::Instance;

/// Pins container.
///
/// Contains the results of setting up the GPIOs,
/// including access to switches, the HUB75E interface, and the GPIOs.
#[allow(clippy::manual_non_exhaustive)]
pub struct Pins {
    pub en1: OutputPin,
    pub en2: OutputPin,
    pub en3: OutputPin,
    pub led: OutputPin,
    _private: (),
}

pub fn setup(gpioa: Gpio, gpiob: Gpio, gpioc: Gpio, _gpiod: Gpio, gpioe: Gpio) -> Pins {
    // GPIOA
    // PA0: BLDC Phase current I_A ADC1_IN1
    // PA1: Input voltage divider ADC2_IN2
    // PA2: Unused
    // PA3: Unused
    // PA4: Unused
    // PA5: Output, Nucleo LED
    // PA6: Unused
    // PA7: Unused
    // PA8: AF6 PWM BLDC U_H
    // PA9: AF6 PWM BLDC V_H
    // PA10: AF6 PWM BLDC W_H
    // PA11: Unused
    // PA12: Unused
    // PA13: AF0 SWDIO pulled up
    // PA14: AF0 SWCLK pulled down
    // PA15: Unused.
    write_reg!(gpio, gpioa, ODR, 0);
    write_reg!(gpio, gpioa, MODER, MODER0: Analog, MODER1: Analog, MODER5: Output,
                                   MODER8: Alternate, MODER9: Alternate, MODER10: Alternate,
                                   MODER13: Alternate, MODER14: Alternate);
    write_reg!(gpio, gpioa, PUPDR, PUPDR13: PullUp, PUPDR14: PullDown);

    write_reg!(gpio, gpioa, AFRH, AFRH8: 6, AFRH9: 6, AFRH10: 6, AFRH13: 0, AFRH14: 0);

    // GPIOB
    // PB0-12: Unused
    // PB13: AF6 PWM BLDC U_L
    // PB14: AF6 PWM BLDC V_L
    // PB15: AF4 PWM BLDC W_L
    write_reg!(gpio, gpiob, ODR, 0);
    write_reg!(gpio, gpiob, MODER, MODER13: Alternate, MODER14: Alternate, MODER15: Alternate);

    write_reg!(gpio, gpiob, AFRH, AFRH13: 6, AFRH14: 6, AFRH15: 4);

    // GPIOC
    // PC0: Unused
    // PC1: BLDC Phase current I_B ADC2_IN7
    // PC2: PCB/FET temperature sensor ADC1_IN8
    // PC3: Unused
    // PC4: Unused
    // PC5: Unused
    // PC6: Unused
    // PC7: Unused
    // PC8: Unused
    // PC9: Unused
    // PC10: Output, en1
    // PC11: Output, en2
    // PC12: Output, en3
    // PC13: Unused
    // PC14: Unused
    // PC15: Unused.
    write_reg!(gpio, gpioc, ODR, 0);
    write_reg!(gpio, gpioc, MODER, MODER1: Analog, MODER2: Analog, MODER10: Output,
                                   MODER11: Output, MODER12: Output);

    // GPIOD
    // PD0-15: Unused

    // GPIOE
    // PE0-15: Unused

    Pins {
        en1:        OutputPin::new(&gpioc, 10),
        en2:        OutputPin::new(&gpioc, 11),
        en3:        OutputPin::new(&gpioc, 12),
        led:        OutputPin::new(&gpioa, 5),
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

/// Force on the onboard LED from any context.
pub fn led_on() {
    // NOTE(unsafe): Atomic write-only register.
    unsafe {
        write_reg!(gpio, GPIOA, BSRR, BS5: 1);
    }
}

/// Force off the onboard LED from any context.
pub fn led_off() {
    // NOTE(unsafe): Atomic write-only register.
    unsafe {
        write_reg!(gpio, GPIOA, BSRR, BR5: 1);
    }
}
