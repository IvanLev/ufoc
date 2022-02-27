use stm32ral::{dma, dmamux};
use stm32ral::{read_reg, write_reg, modify_reg};

/// Driver for the DMAMUX1 peripheral.
pub struct DMAMux1 {
    dmamux1: dmamux1::Instance,
}

impl DMAMux1 {
    /// Create a new DMAMux1 driver.
    pub fn new(dmamux1: dmamux1::Instance) -> Self {
        Self { dmamux1 }
    }

    /// Configures the requested channel, which must be in 0..15, to mux the
    /// requested DMAREQ ID.
    ///
    /// Does not enable or support synchronisation or event generation.
    pub fn set(&self, channel: u32, id: u32) {
        match channel {
            0 => write_reg!(dmamux1, self.dmamux1, C0CR, DMAREQ_ID: id),
            1 => write_reg!(dmamux1, self.dmamux1, C1CR, DMAREQ_ID: id),
            2 => write_reg!(dmamux1, self.dmamux1, C2CR, DMAREQ_ID: id),
            3 => write_reg!(dmamux1, self.dmamux1, C3CR, DMAREQ_ID: id),
            4 => write_reg!(dmamux1, self.dmamux1, C4CR, DMAREQ_ID: id),
            5 => write_reg!(dmamux1, self.dmamux1, C5CR, DMAREQ_ID: id),
            6 => write_reg!(dmamux1, self.dmamux1, C6CR, DMAREQ_ID: id),
            7 => write_reg!(dmamux1, self.dmamux1, C7CR, DMAREQ_ID: id),
            8 => write_reg!(dmamux1, self.dmamux1, C8CR, DMAREQ_ID: id),
            9 => write_reg!(dmamux1, self.dmamux1, C9CR, DMAREQ_ID: id),
            10 => write_reg!(dmamux1, self.dmamux1, C10CR, DMAREQ_ID: id),
            11 => write_reg!(dmamux1, self.dmamux1, C11CR, DMAREQ_ID: id),
            12 => write_reg!(dmamux1, self.dmamux1, C12CR, DMAREQ_ID: id),
            13 => write_reg!(dmamux1, self.dmamux1, C13CR, DMAREQ_ID: id),
            14 => write_reg!(dmamux1, self.dmamux1, C14CR, DMAREQ_ID: id),
            15 => write_reg!(dmamux1, self.dmamux1, C15CR, DMAREQ_ID: id),
            _ => panic!("Unknown DMAMUX1 channel {}", channel),
        }
    }
}

/// Safe construction of all 8 channels in a DMA peripheral.
pub struct DMA {
    pub c1: DMAChannel,
    pub c2: DMAChannel,
    pub c3: DMAChannel,
    pub c4: DMAChannel,
    pub c5: DMAChannel,
    pub c6: DMAChannel,
    pub c7: DMAChannel,
    pub c8: DMAChannel,
}

impl DMA {
    /// Create the set of channels for a DMA peripheral, consuming it in the process.
    pub fn new(dma: dma::Instance) -> Self {
        // NOTE(unsafe): We just have to ensure only one DMAChannel instance
        // NOTE(unsafe): is created for each DMA channel.
        unsafe {
            Self {
                c1: DMAChannel::new(&dma, 1),
                c2: DMAChannel::new(&dma, 2),
                c3: DMAChannel::new(&dma, 3),
                c4: DMAChannel::new(&dma, 4),
                c5: DMAChannel::new(&dma, 5),
                c6: DMAChannel::new(&dma, 6),
                c7: DMAChannel::new(&dma, 7),
                c8: DMAChannel::new(&dma, 8),
            }
        }
    }
}

/// Driver for controlling a DMA stream.
pub struct DMAChannel {
    dma: dma::Instance,
    channel: usize,
}

impl DMAChannel {
    /// Create a new DMAChannel for the provided dma instance and stream number.
    ///
    /// # Safety
    /// Must only create one instance per stream.
    pub unsafe fn new(dma: &dma::Instance, channel: usize) -> DMAChannel {
        // NOTE(unsafe): Make a copy of `dma` which we will only modify
        // NOTE(unsafe): in ways relating exclusively to our stream.
        let dma = core::mem::transmute_copy(dma);
        DMAChannel { dma, channel }
    }

    pub fn setup(&self) {
        //let dma = &self.dma;

        // Configure channel 1 for ADC1
        //write_reg!(dma, dma, CCR1, MSIZE: 1, PSIZE: 1, MINC: 1, CIRC: 1);
        //write_reg!(dma, dma, PAR1, stm32ral::adc::ADC1 as u32 + ADC_DR_OFFSET);

        // Configure channel 2 for ADC2
        //write_reg!(dma, dma, CR2, MEM2MEM: Disabled, PL: Medium, MSIZE: Bits16, PSIZE: Bits16,
        //                          MINC: Enabled, PINC: Disabled, CIRC: Enabled,
        //                          DIR: FromPeripheral, TCIE: Disabled, EN: Disabled);
        //write_reg!(dma, dma, PAR2, stm32ral::adc::ADC2 as u32 + ADC_DR_OFFSET);

        // Configure channel 4 for USART1 TX
        // write_reg!(dma, dma, CR4, MEM2MEM: Disabled, PL: Low, MSIZE: Bits8, PSIZE: Bits8,
        //                          MINC: Enabled, PINC: Disabled, CIRC: Disabled, DIR: FromMemory,
        //                          TEIE: Disabled, HTIE: Disabled, TCIE: Disabled, EN: Disabled);
        //write_reg!(dma, dma, PAR4, stm32ral::usart::USART1 as u32 + USART_TDR_OFFSET);
    }

    pub fn adc1_enable(&self, buf: &mut [u16]) {

    }

    pub fn adc2_enable(&self, buf: &mut [u16]) {

    }

    /// Cancel any ongoing DMA transfer.
    pub fn stop(&self) {
        let channel = self.channel();
        modify_reg!(dma, channel, CCR1, EN: Disabled);
        while read_reg!(dma, channel, CCR1, EN != Disabled) {}
    }

    /// Get the value of the TCIF flag for this stream.
    pub fn tcif(&self) -> bool {
        match self.stream {
            1 => read_reg!(dma, self.dma, ISR, TCIF1 == Complete),
            2 => read_reg!(dma, self.dma, ISR, TCIF2 == Complete),
            3 => read_reg!(dma, self.dma, ISR, TCIF3 == Complete),
            4 => read_reg!(dma, self.dma, ISR, TCIF4 == Complete),
            5 => read_reg!(dma, self.dma, ISR, TCIF5 == Complete),
            6 => read_reg!(dma, self.dma, ISR, TCIF6 == Complete),
            7 => read_reg!(dma, self.dma, ISR, TCIF7 == Complete),
            8 => read_reg!(dma, self.dma, ISR, TCIF8 == Complete),
            _ => false,
        }
    }

    /// Get the value of the TCIF flag for this channel.
    pub fn flags(&self) -> u32 {
        read_reg!(dma, self.dma, ISR)
    }

    /// Clear transfer-complete flag for this channel.
    pub fn clear_tcif(&self) {
        match self.channel {
            1 => write_reg!(dma, self.dma, IFCR, CTCIF1: Clear),
            2 => write_reg!(dma, self.dma, IFCR, CTCIF2: Clear),
            3 => write_reg!(dma, self.dma, IFCR, CTCIF3: Clear),
            4 => write_reg!(dma, self.dma, IFCR, CTCIF4: Clear),
            5 => write_reg!(dma, self.dma, IFCR, CTCIF5: Clear),
            6 => write_reg!(dma, self.dma, IFCR, CTCIF6: Clear),
            7 => write_reg!(dma, self.dma, IFCR, CTCIF7: Clear),
            8 => write_reg!(dma, self.dma, IFCR, CTCIF8: Clear),
            _ => unreachable!(),
        }
    }

    /// Clear all flags for this channel.
    pub fn clear_flags(&self) {
        match self.channel {
            1 => write_reg!(dma, self.dma, IFCR, 0x0000_000F),
            2 => write_reg!(dma, self.dma, IFCR, 0x0000_00F0),
            3 => write_reg!(dma, self.dma, IFCR, 0x0000_0F00),
            4 => write_reg!(dma, self.dma, IFCR, 0x0000_F000),
            5 => write_reg!(dma, self.dma, IFCR, 0x000F_0000),
            6 => write_reg!(dma, self.dma, IFCR, 0x00F0_0000),
            7 => write_reg!(dma, self.dma, IFCR, 0x0F00_0000),
            8 => write_reg!(dma, self.dma, IFCR, 0xF000_0000),
            _ => unreachable!(),
        }
    }

    /// Return a special dma::Instance where the 0th channel register
    /// maps to our specific channel.
    ///
    /// Do not access ISR/IFCR through this instance!
    fn channel(&self) -> dma::Instance {
        let ptr = &*self.dma as *const _ as *const u32;
        unsafe { core::mem::transmute(ptr.offset(4 * self.channel as isize)) }
    }
}
