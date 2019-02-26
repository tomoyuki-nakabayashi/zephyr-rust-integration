#![no_std]

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
            if echo(client_desc).is_err() {
                break;
            }
        }

        socket::close(client_desc).expect("fail to close");
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
pub extern "C" fn echo(client_desc: cty::c_int) -> Result<isize, Errno> {
    let mut buf: [cty::c_char; 128] = [0; 128];
    let len = socket::recv(client_desc, &mut buf)?;
    let buf = &buf[0..len as usize];

    socket::send(client_desc, &buf)
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("panic!");
    loop {}
}
