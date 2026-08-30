use crate::{constants, image_data, systick, utils};
use core::{
    ptr::addr_of_mut,
    sync::atomic::{AtomicU8, Ordering},
};

const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;

const DMA_ONE_TIME_ROWS: u32 = 20;
const DMA_ONE_TIME_DATA: u32 = 2 * 240 * DMA_ONE_TIME_ROWS;

static mut SPI_DMA_BUFFER: [u8; DMA_ONE_TIME_DATA as usize] = [0u8; DMA_ONE_TIME_DATA as usize];

#[repr(u8)]
enum DrawItem {
    Circle = 0,
    Image = 1,
    Rectangle = 2,
}
static CURRENT_SHAPE: AtomicU8 = AtomicU8::new(0);

impl DrawItem {
    fn set_shape(shape: DrawItem) {
        CURRENT_SHAPE.store(shape as u8, Ordering::Relaxed);
    }

    fn get_shape() -> Self {
        match CURRENT_SHAPE.load(Ordering::Relaxed) {
            0 => DrawItem::Circle,
            1 => DrawItem::Image,
            2 => DrawItem::Rectangle,
            _ => unreachable!("invalid DrawItem"),
        }
    }
}

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
    configure_dma2();

    // Bit[0] : CPHA => 0
    // Bit[1] : CPOL => 0
    // Bit[2] : MSTR => 1 (board is in master mode)
    // Bit[5:3] : BR => 000 => (f_PCLK/4 => 45MHz)
    //     clock::configure_sysclk() has already run by this point, so:
    //     System Clock is PLL_P => 180MHz
    //     AHB Prescaler => 1
    //     APB2 Prescaler => 2
    //     f_PCLK => 90MHz
    //     45MHz is out of spec's(10MHz) but works with short jumper wires
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

    // Bit[1] : (TXDMAEN) Tx buffer DMA enable
    let spi_cr2_addr = (constants::SPI1_BASE_ADDR + constants::SPI_CR2_OFFSET) as *mut u16;
    utils::register_set_bit_u16(spi_cr2_addr, 1);

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
        PinStatus::High => utils::register_write_val(bsrr_address, 1u32 << 1),
        PinStatus::Low => utils::register_write_val(bsrr_address, 1u32 << (1 + 16)),
    }
}

fn set_lcd_cs(val: PinStatus) {
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    match val {
        PinStatus::High => utils::register_write_val(bsrr_address, 1u32 << 0),
        PinStatus::Low => utils::register_write_val(bsrr_address, 1u32 << 16),
    }
}

fn set_lcd_reset(val: PinStatus) {
    let bsrr_address = (constants::GPIO_B_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    match val {
        PinStatus::High => utils::register_write_val(bsrr_address, 1u32 << 2),
        PinStatus::Low => utils::register_write_val(bsrr_address, 1u32 << (16 + 2)),
    }
}

fn get_spi_flag(flag: SpiFlag) -> bool {
    let spi_sr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_SR_OFFSET) as *mut u16;
    utils::read_register_bit_u16(spi_sr_addr, flag as u8)
}

fn init_display() {
    lcd_reset();

    lcd_write_command(LcdCommand::SWRESET, &[]);
    // datasheet wants 5ms after a software reset, 120ms if the panel
    // was in sleep-out mode when the reset was issued
    systick::delay_ms(120);

    lcd_write_command(LcdCommand::SLEEPOUT, &[]);
    // datasheet wants 120ms after sleep out before the next command
    systick::delay_ms(120);

    // 16bit/pixel RGB565
    lcd_write_command(LcdCommand::PIXSET, &[0x55]);

    // configure for orientation and rotation
    lcd_write_command(LcdCommand::MADCTL, &[0b0000_0000]);

    lcd_write_command(LcdCommand::DISPLAYON, &[]);

    draw(DrawItem::get_shape());
}

fn lcd_reset() {
    set_lcd_reset(PinStatus::High);
    systick::delay_ms(2);

    // datasheet wants RESET held low for atleast 10us
    set_lcd_reset(PinStatus::Low);
    systick::delay_ms(2);

    // atleast 120ms before the first command can be sent
    set_lcd_reset(PinStatus::High);
    systick::delay_ms(120);
}

fn lcd_write_command(cmd: LcdCommand, params: &[u8]) {
    set_lcd_cs(PinStatus::Low);
    set_lcd_dc(PinStatus::Low);

    spi_wait_txe();

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    utils::register_write_val_u8(spi_dr_addr, cmd as u8);

    spi_wait_idle();

    set_lcd_dc(PinStatus::High);

    if !params.is_empty() {
        for &data in params {
            spi_wait_txe();
            utils::register_write_val_u8(spi_dr_addr, data);
        }

        spi_wait_idle();
    }

    set_lcd_cs(PinStatus::High);
}

/// Wait until the TX buffer can accept another byte.
/// This is all that is needed before writing to DR.
fn spi_wait_txe() {
    while !get_spi_flag(SpiFlag::Txe) {}
}

/// Wait until the shift register has drained.
/// Required before toggling CS or DC, otherwise the pin moves
/// while a byte is still on the wire.
fn spi_wait_idle() {
    spi_wait_txe();
    while get_spi_flag(SpiFlag::Bsy) {}
}

fn set_address_window(x_start: u16, x_end: u16, y_start: u16, y_end: u16) {
    lcd_write_command(
        LcdCommand::CASET,
        &[
            (x_start >> 8) as u8,
            x_start as u8,
            (x_end >> 8) as u8,
            x_end as u8,
        ],
    );
    lcd_write_command(
        LcdCommand::PASET,
        &[
            (y_start >> 8) as u8,
            y_start as u8,
            (y_end >> 8) as u8,
            y_end as u8,
        ],
    );
}

fn configure_dma2() {
    let rcc_ahb1enr_addr = (constants::RCC_BASE + constants::RCC_AHB1ENR_OFFSET) as *mut u32;
    let dma2_s3cr_addr = (constants::DMA2_BASE_ADDR + constants::DMA_S3CR_OFFSET) as *mut u32;
    let dma2_s3ndtr_addr = (constants::DMA2_BASE_ADDR + constants::DMA_S3NDTR_OFFSET) as *mut u32;
    let dma2_s3par_addr = (constants::DMA2_BASE_ADDR + constants::DMA_S3PAR_OFFSET) as *mut u32;

    utils::register_set_bit(rcc_ahb1enr_addr, 22);
    utils::register_write_val(dma2_s3cr_addr, 0b0000_0110_0000_0011_0000_0100_0100_0000);
    utils::register_write_val(dma2_s3ndtr_addr, DMA_ONE_TIME_DATA);
    utils::register_write_val(
        dma2_s3par_addr,
        constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET,
    );
}

fn dma2_send_data() {
    let dma2_s3m0ar_addr = (constants::DMA2_BASE_ADDR + constants::DMA_S3M0AR_OFFSET) as *mut u32;
    let dma2_lifcr_addr = (constants::DMA2_BASE_ADDR + constants::DMA_LIFCR_OFFSET) as *mut u32;
    let dma2_hifcr_addr = (constants::DMA2_BASE_ADDR + constants::DMA_HIFCR_OFFSET) as *mut u32;
    let dma2_s3cr_addr = (constants::DMA2_BASE_ADDR + constants::DMA_S3CR_OFFSET) as *mut u32;
    let dma2_lisr_adddr = (constants::DMA2_BASE_ADDR + constants::DMA_LISR_OFFSET) as *mut u32;

    utils::register_write_val(dma2_s3m0ar_addr, addr_of_mut!(SPI_DMA_BUFFER) as u32);

    while utils::read_register_bit(dma2_s3cr_addr, 0) {}

    utils::register_write_val(dma2_lifcr_addr, 0b0000_1111_0111_1101_0000_1111_0111_1101);
    utils::register_write_val(dma2_hifcr_addr, 0b0000_1111_0111_1101_0000_1111_0111_1101);
    utils::register_set_bit(dma2_s3cr_addr, 0);

    while !utils::read_register_bit(dma2_lisr_adddr, 27) {}

    // disable DMA
    utils::register_write_bits(dma2_s3cr_addr, 0, 1, 0);
}

fn draw(item: DrawItem) {
    set_address_window(0, LCD_WIDTH - 1, 0, LCD_HEIGHT - 1);

    set_lcd_cs(PinStatus::Low);
    set_lcd_dc(PinStatus::Low);

    spi_wait_txe();

    let spi_dr_addr = (constants::SPI1_BASE_ADDR + constants::SPI_DR_OFFSET) as *mut u8;
    utils::register_write_val_u8(spi_dr_addr, LcdCommand::RAMWR as u8);

    spi_wait_idle();

    set_lcd_dc(PinStatus::High);

    send_shape_data(item);

    set_lcd_cs(PinStatus::High);
}

fn send_shape_data(item: DrawItem) {
    match item {
        DrawItem::Circle => {
            let mut y = 0;
            let high_byte = 0b1111_1000;
            let low_byte = 0b0000_0000;

            for _ in 0..16 {
                let mut idx = 0;
                for _ in 0..DMA_ONE_TIME_ROWS {
                    for i in 0..LCD_WIDTH as i32 {
                        let x = i;
                        let r = (x - (LCD_WIDTH as i32 / 2)) * (x - (LCD_WIDTH as i32 / 2))
                            + (y - (LCD_HEIGHT as i32 / 2)) * (y - (LCD_HEIGHT as i32 / 2));
                        unsafe {
                            SPI_DMA_BUFFER[idx] = if r < 10000 { high_byte } else { 0u8 };
                            SPI_DMA_BUFFER[idx + 1] = if r < 10000 { low_byte } else { 0u8 };
                        }
                        idx += 2;
                    }
                    y += 1;
                }

                dma2_send_data();
                spi_wait_idle();
            }
        }
        DrawItem::Image => {
            let mut img_idx = 0;
            for _ in 0..16 {
                let mut idx = 0;
                for _ in 0..DMA_ONE_TIME_ROWS {
                    for _ in 0..LCD_WIDTH as i32 {
                        // write to the display
                        let high_byte = image_data::IMAGE_DATA[img_idx];
                        let low_byte = image_data::IMAGE_DATA[img_idx + 1];

                        unsafe {
                            SPI_DMA_BUFFER[idx] = high_byte;
                            SPI_DMA_BUFFER[idx + 1] = low_byte;
                        }
                        img_idx += 2;
                        idx += 2;
                    }
                }

                dma2_send_data();
                spi_wait_idle();
            }
        }
        DrawItem::Rectangle => {
            let mut y = 0;
            let high_byte = 0b1111_1000;
            let low_byte = 0b0000_0000;

            for _ in 0..16 {
                let mut idx = 0;
                for _ in 0..DMA_ONE_TIME_ROWS {
                    for i in 0..LCD_WIDTH as i32 {
                        let x = i;
                        let in_shape = x > 50
                            && (x < (LCD_WIDTH as i32 - 50))
                            && y > 100
                            && (y < (LCD_HEIGHT as i32 - 100));

                        unsafe {
                            SPI_DMA_BUFFER[idx] = if in_shape { high_byte } else { 0u8 };
                            SPI_DMA_BUFFER[idx + 1] = if in_shape { low_byte } else { 0u8 };
                        }
                        idx += 2;
                    }
                    y += 1;
                }

                dma2_send_data();
                spi_wait_idle();
            }
        }
    }
}

pub fn draw_next_shape() {
    let current_shape = DrawItem::get_shape();
    match current_shape {
        DrawItem::Circle => {
            draw(DrawItem::Image);
            DrawItem::set_shape(DrawItem::Image);
        }
        DrawItem::Image => {
            draw(DrawItem::Rectangle);
            DrawItem::set_shape(DrawItem::Rectangle);
        }
        DrawItem::Rectangle => {
            draw(DrawItem::Circle);
            DrawItem::set_shape(DrawItem::Circle);
        }
    }
}
