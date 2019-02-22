#![no_std]

use bindings as zephyr;
use cty;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        zephyr::printk(b"Hello from %s\0".as_ptr() as *const cty::c_char,
                       b"Rust\0".as_ptr());
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}