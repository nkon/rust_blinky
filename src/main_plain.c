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
