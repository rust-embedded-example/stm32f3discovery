# BSP工程

本章节的工程建立在 [stm32f3-discovery](https://github.com/rubberduck203/stm32f3-discovery) crate之上。这个crate使用了HAL库，HAL库使用了PAC库。

## 什么是BSP

BSP（Board Support Package，板级支持包）是为特定开发板提供硬件抽象层的软件包。它封装了开发板上硬件外设的驱动程序和初始化代码，为应用程序提供简洁易用的API接口。

在我们的STM32F3 Discovery项目中，BSP主要提供：
- LED控制接口
- 延时功能
- 硬件初始化

## BSP架构层级

```
应用层 (main.rs)
    ↓
BSP层 (bsp.rs) - 板级抽象
    ↓  
HAL层 (stm32f3xx-hal) - 硬件抽象层
    ↓
PAC层 (stm32f3xx-pac) - 外设访问层
    ↓
硬件层 (STM32F303 MCU)
```

## 主要功能

### 1. LED控制

BSP提供8个LED的统一控制接口：

```rust
pub type LedArray = [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];
```

每个LED支持以下操作：
- `led.on()` - 点亮LED
- `led.off()` - 熄灭LED
- `led.toggle()` - 切换LED状态

### 2. 延时功能

提供基于系统滴答定时器的精确延时：

```rust
pub use stm32f3xx_hal::delay::Delay;
```

支持毫秒级延时：
- `delay.delay_ms(time)` - 延时指定毫秒数

### 3. 硬件初始化

`init()` 函数负责完成所有硬件的初始化工作：

```rust
pub fn init() -> (Delay, LedArray) {
    // 获取外设句柄
    let device_periphs = pac::Peripherals::take().unwrap();
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    
    // 配置时钟系统
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    
    // 初始化延时器
    let delay = Delay::new(core_periphs.SYST, clocks);
    
    // 初始化LED阵列
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(/* GPIO引脚配置 */);
    
    (delay, leds.into_array())
}
```

## 使用示例

完整的BSP使用示例（参考`main.rs`）：

```rust
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f303::bsp::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    // 初始化BSP
    let (mut delay, mut leds): (Delay, LedArray) = init();
    rprintln!("BSP初始化完成！");
    
    let half_period = 500_u16;
    let mut counter = 0u32;

    // 关闭所有LED
    for led in leds.iter_mut() {
        led.off().ok();
    }

    loop {
        delay.delay_ms(half_period);

        // 实现流水灯效果
        for led in leds.iter_mut() {
            led.off().ok();
        }

        // 点亮三个连续的LED
        let positions = [counter % 8, (counter + 1) % 8, (counter + 2) % 8];
        for &pos in &positions {
            leds[pos as usize].on().ok();
        }

        rprintln!("流水灯位置: {:?}", positions);
        counter += 1;

        if counter % 8 == 0 {
            counter = 0;
        }
    }
}
```

## 依赖库说明

BSP工程的主要依赖：

- **stm32f3-discovery**: STM32F3 Discovery开发板的BSP库
- **stm32f3xx-hal**: STM32F3系列的HAL库
- **cortex-m**: Cortex-M处理器核心库
- **cortex-m-rt**: Cortex-M运行时库
- **rtt-target**: RTT调试输出库
- **panic-rtt-target**: RTT恐慌处理库

## BSP的优势

1. **硬件抽象**: 隐藏底层硬件细节，提供统一的API接口
2. **代码复用**: 同一套BSP可以支持多个应用程序
3. **易于维护**: 硬件相关代码集中管理，便于更新和维护
4. **快速开发**: 简化硬件操作，专注于应用逻辑开发
5. **可移植性**: 通过BSP抽象，应用代码可以较容易地移植到其他硬件平台
