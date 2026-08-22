use crate::{constants, utils};

pub fn init() {
    // Step1: enable GPIO_A clock
    // why?: LD2 which is the only user-controlled LED on board is controlled by
    // PA5 pin which is inside GPIO_A port.
    utils::register_set_bit(
        (constants::RCC_BASE + constants::RCC_AHB1ENR_OFFSET) as *mut u32,
        0,
    );

    // Step2: set GPIO_A MODER register to output mode for PA5(because CPU would
    // change the pin and make LED on)
    utils::register_write_bits(
        (constants::GPIO_A_BASE + constants::GPIO_MODER_OFFSET) as *mut u32,
        5 * 2,
        2,
        0b01,
    );

    // Step3: OTYPER's reset state is push-pull so no need to change

    // Step4: OSPEEDR's reset state is low speed so no need to change
}

pub fn toggle_led() {
    let is_led_on = self::is_led_on();

    let bsrr_addr = (constants::GPIO_A_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    if is_led_on {
        utils::register_write_val(bsrr_addr, 1u32 << (5 + 16));
    } else {
        utils::register_write_val(bsrr_addr, 1u32 << 5);
    }
}

pub fn is_led_on() -> bool {
    utils::read_register_bit(
        (constants::GPIO_A_BASE + constants::GPIO_ODR_OFFSET) as *mut u32,
        5,
    )
}
