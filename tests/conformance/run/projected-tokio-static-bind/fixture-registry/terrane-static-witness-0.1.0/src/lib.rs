use std::net::TcpListener;

pub fn available_loopback_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback bind should succeed");
    listener
        .local_addr()
        .expect("loopback listener should have an address")
        .to_string()
}

pub struct GenericFactory;

impl GenericFactory {
    pub fn open<A: AsRef<str>>(address: A) -> String {
        address.as_ref().to_owned()
    }
}
