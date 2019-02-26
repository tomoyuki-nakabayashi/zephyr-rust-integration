//! IO wrapper which provides `safe` socket interface using Zephyr socket API.
//! Currently supports only TCP & UDP.

use bindings::zephyr;
use core::mem::size_of;

type RawFd = cty::c_int;
type Errno = cty::c_int;

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
pub struct Ipv4Addr(zephyr::in_addr);

impl Ipv4Addr {
    pub fn new(addr: u32) -> Ipv4Addr {
        use zephyr::in_addr__bindgen_ty_1 as in_addr_union;
        Ipv4Addr(
            zephyr::in_addr {
                __bindgen_anon_1: in_addr_union { s_addr: addr.to_be() },
            }
        )
    }

    pub fn any() -> Ipv4Addr {
        use zephyr::in_addr__bindgen_ty_1 as in_addr_union;
        Ipv4Addr(
            zephyr::in_addr {
                __bindgen_anon_1: in_addr_union { s_addr: zephyr::INADDR_ANY.to_be() },
            }
        )
    }
}

/// New type wrapping socket address.
/// Current limitation: This only describes IPv4.
/// TODO: Need higher level abstraction which contains both IPv4 and IPv6.
pub struct InetAddr(zephyr::sockaddr_in);

impl InetAddr {
    /// Create new IPv4 address.
    pub fn new(ip: Ipv4Addr, port: u16) -> InetAddr {
        InetAddr(
            zephyr::sockaddr_in {
                sin_family: AddressFamily::Inet as u16,
                sin_port: port.to_be(),
                sin_addr: ip.0,
            }
        )
    }

    /// Conversion from nix's SockAddr type to the underlying libc sockaddr type.
    pub unsafe fn as_ffi_pair(&self) -> (&zephyr::sockaddr, usize) {
        (core::mem::transmute(&self.0), size_of::<zephyr::sockaddr>())
    }
}

pub fn socket(domain: AddressFamily, ty: SockType, protocol: SockProtocol) -> Result<RawFd, Errno> {
    let res = unsafe { zephyr::_impl_zsock_socket(domain as i32, ty as i32, protocol as i32) };

    if res < 0 {
        Err( unsafe { *zephyr::_impl_z_errno() } )
    } else {
        Ok( res )
    }
}

pub fn bind(fd: RawFd, addr: &InetAddr) -> Result<(), Errno> {
    let res = unsafe {
        let (ptr, len) = addr.as_ffi_pair();
        zephyr::_impl_zsock_bind(fd, ptr, len)
    };
    
    if res < 0 {
        Err( unsafe { *zephyr::_impl_z_errno() } )
    } else {
        Ok(())
    }
}