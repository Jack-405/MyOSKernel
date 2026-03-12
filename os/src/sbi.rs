#![allow(unused)]

use core::hint::unreachable_unchecked;


/// use sbi call to putchar in console (qemu uart handler)
#[allow(deprecated)]
pub fn console_putchar(c:usize) {
    unsafe {
        sbi_rt::legacy::console_putchar(c);
    }
}

//shut down 
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if failure {
        system_reset(Shutdown, SystemFailure);
        
    }else {
        system_reset(Shutdown, NoReason);
    }
    unreachable!();
}