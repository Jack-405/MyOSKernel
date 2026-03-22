#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod sync;

mod trap;
mod batch;
mod syscall;

use core::arch::global_asm;

// 引入汇编入口
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    
    println!("[kernel] rCore batch OS starting...");

    trap::init();          // 设置 stvec = __alltraps
    
    batch::init();    // 加载应用程序到内存
    batch::run_next_app(); // 构造 TrapContext → __restore → sret → 跳到用户态

    panic!("Unreachable in rust_main");
}

/// 清空 BSS 段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    for addr in sbss as usize..ebss as usize {
        unsafe { (addr as *mut u8).write_volatile(0); }
    }
}
