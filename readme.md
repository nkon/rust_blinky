# rust でLチカする

rust で STM32(Cortex-M3)の上で動作するベアメタル(freestanding)オブジェクトをLunux上からクロスコンパイルする。


## target board

ターゲットボードは、展示会で配られていた<a href="http://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-eval-tools/stm32-mcu-eval-tools/stm32-mcu-nucleo/nucleo-f103rb.html">Nucleo-F103FB</a>としよう。MCUは、<a href="">STM32F103RB</a>で、主要スペックは次の通り。

* ARM® 32-bit Cortex® -M3 CPU Core
    + 72 MHz maximum frequency,1.25 DMIPS/MHz (Dhrystone 2.1) performance at 0 wait state memory access
    + Single-cycle multiplication and hardware division
* Memories
    + 64 or 128 Kbytes of Flash memory
    + 20 Kbytes of SRAM

## nightly コンパイラを用意する

2017-01時点で、後で使う #![feature]は nightly でしか使えないので、nightly コンパイラをインストールする。

```
$ rustup update nightly
```
とすると ~/.rustup/toolchain/nightly-x86_64-unknown-linux-gnu/ 以下にツールチェインがインストールされる。
```
$ rustup show
Default host: x86_64-unknown-linux-gnu

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.14.0 (e8a012324 2016-12-16)
```
と確認できる。


## プロジェクトの生成

```
$ cargo new --bin led
	Created binary (application) `led` project
```
とすれば led というプロジェクトを(ライブラリではなく)実行ファイル作成用に生成する。

```
$ cd led
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```
試しに通常通りビルドしてみよう。
```
$ cargo build
   Compiling led v0.1.0 (file:///home/nkon/src/rust/led)
    Finished debug [unoptimized + debuginfo] target(s) in 0.34 secs
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/led`
Hello, world!
$ tree .
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── led
        └── native

7 directories, 4 files
$ cat src/main.rs 
fn main() {
    println!("Hello, world!");
}
```

## クロス環境向けにビルド

クロスビルドするには、`--target`オプションを使う。`--target`で指定できるものは、次のコマンドで得ることができる。
```
$ rustc --print target-list
aarch64-linux-android
aarch64-unknown-fuchsia
aarch64-unknown-linux-gnu
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
armv7-linux-androideabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabihf
asmjs-unknown-emscripten
i586-pc-windows-msvc
i586-unknown-linux-gnu
i686-apple-darwin
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-msvc
i686-unknown-dragonfly
i686-unknown-freebsd
i686-unknown-haiku
i686-unknown-linux-gnu
i686-unknown-linux-musl
le32-unknown-nacl
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips-unknown-linux-uclibc
mips64-unknown-linux-gnuabi64
mips64el-unknown-linux-gnuabi64
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
mipsel-unknown-linux-uclibc
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64le-unknown-linux-gnu
s390x-unknown-linux-gnu
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
wasm32-unknown-emscripten
x86_64-apple-darwin
x86_64-pc-windows-gnu
x86_64-pc-windows-msvc
x86_64-rumprun-netbsd
x86_64-sun-solaris
x86_64-unknown-bitrig
x86_64-unknown-dragonfly
x86_64-unknown-freebsd
x86_64-unknown-fuchsia
x86_64-unknown-haiku
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
x86_64-unknown-netbsd
x86_64-unknown-openbsd
```
いっぱいあるが、この中で `thumbv7m-none-eabi`が求めるものだ。

この記法は target triple と呼ばれているもので、一番目の項目がアーキテクチャ、二番目の項目が OS、三番目の項目が呼び出し規約を表す。

Cortex-M0+の場合は<a href="http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0662b/BABIHJGA.html">CPUアーキテクチャが ARMv6-M</a>,<a href="http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0337gj/index.html">命令アーキテクチャが Thumb</a> なので、あわせて thumbv6m。

Cortex-M3の場合は<a href="http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0337gj/index.html">CPUアーキテクチャが ARMv7-M</a>, <a href="">命令セットが Thumb2</a>なので、あわせてthumbv7m になる。

Cortex-M4の場合は<a href="http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0337gj/index.html">CPUアーキテクチャが ARMv7E-M</a>,HardFP有りだったりするので注意のこと。

none は OS無しということ。

eabi は eabi とう呼び出し規約。古くは OABIというものがあって、それと区別するために名前が付いているが、最近は eabi一択でOK。最後の hf は、Hard Float(ハードウエアによる浮動小数点演算)。

では、クロスビルドをする。
```
$ cargo build --target thumbv7m-none-eabi 
   Compiling led v0.1.0 (file:///home/nkon/src/rust/led)
error[E0463]: can't find crate for `std`

error: aborting due to previous error

error: Could not compile `led`.

To learn more, run the command again with --verbose.
```
ダメである。

std クレートは os の存在を前提としているが、thumbv7m-none-eabi 環境向けの std クレートは存在しないので、エラーとなる。OS無し環境では、libcore を使うことができる。

## libcore のクロスビルド

これらのことをするためには、stable ではなく nightly を使うようにする。override を使うことによって、そのディレクトリだけ設定を変更することができる。その情報は、当該ディレクトリではなく~/.rustup/settings.toml に書かれるので引越し時は注意。

```
$ rustup override set nightly-x86_64-unknown-linux-gnu
```

ソースを取ってくる。

通常どおり git でとってきても良いが、次のコマンドを実行すれば、`~/.cargo` の下にソースを取ってきてくれる。
```
rustup component add rust-src
```
そして、環境変数 RUST_SRC_PATH にセットしておくのがマナーだ。rustfmt などのコマンドでもソースを参照するので、持っておくのが基本みたい。

ビルドする。libcore.rlib ができる。
```
$ mkdir libcore-thumbv6m
$ rustc -C opt-level=2 -Z no-landing-pads --target thumbv6m-none-eabi -g ${RUST_SRC_PATH}/libcore/lib.rs --out-dir libcore-thumbv6m
$ tree libcore-thumbv6m
libcore-thumbv6m/
└── libcore.rlib

0 directories, 1 file
```
やりかたは、<a href="https://spin.atomicobject.com/2015/02/20/rust-language-c-embedded/">こちら</a>を参照した。

## 最小限ブログラム

次に本体を修正する。<a href="http://qiita.com/mopp/items/9c816d58104752180207">こちら</a>を参照した。

これが最小限のプログラムだ。

```
#![no_std] // std を使わない。1.6.0以降だと、これで自動的に libcore が使われる。
#![no_main] // rust の標準的な main を使わない
#![feature(lang_items)] // #[lang="..."] を使う宣言。具体的には、下の #[lang="panic_fmt"]
#![feature(start)] // #[start] を使う宣言。

#[no_mangle] // mangling(名前修飾)を使わない。
#[start] // エントリーポイントを指定。
pub extern fn main() {
	loop {}
}

#[lang="panic_fmt"] // コンパイラの失敗メカニズムのために必要な関数
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
	loop {}
}

#[lang="eh_personality"] // コンパイラの失敗メカニズムのために必要な関数
extern fn eh_personality (){}
```

ビルドしてみよう。
```
$ rustc -C opt-level=2 -Z no-landing-pads --target thumbv6m-none-eabi -g --emit obj -L ../libcore-thumbv6m -o led.o src/main.rs
```
エラー無く終って、カレントディレクトリに led.o ができていけばOK。

## ハードウエア固有の設定

Cで開発するときと同様に、CubeMXで初期化コードを生成する。toolchain には SW4STM32を選ぶ。SW4STM32は、OpenOCD+Eclipse+gccという、オープン系のワークベンチだ。それを使って、[最小限の Lチカ・コード](cubemx/readme.md)を書き、それを rust に移植する。

## Lチカ

Nucleo-F103FBには、User LED(LD2)が搭載されている。LD2は、PA5ピンに接続されている。ここで、CubeMXのコードをカンニングすれば、それを操作するためのアドレスがわかる。

```
#![no_std] // std を使わない。1.6.0以降だと、これで自動的に libcore が使われる。
#![no_main] // rust の標準的な main を使わない
#![feature(lang_items)] // #[lang="..."] を使う宣言。具体的には、下の #[lang="panic_fmt"]
#![feature(start)] // #[start] を使う宣言。

#![feature(asm)]  // asm を使う。
#![feature(core_intrinsics)] // core_intrinsics を使う。
use core::intrinsics::volatile_store; // メモリ直書きには volatile_store を使う。

// レジスタアドレスの定義
pub const PERIPH_BASE: u32      = 0x40000000;

pub const APB2PERIPH_BASE: u32  = PERIPH_BASE + 0x10000;
pub const GPIOA_BASE: u32       = APB2PERIPH_BASE + 0x0800;
pub const BSRR_OFFSET: u32      = 0x10;
pub const GPIO_PIN_5: u32       = 5;

#[no_mangle] // mangling(名前修飾)を使わない。
#[start] // エントリーポイントを指定。
pub extern fn main() {
    let bsrr    = (GPIOA_BASE+BSRR_OFFSET)  as *mut u32;

    loop {
        unsafe {
            volatile_store(bsrr, 1 << GPIO_PIN_5);  // 点灯
        }
        for _ in 1..400000 {
            unsafe {asm!("");}
        }

        unsafe {
            volatile_store(bsrr, (1 << GPIO_PIN_5) << 16); // 消灯
        }

        for _ in 1..400000 {
            unsafe {asm!("");}
        }
	}
}

#[lang="panic_fmt"] // コンパイラの失敗メカニズムのために必要な関数
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
	loop {}
}

#[lang="eh_personality"] // コンパイラの失敗メカニズムのために必要な関数
extern fn eh_personality (){}
```

ビット演算を Rust でやる方法がよくわからなかったので、それらは `system_init.c`にまとめた。

```
#define PERIPH_BASE         (0x40000000) /*!< Peripheral base address in the alias region */

#define APB2PERIPH_BASE     (PERIPH_BASE + 0x10000)
#define GPIOA_BASE          (APB2PERIPH_BASE+0x0800)
#define CRL_OFFSET          0x00

#define AHBPERIPH_BASE      (PERIPH_BASE + 0x20000)
#define RCC_BASE            (AHBPERIPH_BASE + 0x1000)
#define CR_OFFSET           0x00
#define CFGR_OFFSET         0x04
#define CIR_OFFSET          0x08
#define APB2ENR_OFFSET      0x18

#define FLASH_BASE          (0x08000000) /*!< FLASH base address in the alias region */
#define VECT_TAB_OFFSET     0x0 /*!< Vector Table base offset field. This value must be a multiple of 0x200. */
#define VTOR_OFFSET         8

#define SCS_BASE            (0xE000E000UL)                            /*!< System Control Space Base Address */
#define SCB_BASE            (SCS_BASE +  0x0D00UL)                    /*!< System Control Block Base Address */

void SystemInit (void)
{
    volatile unsigned long *rcc_reg;
    volatile unsigned long *crl;
    volatile unsigned long *apb2enr;

    rcc_reg = (unsigned long *)(RCC_BASE+CR_OFFSET);
    *rcc_reg |= 0x00000001;

    rcc_reg = (unsigned long *)(RCC_BASE+CFGR_OFFSET);
    *rcc_reg &= 0xF0FF0000;
  
    rcc_reg = (unsigned long *)(RCC_BASE+CR_OFFSET);
    *rcc_reg &= 0xFEF6FFFF;

    *rcc_reg &= 0xFFFBFFFF;

    rcc_reg = (unsigned long *)(RCC_BASE+CFGR_OFFSET);
    *rcc_reg &= 0xFF80FFFF;

    rcc_reg = (unsigned long *)(RCC_BASE+CIR_OFFSET);
    *rcc_reg = 0x009F0000;

    rcc_reg = (unsigned long *)(SCB_BASE+VTOR_OFFSET);
    *rcc_reg = FLASH_BASE | VECT_TAB_OFFSET;

    crl     = (unsigned long *)(GPIOA_BASE+CRL_OFFSET);
    apb2enr = (unsigned long *)(RCC_BASE+APB2ENR_OFFSET);

    *apb2enr|= 1 << 2;
    *crl &= (~(6 << 20));   // clear CNF5 : PP mode
    *crl |= (2 << 20);      // set MODE5: Output 2MHz
}
```

startup_stm32f103xb.s も、そのまま使う。

Makefile は、次のようになる。
```
CFLAGS=-mcpu=cortex-m3 -mthumb -mfloat-abi=soft
CFLAGS2=-Og -g3 -Wall -fmessage-length=0 -ffunction-sections -c -fmessage-length=0
LDFLAGS=-specs=nosys.specs -specs=nano.specs -Tcubemx/nucleo-f103rb/STM32F103RBTx_FLASH.ld -Wl,--gc-sections -lm
CC=arm-none-eabi-gcc
AS=arm-none-eabi-as
RUSTC=rustc -C opt-level=2 -Z no-landing-pads --target thumbv6m-none-eabi -g --emit obj -L ../libcore-thumbv6m

startup_stm32f103xb.o: cubemx/nucleo-f103rb/startup/startup_stm32f103xb.s
	$(AS) $(CFLAGS) -o $@ $<

system_init.o: src/system_init.c
	$(CC) $(CFLAGS) $(CFLAGS2) -o $@ $<

main.o: src/main.rs
	$(RUSTC) -o $@ $<

led.elf: main.o startup_stm32f103xb.o system_init.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

.PHONY: flash
flash: led.elf
	sudo openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image led.elf" -c "reset halt" -c "reset run" -c "exit"

.PHONY: clean
clean:
	rm *.elf *.o *.map
```

これで、Rust + STM32 で Lチカできた。

一部、C を用いたが、Rust の型制約の中で、ビット演算ができれば、Cは無くせる。

## 比較

CubeMX で作ったモノ(C)、Cで最小限にしたモノ、rust を比較。Cとrustで、ほとんど同じ。rustにオーバヘッドが無いことがわかる。

```
$ arm-none-eabi-size *.elf_
   text	   data	    bss	    dec	    hex	filename
   4320	     12	   1572	   5904	   1710	cubemx.elf_
    780	      8	   1568	   2356	    934	plain.elf_
    784	      8	   1568	   2360	    938	rust.elf_
```



## HAL をリンクする

このように、MCUのレジスタ・アドレスをデータシートで調べて、バイナリを書き込めば、Rust でプログラムできるが、いちいち調べるのもたいへんだ。そのために CubeMXやHALが用意されているが、それは C で書かれている。Rust の FFI(多言語インターフェイス)を使って、Cで書かれたHALをリンクすれば良い。



