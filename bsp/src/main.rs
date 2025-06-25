#![deny(unsafe_code)]
#![no_main]
#![no_std]

// use cortex_m_semihosting::hprint;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f303::bsp::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let (mut delay, mut leds): (Delay, LedArray) = init();
    rprintln!("RTT initialized! Hello from STM32F3!");
    let half_period = 500_u16;

    let mut counter = 0u32;

    // 初始状态：所有灯灭
    for led in leds.iter_mut() {
        led.off().ok();
    }

    loop {
        delay.delay_ms(half_period);

        // 关闭所有LED
        for led in leds.iter_mut() {
            led.off().ok();
        }

        // 流水灯效果：三个连续LED移动
        let position0 = counter % 8;
        let position1 = (counter + 1) % 8;
        let position2 = (counter + 2) % 8;

        // 点亮三个连续的LED
        leds[position0 as usize].on().ok();
        leds[position1 as usize].on().ok();
        leds[position2 as usize].on().ok();

        rprintln!(
            "流水灯位置: (LED {},{},{} 亮)",
            position0,
            position1,
            position2
        );

        counter += 1;

        if counter % 8 == 0 {
            counter = 0;
        }
    }
}
