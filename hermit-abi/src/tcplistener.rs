//! `tcplistener` provide an interface to establish tcp socket server.

use crate::{Handle, IpAddress, NetworkError};

extern "Rust" {
	fn sys_tcp_listener_accept(port: u16) -> Result<(Handle, IpAddress, u16), NetworkError>;
}

/// Wait for connection at specified address.
#[inline(always)]
pub fn accept(port: u16) -> Result<(Handle, IpAddress, u16), NetworkError> {
	unsafe { sys_tcp_listener_accept(port) }
}
