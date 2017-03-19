#![no_std]
#![allow(non_snake_case)]

// レジスタアドレスの定義
pub const PERIPH_BASE: u32      = 0x40000000;

pub const APB2PERIPH_BASE: u32  = PERIPH_BASE + 0x10000;
pub const GPIOA_BASE: u32       = APB2PERIPH_BASE + 0x0800;
pub const CRL_OFFSET: u32       = 0x00;
pub const BSRR_OFFSET: u32      = 0x10;
pub const GPIO_PIN_5: u32       = 5;

pub const AHBPERIPH_BASE: u32   = PERIPH_BASE + 0x20000;
pub const RCC_BASE: u32         = AHBPERIPH_BASE + 0x1000;
pub const CR_OFFSET: u32        = 0x00;
pub const CFGR_OFFSET: u32      = 0x04;
pub const CIR_OFFSET: u32       = 0x08;
pub const APB2ENR_OFFSET: u32   = 0x18;

pub const FLASH_BASE: u32       = 0x08000000;
pub const VECT_TAB_OFFSET: u32  = 0x0;
pub const VTOR_OFFSET: u32      = 8;

pub const SCS_BASE: u32         = 0xE000E000;
pub const SCB_BASE: u32         = SCS_BASE + 0x0D00;

#[repr(C)] // C の struct のインポート
pub struct GPIO_InitTypeDef {
    pub Pin: u32,
    pub Mode: u32,
    pub Pull: u32,
    pub Speed: u32
}

#[repr(C)]
pub struct GPIO_TypeDef {
    pub CRL: u32,
    pub CRH: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSRR: u32,
    pub BRR: u32,
    pub LCKR: u32
}
