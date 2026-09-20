#![no_std]
#![no_main]

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard {
    // 你的键盘配置，可以引用 keyboard.toml
}