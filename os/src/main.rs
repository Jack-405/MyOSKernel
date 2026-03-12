//! The main module and entrypoint
//!
//! Kernel starts from `entry.asm`, then jumps to `rust_main`.

#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
use crate::sbi::shutdown;

// 引入汇编入口
global_asm!(include_str!("entry.asm"));

/// 内核入口函数
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();

    println!("Hello, world!");
    println!("Hello, world!");

    shutdown(false);
}

/// 清空 BSS 段
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    for addr in sbss as usize..ebss as usize {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    }
}