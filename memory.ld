ENTRY(reset_handler)

MEMORY
{
  RAM (rw) : ORIGIN = 0x20000000, LENGTH = 128K
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 512K
}

/* This `ORIGIN` function is not documented in `lld` or `ld` docs but can be
 * found here: https://www.sourceware.org/binutils/docs-2.25/ld/Builtin-Functions.html#index-ORIGIN_0028_0040var_007bmemory_007d_0029-571
 * end of ram (cortex-m4 stack grows downwards)
 * */
_stack_pointer = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .text : {
    . = ALIGN(4);
    LONG(_stack_pointer)
    KEEP(*(.vector_table))
    *(.text)
    *(.text.*)
    . = ALIGN(4);
  } > FLASH

  .data : {
    . = ALIGN(4);
    _start_data_in_ram = .;
    *(.data)
    *(.data.*)
    _end_data_in_ram = .;
    . = ALIGN(4);
  } > RAM AT> FLASH

  .rodata : {
    . = ALIGN(4);
    *(.rodata)
    *(.rodata.*)
    . = ALIGN(4);
  } > FLASH

  .bss : {
    . = ALIGN(4);
    _start_bss = .;
    *(.bss)
    *(.bss.*)
    _end_bss = .;
    . = ALIGN(4);
  } > RAM

    _start_data = LOADADDR(.data);
}

PROVIDE(nmi = default_handler);
PROVIDE(hard_fault = default_handler);
PROVIDE(mem_manage = default_handler);
PROVIDE(bus_fault = default_handler);
PROVIDE(usage_fault = default_handler);
PROVIDE(sv_call = default_handler);
PROVIDE(debug_monitor = default_handler);
PROVIDE(pend_sv = default_handler);
PROVIDE(systick = default_handler);
PROVIDE(wwdg = default_handler);
PROVIDE(pvd = default_handler);
PROVIDE(tamp_stamp = default_handler);
PROVIDE(rtc_wkup = default_handler);
PROVIDE(flash = default_handler);
PROVIDE(rcc = default_handler);
PROVIDE(exti0 = default_handler);
PROVIDE(exti1 = default_handler);
PROVIDE(exti2 = default_handler);
PROVIDE(exti3 = default_handler);
PROVIDE(exti4 = default_handler);
PROVIDE(dma1_stream0 = default_handler);
PROVIDE(dma1_stream1 = default_handler);
PROVIDE(dma1_stream2 = default_handler);
PROVIDE(dma1_stream3 = default_handler);
PROVIDE(dma1_stream4 = default_handler);
PROVIDE(dma1_stream5 = default_handler);
PROVIDE(dma1_stream6 = default_handler);
PROVIDE(adc = default_handler);
PROVIDE(can1_tx = default_handler);
PROVIDE(can1_rx0 = default_handler);
PROVIDE(can1_rx1 = default_handler);
PROVIDE(can1_sce = default_handler);
PROVIDE(exti9_5 = default_handler);
PROVIDE(tim1_brk_tim9 = default_handler);
PROVIDE(tim1_up_tim10 = default_handler);
PROVIDE(tim1_trg_com_tim11 = default_handler);
PROVIDE(tim1_cc = default_handler);
PROVIDE(tim2 = default_handler);
PROVIDE(tim3 = default_handler);
PROVIDE(tim4 = default_handler);
PROVIDE(i2c1_ev = default_handler);
PROVIDE(i2c1_er = default_handler);
PROVIDE(i2c2_ev = default_handler);
PROVIDE(i2c2_er = default_handler);
PROVIDE(spi1 = default_handler);
PROVIDE(spi2 = default_handler);
PROVIDE(usart1 = default_handler);
PROVIDE(usart2 = default_handler);
PROVIDE(usart3 = default_handler);
PROVIDE(exti15_10 = default_handler);
PROVIDE(stc_alarm = default_handler);
PROVIDE(otg_fs_wkup = default_handler);
PROVIDE(tim8_brk_tim12 = default_handler);
PROVIDE(tim8_up_tim13 = default_handler);
PROVIDE(tim8_trg_com_tim14 = default_handler);
PROVIDE(tim8_cc = default_handler);
PROVIDE(dma1_stream7 = default_handler);
PROVIDE(fmc = default_handler);
PROVIDE(sdio = default_handler);
PROVIDE(tim5 = default_handler);
PROVIDE(spi3 = default_handler);
PROVIDE(uart4 = default_handler);
PROVIDE(uart5 = default_handler);
PROVIDE(tim6_dac = default_handler);
PROVIDE(tim7 = default_handler);
PROVIDE(dma2_stream0 = default_handler);
PROVIDE(dma2_stream1 = default_handler);
PROVIDE(dma2_stream2 = default_handler);
PROVIDE(dma2_stream3 = default_handler);
PROVIDE(dma2_stream4 = default_handler);
PROVIDE(can2_tx = default_handler);
PROVIDE(can2_rx0 = default_handler);
PROVIDE(can2_rx1 = default_handler);
PROVIDE(can2_sce = default_handler);
PROVIDE(otg_fs = default_handler);
PROVIDE(dma2_stream5 = default_handler);
PROVIDE(dma2_stream6 = default_handler);
PROVIDE(dma2_stream7 = default_handler);
PROVIDE(usart6 = default_handler);
PROVIDE(i2c3_ev = default_handler);
PROVIDE(i2c3_er = default_handler);
PROVIDE(otg_hs_ep1_out = default_handler);
PROVIDE(otg_hs_ep1_in = default_handler);
PROVIDE(otg_hs_wkup = default_handler);
PROVIDE(otg_hs = default_handler);
PROVIDE(dcmi = default_handler);
PROVIDE(fpu = default_handler);
PROVIDE(spi4 = default_handler);
PROVIDE(sai1 = default_handler);
PROVIDE(sai2 = default_handler);
PROVIDE(quad_spi = default_handler);
PROVIDE(hdmi_cec = default_handler);
PROVIDE(spdif_rx = default_handler);
PROVIDE(fmpi2c1 = default_handler);
PROVIDE(fmpi2c1_error = default_handler);
