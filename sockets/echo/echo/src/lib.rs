#![no_std]

use bindings::zephyr;
use cty;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        zephyr::printf(b"Hello from %s\0".as_ptr() as *const cty::c_char,
                       b"Rust\n\0".as_ptr());
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
