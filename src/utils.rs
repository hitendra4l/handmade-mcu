use core::ptr::{read_volatile, write_volatile};

pub fn register_write_val(addr: *mut u32, val: u32) {
    unsafe {
        write_volatile(addr, val);
    }
}

/// `bit_position` => 0 to 31 inclusive
/// does read-modify-write
pub fn register_set_bit(addr: *mut u32, bit_position: u8) {
    let mask = 1u32 << bit_position;
    let val = read_register(addr);
    unsafe { write_volatile(addr, val | mask) };
}

/// this function does read-modify-write
pub fn register_write_bits(addr: *mut u32, offset: u8, width: u8, value: u32) {
    debug_assert!(width > 0 && width <= 32 && offset + width <= 32);

    let mask = if width == 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };

    let mut reg = unsafe { read_volatile(addr) };

    reg &= !(mask << offset); // Clear field
    reg |= (value & mask) << offset; // Write new value

    register_write_val(addr, reg);
}

pub fn read_register(addr: *mut u32) -> u32 {
    unsafe { read_volatile(addr) }
}

pub fn read_register_bit(addr: *mut u32, bit_position: u8) -> bool {
    let val = unsafe { read_volatile(addr) };
    (val & (1 << bit_position)) != 0
}

//-----------------------
// 16bit register helpers
//-----------------------

pub fn register_write_val_u16(addr: *mut u16, val: u16) {
    unsafe {
        write_volatile(addr, val);
    }
}

/// read-modify-write on a 16-bit register
pub fn register_write_bits_u16(addr: *mut u16, offset: u8, width: u8, value: u16) {
    debug_assert!(width > 0 && width <= 16 && offset + width <= 16);

    let mask = if width == 16 {
        u16::MAX
    } else {
        (1u16 << width) - 1
    };

    let mut reg = unsafe { read_volatile(addr) };

    reg &= !(mask << offset); // Clear field
    reg |= (value & mask) << offset; // Write new value

    register_write_val_u16(addr, reg);
}

// pub fn read_register_u16(addr: *mut u16) -> u16 {
//     unsafe { read_volatile(addr) }
// }

pub fn read_register_bit_u16(addr: *mut u16, bit_position: u8) -> bool {
    let val = unsafe { read_volatile(addr) };
    (val & (1 << bit_position)) != 0
}

//-----------------------
// 8bit register helpers
//-----------------------

pub fn register_write_val_u8(addr: *mut u8, val: u8) {
    unsafe {
        write_volatile(addr, val);
    }
}
