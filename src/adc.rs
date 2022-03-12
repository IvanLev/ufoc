use stm32ral::{modify_reg, write_reg, read_reg};
use stm32ral::{adc, adc12_common};

pub struct Adc {
    pub adc: adc::Instance,
    vref_cal: f32,
}

impl Adc {
    pub fn new(adc: adc::Instance) -> Self {
        Self { adc, vref_cal: 0.0 }
    }

    pub fn setup_adc1(&mut self, adc12: stm32ral::adc12_common::Instance) {


        modify_reg!(adc, self.adc, CR, DEEPPWD: Disabled);

        modify_reg!(adc12_common, adc12, CCR, VREFEN: Enabled, DMACFG: 1);
        modify_reg!(adc, self.adc, CR, ADVREGEN: Enabled);
        while read_reg!(adc, self.adc, CR, ADVREGEN != 1) {} // Wait for the avrgen

        cortex_m::asm::delay(300_000);

        modify_reg!(adc, self.adc, CR, ADCAL: Calibration, ADCALDIF: SingleEnded);
        while read_reg!(adc, self.adc, CR, ADCAL != 0) {} // Wait for the calibration

        self.calc_vref();

        write_reg!(adc, self.adc, CFGR, JQDIS: Enabled, DMAEN: Enabled, DMACFG: Circular);
        write_reg!(adc, self.adc, CFGR2, 0);

        //set sampling time for I_A and Temp
        write_reg!(adc, self.adc, SMPR1, SMP1: Cycles2_5, SMP8: Cycles24_5);

        //set  regular sequence to just Temp
        write_reg!(adc, self.adc, SQR1, L: 7, SQ1: 3, SQ2: 3, SQ3: 3, SQ4: 3);
        write_reg!(adc, self.adc, SQR2, SQ5: 3, SQ6: 3, SQ7: 3, SQ8: 3);

        // Set current injected channels
        write_reg!(adc, self.adc, JSQR, JL: 0b1, JEXTSEL: TIM1_TRGO, JEXTEN: RisingEdge, JSQ1: 13, JSQ2: 13);

        modify_reg!(adc, self.adc, CFGR, OVRMOD: Overwrite, EXTSEL: TIM1_TRGO2, EXTEN: RisingEdge);

        modify_reg!(adc, self.adc, IER, JEOSIE: Enabled);

    }

    pub fn setup_adc2(&self) {
        modify_reg!(adc, self.adc, CR, DEEPPWD: Disabled);

        modify_reg!(adc, self.adc, CR, ADVREGEN: Enabled);
        while read_reg!(adc, self.adc, CR, ADVREGEN != 1) {} // Wait for the avrgen

        cortex_m::asm::delay(300_000);

        modify_reg!(adc, self.adc, CR, ADCAL: Calibration, ADCALDIF: SingleEnded);
        while read_reg!(adc, self.adc, CR, ADCAL != 0) {} // Wait for the calibration

        write_reg!(adc, self.adc, CFGR, JQDIS: Enabled, DMAEN: Enabled, DMACFG: Circular);
        write_reg!(adc, self.adc, CFGR2, 0);

        // Set sampling time for I_B and VM
        write_reg!(adc, self.adc, SMPR1, SMP7: Cycles2_5, SMP2: Cycles24_5);

        //set  regular sequence to just VM
        write_reg!(adc, self.adc, SQR1, L: 7, SQ1: 1, SQ2: 1, SQ3: 1, SQ4: 1);
        write_reg!(adc, self.adc, SQR2, SQ5: 1, SQ6: 1, SQ7: 1, SQ8: 1);

        // Set current injected channels
        write_reg!(adc, self.adc, JSQR, JL: 0b1, JEXTSEL: TIM1_TRGO, JEXTEN: RisingEdge,
                                        JSQ1: 16, JSQ2: 16);

        modify_reg!(adc, self.adc, CFGR, OVRMOD: Overwrite, EXTSEL: TIM1_TRGO2, EXTEN: RisingEdge);
    }
    
    fn calc_vref(&mut self) {
        const VREFINTCAL_MIN : u16 = 1570;
        const VREFINTCAL_MAX : u16 = 1734;
        const ADC_FAC_CAL_VOL : f32 = 3.0;
        const FLT_MAXCNT : f32 = 4095.0;
        const VREFINT_CAL_DEF : f32 = 1.212;
        const ADC_SMPLS : u16 = 128;

        let temp_smpr2 = read_reg!(adc, self.adc, SMPR2);
        let temp_sqr1 = read_reg!(adc, self.adc, SQR1);
        let temp_cfgr = read_reg!(adc, self.adc, CFGR);
        let temp_cfgr2 = read_reg!(adc, self.adc, CFGR2);
        let temp_ier = read_reg!(adc, self.adc, IER);

        write_reg!(adc, self.adc, SMPR2, 0);
        write_reg!(adc, self.adc, SQR1 , 0);
        write_reg!(adc, self.adc, CFGR , 0);
        write_reg!(adc, self.adc, CFGR2, 0);
        write_reg!(adc, self.adc, IER  , 0);

        let mut vref_int : f32 = 0.0;
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
        write_reg!(adc, self.adc, SMPR2, SMP18: Cycles247_5);
        write_reg!(adc, self.adc, SQR1, SQ1: 18);

        let mut vref_sum : f32 = 0.0;
        for i in 0..ADC_SMPLS {
            modify_reg!(adc, self.adc, CR, ADSTART: Start);
            while read_reg!(adc, self.adc, ISR, EOC != 1) {}
            modify_reg!(adc, self.adc, ISR, EOC: Clear);
            vref_sum = vref_sum + (read_reg!(adc, self.adc, DR) as f32);
        }

        self.vref_cal = vref_int / ( ( vref_sum / f32::from(ADC_SMPLS) ) / FLT_MAXCNT );

        write_reg!(adc, self.adc, SMPR2, temp_smpr2);
        write_reg!(adc, self.adc, SQR1 , temp_sqr1);
        write_reg!(adc, self.adc, CFGR , temp_cfgr);
        write_reg!(adc, self.adc, CFGR2, temp_cfgr2);
        write_reg!(adc, self.adc, IER  , temp_ier);
    }

    pub fn start(&self) {
        self.enable();
        modify_reg!(adc, self.adc, CR, ADSTART: Start, JADSTART: Start);
    }

    pub fn dr(&self) -> u32 {
        &self.adc.DR as *const _ as u32
    }

    fn enable(&self) {
        if read_reg!(adc, self.adc, CR, ADEN == 0) {
            modify_reg!(adc, self.adc, ISR, ADRDY: Clear);
            modify_reg!(adc, self.adc, CR, ADEN: Enable);

            while read_reg!(adc, self.adc, ISR, ADRDY == 0) {}
        }
    }

    // Clear JEOS interrupt flag
    pub fn clear_jeos(&self) {
        modify_reg!(adc, self.adc, ISR, JEOS: Clear);
    }

    pub fn printRegDump(&self) {
        unsafe {
            defmt::println!("ADC1 registers:");
            defmt::println!("ISR    : {:032b}", core::ptr::read_volatile(0x5000_0000 as *const u32));
            defmt::println!("IER    : {:032b}", core::ptr::read_volatile(0x5000_0004 as *const u32));
            defmt::println!("CR     : {:032b}", core::ptr::read_volatile(0x5000_0008 as *const u32));
            defmt::println!("CFGR   : {:032b}", core::ptr::read_volatile(0x5000_000C as *const u32));
            defmt::println!("CFGR2  : {:032b}", core::ptr::read_volatile(0x5000_0010 as *const u32));
            defmt::println!("SMPR1  : {:032b}", core::ptr::read_volatile(0x5000_0014 as *const u32));
            defmt::println!("SMPR2  : {:032b}", core::ptr::read_volatile(0x5000_0018 as *const u32));
            defmt::println!("TR1    : {:032b}", core::ptr::read_volatile(0x5000_0020 as *const u32));
            defmt::println!("TR2    : {:032b}", core::ptr::read_volatile(0x5000_0024 as *const u32));
            defmt::println!("TR3    : {:032b}", core::ptr::read_volatile(0x5000_0028 as *const u32));
            defmt::println!("SQR1   : {:032b}", core::ptr::read_volatile(0x5000_0030 as *const u32));
            defmt::println!("SQR2   : {:032b}", core::ptr::read_volatile(0x5000_0034 as *const u32));
            defmt::println!("SQR3   : {:032b}", core::ptr::read_volatile(0x5000_0038 as *const u32));
            defmt::println!("SQR4   : {:032b}", core::ptr::read_volatile(0x5000_003C as *const u32));
            defmt::println!("DR     : {:032b}", core::ptr::read_volatile(0x5000_0040 as *const u32));
            defmt::println!("JSQR   : {:032b}", core::ptr::read_volatile(0x5000_004C as *const u32));
            defmt::println!("CALFACT: {:032b}", core::ptr::read_volatile(0x5000_00B4 as *const u32));
        }
    }
}
