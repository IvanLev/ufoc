use stm32ral::{rcc, pwr, flash, read_reg, write_reg, modify_reg};

/// Frequencies for each clock in the system, in Hz.
#[derive(Copy, Clone, Debug)]
pub struct Clocks {
    pub sys_ck: u32,
    pub ahb_ck: u32,
    pub apb_ck: u32,
    pub tim_ck: u32,
}

pub fn setup(rcc: rcc::Instance, pwr: pwr::Instance, flash: flash::Instance) -> Clocks {
    modify_reg!(rcc, rcc, APB1ENR1, PWREN: 1);// Enable access to power control interface
    modify_reg!(pwr, pwr, CR3, UCPD1_DBDIS: 1);// Disable USB-PD dead battery pull-downs
    //Check voltage scaling range. Skip change if already in range 1
    if read_reg!(pwr, pwr, CR1, VOS != 0b01) {
        modify_reg!(pwr, pwr, CR1, VOS: 0b01);//Change to voltage scaling range 1
    }
    while read_reg!(pwr, pwr, SR2, VOSF != 0) {}//Wait for change completion

    modify_reg!(rcc, rcc, AHB1ENR, FLASHEN: 1);// Enable access to flash memory interface
    modify_reg!(flash, flash, ACR, DCEN: 1, ICEN:1, PRFTEN: 1,  LATENCY: 8);
    while read_reg!(flash, flash, ACR, LATENCY != 8) {}//Wait for latency to change

    //Configuration of regulator into boost mode for 170Mhz operation
    //To change mode AHB prescaler must be set to div 2 for 1us
    if read_reg!(pwr, pwr, CR5, R1MODE != 0) {
        modify_reg!(rcc, rcc, CFGR, HPRE: 0b1000);
        modify_reg!(pwr, pwr, CR5, R1MODE: 0);
    }

    //Configure PLL for 170MHz
    modify_reg!(rcc, rcc, CR, PLLON: Off); // Turn off PLL
    while read_reg!(rcc, rcc, CR, PLLRDY != 0) {} // Wait for PLL to turn off
    write_reg!(rcc, rcc, PLLCFGR, PLLSRC: HSI16,
                                  PLLM: Div4,
                                  PLLN: Div85,
                                  PLLR: Div2,
                                  PLLREN: 1,
                                  PLLPEN: 1,
                                  PLLPDIV: Div4);
    modify_reg!(rcc, rcc, CR, PLLON: On); // Turn on PLL
    while read_reg!(rcc, rcc, CR, PLLRDY != 1) {} // Wait for PLL to turn on
    modify_reg!(rcc, rcc, CFGR, SW: PLL); // Switch to PLL as system clock
    while read_reg!(rcc, rcc, CFGR, SWS != 0b11) {} // Wait for the switch to take effect




    //Set HCLK, APB1 and APB2 dividers
    modify_reg!(rcc, rcc, CFGR, HPRE: Div1, PPRE1: Div1, PPRE2: Div1);

    //Enable AHB1 peripherals: Cordic, DMA1, DMAMUX
    modify_reg!(rcc, rcc, AHB1ENR, CORDICEN: Enabled, DMA1EN: Enabled, DMAMUXEN: Enabled);

    //Enable APB2 peripherals: TIM1
    modify_reg!(rcc, rcc, APB2ENR, SYSCFGEN: Enabled, TIM1EN: Enabled, SPI1EN: Enabled);

    //Enable AHB2 peripherals: ADC1, ADC2, GPIOA, GPIOC
    modify_reg!(rcc, rcc, AHB2ENR, ADC12EN: Enabled, GPIOAEN: Enabled, GPIOBEN: Enabled,
                                   GPIOFEN: Enabled, GPIOGEN: Enabled);

    //Select ADC Clock source
    modify_reg!(rcc, rcc, CCIPR, ADC12SEL: PLLP);



    // Return generated clock frequencies for easy reference elsewhere.
    Clocks {
        sys_ck: 170_000_000,
        ahb_ck: 170_000_000,
        apb_ck: 170_000_000,
        tim_ck: 170_000_000,
    }
}