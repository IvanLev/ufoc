//use stm32ral::{opamp, read_reg, write_reg, modify_reg};
use stm32g4xx_hal::stm32::OPAMP;

pub struct Opamp {
    opamp: OPAMP,
}

impl Opamp {
    pub fn new(opamp: OPAMP) -> Self {
        Self { opamp }
    }

    pub fn init(&self) {
        self.opamp.opamp1_csr.write(|w|
            w.pga_gain().gain8().opaintoen().adcchannel()
                .vm_sel().pga().vp_sel().vinp0().opaen().enabled());
        self.opamp.opamp2_csr.write(|w|
            w.pga_gain().gain8().opaintoen().adcchannel()
                .vm_sel().pga().vp_sel().vinp0().opaen().enabled());
        //write_reg!(opamp, self.opamp, OPAMP1_CSR, OPAINTOEN: ADCChannel,
        //                                          VM_SEL: Output, VP_SEL: VINP0, OPAEN: Enabled);
        //write_reg!(opamp, self.opamp, OPAMP2_CSR, OPAINTOEN: ADCChannel,
        //                                          VM_SEL: Output, VP_SEL: VINP0, OPAEN: Enabled);
    }
}
