#![no_std]

use bindings::zephyr;
use cty;
use zephyr_ffi::socket::{self, RawFd, Errno, AddressFamily, InetAddr, Ipv4Addr, SockProtocol, SockType};
use zephyr_ffi::{print, println};

const PORT: u16 = 4242u16;

#[no_mangle]
pub extern "C" fn rust_main() {
    let server_desc = socket_init().expect("fail to initialize socket.");
    println!("TCP echo server waits for a connection on port {}...", PORT);

    loop {
        let client_desc = establish_connection(server_desc);

        loop {
            if echo(client_desc) < 0 {
                break;
            }
        }

        unsafe { zephyr::_impl_zsock_close(client_desc) };
        println!("Connection closed");
    }
}

/// Initialize socket.
/// Return the new socket file descriptor of server.
#[no_mangle]
pub extern "C" fn socket_init() -> Result<RawFd, Errno> {
    let fd = socket::socket(AddressFamily::Inet, SockType::Stream, SockProtocol::Tcp)?;

    let bind_addr = InetAddr::new(Ipv4Addr::any(), PORT);
    socket::bind(fd, &bind_addr)?;
    socket::listen(fd, 5)?;

    Ok(fd)
}

/// Establish connection with a client.
///
/// - server_dsc: socket file descriptor of server.
/// - return: client socket file descriptor.
#[no_mangle]
pub extern "C" fn establish_connection(server_dsc: cty::c_int) -> i32 {
    println!("wait for client on socket #{}", server_dsc);
    socket::accept(server_dsc).expect("fail to accept")
}

#[no_mangle]
pub extern "C" fn echo(client_desc: cty::c_int) -> cty::ssize_t {
    let mut buf: [cty::c_char; 128] = [0; 128];
    let len = socket::recv(client_desc, &mut buf).expect("fail to recieve.");

    if len < 0 {
        return len as cty::ssize_t;
    }

    socket::send(client_desc, &buf, len as usize).expect("fail to send.") as isize
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("panic!");
    loop {}
}
