# 创建PAC crate

pac 是使用`svd2rust`工具处理芯片的`svd`文件得到的。

## 获取stm32f303的`svd`文件

推荐从官网获取,[点击此处,开始下载](https://www.st.com/resource/en/svd/stm32f3-svd.zip)。将跳转到ST官网并进行下载。下载完成后得到一个压缩包，解压并找到`STM32F303.svd`文件。

## 创建stm32f303pac crate

在根目录下创建一个crate：
```bash
cargo new stm32f303pac
```

将`STM32F303.svd`文件移动到`./stm32f303pac/下。

## 安装`svd2rust`

`svd2rust`用以提取`STM32F303.svd`文件中的芯片信息，并生成crate文件。运行：
```bash
cargo install svd2rust
```

## 安装`cargo-form`

安装`cargo-form`用以格式化pac crate，运行：
```bash
cargo install form
```

## 生成pac crate

在路径`./stm32f303pac/`下运行：
```bash
svd2rust -i STM32F303.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
```

## 修改`./stm32f303pac/Cargo.toml`

需要修改`./stm32f303pac/Cargo.toml`，使得启用`rt`,更方便使用这个crate。修改后如下：
```bash
[package]
name = "stm32f303pac"
version = "0.1.0"
edition = "2024"

[dependencies]
critical-section = { version = "1.2", optional = true }
cortex-m = "0.7.7"
cortex-m-rt = { version = "0.7.5", optional = true }
vcell = "0.1.3"

[features]
rt = ["cortex-m-rt/device"]
```

## 编译
在`./stm32f303pac/`路径下运行:
```bash
cargo build
```
若有错误与告警，按提示修复错误与告警，可以使用AI辅助。

完成后得到一个`stm32f303pac crate`。