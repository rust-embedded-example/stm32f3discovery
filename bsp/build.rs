//! This build script sets up the linker to find the `memory.x` and `link.x` files
//! in the crate root directory and configures the necessary linker arguments.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link.x");

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    println!("cargo:rustc-link-arg=-Tlink.x");
}
