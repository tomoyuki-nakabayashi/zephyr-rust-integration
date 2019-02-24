#![no_std]

use bindings::zephyr;
use cty;

const PORT: u16 = 4242u16;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        zephyr::printf(b"Hello from %s\0".as_ptr() as *const cty::c_char,
                       b"Rust\n\0".as_ptr());
    }
}

#[no_mangle]
extern "C" fn socket_init() -> i32 {
    let serv = unsafe { zephyr::_impl_zsock_socket(zephyr::AF_INET as i32,
            zephyr::net_sock_type_SOCK_STREAM as i32,
            zephyr::net_ip_protocol_IPPROTO_TCP as i32) };

    let bind_addr = {
        let port = PORT.to_be_bytes();
        let addr = (zephyr::INADDR_ANY).to_be_bytes();

        let mut data: [u8; 6] = [0; 6];
        for (pos, byte) in port.iter().chain(&addr).enumerate() {
            data[pos] = *byte;
        }

        zephyr::sockaddr {
            sa_family: zephyr::AF_INET as cty::c_ushort,
            data,
        }
    };

    if unsafe { zephyr::_impl_zsock_bind(serv, &bind_addr, core::mem::size_of::<zephyr::sockaddr>()) } < 0 {
        unsafe { zephyr::printf(b"error: bind\n\0".as_ptr() as *const cty::c_char) };
        panic!();
    }

    if unsafe { zephyr::_impl_zsock_listen(serv, 5) } < 0 {
        unsafe { zephyr::printf(b"error: listen\n\0".as_ptr() as *const cty::c_char) };
        panic!();
    }

    serv
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
