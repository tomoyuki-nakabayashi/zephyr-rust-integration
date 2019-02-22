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

#[no_mangle]
extern "C" fn net_app_get_net_pkt_from_rust(
        ctx: *mut zephyr::net_app_ctx,
        pkt: *mut zephyr::net_pkt) -> *mut zephyr::net_pkt
{
    unsafe { zephyr::net_app_get_net_pkt(ctx, (*pkt).family() as u16, 100) }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}