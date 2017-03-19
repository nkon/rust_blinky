#![no_std] // std を使わない。1.6.0以降だと、これで自動的に libcore が使われる。
#![no_main] // rust の標準的な main を使わない
#![feature(lang_items)] // #[lang="..."] を使う宣言。具体的には、下の #[lang="panic_fmt"]
#![feature(start)] // #[start] を使う宣言。

#![feature(asm)] // asm を使う。
#![feature(core_intrinsics)] // core_intrinsics を使う。
use core::intrinsics::volatile_store; // メモリ直書きには volatile_store を使う。

extern crate stm32f1xx_hal_gpio;
use stm32f1xx_hal_gpio::*;

#[no_mangle]
// mangling(名前修飾)を使わない。
#[start] // エントリーポイントを指定。
pub extern "C" fn main() {

    GPIOA_CLK_ENABLE();

    let mut gpio_init_struct = GPIO_InitTypeDef{Pin: 0, Mode: 0, Pull: 0, Speed: 0};
    gpio_init_struct.Pin = 0x0020;
    gpio_init_struct.Mode = 0x0001;
    gpio_init_struct.Speed = 0x0002;

    Init(GPIOA(), &gpio_init_struct);

    loop {
        WritePin(GPIOA(), 0x0020, 1);
        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }
        WritePin(GPIOA(), 0x0020, 0);
        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SystemInit() {
    let rcc_cr = (RCC_BASE + CR_OFFSET) as *mut u32;
    let rcc_cfgr = (RCC_BASE + CFGR_OFFSET) as *mut u32;
    let rcc_cir = (RCC_BASE + CIR_OFFSET) as *mut u32;
    let scb_vtor = (SCB_BASE + VTOR_OFFSET) as *mut u32;

    unsafe {
        volatile_store(rcc_cr, *rcc_cr | 0x00000001);
        volatile_store(rcc_cfgr, *rcc_cfgr & 0xf0f0000);
        volatile_store(rcc_cr, *rcc_cr & 0xfef6ffff);
        volatile_store(rcc_cr, *rcc_cr & 0xfffbffff);
        volatile_store(rcc_cfgr, *rcc_cfgr & 0xff80ffff);
        volatile_store(rcc_cir, 0x009f0000);
        volatile_store(scb_vtor, FLASH_BASE | VECT_TAB_OFFSET);
    }
}

#[lang="panic_fmt"] // コンパイラの失敗メカニズムのために必要な関数
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
    loop {}
}

#[lang="eh_personality"] // コンパイラの失敗メカニズムのために必要な関数
extern "C" fn eh_personality() {}
