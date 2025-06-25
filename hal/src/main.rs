#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f3xx_hal::{pac, prelude::*};

// critical-section 实现由 cortex-m 的 "critical-section-single-core" 特性自动提供

#[entry]
fn main() -> ! {
    // 初始化 RTT
    rtt_init_print!();
    rprintln!("STM32F3 Discovery RTT 初始化完成!");

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    rprintln!("LED 初始化完成，开始闪烁...");

    loop {
        led.toggle().unwrap();

        rprintln!("LED 闪烁");

        asm::delay(1_000_000);
    }
}
