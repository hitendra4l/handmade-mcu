use crate::{
    constants, systick,
    utils::{self, register_write_val},
};

const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;

#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
enum LcdCommand {
    SWRESET = 0x01,
    SLEEPOUT = 0x11,
    DISPLAYON = 0x29,
    PIXSET = 0x3A,
    CASET = 0x2A,
    PASET = 0x2B,
    RAMWR = 0x2C,
    MADCTL = 0x36,
}

enum PinStatus {
    High,
    Low,
}

#[repr(u8)]
enum SpiFlag {
    Txe = 1,
    Bsy = 7,
}

pub fn init() {
    enable_apb2_spi1_peripheral();

    // Bit[0] : CPHA => 0
    // Bit[1] : CPOL => 0
    // Bit[2] : MSTR => 1 (board is in master mode)
    // Bit[5:3] : BR => 000 => (f_PCKL/1 => 8MHz)
    //     RCC_CFGR register is in it's reset state which means
    //     System Clock is HSI => 16MHz
    //     AHB Prescaler => 1
    //     APB2 Prescaler => 1
    //     f_PCLK => 16MHz
    // Bit[6] : SPE => 0 (SPI disabled, enable after setting config and AF)
    // Bit[7] : LSBFIRST => 0 (MSB First)
    // Bit[8] : SSI => 1 (NSS High)
    // Bit[9] : SSM => 1 (SSM is enabled because CS is another GPIO)
    // Bit[10] : RXONLY => 0 (half duplex, don't care)
    // Bit[11] : DFF => 0 (8-bit data frame format)
    // Bit[12] : CRCNEXT => 0 (not using CRC)
    // Bit[13] : CRCEN => 0 (not using CRC)
    // Bit[14] : BIDIOE => 1 (transmit-only mode)
    // Bit[15] : BIDIMODE => 1 (1 line bi-directional data mode)
    let spi_cr1_addr = (constants::SPI1_BASE_ADDR + constants::SPI_CR1_OFFSET) as *mut u16;
    utils::register_write_val_u16(spi_cr1_addr, 0b1100_0011_0000_0100);

    // Alternate Functions:
    //     PB3 => SPI1_SCK
    //     PA7 => SPI1_MOSI
    enable_ahb1_gpio_peripheral();
    configure_af_gpio_pins();

    // PB0 => CS
    // PB1 => DC
    // PB2 => RESET
    configure_output_gpio_pins();

    enable_spi();

    init_display();
}

fn enable_spi() {
    let spi_cr1_addr = (constants::SPI1_BASE_ADDR + constants::SPI_CR1_OFFSET) as *mut u16;
    utils::register_write_bits_u16(spi_cr1_addr, 6, 1, 1);
}

fn enable_apb2_spi1_peripheral() {
    let rcc_abp2_enr = (constants::RCC_BASE + constants::RCC_APB2ENR_OFFSET) as *mut u32;
    utils::register_set_bit(rcc_abp2_enr, 12);
}

fn enable_ahb1_gpio_peripheral() {
    let rcc_ahb1_enr = (constants::RCC_BASE + constants::RCC_AHB1ENR_OFFSET) as *mut u32;
    utils::register_set_bit(rcc_ahb1_enr, 0); // GPIO A
    utils::register_set_bit(rcc_ahb1_enr, 1); // GPIO B
}

fn configure_af_gpio_pins() {
    // PB3 => SPI1_SCK => AF5(alternate function)
    // Already in Push-Pull state
    utils::register_write_bits(
        (constants::GPIO_B_BASE + constants::GPIO_MODER_OFFSET) as *mut u32,
        6,
        2,
        0b10,
    );
    utils::register_write_bits(
        (constants::GPIO_B_BASE + constants::GPIO_OSPEEDER_OFFSET) as *mut u32,
        6,
        2,
        0b11,
    );
    utils::register_write_bits(
        (constants::GPIO_B_BASE + constants::GPIO_AFRL_OFFSET) as *mut u32,
        12,
        4,
        0b0101,
    );

    // PA7 => SPI1_MOSI => AF5(alternate function)
    // Already in Push-Pull state
    utils::register_write_bits(
        (constants::GPIO_A_BASE + constants::GPIO_MODER_OFFSET) as *mut u32,
        14,
        2,
        0b10,
    );
    utils::register_write_bits(
        (constants::GPIO_A_BASE + constants::GPIO_OSPEEDER_OFFSET) as *mut u32,
        14,
        2,
        0b11,
    );
    utils::register_write_bits(
        (constants::GPIO_A_BASE + constants::GPIO_AFRL_OFFSET) as *mut u32,
        28,
        4,
        0b0101,
    );
}

fn configure_output_gpio_pins() {
    // PB0 => CS
    // PB1 => DC
    // PB2 => RESET
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;
    utils::register_write_val(bsrr_address, 0b111);

    for pin in [0, 1, 2] {
        utils::register_write_bits(
            (constants::GPIO_B_BASE + constants::GPIO_MODER_OFFSET) as *mut u32,
            2 * pin,
            2,
            0b01,
        );

        utils::register_write_bits(
            (constants::GPIO_B_BASE + constants::GPIO_OSPEEDER_OFFSET) as *mut u32,
            2 * pin,
            2,
            0b11,
        );
    }
}

fn set_lcd_dc(val: PinStatus) {
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    match val {
        PinStatus::High => register_write_val(bsrr_address, 1u32 << 1),
        PinStatus::Low => register_write_val(bsrr_address, 1u32 << (1 + 16)),
    }
}

fn set_lcd_cs(val: PinStatus) {
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    match val {
        PinStatus::High => register_write_val(bsrr_address, 1u32 << 0),
        PinStatus::Low => register_write_val(bsrr_address, 1u32 << 16),
    }
}

fn set_lcd_reset(val: PinStatus) {
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    match val {
        PinStatus::High => register_write_val(bsrr_address, 1u32 << 2),
        PinStatus::Low => register_write_val(bsrr_address, 1u32 << (16 + 2)),
    }
}

fn get_spi_flag(flag: SpiFlag) -> bool {
    let spi_sr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_SR_OFFSET) as *mut u16;
    utils::read_register_bit_u16(spi_sr_addr, flag as u8)
}

fn init_display() {
    lcd_reset();

    lcd_write_command(LcdCommand::SWRESET, &[]);

    lcd_write_command(LcdCommand::SLEEPOUT, &[]);

    // 16bit/pixel RGB565
    lcd_write_command(LcdCommand::PIXSET, &[0x55]);

    // configure for orientation and rotation
    lcd_write_command(LcdCommand::MADCTL, &[0b0000_0000]);

    lcd_write_command(LcdCommand::DISPLAYON, &[]);

    // clear_display();
    draw_on_display();
}

fn lcd_reset() {
    set_lcd_reset(PinStatus::High);
    systick::delay_ms(1);

    // datasheet wants RESET held low for atleast 10us
    set_lcd_reset(PinStatus::Low);
    systick::delay_ms(1);

    // atleast 120ms before the first command can be sent
    set_lcd_reset(PinStatus::High);
    systick::delay_ms(120);
}

fn lcd_write_command(cmd: LcdCommand, params: &[u8]) {
    set_lcd_cs(PinStatus::Low);
    set_lcd_dc(PinStatus::Low);

    spi_wait_available();

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    utils::register_write_val_u8(spi_dr_addr, cmd as u8);

    spi_wait_available();

    set_lcd_dc(PinStatus::High);

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    for &data in params {
        while !get_spi_flag(SpiFlag::Txe) {}
        utils::register_write_val_u8(spi_dr_addr, data);
    }

    spi_wait_available();

    set_lcd_cs(PinStatus::High);
}

fn spi_wait_available() {
    while !get_spi_flag(SpiFlag::Txe) {}
    while get_spi_flag(SpiFlag::Bsy) {}
}

fn draw_on_display() {
    // set address window
    let x_end = LCD_WIDTH - 1;
    let y_end = LCD_HEIGHT - 1;
    lcd_write_command(LcdCommand::CASET, &[0, 0, (x_end >> 8) as u8, x_end as u8]);
    lcd_write_command(LcdCommand::PASET, &[0, 0, (y_end >> 8) as u8, y_end as u8]);

    // write to the display
    let high_byte = 0b1111_1000;
    let low_byte = 0b0000_0000;

    set_lcd_cs(PinStatus::Low);
    set_lcd_dc(PinStatus::Low);

    spi_wait_available();

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    utils::register_write_val_u8(spi_dr_addr, LcdCommand::RAMWR as u8);

    spi_wait_available();

    set_lcd_dc(PinStatus::High);

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    for j in 0..LCD_HEIGHT as i32 {
        for i in 0..LCD_WIDTH as i32 {
            while !get_spi_flag(SpiFlag::Txe) {}
            let r = (i - 100i32) * (i - 100i32) + j * j;
            // let r = i * i + j * j;

            utils::register_write_val_u8(spi_dr_addr, if r < 5000 { high_byte } else { 0u8 });
            while !get_spi_flag(SpiFlag::Txe) {}
            utils::register_write_val_u8(spi_dr_addr, if r < 5000 { low_byte } else { 0u8 });
        }
    }

    spi_wait_available();
    set_lcd_cs(PinStatus::High);
}

// fn clear_display() {
//     // set address window
//     let x_end = LCD_WIDTH - 1;
//     let y_end = LCD_HEIGHT - 1;
//     lcd_write_command(LcdCommand::CASET, &[0, 0, (x_end >> 8) as u8, x_end as u8]);
//     lcd_write_command(LcdCommand::PASET, &[0, 0, (y_end >> 8) as u8, y_end as u8]);
//
//     set_lcd_cs(PinStatus::Low);
//     set_lcd_dc(PinStatus::Low);
//
//     spi_wait_available();
//
//     let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
//     utils::register_write_val_u8(spi_dr_addr, LcdCommand::RAMWR as u8);
//
//     spi_wait_available();
//
//     set_lcd_dc(PinStatus::High);
//
//     let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
//     for _ in 0..LCD_HEIGHT as i32 {
//         for _ in 0..LCD_WIDTH as i32 {
//             utils::register_write_val_u8(spi_dr_addr, 0u8);
//             while !get_spi_flag(SpiFlag::Txe) {}
//             utils::register_write_val_u8(spi_dr_addr, 0u8);
//         }
//     }
//
//     spi_wait_available();
//     set_lcd_cs(PinStatus::High);
// }
