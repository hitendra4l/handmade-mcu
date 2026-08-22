#![no_std]
#![no_main]

mod button;
mod constants;
mod led;
mod spi;
mod systick;
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
    Some(systick::systick),
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
    Some(button::exti15_10),
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

unsafe extern "C" {
    static _start_data_in_ram: u32;
    static _end_data_in_ram: u32;
    static _start_data: u32;
    static _start_bss: u32;
    static _end_bss: u32;
}

#[unsafe(no_mangle)]
extern "C" fn reset_handler() {
    // copy flash .data to ram
    unsafe {
        let mut flash_ptr = &raw const _start_data;
        let mut ram_ptr = &raw const _start_data_in_ram as *mut u32;
        let end_ram = &raw const _end_data_in_ram;

        while (ram_ptr as u32) < (end_ram as u32) {
            *ram_ptr = *flash_ptr;
            flash_ptr = flash_ptr.add(1);
            ram_ptr = ram_ptr.add(1);
        }
    }

    // zero out the .bss section
    let mut bss_ptr = &raw const _start_bss as *mut u32;
    let end_bss = &raw const _end_bss;

    while (bss_ptr as u32) < (end_bss as u32) {
        unsafe {
            *bss_ptr = 0;
            bss_ptr = bss_ptr.add(1);
        }
    }

    main();
}

fn main() -> ! {
    led::init();
    button::init();
    systick::init();
    spi::init();

    #[allow(clippy::empty_loop)]
    loop {}
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
