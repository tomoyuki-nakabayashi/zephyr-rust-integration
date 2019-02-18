#![no_std]

extern "C" {
    fn puts_c(c_str: *const u8);
}

#[no_mangle]
pub extern "C" fn rust_main() {
    const HELLO: &[u8] = b"Hello from Rust.\0";
    unsafe { puts_c(HELLO.as_ptr()) };
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}