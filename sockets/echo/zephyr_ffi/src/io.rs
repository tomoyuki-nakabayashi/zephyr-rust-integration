//! IO wrapper which provides `safe` println macro using Zephyr API.

use bindings::zephyr::printf;
use core::fmt;
use cty;

const WRITE_BUF_LEN: usize = 256;

/// Pseudo writer which uses Zephyr `printf` API.
/// Because `printf` does not guarantee its atomicity, this wrapper
/// does not provide any lock mechanism.
pub struct DebugWriter {}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO: Remove the data copy.
        // Now an additional copy is required because the argument `s` is NOT
        // NULL terminated as C string.
        // To solve this problem, we need a net format macro
        // which inserts `\0` in the end of the argument `s`.
        let iter = s.bytes().chain("\0".bytes());
        let mut buf: [u8; WRITE_BUF_LEN] = [0; WRITE_BUF_LEN];
        for (i, byte) in iter.enumerate() {
            buf[i] = byte;
        }

        // safe: `printf` does not need to guarantee the atomicity.
        // Both `fmt` and `buf` are null-terminated.
        unsafe {
            printf(b"%s\0".as_ptr() as *const cty::c_char, &buf);
        }
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but calls printf
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but calls printf
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = DebugWriter {};
    writer.write_fmt(args).unwrap(); // Always returns Ok.
}
