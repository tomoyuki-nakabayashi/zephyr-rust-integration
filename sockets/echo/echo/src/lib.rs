#![no_std]

use zephyr_ffi::socket::{self, RawFd, Errno, AddressFamily, InetAddr, Ipv6Addr, SockProtocol, SockType};
use zephyr_ffi::{print, println};

const PORT: u16 = 4242u16;

#[no_mangle]
pub extern "C" fn rust_main() {
    // Create a new socket.
    let server = socket::socket(AddressFamily::Inet6, SockType::Stream, SockProtocol::Tcp)
        .expect("fail to create new socket.");
    let bind_addr = InetAddr::new(Ipv6Addr::any(), PORT);

    // Bind and listen on the created socket.
    socket::bind(server, &bind_addr).expect("fail to bind.");
    socket::listen(server, 5).expect("fail to listen.");

    println!("TCP echo server waits for a connection on port {}...", PORT);

    loop {
        println!("wait for client on socket #{}", server);
        let client = socket::accept(server).expect("fail to accept");

        // Echo, repeatedly until we encounter an error.
        while let Ok(_) = echo(client) {}

        socket::close(client).expect("fail to close");
        println!("Connection closed");
    }
}

fn echo(client_desc: RawFd) -> Result<isize, Errno> {
    let mut buf: [u8; 128] = [0; 128];
    let len = socket::recv(client_desc, &mut buf)?;
    let buf = &buf[0..len as usize];

    socket::send(client_desc, &buf)
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic! {:?}", info);
    loop {}
}
