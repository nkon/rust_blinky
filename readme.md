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

* `#![no_std]`で std を使わない。自動的に libcore が使われる。
  - `#![...]`は、それが含まれるモノ(この場合はプログラム全体)を変更する。
  - `#[...]`次に来るものを変更する。
* `#![no_main]`で 標準的な main を使わない。以下で main と書いても、単なる、アセンブラから呼ばれるルーチンとなる。
* `#![feature(lang_items)]`で`#![lang="..."]`を使う準備をする。
* `#![feature(start)]`で `#[start]`を使う準備をする。
* `main`の定義
  - `#[no_mangle]`で名前修飾を使わない。
  - `#[start]`でエントリポイントを指定。
* 後半の関数は、コンパイラが使う関数。現時点ではおまじない。

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

ビルドしてみよう。オプションは写経。

* `-g`: `-C debuginfo=2` と等しい。
* `-O`: `-C opt-level=2` と等しい。
  + `-C XXX` は codegen のオプション。
* `-Z no-landing-pads`
  + `-Z XXX` は internal option for debugging rustc
* `--target thumbv6m-none-eabi`: 上述。
* `--emit obj`: object file を出力。
* `-L ../libcore-thumbv6m`: ライブラリを指定。
* `-o led.o`: 出力ファイル名。
* `src/main.rs`: ソースファイル名。

```
$ rustc -g -O -Z no-landing-pads --target thumbv6m-none-eabi --emit obj -L ../libcore-thumbv6m -o led.o src/main.rs
```
エラー無く終って、カレントディレクトリに led.o ができていけばOK。

## ハードウエア固有の設定

Cで開発するときと同様に、CubeMXで初期化コードを生成する。toolchain には SW4STM32を選ぶ。SW4STM32は、OpenOCD+Eclipse+gccという、オープン系のワークベンチだ。それを使って、[最小限の Lチカ・コード](cubemx/readme.md)を書き、それを rust に移植する。

## Lチカ

Nucleo-F103FBには、User LED(LD2)が搭載されている。LD2は、PA5ピンに接続されている。ここで、CubeMXのコードをカンニングすれば、それを操作するためのアドレスがわかる。

* レジスタを `let xxx = ADDRESS as *mut u32;` のように、`u32` の mutable なポインタとして定義して、アドレスで初期化。
* レジスタに書くときは、`volatile_store`。
  - `#![feature(core_intrinsics)]` →`use core::intrinsics::volatile_store`で使えるようになる。
  - `unsafe` で囲む。
* `system_stm32f1xx.c` で提供されていた `SystemInit`(スタートアップから呼ばれる)も `#[no_mangle]`で実装する。

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

#[no_mangle] // mangling(名前修飾)を使わない。
#[start] // エントリーポイントを指定。
pub extern fn main() {

    let apb2enr = (RCC_BASE+APB2ENR_OFFSET) as *mut u32;
    let crl     = (GPIOA_BASE+CRL_OFFSET) as *mut u32;

    unsafe {
        volatile_store(apb2enr, *apb2enr | (1<<2));
        volatile_store(crl, *crl & (!(6<<20)));
        volatile_store(crl, *crl | (2<<20));
    }

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

#[no_mangle]
pub extern fn SystemInit(){
    let rcc_cr   = (RCC_BASE+CR_OFFSET) as *mut u32;
    let rcc_cfgr = (RCC_BASE+CFGR_OFFSET) as *mut u32;
    let rcc_cir  = (RCC_BASE+CIR_OFFSET) as *mut u32;
    let scb_vtor = (SCB_BASE+VTOR_OFFSET) as *mut u32;

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
extern fn eh_personality (){}
```

startup_stm32f103xb.s は、そのまま使う。

Makefile は、次のようになる。
```
CFLAGS=-mcpu=cortex-m3 -mthumb -mfloat-abi=soft
CFLAGS2=-Og -g3 -Wall -fmessage-length=0 -ffunction-sections -c -fmessage-length=0
LDFLAGS=-specs=nosys.specs -specs=nano.specs -Tcubemx/nucleo-f103rb/STM32F103RBTx_FLASH.ld -Wl,--gc-sections -lm
CC=arm-none-eabi-gcc
AS=arm-none-eabi-as
RUSTC=rustc -O -g -Z no-landing-pads --target thumbv6m-none-eabi --emit obj -L ../libcore-thumbv6m

startup_stm32f103xb.o: cubemx/nucleo-f103rb/startup/startup_stm32f103xb.s
	$(AS) $(CFLAGS) -o $@ $<

main.o: src/main.rs
	$(RUSTC) -o $@ $<

led.elf: main.o startup_stm32f103xb.o
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


## デバッグ

これまで見たように、フロントエンドは Rust だが、バックエンドは gcc なので、普通に gdb が使える。

OpenOCD を使って書き込んだ後で、OpenOCD のサーバを起動し、`target remote localhost:3333`で接続する。

```
$ arm-none-eabi-gdb led.elf
GNU gdb (7.10-1ubuntu3+9) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "--host=x86_64-linux-gnu --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from led.elf...done.
(gdb) target remote localhost:3333
Remote debugging using localhost:3333
0x080001e0 in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) continue 
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x080001e0 in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) l
503	{
504	    type Item = A;
505	
506	    #[inline]
507	    fn next(&mut self) -> Option<A> {
508	        if self.start < self.end {
509	            let mut n = self.start.add_one();
510	            mem::swap(&mut n, &mut self.start);
511	            Some(n)
512	        } else {
(gdb) finish
Run till exit from #0  0x080001e0 in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
main::main () at /home/nkon/src/rust/led/src/main.rs:52
52	        for _ in 1..400000 {
(gdb) ni
0x080001de in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) si
0x080001e0	508	        if self.start < self.end {
(gdb) finish 
Run till exit from #0  0x080001e0 in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
main::main () at /home/nkon/src/rust/led/src/main.rs:52
52	        for _ in 1..400000 {
(gdb) si
0x080001de in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) next
^C
Program received signal SIGINT, Interrupt.
0x080001e0 in core::iter::range::{{impl}}::next<i32> (self=<optimized out>)
    at /home/nkon-ubuntu1604/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) quit
A debugging session is active.

	Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: /home/nkon/src/rust/led/led.elf, Remote target
Ending remote debugging.
```

## HAL をリンクする

このように、MCUのレジスタ・アドレスをデータシートで調べて、バイナリを書き込めば、Rust でプログラムできるが、いちいち調べるのもたいへんだ。そのために CubeMXやHALが用意されているが、それは C で書かれている。Rust の FFI(多言語インターフェイス)を使って、Cで書かれたHALをリンクすれば良い。



## Cargo + build.rs