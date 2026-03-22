mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;

use core::arch::global_asm;

use riscv::register::{
    scause::{self, Trap},
    stval,
    stvec::{self, Stvec, TrapMode},
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(Stvec::new(__alltraps as usize, TrapMode::Direct));
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        // ================= syscall =================
        Trap::Exception(8) => { // UserEnvCall
            cx.sepc += 4; // 跳过 ecall

            cx.x[10] = syscall(
                cx.x[17],                     // syscall id (a7)
                [cx.x[10], cx.x[11], cx.x[12]] // a0,a1,a2
            ) as usize;
        }

        // ================= PageFault =================
        Trap::Exception(7) | Trap::Exception(15) => {
            println!("[kernel] PageFault, killed.");
            run_next_app();
        }

        // ================= Illegal =================
        Trap::Exception(2) => {
            println!("[kernel] IllegalInstruction, killed.");
            run_next_app();
        }

        _ => {
            println!(
                "[kernel] Unsupported trap {:?}, stval={:#x}",
                scause.cause(),
                stval
            );
            run_next_app();
        }
    }

    cx
}

pub use context::TrapContext;