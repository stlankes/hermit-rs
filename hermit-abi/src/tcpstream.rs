//! `tcpstream` provide an interface to establish tcp socket client.

use crate::{Handle, IpAddress, NetworkError};

extern "Rust" {
	fn sys_tcp_stream_connect(
		ip: &[u8],
		port: u16,
		timeout: Option<u64>,
	) -> Result<Handle, NetworkError>;
	fn sys_tcp_stream_close(handle: Handle) -> Result<(), NetworkError>;
	fn sys_tcp_stream_read(
		handle: Handle,
		buffer: &mut [u8],
		blocking: bool,
	) -> Result<usize, NetworkError>;
	fn sys_tcp_stream_write(
		handle: Handle,
		buffer: &[u8],
		blocking: bool,
	) -> Result<usize, NetworkError>;
	fn sys_tcp_stream_set_read_timeout(
		handle: Handle,
		timeout: Option<u64>,
	) -> Result<(), NetworkError>;
	fn sys_tcp_stream_get_read_timeout(handle: Handle) -> Result<Option<u64>, NetworkError>;
	fn sys_tcp_stream_set_write_timeout(
		handle: Handle,
		timeout: Option<u64>,
	) -> Result<(), NetworkError>;
	fn sys_tcp_stream_get_write_timeout(handle: Handle) -> Result<Option<u64>, NetworkError>;
	fn sys_tcp_stream_peek(handle: Handle, buf: &mut [u8]) -> Result<usize, NetworkError>;
	fn sys_tcp_stream_set_tll(handle: Handle, ttl: u32) -> Result<(), NetworkError>;
	fn sys_tcp_stream_get_tll(handle: Handle) -> Result<u32, NetworkError>;
	fn sys_tcp_stream_shutdown(handle: Handle, how: i32) -> Result<(), NetworkError>;
	fn sys_tcp_stream_peer_addr(handle: Handle) -> Result<(IpAddress, u16), NetworkError>;
}

/// Opens a TCP connection to a remote host.
#[inline(always)]
pub fn connect(ip: &[u8], port: u16, timeout: Option<u64>) -> Result<Handle, NetworkError> {
	unsafe { sys_tcp_stream_connect(ip, port, timeout) }
}

/// Close a TCP connection
#[inline(always)]
pub fn close(handle: Handle) -> Result<(), NetworkError> {
	unsafe { sys_tcp_stream_close(handle) }
}

#[inline(always)]
pub fn peek(handle: Handle, buf: &mut [u8]) -> Result<usize, NetworkError> {
	unsafe { sys_tcp_stream_peek(handle, buf) }
}

#[inline(always)]
pub fn peer_addr(handle: Handle) -> Result<(IpAddress, u16), NetworkError> {
	unsafe { sys_tcp_stream_peer_addr(handle) }
}

#[inline(always)]
pub fn read(handle: Handle, buffer: &mut [u8], blocking: bool) -> Result<usize, NetworkError> {
	unsafe { sys_tcp_stream_read(handle, buffer, blocking) }
}

#[inline(always)]
pub fn write(handle: Handle, buffer: &[u8], blocking: bool) -> Result<usize, NetworkError> {
	unsafe { sys_tcp_stream_write(handle, buffer, blocking) }
}

#[inline(always)]
pub fn set_read_timeout(handle: Handle, timeout: Option<u64>) -> Result<(), NetworkError> {
	unsafe { sys_tcp_stream_set_read_timeout(handle, timeout) }
}

#[inline(always)]
pub fn set_write_timeout(handle: Handle, timeout: Option<u64>) -> Result<(), NetworkError> {
	unsafe { sys_tcp_stream_set_write_timeout(handle, timeout) }
}

#[inline(always)]
pub fn get_read_timeout(handle: Handle) -> Result<Option<u64>, NetworkError> {
	unsafe { sys_tcp_stream_get_read_timeout(handle) }
}

#[inline(always)]
pub fn get_write_timeout(handle: Handle) -> Result<Option<u64>, NetworkError> {
	unsafe { sys_tcp_stream_get_write_timeout(handle) }
}

#[inline(always)]
pub fn set_nodelay(_: Handle, mode: bool) -> Result<(), NetworkError> {
	// smoltcp does not support Nagle's algorithm
	// => to enable Nagle's algorithm isn't possible
	if mode {
		Ok(())
	} else {
		Err(NetworkError::Unsupported)
	}
}

#[inline(always)]
pub fn nodelay(_: Handle) -> Result<bool, NetworkError> {
	// smoltcp does not support Nagle's algorithm
	// => return always true
	Ok(true)
}

#[inline(always)]
pub fn set_tll(handle: Handle, ttl: u32) -> Result<(), NetworkError> {
	unsafe { sys_tcp_stream_set_tll(handle, ttl) }
}

#[inline(always)]
pub fn get_tll(handle: Handle) -> Result<u32, NetworkError> {
	unsafe { sys_tcp_stream_get_tll(handle) }
}

#[inline(always)]
pub fn shutdown(handle: Handle, how: i32) -> Result<(), NetworkError> {
	unsafe { sys_tcp_stream_shutdown(handle, how) }
}
