//! IO wrapper which provides `safe` println macro using Zephyr API.

use bindings::zephyr::printf;
use core::fmt;
use cty;

static mut WRITER: DebugWriter = DebugWriter {};

#[derive(Default)]
pub struct DebugWriter {}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            printf(
                b"%s\0".as_ptr() as *const cty::c_char,
                s.as_ptr() as *const cty::c_char,
            );
        }
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but calls printk
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print(format_args!($($arg)*)));
}

/// Like the `print!` macro in the standard library, but calls printk
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}
