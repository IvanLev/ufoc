use stm32ral::{dma, dmamux};
use stm32ral::{read_reg, write_reg, modify_reg};

/// Driver for the DMAMUX peripheral.
pub struct DMAMux {
    dmamux: dmamux::Instance,
}

impl DMAMux {
    /// Create a new DMAMux1 driver.
    pub fn new(dmamux: dmamux::Instance) -> Self {
        Self { dmamux }
    }

    /// Configures the requested channel, which must be in 0..15, to mux the
    /// requested DMAREQ ID.
    ///
    /// Does not enable or support synchronisation or event generation.
    pub fn set(&self, channel: u32, id: u32) {
        match channel {
            0 => write_reg!(dmamux, self.dmamux, C0CR, DMAREQ_ID: id),
            1 => write_reg!(dmamux, self.dmamux, C1CR, DMAREQ_ID: id),
            2 => write_reg!(dmamux, self.dmamux, C2CR, DMAREQ_ID: id),
            3 => write_reg!(dmamux, self.dmamux, C3CR, DMAREQ_ID: id),
            4 => write_reg!(dmamux, self.dmamux, C4CR, DMAREQ_ID: id),
            5 => write_reg!(dmamux, self.dmamux, C5CR, DMAREQ_ID: id),
            6 => write_reg!(dmamux, self.dmamux, C6CR, DMAREQ_ID: id),
            7 => write_reg!(dmamux, self.dmamux, C7CR, DMAREQ_ID: id),
            8 => write_reg!(dmamux, self.dmamux, C8CR, DMAREQ_ID: id),
            9 => write_reg!(dmamux, self.dmamux, C9CR, DMAREQ_ID: id),
            10 => write_reg!(dmamux, self.dmamux, C10CR, DMAREQ_ID: id),
            11 => write_reg!(dmamux, self.dmamux, C11CR, DMAREQ_ID: id),
            12 => write_reg!(dmamux, self.dmamux, C12CR, DMAREQ_ID: id),
            13 => write_reg!(dmamux, self.dmamux, C13CR, DMAREQ_ID: id),
            14 => write_reg!(dmamux, self.dmamux, C14CR, DMAREQ_ID: id),
            15 => write_reg!(dmamux, self.dmamux, C15CR, DMAREQ_ID: id),
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

/// Driver for controlling a DMA channel.
pub struct DMAChannel {
    pub dma: dma::Instance,
    channel: usize,
}

impl DMAChannel {
    /// Create a new DMAChannel for the provided dma instance and channel number.
    ///
    /// # Safety
    /// Must only create one instance per channel.
    pub unsafe fn new(dma: &dma::Instance, channel: usize) -> DMAChannel {
        // NOTE(unsafe): Make a copy of `dma` which we will only modify
        // NOTE(unsafe): in ways relating exclusively to our channel.
        let dma = core::mem::transmute_copy(dma);
        DMAChannel { dma, channel }
    }

    pub fn setup_adc_circ(&self, par0: u32) {
        let channel = self.channel();
        write_reg!(dma, channel, CCR1, EN: 0);
        while read_reg!(dma, channel, CCR1, EN != 0) {}
        write_reg!(dma, channel, CCR1, MSIZE: 1, PSIZE: 1, MINC: 1, CIRC: 1, TCIE: 1);
        unsafe { write_reg!(dma, channel, CPAR1, par0); }
    }

    pub fn start_adc_rx(&self, m0ar0: &mut [u16]) {
        self.clear_flags();
        let channel = self.channel();
        unsafe { write_reg!(dma, channel, CMAR1, m0ar0.as_ptr() as u32); }
        write_reg!(dma, channel, CNDTR1, m0ar0.len() as u32);
        modify_reg!(dma, channel, CCR1, EN: 1);
    }

    /// Cancel any ongoing DMA transfer.
    pub fn stop(&self) {
        let channel = self.channel();
        modify_reg!(dma, channel, CCR1, EN: 0);
        while read_reg!(dma, channel, CCR1, EN != 0) {}
    }

    /// Get the value of the TCIF flag for this channel.
    pub fn tcif(&self) -> bool {
        match self.channel {
            1 => read_reg!(dma, self.dma, ISR, TCIF1 == 1),
            2 => read_reg!(dma, self.dma, ISR, TCIF2 == 1),
            3 => read_reg!(dma, self.dma, ISR, TCIF3 == 1),
            4 => read_reg!(dma, self.dma, ISR, TCIF4 == 1),
            5 => read_reg!(dma, self.dma, ISR, TCIF5 == 1),
            6 => read_reg!(dma, self.dma, ISR, TCIF6 == 1),
            7 => read_reg!(dma, self.dma, ISR, TCIF7 == 1),
            8 => read_reg!(dma, self.dma, ISR, TCIF8 == 1),
            _ => false,
        }
    }

    /// Get ISR.
    pub fn flags(&self) -> u32 {
        read_reg!(dma, self.dma, ISR)
    }

    /// Clear transfer-complete flag for this channel.
    pub fn clear_tcif(&self) {
        match self.channel {
            1 => write_reg!(dma, self.dma, IFCR, TCIF1: 1),
            2 => write_reg!(dma, self.dma, IFCR, TCIF2: 1),
            3 => write_reg!(dma, self.dma, IFCR, TCIF3: 1),
            4 => write_reg!(dma, self.dma, IFCR, TCIF4: 1),
            5 => write_reg!(dma, self.dma, IFCR, TCIF5: 1),
            6 => write_reg!(dma, self.dma, IFCR, TCIF6: 1),
            7 => write_reg!(dma, self.dma, IFCR, TCIF7: 1),
            8 => write_reg!(dma, self.dma, IFCR, TCIF8: 1),
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

    /// Return a special dma::Instance where the 1st channel register
    /// maps to our specific channel.
    ///
    /// Do not access ISR/IFCR through this instance!
    fn channel(&self) -> dma::Instance {
        let ptr = &*self.dma as *const _ as *const u32;
        unsafe { core::mem::transmute(ptr.offset(5 * (self.channel - 1) as isize)) }
    }
}
