use crate::{constants, led, spi, utils};

pub fn init() {
    // enable GPIO_C clock
    // why?: user button is controlled by PC13 pin which is inside GPIO_C port
    let rcc_ahb1enr = (constants::RCC_BASE + constants::RCC_AHB1ENR_OFFSET) as *mut u32;
    utils::register_set_bit(rcc_ahb1enr, 2);

    // enable SYSCFG clock
    utils::register_set_bit(
        (constants::RCC_BASE + constants::RCC_APB2ENR_OFFSET) as *mut u32,
        14,
    );

    // enable interrupt
    // Position is 40(from reference manual, Table 38<vector table>)
    utils::register_set_bit(constants::NVIC_ISER1_ADDR as *mut u32, 8);

    // GPIO_C's MODER register bit is in input mode by default so no need to change

    // GPIO_C's PUPD register set to PULL DOWN for pin 13
    utils::register_write_bits(
        (constants::GPIO_C_BASE + constants::GPIO_PUPDR_OFFSET) as *mut u32,
        13 * 2,
        2,
        0b10,
    );

    // EXTI_IMR enable
    utils::register_set_bit(
        (constants::EXTI_BASE + constants::EXTI_IMR_OFFSET) as *mut u32,
        13,
    );

    // EXTI_RTSR enable
    utils::register_set_bit(
        (constants::EXTI_BASE + constants::EXTI_RTSR_OFFSET) as *mut u32,
        13,
    );

    // SYSCFG_EXTICR4 to PC13 because PC 13 falls into EXTI4
    utils::register_write_bits(
        (constants::SYSCFG_BASE + constants::SYSCFG_EXTICR4_OFFSET) as *mut u32,
        4,
        4,
        0b0010,
    );
}

pub extern "C" fn exti15_10() {
    led::toggle_led();
    spi::draw_next_shape();

    let addr = (constants::EXTI_BASE + constants::EXTI_PR_OFFSET) as *mut u32;
    utils::register_set_bit(addr, 13);
}
