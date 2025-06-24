# 使用gdb调试

STM32F3DISCOVRETY开发板中有ST-LINK接口，probe-rs与ST-LINK连接，GDB再与probe-rs连接，即可使用GDB调试。

## 创建`./Embed.toml`

为了让probe-rs与GDB进行通信，需要进行一些配置，创建配置文件`./Embed.toml`,内容为：
```toml
[default.probe]
protocol = "Swd"

[default.flashing]
enabled = true
restore_unwritten_bytes = false

[default.reset]
enabled = true

[default.general]
chip = "STM32F303VCTx"
log_level = "WARN"
connect_under_reset = false

[default.rtt]
enabled = true
up_mode = "NoBlockSkip"

[default.gdb]
enabled = true
gdb_connection_string = "127.0.0.1:1337"
```

## 配置文件详解

这个 `Embed.toml` 配置文件是 probe-rs 工具的配置文件，用于配置调试器与 STM32F3 开发板的连接和调试参数。下面是各个配置段的详细说明：

### `[default.probe]` - 调试探针配置
- `protocol = "Swd"`: 指定使用 SWD (Serial Wire Debug) 协议进行调试通信。SWD 是 ARM Cortex-M 处理器常用的调试协议，只需要两根线（SWDIO 和 SWCLK）。

### `[default.flashing]` - 程序烧录配置
- `enabled = true`: 启用程序烧录功能，允许 probe-rs 将编译好的程序烧录到芯片的 Flash 存储器中。
- `restore_unwritten_bytes = false`: 不恢复未写入的字节。设置为 false 可以提高烧录速度，因为不需要保留 Flash 中未被覆盖的原有数据。

### `[default.reset]` - 复位配置
- `enabled = true`: 启用芯片复位功能。在调试开始时会自动复位芯片，确保程序从头开始执行。

### `[default.general]` - 通用配置
- `chip = "STM32F303VCTx"`: 指定目标芯片型号为 STM32F303VCTx，这是 STM32F3DISCOVERY 开发板上使用的微控制器。
- `log_level = "WARN"`: 设置日志级别为警告级别，只显示警告和错误信息，减少调试输出的冗余信息。
- `connect_under_reset = false`: 不在复位状态下连接芯片。设置为 false 表示正常连接，不需要保持芯片在复位状态。

### `[default.rtt]` - RTT (Real Time Transfer) 配置
- `enabled = true`: 启用 RTT 功能。RTT 是一种实时数据传输技术，允许在调试过程中高速传输数据而不影响程序执行。
- `up_mode = "NoBlockSkip"`: 设置上行数据传输模式。"NoBlockSkip" 表示当缓冲区满时跳过新数据而不阻塞程序执行，确保实时性。

### `[default.gdb]` - GDB 调试器配置
- `enabled = true`: 启用 GDB 调试功能，允许使用 GDB 调试器连接到 probe-rs。
- `gdb_connection_string = "127.0.0.1:1337"`: 设置 GDB 连接字符串。probe-rs 会在本地 IP 地址 127.0.0.1 的 1337 端口上启动 GDB 服务器，GDB 客户端可以通过这个地址连接进行调试。

这个配置文件确保了 probe-rs 能够正确识别和连接 STM32F3DISCOVERY 开发板，并提供完整的调试功能，包括程序烧录、芯片复位、实时数据传输和 GDB 调试支持。

## 运行probe-rs

执行：
```bash
cargo embed
```
即可运行probe-rs。
运行成功将输出类似：
```bash
Terminal
Starting simple LED blink program
22:14:10.158: LED blink initialized on PE8
22:14:10.158: LED ON
```
的内容。

## 运行GDB

执行：
```bash
gdb-multiarch -q target/thumbv7em-none-eabihf/debug/pac
```
即可运行GDB。
运行成功将输出类似：
```bash
Reading symbols from target/thumbv7em-none-eabihf/debug/pac...
(gdb)
```
的内容。

连接probe-rs，在gdb命令输入框中执行：
```bash
target remote 127.0.0.1:1337
```
成功后将输出类似：
```bash
(gdb) target remote 127.0.0.1:1337
Remote debugging using 127.0.0.1:1337
0x08000b5e in core::iter::range::{impl#5}::spec_next<u32> (self=0x20009e5c)
    at /home/a/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:764
764	    fn spec_next(&mut self) -> Option<T> {
(gdb)
```
的内容。

之后可使用gdb命令进行调试。