//! SBI console driver, for text output

use crate::sbi::console_putchar;
use core::fmt::{self, Write};
//Standard output struct
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            console_putchar(b as usize);
        }
        Ok(())
    }

    
}

pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr $(, $($arg:tt)+)?) => {
        $crate::print!(concat!($fmt, "\n") $(, $($arg)+)?);

    };
}