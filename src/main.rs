#![no_std]
#![no_main]

mod constants;
mod utils;

unsafe extern "C" {
    fn nmi();
    fn hard_fault();
    fn mem_manage();
    fn bus_fault();
    fn usage_fault();
    fn sv_call();
    fn debug_monitor();
    fn pend_sv();
    fn systick();
    fn wwdg();
    fn pvd();
    fn tamp_stamp();
    fn rtc_wkup();
    fn flash();
    fn rcc();
    fn exti0();
    fn exti1();
    fn exti2();
    fn exti3();
    fn exti4();
    fn dma1_stream0();
    fn dma1_stream1();
    fn dma1_stream2();
    fn dma1_stream3();
    fn dma1_stream4();
    fn dma1_stream5();
    fn dma1_stream6();
    fn adc();
    fn can1_tx();
    fn can1_rx0();
    fn can1_rx1();
    fn can1_sce();
    fn exti9_5();
    fn tim1_brk_tim9();
    fn tim1_up_tim10();
    fn tim1_trg_com_tim11();
    fn tim1_cc();
    fn tim2();
    fn tim3();
    fn tim4();
    fn i2c1_ev();
    fn i2c1_er();
    fn i2c2_ev();
    fn i2c2_er();
    fn spi1();
    fn spi2();
    fn usart1();
    fn usart2();
    fn usart3();
    // fn exti15_10();
    fn stc_alarm();
    fn otg_fs_wkup();
    fn tim8_brk_tim12();
    fn tim8_up_tim13();
    fn tim8_trg_com_tim14();
    fn tim8_cc();
    fn dma1_stream7();
    fn fmc();
    fn sdio();
    fn tim5();
    fn spi3();
    fn uart4();
    fn uart5();
    fn tim6_dac();
    fn tim7();
    fn dma2_stream0();
    fn dma2_stream1();
    fn dma2_stream2();
    fn dma2_stream3();
    fn dma2_stream4();
    fn can2_tx();
    fn can2_rx0();
    fn can2_rx1();
    fn can2_sce();
    fn otg_fs();
    fn dma2_stream5();
    fn dma2_stream6();
    fn dma2_stream7();
    fn usart6();
    fn i2c3_ev();
    fn i2c3_er();
    fn otg_hs_ep1_out();
    fn otg_hs_ep1_in();
    fn otg_hs_wkup();
    fn otg_hs();
    fn dcmi();
    fn fpu();
    fn spi4();
    fn sai1();
    fn sai2();
    fn quad_spi();
    fn hdmi_cec();
    fn spdif_rx();
    fn fmpi2c1();
    fn fmpi2c1_error();
}

unsafe extern "C" {
    static _start_data_in_ram: *mut u32;
    static _end_data_in_ram: *const u32;
    static _start_data: *const u32;
    static _start_bss: *mut u32;
    static _end_bss: *const u32;
}

#[unsafe(no_mangle)]
extern "C" fn reset_handler() {
    // copy flash .data to ram
    unsafe {
        let mut flash_ptr = _start_data;
        let mut ram_ptr = _start_data_in_ram;

        while (ram_ptr as u32) < (_end_data_in_ram as u32) {
            *ram_ptr = *flash_ptr;

            flash_ptr = flash_ptr.add(1);
            ram_ptr = ram_ptr.add(1);
        }
    }

    // zero out the .bss section
    unsafe {
        let mut bss_ptr = _start_bss;

        while (bss_ptr as u32) < (_end_bss as u32) {
            *bss_ptr = 0;
            bss_ptr = bss_ptr.add(1);
        }
    }

    main();
}

fn main() -> ! {
    // Step1: enable GPIO_A clock
    // why?: LD2 which is the only user-controlled LED on board is controlled by
    // PA5 pin which is inside GPIO_A port.
    let rcc_ahb1enr = (constants::RCC_BASE + constants::RCC_AHB1ENR_OFFSET) as *mut u32;
    utils::register_set_bit(rcc_ahb1enr, 0);

    // Step2: set GPIO_A MODER register to output mode for PA5(because CPU would change the
    // pin and make LED on)
    let gpio_a_moder = (constants::GPIO_A_BASE + constants::GPIO_MODER_OFFSET) as *mut u32;
    utils::register_write_bits(gpio_a_moder, 5 * 2, 2, 0b01);

    // Step3: OTYPER's reset state is push-pull so no need to change

    // Step4: OSPEEDR's reset state is low speed so no need to change

    /**** button interrupt ****/
    // enable GPIO_A clock
    // why?: user button is controlled by PC13 pin which is inside GPIO_C port
    utils::register_set_bit(rcc_ahb1enr, 2);

    // enable SYSCNG clock
    let addr = (constants::RCC_BASE + constants::RCC_APB2ENR_OFFSET) as *mut u32;
    utils::register_set_bit(addr, 14);

    // enable interrupt
    utils::register_set_bit(constants::NVIC_ISER1_ADDR as *mut u32, 8);

    // GPIO_C's MODER register bit is in input mode by default so no need to change

    // GPIO_C's PUPD register set to PULL DOWN for pin 13
    let gpio_c_moder = (constants::GPIO_C_BASE + constants::GPIO_PUPDR_OFFSET) as *mut u32;
    utils::register_write_bits(gpio_c_moder, 13 * 2, 2, 0b10);

    // EXTI_IMR enable
    let addr = (constants::EXTI_BASE + constants::EXTI_IMR_OFFSET) as *mut u32;
    utils::register_set_bit(addr, 13);

    // EXTI_RTSR enable
    let addr = (constants::EXTI_BASE + constants::EXTI_RTSR_OFFSET) as *mut u32;
    utils::register_set_bit(addr, 13);

    // SYSCFG_EXTICR4 to PC13 because PC 13 falls into EXTI4
    let syscfg_exticr4 = (constants::SYSCFG_BASE + constants::SYSCFG_EXTICR4_OFFSET) as *mut u32;
    utils::register_write_bits(syscfg_exticr4, 4, 4, 0b0010);

    #[allow(clippy::empty_loop)]
    loop {
        // Step5: busy wait and toggle the led
        // for _ in 0..100000 {}
        // toggle_led();
    }
}

fn toggle_led() {
    let odr_addr = (constants::GPIO_A_BASE + constants::GPIO_ODR_OFFSET) as *mut u32;
    let is_led_on = utils::read_register_bit(odr_addr, 5);

    let bsrr_addr = (constants::GPIO_A_BASE + constants::GPIO_BSRR_OFFSET) as *mut u32;

    if is_led_on {
        utils::register_write_val(bsrr_addr, 1u32 << (5 + 16));
    } else {
        utils::register_write_val(bsrr_addr, 1u32 << 5);
    }
}

extern "C" fn exti15_10() {
    toggle_led();

    let addr = (constants::EXTI_BASE + constants::EXTI_PR_OFFSET) as *mut u32;
    utils::register_set_bit(addr, 13);
}

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn default_handler() {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[used]
#[unsafe(link_section = ".vector_table")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 112] = [
    Some(reset_handler),
    Some(nmi),
    Some(hard_fault),
    Some(mem_manage),
    Some(bus_fault),
    Some(usage_fault),
    None,
    None,
    None,
    None,
    Some(sv_call),
    Some(debug_monitor),
    None,
    Some(pend_sv),
    Some(systick),
    Some(wwdg),
    Some(pvd),
    Some(tamp_stamp),
    Some(rtc_wkup),
    Some(flash),
    Some(rcc),
    Some(exti0),
    Some(exti1),
    Some(exti2),
    Some(exti3),
    Some(exti4),
    Some(dma1_stream0),
    Some(dma1_stream1),
    Some(dma1_stream2),
    Some(dma1_stream3),
    Some(dma1_stream4),
    Some(dma1_stream5),
    Some(dma1_stream6),
    Some(adc),
    Some(can1_tx),
    Some(can1_rx0),
    Some(can1_rx1),
    Some(can1_sce),
    Some(exti9_5),
    Some(tim1_brk_tim9),
    Some(tim1_up_tim10),
    Some(tim1_trg_com_tim11),
    Some(tim1_cc),
    Some(tim2),
    Some(tim3),
    Some(tim4),
    Some(i2c1_ev),
    Some(i2c1_er),
    Some(i2c2_ev),
    Some(i2c2_er),
    Some(spi1),
    Some(spi2),
    Some(usart1),
    Some(usart2),
    Some(usart3),
    Some(exti15_10),
    Some(stc_alarm),
    Some(otg_fs_wkup),
    Some(tim8_brk_tim12),
    Some(tim8_up_tim13),
    Some(tim8_trg_com_tim14),
    Some(tim8_cc),
    Some(dma1_stream7),
    Some(fmc),
    Some(sdio),
    Some(tim5),
    Some(spi3),
    Some(uart4),
    Some(uart5),
    Some(tim6_dac),
    Some(tim7),
    Some(dma2_stream0),
    Some(dma2_stream1),
    Some(dma2_stream2),
    Some(dma2_stream3),
    Some(dma2_stream4),
    None,
    None,
    Some(can2_tx),
    Some(can2_rx0),
    Some(can2_rx1),
    Some(can2_sce),
    Some(otg_fs),
    Some(dma2_stream5),
    Some(dma2_stream6),
    Some(dma2_stream7),
    Some(usart6),
    Some(i2c3_ev),
    Some(i2c3_er),
    Some(otg_hs_ep1_out),
    Some(otg_hs_ep1_in),
    Some(otg_hs_wkup),
    Some(otg_hs),
    Some(dcmi),
    None,
    None,
    Some(fpu),
    None,
    None,
    Some(spi4),
    None,
    None,
    Some(sai1),
    None,
    None,
    None,
    Some(sai2),
    Some(quad_spi),
    Some(hdmi_cec),
    Some(spdif_rx),
    Some(fmpi2c1),
    Some(fmpi2c1_error),
];
