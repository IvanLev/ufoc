use stm32ral::{spi, read_reg, write_reg, modify_reg};
use crate::gpio;

pub struct TLE5012 {
    spi: spi::Instance,
    enc_nss: gpio::OutputPin,
}

impl TLE5012 {
    pub fn new(spi: spi::Instance, enc_nss: gpio::OutputPin) -> Self { Self { spi , enc_nss } }

    pub fn init(&self) {
        self.enc_nss.set_high();
        write_reg!(spi, self.spi, CR1, BR: 0b010, CPOL: 0, CPHA: 1, BIDIOE: 1, BIDIMODE: 1, SSI:1, SSM:1, MSTR: 1);
        write_reg!(spi, self.spi, CR2, DS: 15, FRF: 0);
    }

    pub fn read_angle(&self) -> u16 {
        modify_reg!(spi, self.spi, CR1, BIDIOE: 1);
        modify_reg!(spi, self.spi, CR1, SPE: 1);
        self.enc_nss.set_low();
        write_reg!(spi, self.spi, DR, 0b1000000000100001);
        while read_reg!(spi, self.spi, SR, BSY == 1) {}
        modify_reg!(spi, self.spi, CR1, SPE: 0);

        modify_reg!(spi, self.spi, CR1, BIDIOE: 0);
        while read_reg!(spi, self.spi, SR, RXNE == 1) {
            let _dump = read_reg!(spi, self.spi, DR);
        }
        modify_reg!(spi, self.spi, CR1, SPE: 1);
        while read_reg!(spi, self.spi, SR, RXNE == 0) {}
        let angle = read_reg!(spi, self.spi, DR) as u16 & 0x7FFF;
        modify_reg!(spi, self.spi, CR1, SPE: 0);
        while read_reg!(spi, self.spi, SR, RXNE == 0) {}
        let safety = read_reg!(spi, self.spi, DR) as u16 & 0x7FFF;
        self.enc_nss.set_high();
        angle
    }

}