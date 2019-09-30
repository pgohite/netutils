use std::net::TcpListener;

/// Returns a free, ready to use tcp port from operating system
/// # Examples
///
/// ```no_run
/// use netutils::tcp;
///
/// fn main() {
///     println!("Port {} is available", tcp::free_port().unwrap());
/// }
/// ```
pub fn free_port() -> Option<u16> {
    match TcpListener::bind("0.0.0.0:0") {
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
    fn test_tcp_free_port() {
        assert_ne!(super::free_port(), None);
    }
}
