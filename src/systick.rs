use crate::{constants, led, utils};
use core::sync::atomic::{AtomicU32, Ordering};

pub fn init() {
    // max = 16_777_215 = 0x00FFFFFF;
    // 2MHz * 2 = 2_000_000 * 2 = 4_000_000 => 2 seconds
    // Set value in load register
    utils::register_write_bits(constants::STK_LOAD_ADDR as *mut u32, 0, 24, 4_000_000);

    // clear current value
    utils::register_write_bits(constants::STK_VAL_ADDR as *mut u32, 0, 32, 0);

    // enable systick
    utils::register_write_bits(constants::STK_CTRL_ADDR as *mut u32, 0, 3, 0b011);
}

pub fn disable() {
    utils::register_write_bits(constants::STK_CTRL_ADDR as *mut u32, 0, 1, 0b0);
}

static SYSTICK_COUNT: AtomicU32 = AtomicU32::new(0);

pub extern "C" fn systick() {
    if SYSTICK_COUNT.load(Ordering::Relaxed) == 10 {
        self::disable();
        SYSTICK_COUNT.store(0, Ordering::Relaxed);
    } else {
        led::toggle_led();
        SYSTICK_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}
