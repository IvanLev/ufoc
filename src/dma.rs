#![allow(dead_code)]

use stm32g4xx_hal::stm32::{DMAMUX, DMA1};

/// Driver for the DMAMUX peripheral.
pub struct DMAMux {
    dmamux: DMAMUX,
}

impl DMAMux {
    /// Create a new DMAMux1 driver.
    pub fn new(dmamux: DMAMUX) -> Self {
        Self { dmamux }
    }

    /// Configures the requested channel, which must be in 0..15, to mux the
    /// requested DMAREQ ID.
    ///
    /// Does not enable or support synchronisation or event generation.
    pub fn set(&self, channel: u8, id: u8) {
        match channel {
            0 => self.dmamux.c0cr.write(|w| w.dmareq_id().variant(id)),
            1 => self.dmamux.c1cr.write(|w| w.dmareq_id().variant(id)),
            2 => self.dmamux.c2cr.write(|w| w.dmareq_id().variant(id)),
            3 => self.dmamux.c3cr.write(|w| w.dmareq_id().variant(id)),
            4 => self.dmamux.c4cr.write(|w| w.dmareq_id().variant(id)),
            5 => self.dmamux.c5cr.write(|w| w.dmareq_id().variant(id)),
            6 => self.dmamux.c6cr.write(|w| w.dmareq_id().variant(id)),
            7 => self.dmamux.c7cr.write(|w| w.dmareq_id().variant(id)),
            8 => self.dmamux.c8cr.write(|w| w.dmareq_id().variant(id)),
            9 => self.dmamux.c9cr.write(|w| w.dmareq_id().variant(id)),
            10 => self.dmamux.c10cr.write(|w| w.dmareq_id().variant(id)),
            11 => self.dmamux.c11cr.write(|w| w.dmareq_id().variant(id)),
            12 => self.dmamux.c12cr.write(|w| w.dmareq_id().variant(id)),
            13 => self.dmamux.c13cr.write(|w| w.dmareq_id().variant(id)),
            14 => self.dmamux.c14cr.write(|w| w.dmareq_id().variant(id)),
            15 => self.dmamux.c15cr.write(|w| w.dmareq_id().variant(id)),
            _ => panic!("Unknown DMAMUX channel {}", channel),
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
    pub fn new(dma: DMA1) -> Self {
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
    pub dma: DMA1,
    channel: usize,
}

impl DMAChannel {
    /// Create a new DMAChannel for the provided dma instance and channel number.
    ///
    /// # Safety
    /// Must only create one instance per channel.
    pub unsafe fn new(dma: &DMA1, channel: usize) -> DMAChannel {
        // NOTE(unsafe): Make a copy of `dma` which we will only modify
        // NOTE(unsafe): in ways relating exclusively to our channel.
        let dma = core::mem::transmute_copy(dma);
        DMAChannel { dma, channel }
    }

    //pub fn setup_adc_circ(&self, par0: u32) {
    //    let channel = self.channel();
    //    self.dma.ccr1.write(|w| w.en().clear_bit());
    //    while self.dma.ccr1.read().en().bit_is_set() {}
    //    self.dma.ccr1.write(|w| w.msize().variant(1).psize().variant(1)
    //        .minc().set_bit().circ().set_bit().tcie().set_bit());
    //    self.dma.cpar1.write(|w| w.pa().variant(par0));
    //}

    //pub fn start_adc_rx(&self, m0ar0: &mut [u16]) {
    //    self.clear_flags();
    //    let channel = self.channel();
    //    self.dma.cmar1.write(|w| w.ma().variant(m0ar0.as_ptr() as u32));
    //    self.dma.cndtr1.write(|w| w.ndt().variant(m0ar0.len() as u16));
    //    self.dma.ccr1.modify(|_, w| w.en().set_bit());
    //}

    /// Cancel any ongoing DMA transfer.
    //pub fn stop(&self) {
    //    let channel = self.channel();
    //    self.dma.
    //    while self.dma.ccr1.read().en().bit_is_set() {}
    //}

    /// Get the value of the TCIF flag for this channel.
    pub fn tcif(&self) -> bool {
        match self.channel {
            1 => self.dma.isr.read().tcif1().bit_is_set(),
            2 => self.dma.isr.read().tcif2().bit_is_set(),
            3 => self.dma.isr.read().tcif3().bit_is_set(),
            4 => self.dma.isr.read().tcif4().bit_is_set(),
            5 => self.dma.isr.read().tcif5().bit_is_set(),
            6 => self.dma.isr.read().tcif6().bit_is_set(),
            7 => self.dma.isr.read().tcif7().bit_is_set(),
            8 => self.dma.isr.read().tcif8().bit_is_set(),
            _ => false,
        }
    }

    /// Get ISR.
    pub fn flags(&self) -> u32 {
        self.dma.isr.read().bits()
    }

    /// Clear transfer-complete flag for this channel.
    pub fn clear_tcif(&self) {
        match self.channel {
            1 => self.dma.ifcr.write(|w| w.tcif1().set_bit()),
            2 => self.dma.ifcr.write(|w| w.tcif2().set_bit()),
            3 => self.dma.ifcr.write(|w| w.tcif3().set_bit()),
            4 => self.dma.ifcr.write(|w| w.tcif4().set_bit()),
            5 => self.dma.ifcr.write(|w| w.tcif5().set_bit()),
            6 => self.dma.ifcr.write(|w| w.tcif6().set_bit()),
            7 => self.dma.ifcr.write(|w| w.tcif7().set_bit()),
            8 => self.dma.ifcr.write(|w| w.tcif8().set_bit()),
            _ => unreachable!(),
        }
    }

    /// Clear all flags for this channel.
    pub fn clear_flags(&self) {
        match self.channel {
            1 => self.dma.ifcr.write(|w| w.gif1().set_bit().tcif1().set_bit().htif1().set_bit().teif1().set_bit()),
            2 => self.dma.ifcr.write(|w| w.gif2().set_bit().tcif2().set_bit().htif2().set_bit().teif2().set_bit()),
            3 => self.dma.ifcr.write(|w| w.gif3().set_bit().tcif3().set_bit().htif3().set_bit().teif3().set_bit()),
            4 => self.dma.ifcr.write(|w| w.gif4().set_bit().tcif4().set_bit().htif4().set_bit().teif4().set_bit()),
            5 => self.dma.ifcr.write(|w| w.gif5().set_bit().tcif5().set_bit().htif5().set_bit().teif5().set_bit()),
            6 => self.dma.ifcr.write(|w| w.gif6().set_bit().tcif6().set_bit().htif6().set_bit().teif6().set_bit()),
            7 => self.dma.ifcr.write(|w| w.gif7().set_bit().tcif7().set_bit().htif7().set_bit().teif7().set_bit()),
            8 => self.dma.ifcr.write(|w| w.gif8().set_bit().tcif8().set_bit().htif8().set_bit().teif8().set_bit()),
            _ => unreachable!(),
        }
    }

    /// Return a special dma::Instance where the 1st channel register
    /// maps to our specific channel.
    ///
    /// Do not access ISR/IFCR through this instance!
    fn channel(&self) -> &DMA1 {
        let ptr = &*self.dma as *const _ as *const u32;
        unsafe { core::mem::transmute(ptr.offset(5 * (self.channel - 1) as isize)) }
    }
}
