# STM32F3Discovery Rust 嵌入式开发示例

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![STM32F3](https://img.shields.io/badge/STM32-F303-green.svg)](https://www.st.com/en/microcontrollers-microprocessors/stm32f3-series.html)

这是一个基于 STM32F3Discovery 开发板的 Rust 嵌入式开发教程项目，展示了如何使用 Rust 语言进行嵌入式系统开发。项目包含了从底层 PAC（外设访问层）到高层 BSP（板级支持包）的完整开发示例。

## 📋 项目概述

本项目旨在为 Rust 嵌入式开发者提供一个完整的学习路径，通过三个不同抽象层次的示例工程，帮助开发者理解 Rust 嵌入式开发的核心概念：

- **PAC 工程** - 直接使用外设访问层进行底层硬件操作
- **HAL 工程** - 使用硬件抽象层简化开发流程
- **BSP 工程** - 使用板级支持包实现高层抽象

## 🎯 目标硬件

- **开发板**: STM32F3Discovery
- **微控制器**: STM32F303VCT6
- **架构**: ARM Cortex-M4F
- **调试器**: 板载 ST-LINK/V2

## 🏗️ 项目结构

```
stm32f3discovery/
├── pac/                    # PAC 工程 - 外设访问层示例
│   ├── src/
│   │   └── main.rs        # 直接寄存器操作的 LED 闪烁
│   ├── stm32f303pac/      # 自定义 PAC 库
│   └── Cargo.toml
├── hal/                    # HAL 工程 - 硬件抽象层示例
│   ├── src/
│   │   └── main.rs        # 使用 HAL 库的 LED 闪烁
│   └── Cargo.toml
├── bsp/                    # BSP 工程 - 板级支持包示例
│   ├── src/
│   │   ├── main.rs        # 流水灯效果演示
│   │   └── bsp.rs         # BSP 抽象层实现
│   └── Cargo.toml
└── docs/                   # 详细文档
    ├── src/               # mdBook 源文件
    └── book.toml          # 文档配置
```

## ✨ 主要特性

### 🔧 技术特性
- **内存安全**: 利用 Rust 的所有权系统确保内存安全
- **零成本抽象**: 高级抽象不引入运行时开销
- **实时调试**: 使用 RTT (Real-Time Transfer) 进行实时日志输出
- **多层抽象**: 从底层寄存器到高层 API 的完整示例

### 💡 功能演示
- **LED 控制**: 8 个板载 LED 的独立控制
- **流水灯效果**: 动态的三 LED 流水灯演示
- **延时功能**: 精确的毫秒级延时控制
- **实时输出**: RTT 调试信息实时显示

## 🚀 快速开始

### 环境要求

1. **Rust 工具链** (1.70+)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabihf
```

2. **调试工具**
```bash
# 安装 probe-rs (推荐)
cargo install probe-rs --features cli

# 或者安装 OpenOCD
# Ubuntu/Debian
sudo apt install openocd

# macOS
brew install openocd
```

3. **硬件连接**
   - 使用 USB 线连接 STM32F3Discovery 开发板
   - 确保板载 ST-LINK 调试器正常工作

### 运行示例

#### 1. PAC 工程 - 底层寄存器操作
```bash
cd pac
cargo embed
```
**功能**: PE8 引脚上的 LED 简单闪烁，直接操作寄存器

#### 2. HAL 工程 - 硬件抽象层
```bash
cd hal
cargo embed
```
**功能**: 使用 HAL 库实现的 LED 闪烁，代码更简洁

#### 3. BSP 工程 - 板级支持包
```bash
cd bsp
cargo embed
```
**功能**: 8 个 LED 的流水灯效果，展示高层抽象的威力

### 查看调试输出

运行程序后，可以通过以下方式查看 RTT 输出：

```bash
# 使用 probe-rs
probe-rs rtt

# 或在另一个终端窗口中
cargo embed --no-flash
```

## 📚 学习路径

### 1. 初学者路径
1. 阅读 `docs/` 目录下的完整文档
2. 从 PAC 工程开始，理解底层硬件操作
3. 进阶到 HAL 工程，学习硬件抽象
4. 最后学习 BSP 工程，掌握高层抽象设计

### 2. 进阶开发者路径
1. 直接查看 BSP 工程的架构设计
2. 对比三个工程的实现差异
3. 根据需要选择合适的抽象层次

## 🔍 核心概念解析

### PAC (Peripheral Access Crate)
- **定义**: 外设访问层，提供对硬件寄存器的直接访问
- **特点**: 最大灵活性，完全控制硬件
- **适用场景**: 需要精确控制硬件或实现自定义驱动

### HAL (Hardware Abstraction Layer)
- **定义**: 硬件抽象层，在 PAC 基础上提供更友好的 API
- **特点**: 简化开发，减少样板代码
- **适用场景**: 标准外设操作，快速原型开发

### BSP (Board Support Package)
- **定义**: 板级支持包，为特定开发板提供高层抽象
- **特点**: 开箱即用，隐藏硬件细节
- **适用场景**: 应用开发，专注业务逻辑

## 🛠️ 开发工具

### 调试配置
每个工程都包含完整的调试配置：
- `Embed.toml` - probe-rs 配置
- `openocd.gdb` - OpenOCD + GDB 配置
- `memory.x` - 内存布局定义

### 构建配置
- 优化的发布配置（LTO、代码生成单元优化）
- RTT 调试支持
- 恐慌处理配置

## 📖 详细文档

项目包含完整的中文文档，使用 mdBook 构建：

```bash
cd docs
mdbook serve
```

文档内容包括：
- Rust 嵌入式开发介绍
- 环境搭建指南
- 三个工程的详细说明
- 调试技巧和最佳实践

## 🤝 贡献指南

欢迎贡献代码、文档或提出改进建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Rust Embedded Working Group](https://github.com/rust-embedded) - 提供优秀的嵌入式生态
- [stm32f3xx-hal](https://github.com/stm32-rs/stm32f3xx-hal) - STM32F3 HAL 实现
- [stm32f3-discovery](https://github.com/rubberduck203/stm32f3-discovery) - Discovery 板 BSP

## 📞 联系方式

- 项目维护者: anlang
- 邮箱: 2682525840@qq.com
- 项目地址: [https://github.com/rust-embedded-example/stm32f3discovery](https://github.com/rust-embedded-example/stm32f3discovery)

---

**开始你的 Rust 嵌入式开发之旅吧！** 🚀