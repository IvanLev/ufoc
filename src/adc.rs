#![allow(dead_code)]

use stm32g4xx_hal::stm32::{ADC1, ADC2, ADC12_COMMON};
use stm32g4xx_hal::stm32::adc1::smpr1::SMP0_A::{Cycles245};
use stm32g4xx_hal::stm32::adc1::smpr2::SMP16_A::{Cycles25};
use stm32g4xx_hal::stm32::adc1::smpr2::SMP18_A::{Cycles2475};
use stm32g4xx_hal::stm32::adc1::jsqr::{JEXTSEL_A, JEXTEN_A};
use stm32g4xx_hal::stm32::adc1::cfgr::{OVRMOD_A, EXTSEL_A, EXTEN_A};
use stm32g4xx_hal::stm32::adc12_common::ccr::{DUAL_A};

pub struct Adc1 {
    adc: ADC1,
    vref_cal: f32,
}

pub struct Adc2 {
    adc: ADC2,
    vref_cal: f32,
}

impl Adc1 {
    pub fn new(adc: ADC1) -> Self {
        Self { adc, vref_cal: 0.0 }
    }

    #[inline(always)]
    pub fn setup(&mut self, adc12: ADC12_COMMON) {
        unsafe {
            let rcc_ptr = &(*stm32g4xx_hal::stm32::RCC::ptr());
            rcc_ptr.cfgr.modify(|_, w| w.hpre().div1().ppre1().div1().ppre2().div1());
            rcc_ptr.ahb2enr.modify(|_, w| w.adc12en().set_bit());
            rcc_ptr.ccipr.modify(|_, w| w.adc12sel().pllp());
        }
        self.adc.cr.modify(|_, w| w.deeppwd().clear_bit());
        adc12.ccr.modify(|_, w| w.dual().variant(DUAL_A::DualRj));

        adc12.ccr.modify(|_, w| w.vrefen().set_bit().dmacfg().set_bit());
        self.adc.cr.modify(|_, w| w.advregen().set_bit());
        defmt::println!("Wait");
        while self.adc.cr.read().advregen().bit_is_clear() {} // Wait for the avrgen
        defmt::println!("Before delay");

        cortex_m::asm::delay(300_000);
        defmt::println!("After delay");
        self.adc.cr.modify(|_, w| w.adcal().set_bit().adcaldif().single_ended());
        while self.adc.cr.read().adcal().bit_is_set() {
            defmt::println!("Calibrating");
        } // Wait for the calibration
        defmt::println!("Calc VREF");
        self.calc_vref();

        self.adc.cfgr.write(|w| w.jqdis().disabled().dmaen().enabled().dmacfg().circular());
        self.adc.cfgr2.write(|w| unsafe {w.bits(0)});

        //set sampling time for I_A and Temp
        self.adc.smpr1.write(|w| w.smp3().variant(Cycles245));
        self.adc.smpr2.write(|w| w.smp13().variant(Cycles25));

        //set  regular sequence to just Temp
        self.adc.sqr1.write(|w| w.l().variant(7).sq1().variant(3).sq2().variant(3)
            .sq3().variant(3).sq4().variant(3));
        self.adc.sqr2.write(|w| w.sq5().variant(3).sq6().variant(3)
            .sq7().variant(3).sq8().variant(3));

        // Set current injected channels
        self.adc.jsqr.write(|w| w.jl().variant(1).jextsel().variant(JEXTSEL_A::Tim1Trgo)
            .jexten().variant(JEXTEN_A::RisingEdge).jsq1().variant(13).jsq2().variant(13));

        self.adc.cfgr.modify(|_, w| w.ovrmod().overwrite()
            .extsel().variant(EXTSEL_A::Tim1Trgo2).exten().variant(EXTEN_A::RisingEdge));

        self.adc.ier.modify(|_, w| w.jeosie().enabled());

    }
    
    fn calc_vref(&mut self) {
        const VREFINTCAL_MIN : u16 = 1570;
        const VREFINTCAL_MAX : u16 = 1734;
        const ADC_FAC_CAL_VOL : f32 = 3.0;
        const FLT_MAXCNT : f32 = 4095.0;
        const VREFINT_CAL_DEF : f32 = 1.212;
        const ADC_SMPLS : u16 = 128;

        let temp_smpr2 = self.adc.smpr2.read().bits();
        let temp_sqr1 = self.adc.sqr1.read().bits();
        let temp_cfgr = self.adc.cfgr.read().bits();
        let temp_cfgr2 = self.adc.cfgr2.read().bits();
        let temp_ier = self.adc.ier.read().bits();

        self.adc.smpr2.write(|w| unsafe {w.bits(0)});
        self.adc.sqr1.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(0)});
        self.adc.ier.write(|w| unsafe {w.bits(0)});

        let vref_int : f32;
        let vref_cal : u16;

        unsafe {
            vref_cal = core::ptr::read_volatile(0x1FFF_75AA as *const u16);
        }

        if (vref_cal >= VREFINTCAL_MIN) && (vref_cal <= VREFINTCAL_MAX) {
            vref_int = (ADC_FAC_CAL_VOL) * f32::from(vref_cal) / FLT_MAXCNT;
        } else {
            vref_int = VREFINT_CAL_DEF;
        }

        self.enable();

        // Set sampling time for Vrefint
        self.adc.smpr2.write(|w| w.smp18().variant(Cycles2475));
        self.adc.sqr1.write(|w| w.sq1().variant(18));

        let mut vref_sum : f32 = 0.0;
        for _i in 0..ADC_SMPLS {
            self.adc.cr.modify(|_, w| w.adstart().set_bit());
            while self.adc.isr.read().eoc().bit_is_clear() {}
            self.adc.isr.modify(|_, w| w.eoc().clear_bit());
            vref_sum = vref_sum + (self.adc.dr.read().bits() as f32);
        }

        self.vref_cal = vref_int / ( ( vref_sum / f32::from(ADC_SMPLS) ) / FLT_MAXCNT );

        self.adc.smpr2.write(|w| unsafe {w.bits(temp_smpr2)});
        self.adc.sqr1.write(|w| unsafe {w.bits(temp_sqr1)});
        self.adc.cfgr.write(|w| unsafe {w.bits(temp_cfgr)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(temp_cfgr2)});
        self.adc.ier.write(|w| unsafe {w.bits(temp_ier)});
    }

    pub fn get_avg_reading(&mut self, chan: u16) -> u16 {
        const ADC_SMPLS : u16 = 64;

        let temp_smpr2 = self.adc.smpr2.read().bits();
        let temp_sqr1 = self.adc.sqr1.read().bits();
        let temp_cfgr = self.adc.cfgr.read().bits();
        let temp_cfgr2 = self.adc.cfgr2.read().bits();
        let temp_ier = self.adc.ier.read().bits();

        self.adc.smpr2.write(|w| unsafe {w.bits(0)});
        self.adc.sqr1.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(0)});
        self.adc.ier.write(|w| unsafe {w.bits(0)});

        self.enable();

        // Set sampling time
        self.adc.smpr2.write(|w| w.smp18().variant(Cycles2475));
        self.adc.sqr1.write(|w| w.sq1().variant(chan as u8));

        let mut res_sum : f32 = 0.0;
        for _i in 0..ADC_SMPLS {
            self.adc.cr.modify(|_, w| w.adstart().set_bit());
            while self.adc.isr.read().eoc().bit_is_clear() {}
            self.adc.isr.modify(|_, w| w.eoc().clear_bit());
            res_sum = res_sum + (self.adc.dr.read().bits() as f32);
        }

        self.adc.smpr2.write(|w| unsafe {w.bits(temp_smpr2)});
        self.adc.sqr1.write(|w| unsafe {w.bits(temp_sqr1)});
        self.adc.cfgr.write(|w| unsafe {w.bits(temp_cfgr)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(temp_cfgr2)});
        self.adc.ier.write(|w| unsafe {w.bits(temp_ier)});
        (res_sum / f32::from(ADC_SMPLS)) as u16
    }

    pub fn start(&self) {
        self.enable();
        self.adc.cr.modify(|_, w| w.adstart().set_bit().jadstart().set_bit());
    }

    pub fn dr(&self) -> u32 { &self.adc.dr.read().bits() as *const _ as u32 }

    fn enable(&self) {
        if self.adc.cr.read().aden().bit_is_clear() {
            self.adc.isr.modify(|_, w| w.adrdy().clear_bit());
            self.adc.cr.modify(|_, w| w.aden().set_bit());

            while self.adc.isr.read().adrdy().bit_is_clear() {}
        }
    }

    // Clear JEOS interrupt flag
    pub fn clear_jeos(&self) { self.adc.isr.modify(|_, w| w.jeos().clear_bit()); }

    pub fn read_jeos(&self) -> bool { self.adc.isr.read().jeos().bit() }

    pub fn get_inj_data(&self) -> u16{ self.adc.jdr1.read().jdata().bits() as u16 }
}

impl Adc2 {
    pub fn new(adc: ADC2) -> Self {
        Self { adc, vref_cal: 0.0 }
    }

    pub fn setup(&self) {
        self.adc.cr.modify(|_, w| w.deeppwd().clear_bit());

        self.adc.cr.modify(|_, w| w.advregen().set_bit());
        while self.adc.cr.read().advregen().bit_is_clear() {} // Wait for the avrgen

        cortex_m::asm::delay(300_000);

        self.adc.cr.modify(|_, w| w.adcal().set_bit().adcaldif().single_ended());
        while self.adc.cr.read().adcal().bit_is_set() {} // Wait for the calibration

        self.adc.cfgr.modify(|_, w| w.jqdis().set_bit().dmaen().set_bit().dmacfg().set_bit());
        self.adc.cfgr2.write(|w| unsafe {w.bits(0)});

        // Set sampling time for I_B and VM
        self.adc.smpr1.write(|w| w.smp4().variant(Cycles245));
        self.adc.smpr2.write(|w| w.smp16().variant(Cycles25));

        //set  regular sequence to just VM
        self.adc.sqr1.write(|w| w.l().variant(7).sq1().variant(1).sq2().variant(1)
            .sq3().variant(1).sq4().variant(1));
        self.adc.sqr2.write(|w| w.sq5().variant(1).sq6().variant(1)
            .sq7().variant(1).sq8().variant(1));

        // Set current injected channels
        self.adc.jsqr.write(|w| w.jl().variant(1).jextsel().variant(JEXTSEL_A::Tim1Trgo)
            .jexten().variant(JEXTEN_A::RisingEdge).jsq1().variant(16).jsq2().variant(16));
        self.adc.cfgr.modify(|_, w| w.ovrmod().variant(OVRMOD_A::Overwrite)
            .extsel().variant(EXTSEL_A::Tim1Trgo2).exten().variant(EXTEN_A::RisingEdge));
    }

    pub fn get_avg_reading(&mut self, chan: u16) -> u16 {
        const ADC_SMPLS : u16 = 64;

        let temp_smpr2 = self.adc.smpr2.read().bits();
        let temp_sqr1 = self.adc.sqr1.read().bits();
        let temp_cfgr = self.adc.cfgr.read().bits();
        let temp_cfgr2 = self.adc.cfgr2.read().bits();
        let temp_ier = self.adc.ier.read().bits();

        self.adc.smpr2.write(|w| unsafe {w.bits(0)});
        self.adc.sqr1.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr.write(|w| unsafe {w.bits(0)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(0)});
        self.adc.ier.write(|w| unsafe {w.bits(0)});

        self.enable();

        // Set sampling time
        self.adc.smpr2.write(|w| w.smp18().variant(Cycles2475));
        self.adc.sqr1.write(|w| w.sq1().variant(chan as u8));

        let mut res_sum : f32 = 0.0;
        for _i in 0..ADC_SMPLS {
            self.adc.cr.modify(|_, w| w.adstart().set_bit());
            while self.adc.isr.read().eoc().bit_is_clear() {}
            self.adc.isr.modify(|_, w| w.eoc().clear_bit());
            res_sum = res_sum + (self.adc.dr.read().bits() as f32);
        }

        self.adc.smpr2.write(|w| unsafe {w.bits(temp_smpr2)});
        self.adc.sqr1.write(|w| unsafe {w.bits(temp_sqr1)});
        self.adc.cfgr.write(|w| unsafe {w.bits(temp_cfgr)});
        self.adc.cfgr2.write(|w| unsafe {w.bits(temp_cfgr2)});
        self.adc.ier.write(|w| unsafe {w.bits(temp_ier)});
        (res_sum / f32::from(ADC_SMPLS)) as u16
    }

    pub fn start(&self) {
        self.enable();
        self.adc.cr.modify(|_, w| w.adstart().set_bit().jadstart().set_bit());
    }

    pub fn dr(&self) -> u32 { &self.adc.dr.read().bits() as *const _ as u32 }

    fn enable(&self) {
        //if read_reg!(adc, self.adc, CR, ADEN == 0) {
        if self.adc.cr.read().aden().bit_is_clear() {
            self.adc.isr.modify(|_, w| w.adrdy().clear_bit());
            self.adc.cr.modify(|_, w| w.aden().set_bit());

            while self.adc.isr.read().adrdy().bit_is_clear() {}
        }
    }

    // Clear JEOS interrupt flag
    pub fn clear_jeos(&self) { self.adc.isr.modify(|_, w| w.jeos().clear_bit()); }

    pub fn read_jeos(&self) -> bool { self.adc.isr.read().jeos().bit() }

    pub fn get_inj_data(&self) -> u16{ self.adc.jdr1.read().jdata().bits() as u16 }
}
