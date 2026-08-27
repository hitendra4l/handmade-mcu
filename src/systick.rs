use crate::{constants, utils};
use core::sync::atomic::{AtomicU32, Ordering};

const HCLK_HZ: u32 = 16_000_000;

/// milliseconds since `init()`, wraps after ~49 days
static MILLIS: AtomicU32 = AtomicU32::new(0);

pub fn init() {
    // CLKSOURCE is set to processor clock(16MHz) below, so 16000 ticks => 1ms.
    // interrupt fires after LOAD+1 ticks, hence the -1
    utils::register_write_bits(
        constants::STK_LOAD_ADDR as *mut u32,
        0,
        24,
        HCLK_HZ / 1000 - 1,
    );

    // clear current value
    utils::register_write_val(constants::STK_VAL_ADDR as *mut u32, 0);

    // Bit[0] : ENABLE => 1
    // Bit[1] : TICKINT => 1 (interrupt on reload)
    // Bit[2] : CLKSOURCE => 1 (processor clock, not HCLK/8)
    utils::register_write_bits(constants::STK_CTRL_ADDR as *mut u32, 0, 3, 0b111);
}

fn now_ms() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

/// blocking delay
///
/// spins on a counter only the systick interrupt advances, so this deadlocks if
/// called with interrupts masked or from a handler at systick's priority or higher
pub fn delay_ms(ms: u32) {
    let start = now_ms();
    // wrapping_sub keeps this correct across the MILLIS rollover
    while now_ms().wrapping_sub(start) < ms {}
}

pub extern "C" fn systick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}
