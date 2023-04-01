#![allow(dead_code)]

use stm32g4xx_hal::stm32::TIM1;
use stm32g4xx_hal::stm32::tim1::ccmr1_output::{OC1M_A, OC2M_A};
use stm32g4xx_hal::stm32::tim1::ccmr2_output::{OC3M_A, OC4M_A};
use stm32g4xx_hal::stm32::tim1::ccmr3_output::OC5M_A;


/// Generic timer driver.
///
/// This driver does not type-check the provided timer peripheral, and so
/// if used incorrectly may try to enable an output on a timer without one.
pub struct PwmTim {
    tim: TIM1,
}

impl PwmTim {
    /// Create a new timer driver.
    pub fn new(tim: TIM1) -> Self {
        Self { tim }
    }

    /// Start the timer running by setting the CEN bit.
    pub fn start(&self) {
        self.tim.cr1.modify(|_, w| w.cen().set_bit());
    }

    /// Start the timer running by setting the CEN bit.
    pub fn motor_on(&self) {
        self.tim.bdtr.modify(|_, w| w.moe().set_bit());
    }

    /// Stop the timer running by clearing the CEN bit.
    pub fn stop(&self) {
        self.tim.cr1.modify(|_, w| w.cen().clear_bit());
        self.tim.cnt.write(|w| w.cnt().variant(0));
    }

    /// Start the timer in one-pulse mode for `ticks` period.
    pub fn start_oneshot(&self, ticks: u32) {
        self.tim.arr.write(|w| w.arr().variant(ticks));
        self.tim.cr1.modify(|_, w| w.cen().set_bit().opm().set_bit());
    }

    /// Clear ISR flags.
    pub fn clear_uif(&self) { self.tim.sr.write(|w| w.uif().clear_bit()); }

    /// Configure timer for three phase PWM generation
    pub fn setup_bldc_pwm(&self, period: u32) {
        // Ensure timer is disabled and use defaults for CR1 and CR2.
        self.tim.cr1.modify(|_, w| w.cen().clear_bit());
        self.tim.cr2.write(|w| unsafe { w.bits(0) });

        //Set total period, which divides the timer clock.
        self.tim.arr.write(|w| w.arr().variant(period - 1));
        //Generate an update to load the preloaded registers.
        self.tim.egr.write(|w| w.ug().set_bit());

        // Don't prescale, run timer at full timer clock.
        self.tim.psc.write(|w| w.psc().variant(0));

        // Update occurs every full cycle of the PWM timer
        self.tim.rcr.write(|w| unsafe { w.bits(1) });

        // Set center-aligned mode 1
        self.tim.cr1.write(|w| w.cms().variant(0b01));

        //enable OC4REF as trigger out and OC5REF as trigger out 2
        self.tim.cr2.write(|w| w.mms().variant(0b111).mms2().variant(0b1000));

        //Set CC mode to PWM mode 1(active in upcounting, inactive in downcounting)
        self.tim.ccmr1_output().write(|w| w.oc1m().variant(OC1M_A::PwmMode1).oc2m().variant(OC2M_A::PwmMode1));
        //Enable preload for 2 channels
        self.tim.ccmr1_output().modify(|_, w| w.oc1pe().set_bit().oc2pe().set_bit());
        //Set CC mode to PWM mode 1
        self.tim.ccmr2_output().write(|w| w.oc3m().variant(OC3M_A::PwmMode1).oc4m().variant(OC4M_A::PwmMode1));
        //Enable preload for 2 channels
        self.tim.ccmr2_output().modify(|_, w| w.oc3pe().set_bit().oc4pe().set_bit());
        //Set CC mode to PWM mode 1
        self.tim.ccmr3_output.write(|w| w.oc5m().variant(OC5M_A::PwmMode1));
        //Enable preload
        self.tim.ccmr3_output.modify(|_, w| w.oc5pe().set_bit());

        //Enable complementary outputs for 3 channels
        self.tim.ccer.write(|w|
            w.cc1e().set_bit().cc1ne().set_bit()
            .cc2e().set_bit().cc2ne().set_bit()
            .cc3e().set_bit().cc3ne().set_bit()
        );

        //Enable dead time to 500ns (clock 170Mhz) and make outputs low when MOE is 0
        self.tim.bdtr.write(|w| w.dtg().variant(85).ossi().set_bit());

        //Setup PWM to 0% and set trigger channels
        self.tim.ccr1().write(|w| w.ccr().variant(0));
        self.tim.ccr2().write(|w| w.ccr().variant(0));
        self.tim.ccr3().write(|w| w.ccr().variant(0));
        self.tim.ccr4().write(|w| w.ccr().variant(period - 2)); //Triggers when downcounting, after reload
        self.tim.ccr5.write(|w| w.ccr().variant(1)); //Triggers when downcounting, before zero


        self.tim.rcr.write(|w| unsafe { w.bits(1) });
        //Generate an update to load the preloaded registers.
        self.tim.egr.write(|w| w.ug().set_bit());
        //Start the PWM output.
        self.tim.cr1.modify(|_, w| w.cen().set_bit());

        self.clear_uif();
        self.tim.dier.modify(|_, w| w.uie().set_bit().bie().set_bit());
    }

    pub fn set_bldc_pwm(&self, ch1: u32, ch2: u32, ch3: u32) {
        //TODO: cache ARR?
        let arr = self.tim.arr.read().arr().bits();
        self.tim.ccr1().write(|w| w.ccr().variant(ch1 * arr / 65536));
        self.tim.ccr2().write(|w| w.ccr().variant(ch2 * arr / 65536));
        self.tim.ccr3().write(|w| w.ccr().variant(ch3 * arr / 65536));
    }

}