use crate::{constants, led, utils};
use core::sync::atomic::{AtomicU32, Ordering};

pub fn init() {
    let max = 0x00FFFFFF;
    let systick = max / 8;
    // Set value in load register => 2 seconds
    utils::register_write_bits(constants::STK_LOAD_ADDR as *mut u32, 0, 24, systick * 2);

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
