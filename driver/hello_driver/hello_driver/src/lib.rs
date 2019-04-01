#![no_std]

use zephyr_ffi::{print, println};

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello from Rust!\n");
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic! {:?}", info);
    loop {}
}
