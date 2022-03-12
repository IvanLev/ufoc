use stm32ral::{opamp, read_reg, write_reg, modify_reg};

pub struct Opamp {
    opamp: opamp::Instance,
}

impl Opamp {
    pub fn new(opamp: opamp::Instance) -> Self {
        Self { opamp }
    }

    pub fn init(&self) {
        //write_reg!(opamp, self.opamp, OPAMP1_CSR, PGA_GAIN: Gain8, OPAINTOEN: ADCChannel,
        //                                          VM_SEL: PGA, VP_SEL: VINP0, OPAEN: Enabled);
        //write_reg!(opamp, self.opamp, OPAMP2_CSR, PGA_GAIN: Gain8, OPAINTOEN: ADCChannel,
        //                                          VM_SEL: PGA, VP_SEL: VINP0, OPAEN: Enabled);
        write_reg!(opamp, self.opamp, OPAMP1_CSR, OPAINTOEN: ADCChannel,
                                                  VM_SEL: Output, VP_SEL: VINP0, OPAEN: Enabled);
        write_reg!(opamp, self.opamp, OPAMP2_CSR, OPAINTOEN: ADCChannel,
                                                  VM_SEL: Output, VP_SEL: VINP0, OPAEN: Enabled);
    }
}
