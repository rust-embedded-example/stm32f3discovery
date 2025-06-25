// 简化的中断处理方案
// 使用宏来避免重复代码，这是避免显式定义每个中断函数的最佳方法

use stm32f3xx_hal::pac::interrupt;

// 定义一个宏来生成默认的中断处理程序
// 这样我们只需要写一次逻辑，宏会为所有中断生成相同的处理程序
macro_rules! default_interrupt_handlers {
    ($($interrupt:ident),*) => {
        $(
            #[interrupt]
            fn $interrupt() {
                // 默认处理：什么都不做，直接返回
                // 这比无限循环更安全，允许系统继续运行
                // 如果需要，可以在这里添加日志或错误处理
            }
        )*
    };
}

// 使用宏生成所有中断处理程序
// 这种方法的优点：
// 1. 代码简洁，避免重复
// 2. 易于维护
// 3. 可以统一修改所有中断的默认行为
// 4. 符合 DRY (Don't Repeat Yourself) 原则
default_interrupt_handlers!(
    WWDG, PVD, TAMP_STAMP, RTC_WKUP, FLASH, RCC, EXTI0, EXTI1, EXTI2_TSC, EXTI3, EXTI4,
    DMA1_CH1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5, DMA1_CH6, DMA1_CH7, ADC1_2,
    USB_HP_CAN_TX, USB_LP_CAN_RX0, CAN_RX1, CAN_SCE, EXTI9_5, TIM1_BRK_TIM15, TIM1_UP_TIM16,
    TIM1_TRG_COM_TIM17, TIM1_CC, TIM2, TIM3, TIM4, I2C1_EV_EXTI23, I2C1_ER, I2C2_EV_EXTI24,
    I2C2_ER, SPI1, SPI2, USART1_EXTI25, USART2_EXTI26, USART3_EXTI28, EXTI15_10, RTCALARM,
    USB_WKUP, USB_WKUP_EXTI, TIM8_BRK, TIM8_UP, TIM8_TRG_COM, TIM8_CC, ADC3, FMC, SPI3,
    UART4_EXTI34, UART5_EXTI35, TIM6_DACUNDER, TIM7, DMA2_CH1, DMA2_CH2, DMA2_CH3, DMA2_CH4,
    DMA2_CH5, ADC4, COMP1_2_3, COMP4_5_6, COMP7, I2C3_EV, I2C3_ER, USB_HP, USB_LP,
    TIM20_BRK, TIM20_UP, TIM20_TRG_COM, TIM20_CC, FPU, SPI4
);

// 如果您需要为特定中断实现自定义处理逻辑，
// 只需在这个文件中重新定义该中断处理函数即可，例如：
//
// #[interrupt]
// fn TIM2() {
//     // 自定义的 TIM2 中断处理逻辑
//     // 这会覆盖宏生成的默认处理程序
// }