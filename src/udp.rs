use std::net::UdpSocket;

/// Returns a free, ready to use udp port from operating system
/// # Examples
///
/// ```no_run
/// use netutils::udp;
///
/// fn main() {
///     println!("Port {} is available", udp::free_port().unwrap());
/// }
/// ```
pub fn free_port() -> Option<u16> {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(addr) => match addr.local_addr() {
            Ok(addr) => Some(addr.port()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_udp_free_port() {
        assert_ne!(super::free_port(), None);
    }
}
