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

## Makefile

IDEが自動的に作成した Makefile は `Debug/makefile` である。


