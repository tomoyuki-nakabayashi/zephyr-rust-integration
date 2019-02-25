#![no_std]

use bindings::zephyr;
use core::mem::size_of;
use cty;

const PORT: u16 = 4242u16;

#[no_mangle]
pub extern "C" fn rust_main() {
    let server_desc = socket_init();
    unsafe { zephyr::printf("TCP echo server waits for a connection on port %u...\n\0".as_ptr()  as *const cty::c_char,
        PORT as cty::c_uint); }

    loop {
        let client_desc = establish_connection(server_desc);

        loop {
            if echo(client_desc) < 0 {
                break;
            }
        }

        unsafe { zephyr::_impl_zsock_close(client_desc) };
        unsafe { zephyr::printf("Connection closed\n\0".as_ptr()  as *const cty::c_char); }
    }
}

/// Initialize socket.
/// Return the new socket file descriptor of server.
#[no_mangle]
pub extern "C" fn socket_init() -> i32 {
    let server = unsafe {
        zephyr::_impl_zsock_socket(
            zephyr::AF_INET as i32,
            zephyr::net_sock_type_SOCK_STREAM as i32,
            zephyr::net_ip_protocol_IPPROTO_TCP as i32,
        )
    };

    let bind_addr = {
        // TODO: calculate statically.
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

    if unsafe { zephyr::_impl_zsock_bind(server, &bind_addr, size_of::<zephyr::sockaddr>()) } < 0 {
        unsafe { zephyr::printf(b"error: bind\n\0".as_ptr() as *const cty::c_char) };
        panic!();
    }

    if unsafe { zephyr::_impl_zsock_listen(server, 5) } < 0 {
        unsafe { zephyr::printf(b"error: listen\n\0".as_ptr() as *const cty::c_char) };
        panic!();
    }

    unsafe {
        zephyr::printf(
            b"Socket descriptor is %d\n\0".as_ptr() as *const cty::c_char,
            server,
        );
    }
    server
}

/// Establish connection with a client.
///
/// - server_dsc: socket file descriptor of server.
/// - return: client socket file descriptor.
#[no_mangle]
pub extern "C" fn establish_connection(server_dsc: cty::c_int) -> i32 {
    unsafe {
        zephyr::printf(
            b"wait for client on socket #%d\n\0".as_ptr() as *const cty::c_char,
            server_dsc,
        )
    };
    let mut client_addr = zephyr::sockaddr {
        sa_family: 0,
        data: [0; 6],
    };
    let mut client_addr_len = size_of::<zephyr::sockaddr>();
    let client_dsc =
        unsafe { zephyr::_impl_zsock_accept(server_dsc, &mut client_addr, &mut client_addr_len) };

    if client_dsc < 0 {
        unsafe { zephyr::printf(b"error: accept\n\0".as_ptr() as *const cty::c_char) };
        panic!();
    }

    let ip_addr = &client_addr.data[2..];
    unsafe {
        zephyr::printf(
            b"Connection from %d.%d.%d.%d\n\0".as_ptr() as *const cty::c_char,
            u32::from(ip_addr[0]),
            u32::from(ip_addr[1]),
            u32::from(ip_addr[2]),
            u32::from(ip_addr[3]),
        );
    }

    client_dsc
}

#[no_mangle]
pub extern "C" fn echo(client_desc: cty::c_int) -> cty::ssize_t {
    let mut buf: [cty::c_char; 128] = [0; 128];
    let len = unsafe {
        zephyr::_impl_zsock_recvfrom(
            client_desc,
            buf.as_mut_ptr() as *mut cty::c_void,
            buf.len(),
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };

    if len < 0 {
        return len;
    }

    unsafe {
        zephyr::_impl_zsock_sendto(
            client_desc,
            buf.as_mut_ptr() as *mut cty::c_void,
            len as usize,
            0,
            core::ptr::null_mut(),
            0,
        )
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        zephyr::printf(b"panic!".as_ptr() as *const cty::c_char);
    }
    loop {}
}
