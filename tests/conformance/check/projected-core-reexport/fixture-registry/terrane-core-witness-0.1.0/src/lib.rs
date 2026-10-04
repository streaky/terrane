pub use std::net::{IpAddr, Ipv4Addr};

pub fn is_loopback(address: IpAddr) -> bool {
    address.is_loopback()
}
