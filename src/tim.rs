use stm32ral::{modify_reg, write_reg, read_reg};
use stm32ral::tim1;


/// Generic timer driver.
///
/// This driver does not type-check the provided timer peripheral, and so
/// if used incorrectly may try to enable an output on a timer without one.
pub struct Tim {
    tim: tim1::Instance,
}

macro_rules! impl_tim {
    ($type:ident, $fn:ident) => {
        pub fn $fn(tim: $type::Instance) -> Self {
            // NOTE(unsafe): We'll only transmute various types of timer instance to common TIM1.
            Tim { tim: unsafe { core::mem::transmute(tim) } }
        }
    }
}

impl Tim {
    impl_tim!(tim1, from_tim1);

    /// Start the timer running by setting the CEN bit.
    pub fn start(&self) {
        modify_reg!(tim1, self.tim, CR1, CEN: 1);
    }

    /// Start the timer running by setting the CEN bit.
    pub fn motor_on(&self) {
        modify_reg!(tim1, self.tim, BDTR, MOE: 1);
    }

    /// Stop the timer running by clearing the CEN bit.
    pub fn stop(&self) {
        modify_reg!(tim1, self.tim, CR1, CEN: 0b0);
        write_reg!(tim1, self.tim, CNT, 0);
    }

    /// Start the timer in one-pulse mode for `ticks` period.
    pub fn start_oneshot(&self, ticks: u32) {
        write_reg!(tim1, self.tim, ARR, ticks);
        modify_reg!(tim1, self.tim, CR1, CEN: 0b1, OPM: 0b1);
    }

    /// Clear ISR flags.
    pub fn clear_uif(&self) {
        write_reg!(tim1, self.tim, SR, UIF: 0b0);
    }

    /// Configure timer for three phase PWM generation
    pub fn setup_bldc_pwm(&self, period: u32) {
        // Ensure timer is disabled and use defaults for CR1 and CR2.
        modify_reg!(tim1, self.tim, CR1, CEN: 0);
        write_reg!(tim1, self.tim, CR2, 0);

        //Set total period, which divides the timer clock.
        write_reg!(tim1, self.tim, ARR, period - 1);
        //Generate an update to load the preloaded registers.
        write_reg!(tim1, self.tim, EGR, UG: 1);

        // Don't prescale, run timer at full timer clock.
        write_reg!(tim1, self.tim, PSC, 0);

        // Update occurs every full cycle of the PWM timer
        write_reg!(tim1, self.tim, RCR, 1);

        // Set center-aligned mode 1
        write_reg!(tim1, self.tim, CR1, CMS: 0b01);

        //enable OC4REF as trigger out and OC5REF as trigger out 2
        write_reg!(tim1, self.tim, CR2, MMS: 0b111, MMS2: 0b1000);

        //Set CC mode to PWM mode 1(active in upcounting, inactive in downcounting)
        write_reg!(tim1, self.tim, CCMR1, OC1M: 0b110, OC2M: 0b110);
        //Enable preload for 2 channels
        modify_reg!(tim1, self.tim, CCMR1, OC1PE: 1, OC2PE: 1);
        //Set CC mode to PWM mode 1
        write_reg!(tim1, self.tim, CCMR2, OC3M: 0b110, OC4M: 0b110);
        //Enable preload for 2 channels
        modify_reg!(tim1, self.tim, CCMR2, OC3PE: 1, OC4PE: 1);
        //Set CC mode to PWM mode 1
        write_reg!(tim1, self.tim, CCMR3_Output, OC5M: 0b110);
        //Enable preload
        modify_reg!(tim1, self.tim, CCMR3_Output, OC5PE: 1);

        //Enable complementary outputs for 3 channels
        write_reg!(tim1, self.tim, CCER, CC1E: 1, CC1NE: 1, CC2E: 1, CC2NE: 1, CC3E: 1, CC3NE: 1);

        //Enable dead time to 500ns (clock 170Mhz) and make outputs low when MOE is 0
        write_reg!(tim1, self.tim, BDTR, DTG: 85, OSSI: 1);

        //Setup PWM to 0% and set trigger channels
        //write_reg!(tim1, self.tim, CCR1, 0);
        //write_reg!(tim1, self.tim, CCR2, 0);
        //write_reg!(tim1, self.tim, CCR3, 0);
        write_reg!(tim1, self.tim, CCR1, period/4);
        write_reg!(tim1, self.tim, CCR2, period/2);
        write_reg!(tim1, self.tim, CCR3, 3*(period/4));
        write_reg!(tim1, self.tim, CCR4, period - 1); //Triggers when downcounting, after reload
        write_reg!(tim1, self.tim, CCR5, 1); //Triggers when downcounting, before zero

        write_reg!(tim1, self.tim, RCR, 1);
        //Generate an update to load the preloaded registers.
        write_reg!(tim1, self.tim, EGR, UG: 1);
        //Start the PWM output.
        modify_reg!(tim1, self.tim, CR1, CEN: 1);

        self.clear_uif();
        modify_reg!(tim1, self.tim, DIER, UIE: 1, BIE: 1);
    }

    pub fn set_bldc_pwm(&self, ch1: u16, ch2: u16, ch3: u16) {
        let arr = read_reg!(tim1, self.tim, ARR);
        write_reg!(tim1, self.tim, CCR1, (ch1 as u32) * arr / 65536);
        write_reg!(tim1, self.tim, CCR2, (ch2 as u32) * arr / 65536);
        write_reg!(tim1, self.tim, CCR3, (ch3 as u32) * arr / 65536);
    }

}