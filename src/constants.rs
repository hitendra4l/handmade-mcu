pub const RCC_BASE: u32 = 0x4002_3800;
pub const RCC_AHB1ENR_OFFSET: u32 = 0x30;
pub const RCC_APB2ENR_OFFSET: u32 = 0x44;

pub const GPIO_A_BASE: u32 = 0x4002_0000;
pub const GPIO_C_BASE: u32 = 0x4002_0800;

pub const GPIO_MODER_OFFSET: u32 = 0x00;
pub const GPIO_BSRR_OFFSET: u32 = 0x18;
pub const GPIO_ODR_OFFSET: u32 = 0x14;
pub const GPIO_PUPDR_OFFSET: u32 = 0x0C;

pub const SYSCFG_BASE: u32 = 0x4001_3800;
pub const SYSCFG_EXTICR4_OFFSET: u32 = 0x14;

pub const EXTI_BASE: u32 = 0x4001_3C00;
pub const EXTI_IMR_OFFSET: u32 = 0x00;
pub const EXTI_RTSR_OFFSET: u32 = 0x08;
pub const EXTI_PR_OFFSET: u32 = 0x14;

pub const NVIC_ISER1_ADDR: u32 = 0xE000E100 + 0x04;

pub const STK_CTRL_ADDR: u32 = 0xE000E010;
pub const STK_LOAD_ADDR: u32 = 0xE000E014;
pub const STK_VAL_ADDR: u32 = 0xE000E018;
