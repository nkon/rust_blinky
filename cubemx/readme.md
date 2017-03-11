# Linux + STM32 で Lチカする

開発環境の確認のため Lチカする。

## ターゲットホード

展示会などでもよく配られている <a href="http://www.st.com/ja/evaluation-tools/nucleo-f103rb.html">Nucleo-F103RB</a> をターゲットホードにする。主要スペックは次のとおり。

* MCU
    + <a href="http://www.st.com/content/st_com/ja/products/microcontrollers/stm32-32-bit-arm-cortex-mcus/stm32f1-series/stm32f103/stm32f103rb.html">STM32F103RB</a>
* ARM® 32-bit Cortex® -M3 CPU Core
    + 72 MHz maximum frequency,1.25 DMIPS/MHz (Dhrystone 2.1) performance at 0 wait state memory access
    + Single-cycle multiplication and hardware division
* Memories
    + 64 or 128 Kbytes of Flash memory
    + 20 Kbytes of SRAM

## CubeMX

STのマイコンは、STM32CubeMX というツールで、ベリフェラルを設定して、HALのコードをインポートしたプロジエクトの雛形を生成することができる。

* まずは、<a href="http://www.st.com/content/st_com/ja/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-configurators-and-code-generators/stm32cubemx.html">CubeMX</a>をダウンロード。個人情報が必要になるが仕方がない。
* アーカイブを展開して、Linuxのインストーラを実行すればインストーラ完了。
* STM32F1 のファームウエアをダウンロードしておく。
* ボードを適切に選ぶと、ボードに搭載されているLEDに合わせて、I/Oが設定される。
* 生成タイプに SW4STM32 を選んで Code Generate する。
* プロジエクトを SW4STM32 で開く。

## SW4STM32

<a href="http://www.openstm32.org/HomePage">SW4STM32</a> は、Cross GCC と Eclipse を組み合わせた STM32 向けの IDE だ。

* メンバー登録をして、インストーラをダウンロードしてインストール。

## OpenOCD

Ubuntu なら apt-get で入る。バージョンが古いが、STM32なら大丈夫。念の為、Windows 上の ST-Link Utility でファームアップしておくといい。

## Lチカ

CubeMXで生成した後、main.c に Lチカ・コードを追加する。

* `MX_GPIO_Init()`で、CubeMXで設定したとおりに I/O が設定される。
* `HAL_GPIO_WritePin()`は、HALの関数。
* `LD2_GPIO_Port`、`LD2_Pin`は、CubeMX で Nucleo のボード設定を読み込んだら、PA5にLD2の別名が設定されたので、ポート名ではなく機能名で呼べるのだ。

```
int main(void)
{

  /* USER CODE BEGIN 1 */
	uint32_t i;
  /* USER CODE END 1 */

  /* MCU Configuration----------------------------------------------------------*/

  /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
  HAL_Init();

  /* Configure the system clock */
  SystemClock_Config();

  /* Initialize all configured peripherals */
  MX_GPIO_Init();

  /* USER CODE BEGIN 2 */

  /* USER CODE END 2 */

  /* Infinite loop */
  /* USER CODE BEGIN WHILE */
  while (1)
  {
  /* USER CODE END WHILE */

  /* USER CODE BEGIN 3 */
	  for(i = 0; i < 1000000; i++){
	  }
	  HAL_GPIO_WritePin(LD2_GPIO_Port, LD2_Pin, GPIO_PIN_RESET);
	  for(i = 0; i < 1000000; i++){
	  }
	  HAL_GPIO_WritePin(LD2_GPIO_Port, LD2_Pin, GPIO_PIN_SET);

  }
  /* USER CODE END 3 */

}
```

デフォルトの設定でビルド。Project の右クリック→Target→Program Chip...で OpenOCD→ST-Link経由で焼ける。

通常の Eclipse IDE として Debug もできる。

## システムのツールチェイン

Ac6 の IDE では `~/Ac6/SystemWorkbench/plugins/fr.ac6.mcu.externaltools.arm-none.linux64_1.13.1.201703061524/tools/compiler/`以下のツールが使われるが、基本的には、システムのツールを使いたい。Ac6のツールは、IDEからはバージョンアップができて、arm-none-eabi-gcc 5.4.1 だった。一方、apt-get で標準のリポジトリから入るのは arm-none-eabi-gcc 4.9.3 だった。OpenOCDも、Ac6のものは 現時点で最新の 0.10.0だったが、システムのものは 0.9.0だった(わりとがんばっている)。

IDEでビルドすると Debug/makefile をはじめとする makefile が生成されるので、コマンドラインからビルドでき、その時は、パスが通っているシステムのツールが使われる。

```
$ pwd
........../Debug
$ find . -name '*.o' | xargs rm      # IDE が作ったのを消す。
$ make all
Building file: ../startup/startup_stm32f103xb.s
Invoking: MCU GCC Assembler
........../nucleo-f103rb/Debug
arm-none-eabi-as -mcpu=cortex-m3 -mthumb -mfloat-abi=soft -g -o "startup/startup_stm32f103xb.o" "../startup/startup_stm32f103xb.s"
Finished building: ../startup/startup_stm32f103xb.s
 
Building file: ../Src/main.c
Invoking: MCU GCC Compiler
........../nucleo-f103rb/Debug
arm-none-eabi-gcc -mcpu=cortex-m3 -mthumb -mfloat-abi=soft '-D__weak=__attribute__((weak))' '-D__packed="__attribute__((__packed__))"' -DUSE_HAL_DRIVER -DSTM32F103xB -I"........../nucleo-f103rb/Inc" -I"........../nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Inc" -I"........../nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Inc/Legacy" -I"........../nucleo-f103rb/Drivers/CMSIS/Device/ST/STM32F1xx/Include" -I"........../nucleo-f103rb/Drivers/CMSIS/Include"  -Og -g3 -Wall -fmessage-length=0 -ffunction-sections -c -fmessage-length=0 -MMD -MP -MF"Src/main.d" -MT"Src/main.o" -o "Src/main.o" "../Src/main.c"
Finished building: ../Src/main.c
 
...略...
 
Building target: nucleo-f103rb.elf
Invoking: MCU GCC Linker
arm-none-eabi-gcc -mcpu=cortex-m3 -mthumb -mfloat-abi=soft -specs=nosys.specs -specs=nano.specs -T"../STM32F103RBTx_FLASH.ld" -Wl,-Map=output.map -Wl,--gc-sections -lm -o "nucleo-f103rb.elf" @"objects.list"  
/usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/bin/ld: warning: /usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/lib/armv7-m/libc_nano.a(lib_a-atexit.o) uses 2-byte wchar_t yet the output is to use 4-byte wchar_t; use of wchar_t values across objects may fail
...略...   # wchar_t の warning がでているが、問題なし。 
Finished building target: nucleo-f103rb.elf
 
make --no-print-directory post-build
Generating binary and Printing size information:
arm-none-eabi-objcopy -O binary "nucleo-f103rb.elf" "nucleo-f103rb.bin"
arm-none-eabi-size "nucleo-f103rb.elf"
   text	   data	    bss	    dec	    hex	filename
   4320	     12	   1572	   5904	   1710	nucleo-f103rb.elf     # サイズは Ac6 の時と異なる。

$ sudo openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image nucleo-f103rb.elf" -c "reset halt" -c "reset run" -c "exit"
Open On-Chip Debugger 0.9.0 (2015-09-02-10:42)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
none separate
srst_only separate srst_nogate srst_open_drain connect_deassert_srst
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.249934
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
target state: halted
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08000fbc msp: 0x20005000
Info : device id = 0x20036410
Info : flash size = 128kbytes
stm32x mass erase complete
target state: halted
target halted due to breakpoint, current mode: Thread 
xPSR: 0x61000000 pc: 0x2000003a msp: 0x20005000
wrote 4332 bytes from file nucleo-f103rb.elf in 0.183826s (23.013 KiB/s)
target state: halted
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08001004 msp: 0x20005000
```
## Makefile

これを参考に、Makefile を手書きする。

```
CFLAGS=-mcpu=cortex-m3 -mthumb -mfloat-abi=soft
DFLAGS='-D__weak=__attribute__((weak))' '-D__packed="__attribute__((__packed__))"' -DUSE_HAL_DRIVER -DSTM32F103xB
IFLAGS= -Icubemx/nucleo-f103rb/Inc
IFLAGS+=-Icubemx/nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Inc
IFLAGS+=-Icubemx/nucleo-f103rb/Drivers/CMSIS/Device/ST/STM32F1xx/Include
IFLAGS+=-Icubemx/nucleo-f103rb/Drivers/CMSIS/Include
CFLAGS2=-Og -g3 -Wall -fmessage-length=0 -ffunction-sections -c -fmessage-length=0
LDFLAGS=-specs=nosys.specs -specs=nano.specs -Tcubemx/nucleo-f103rb/STM32F103RBTx_FLASH.ld -Wl,-Map=output.map -Wl,--gc-sections -lm
CC=arm-none-eabi-gcc
AS=arm-none-eabi-as

startup_stm32f103xb.o:cubemx/nucleo-f103rb/startup/startup_stm32f103xb.s
	$(AS) $(CFLAGS) -o $@ $<

main.o:cubemx/nucleo-f103rb/Src/main.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_hal_msp.o:cubemx/nucleo-f103rb/Src/stm32f1xx_hal_msp.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_it.o:cubemx/nucleo-f103rb/Src/stm32f1xx_it.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

system_stm32f1xx.o:cubemx/nucleo-f103rb/Src/system_stm32f1xx.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_hal_gpio.o:cubemx/nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_gpio.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_hal_cortex.o:cubemx/nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_cortex.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_hal_rcc.o:cubemx/nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_rcc.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

stm32f1xx_hal.o:cubemx/nucleo-f103rb/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal.c
	$(CC) $(CFLAGS) $(DFLAGS) $(IFLAGS) $(CFLAGS2) -o $@ $<

led.elf:stm32f1xx_hal.o stm32f1xx_hal_cortex.o stm32f1xx_hal_gpio.o stm32f1xx_hal_rcc.o main.o stm32f1xx_hal_msp.o system_stm32f1xx.o stm32f1xx_it.o startup_stm32f103xb.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

.PHONY: flash
flash:
	sudo openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image led.elf" -c "reset halt" -c "reset run" -c "exit"

.PHONY: clean
clean:
	rm *.elf *.o *.map
```

## HALを使わない

更に単純化して HAL を使わずに、レジスタ直叩きする。

* クロックはデフォルトのまま。
* GPIOにクロックを供給。
* GPIOのモードをセット。
* BSRR に書き込んで、ポートを ON/OFF。
* system_stm32f1xx.c にあった SystemInit(スタートアップから呼ばれる) を main.c に移動。
* レジスタアドレスの定義をした。おかげで、`#include`や、ビルド時の`-I`が不要になった。ついでに、`-D`も不要になった。

```
#define PERIPH_BASE         (0x40000000) /*!< Peripheral base address in the alias region */
#define APB2PERIPH_BASE     (PERIPH_BASE + 0x10000)
#define GPIOA_BASE          (APB2PERIPH_BASE+0x0800)
#define CRL_OFFSET          0x00
#define BSRR_OFFSET         0x10
#define GPIO_PIN_5          (0x0020)  /* Pin 5 selected    */

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

int main(void)
{
    volatile unsigned long *crl;
    volatile unsigned long *bsrr;
    volatile unsigned long *apb2enr;
    unsigned long i;

    bsrr    = (unsigned long *)(GPIOA_BASE+BSRR_OFFSET);
    crl     = (unsigned long *)(GPIOA_BASE+CRL_OFFSET);
    apb2enr = (unsigned long *)(RCC_BASE+APB2ENR_OFFSET);

    *apb2enr|= 1 << 2;
    *crl &= (~(6 << 20));   // clear CNF5 : PP mode
    *crl |= (2 << 20);      // set MODE5: Output 2MHz

    while(1){
        for(i = 0; i < 400000; i++){
	    }
        *bsrr = 0x0020 << 16;
	    for(i = 0; i < 400000; i++){
	    }
        *bsrr = 0x0020;
    }
}

void SystemInit (void)
{
    volatile unsigned long *rcc_reg;

    /* Reset the RCC clock configuration to the default reset state(for debug purpose) */
    /* Set HSION bit */
//  RCC->CR |= (uint32_t)0x00000001;
    rcc_reg = (unsigned long *)(RCC_BASE+CR_OFFSET);
    *rcc_reg |= 0x00000001;

    /* Reset SW, HPRE, PPRE1, PPRE2, ADCPRE and MCO bits */
//  RCC->CFGR &= (uint32_t)0xF0FF0000;
    rcc_reg = (unsigned long *)(RCC_BASE+CFGR_OFFSET);
    *rcc_reg &= 0xF0FF0000;
  
    /* Reset HSEON, CSSON and PLLON bits */
//  RCC->CR &= (uint32_t)0xFEF6FFFF;
    rcc_reg = (unsigned long *)(RCC_BASE+CR_OFFSET);
    *rcc_reg &= 0xFEF6FFFF;

    /* Reset HSEBYP bit */
    *rcc_reg &= 0xFFFBFFFF;

    /* Reset PLLSRC, PLLXTPRE, PLLMUL and USBPRE/OTGFSPRE bits */
//  RCC->CFGR &= (uint32_t)0xFF80FFFF;
    rcc_reg = (unsigned long *)(RCC_BASE+CFGR_OFFSET);
    *rcc_reg &= 0xFF80FFFF;

    /* Disable all interrupts and clear pending bits  */
//  RCC->CIR = 0x009F0000;
    rcc_reg = (unsigned long *)(RCC_BASE+CIR_OFFSET);
    *rcc_reg = 0x009F0000;

//  SCB->VTOR = FLASH_BASE | VECT_TAB_OFFSET; /* Vector Table Relocation in Internal FLASH. */
    rcc_reg = (unsigned long *)(SCB_BASE+VTOR_OFFSET);
    *rcc_reg = FLASH_BASE | VECT_TAB_OFFSET;
}

```
これの Makefile は次のとおり。

* startup_stm32f103xb.s: アセンブラのスタートアップ。
* main_plain.c: main と SystemInit(スタートアップから呼ばれる)
* STM32F103RBTx_FLASH.ld: リンカ・スクリプト。CubeMXが生成したのを、そのまま使う。

```
CFLAGS=-mcpu=cortex-m3 -mthumb -mfloat-abi=soft
CFLAGS2=-Og -g3 -Wall -fmessage-length=0 -ffunction-sections -c -fmessage-length=0
LDFLAGS=-specs=nosys.specs -specs=nano.specs -Tcubemx/nucleo-f103rb/STM32F103RBTx_FLASH.ld -Wl,--gc-sections -lm
CC=arm-none-eabi-gcc
AS=arm-none-eabi-as

startup_stm32f103xb.o: cubemx/nucleo-f103rb/startup/startup_stm32f103xb.s
	$(AS) $(CFLAGS) -o $@ $<

main.o: src/main_plain.c
	$(CC) $(CFLAGS) $(CFLAGS2) -o $@ $<

led.elf: main.o startup_stm32f103xb.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

.PHONY: flash
flash: led.elf
	sudo openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image led.elf" -c "reset halt" -c "reset run" -c "exit"

.PHONY: clean
clean:
	rm *.elf *.o *.map
```