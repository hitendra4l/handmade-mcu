use crate::{constants, utils};

pub const SYSCLK: u32 = 180_000_000;

pub fn configure_sysclk() {
    // enable APB1 to use PWR peripheral registers
    utils::register_set_bit(
        (constants::RCC_BASE + constants::RCC_APB1ENR_OFFSET) as *mut u32,
        28,
    );

    // configure PWR_CR register
    let pwr_cr_addr = (constants::PWR_BASE + constants::PWR_CR_OFFSET) as *mut u32;
    let pwr_csr_addr = (constants::PWR_BASE + constants::PWR_CSR_OFFSET) as *mut u32;

    // Bits[15:14] - (VOS) Voltage scaling output selection - Scale 1
    utils::register_write_bits(pwr_cr_addr, 14, 2, 0b11);

    // Bit[16] - (ODEN) Enable Over-Drive
    utils::register_set_bit(pwr_cr_addr, 16);
    // Bits[16] - (ODRDY) Over-Drive mode ready
    while !utils::read_register_bit(pwr_csr_addr, 16) {}

    // Bits[17] - (ODSWEN) Over-Drive-Switching enabled
    utils::register_set_bit(pwr_cr_addr, 17);
    // Bits[17] - (ODSWRDY) Over-Drive-Switching ready
    while !utils::read_register_bit(pwr_csr_addr, 17) {}

    // configure flash-interface register
    let flash_interface_acr_addr =
        (constants::FLASH_INTERFACE_BASE + constants::FLASH_INTERFACE_ACR_OFFSET) as *mut u32;
    // Bits[3:0] - (LATENCY) cpu wait states
    // 5WS(6 CPU cycles) + Voltage range(2.7V to 3.6V) => 150 < HCLK <= 180
    utils::register_write_bits(flash_interface_acr_addr, 0, 4, 5);

    // Bits[8] - (PRFTEN) prefetch enabled
    // Bits[9] - (ICEN) instruction cache enabled
    // Bits[10] - (DCEN) data cache enabled
    utils::register_write_bits(flash_interface_acr_addr, 8, 3, 0b111);

    //--------------------------------------------------------------------------
    //--------------------------------------------------------------------------

    let rcc_cr_addr = (constants::RCC_BASE + constants::RCC_CR_OFFSET) as *mut u32;

    // make sure HSI is "ON"
    utils::register_set_bit(rcc_cr_addr, 0);
    while !utils::read_register_bit(rcc_cr_addr, 1) {}

    let rcc_pllcfgr_addr = (constants::RCC_BASE + constants::RCC_PLLCFGR_OFFSET) as *mut u32;
    // Bits[5:0] - (PLLM) VCO input. 16/8 = 2MHz
    utils::register_write_bits(rcc_pllcfgr_addr, 0, 6, 8);

    // Bits[14:6] - (PLLN) VCO output. 2*180 = 360MHz
    utils::register_write_bits(rcc_pllcfgr_addr, 6, 9, 180);

    // Bits[17:16] - (PLLP) PLL output. 360/2 = 180MHz
    utils::register_write_bits(rcc_pllcfgr_addr, 16, 2, 0b00);

    // Bits[22] - (PLLSRC) input clock for VCO, HSI 16MHz
    utils::register_write_bits(rcc_pllcfgr_addr, 22, 1, 0b0);

    let rcc_cfgr_addr = (constants::RCC_BASE + constants::RCC_CFGR_OFFSET) as *mut u32;
    // Bits[7:4] - (HPRE) AHB prescaler - not divided, same as
    //              system clock, 180MHz
    utils::register_write_bits(rcc_cfgr_addr, 4, 4, 0b0000);

    // Bits[12:10] - (PPRE1) APB1 prescaler - 180/4 = 45MHz(max)
    utils::register_write_bits(rcc_cfgr_addr, 10, 3, 0b101);

    // Bits[15:13] - (PPRE2) APB2 prescaler - 180/2 = 90MHz(max)
    utils::register_write_bits(rcc_cfgr_addr, 13, 3, 0b100);

    // Turn on PLL
    utils::register_set_bit(rcc_cr_addr, 24);
    while !utils::read_register_bit(rcc_cr_addr, 25) {}

    // Bits[14] - (VOSRDY) regulator voltage scaling output selection ready.
    // checked here rather than next to the vos write because the flag stays low
    // until the pll is locked.
    // the regulator has to be settled before the switch below puts the core on
    // the 180MHz clock.
    while !utils::read_register_bit(pwr_csr_addr, 14) {}

    // Bits[1:0] - select PLL_P as System Clock
    utils::register_write_bits(rcc_cfgr_addr, 0, 2, 0b10);
    loop {
        let val1 = utils::read_register_bit(rcc_cfgr_addr, 2);
        let val2 = utils::read_register_bit(rcc_cfgr_addr, 3);
        if !val1 && val2 {
            break;
        }
    }
}
