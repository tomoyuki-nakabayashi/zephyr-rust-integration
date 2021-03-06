//! IO wrapper which provides `safe` socket interface using Zephyr socket API.
//! Currently supports only TCP & UDP.

use bindings::zephyr;
use core::mem::size_of;

pub type RawFd = cty::c_int;
pub type Errno = cty::c_int;

/// Constants used in `socket()` to specify the domain to communicate;
/// i.e., address family socket.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum AddressFamily {
    Inet = zephyr::AF_INET as i32,
    Inet6 = zephyr::AF_INET6 as i32,
}

/// These constants are used to specify the communication semantics
/// when creating a socket with `socket()`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SockType {
    /// Provides sequenced, reliable, two-way, connection-based byte streams.
    Stream = zephyr::net_sock_type_SOCK_STREAM as i32,
    /// Supports datagrams (connectionless, unreliable messages)
    Datagram = zephyr::net_sock_type_SOCK_DGRAM as i32,
}

/// Constants used in `socket()` to specify the protocol to use.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SockProtocol {
    Tcp = zephyr::net_ip_protocol_IPPROTO_TCP as i32,
    Udp = zephyr::net_ip_protocol_IPPROTO_UDP as i32,
}

/// IPv4 address in socket API.
#[derive(Clone, Copy)]
pub struct Ipv6Addr(zephyr::in6_addr);

impl Ipv6Addr {
    /// Create new IPv6 any address.
    pub fn any() -> Ipv6Addr {
        use zephyr::in6_addr__bindgen_ty_1 as in6_addr_union;
        Ipv6Addr(zephyr::in6_addr {
            __bindgen_anon_1: in6_addr_union {
                s6_addr16: [0; 8],
            },
        })
    }
}

/// New type wrapping socket address.
/// Current limitation: This only describes IPv6.
pub struct InetAddr(zephyr::sockaddr_in6);

impl InetAddr {
    /// Create new IPv6 address.
    pub fn new(ip: Ipv6Addr, port: u16) -> InetAddr {
        InetAddr(zephyr::sockaddr_in6 {
            sin6_family: AddressFamily::Inet6 as u16,
            sin6_port: port.to_be(),
            sin6_addr: ip.0,
            sin6_scope_id: 0,
        })
    }

    /// Conversion from nix's SockAddr type to the underlying libc sockaddr type.
    /// safe: Because `sockaddr_in` is an alternative representation of sockaddr.
    pub unsafe fn as_ffi_pair(&self) -> (&zephyr::sockaddr, usize) {
        (core::mem::transmute(&self.0), size_of::<zephyr::sockaddr_in6>())
    }

    /// Conversion from nix's SockAddr type to the underlying libc sockaddr type.
    /// safe: Because `sockaddr_in` is an alternative representation of sockaddr.
    pub unsafe fn as_ffi_pair_mut(&mut self) -> (&mut zephyr::sockaddr, usize) {
        (core::mem::transmute(&mut self.0), size_of::<zephyr::sockaddr_in6>())
    }
}

// TODO: Must be moved.
fn make_result(value: i32) -> Result<i32, Errno> {
    if value < 0 {
        Err(unsafe { *zephyr::_impl_z_errno() })
    } else {
        Ok(value)
    }
}

/// Create new socket.
pub fn socket(domain: AddressFamily, ty: SockType, protocol: SockProtocol) -> Result<RawFd, Errno> {
    let res = unsafe { zephyr::_impl_zsock_socket(domain as i32, ty as i32, protocol as i32) };
    make_result(res)
}

/// IPv6 version of bind the `addr` for `fd`.
pub fn bind(fd: RawFd, addr: &InetAddr) -> Result<(), Errno> {
    let res = unsafe {
        let (ptr, len) = addr.as_ffi_pair();
        zephyr::_impl_zsock_bind(fd, ptr, len)
    };
    make_result(res).map(drop)
}

/// Start to listen on 'fd`.
pub fn listen(fd: RawFd, backlog: i32) -> Result<(), Errno> {
    let res = unsafe { zephyr::_impl_zsock_listen(fd, backlog) };
    make_result(res).map(drop)
}

fn print_ipv6(addr: &InetAddr) {
    let addr = unsafe { &addr.0.sin6_addr.__bindgen_anon_1.s6_addr16 };
    println!(
        "client connected from {:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        addr[0].to_be(),
        addr[1].to_be(),
        addr[2].to_be(),
        addr[3].to_be(),
        addr[4].to_be(),
        addr[5].to_be(),
        addr[6].to_be(),
        addr[7].to_be()
    );
}

/// Accept a connection on a socket.
pub fn accept(sockfd: RawFd) -> Result<RawFd, Errno> {
    let mut addr = InetAddr::new(Ipv6Addr::any(), 0);
    let (ptr, mut len) = unsafe { addr.as_ffi_pair_mut() };

    let res = unsafe { zephyr::_impl_zsock_accept(sockfd, ptr, &mut len) };
    print_ipv6(&addr);
    make_result(res)
}

/// Receive data from a connection-oriented socket. Returns the number of
/// bytes read
pub fn recv(sockfd: RawFd, buf: &mut [u8]) -> Result<isize, Errno> {
    let len = unsafe {
        zephyr::_impl_zsock_recvfrom(
            sockfd,
            buf.as_mut_ptr() as *mut cty::c_void,
            buf.len(),
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };

    if len < 0 {
        Err(unsafe { *zephyr::_impl_z_errno() })
    } else {
        Ok(len)
    }
}

/// Send data to a connection-oriented socket. Returns the number of bytes read
pub fn send(fd: RawFd, buf: &[u8]) -> Result<isize, Errno> {
    let len = unsafe {
        zephyr::_impl_zsock_sendto(
            fd,
            buf.as_ptr() as *const cty::c_void,
            buf.len(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };

    if len < 0 {
        Err(unsafe { *zephyr::_impl_z_errno() })
    } else {
        Ok(len)
    }
}

/// Close socket
pub fn close(sockfd: RawFd) -> Result<(), Errno> {
    let res = unsafe { zephyr::_impl_zsock_close(sockfd) };
    make_result(res).map(drop)
}