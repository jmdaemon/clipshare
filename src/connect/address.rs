use std::{
    fmt,
    collections::HashMap,
    net::{SocketAddr, Ipv4Addr},
    sync::{Arc, Mutex},
    str::FromStr,
    num::ParseIntError,
};

/// Stores device ip address and port
pub struct Address {
    pub ip: String,
    pub port: u32,
}

/// Stores device address defaults
#[derive(Default)]
pub struct AddressBuilder {
    pub address: Address,
}

impl Default for Address {
    fn default() -> Self {
        let ip = Ipv4Addr::LOCALHOST;
        let port = 5200;
        Address::new(ip.to_string(), port)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = format!("{}:{}", self.ip, self.port);
        write!(f, "{}", addr)
    }
}

impl FromStr for Address {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr_vec: Vec<&str> = s.split(':').collect();
        let (ip, port) = (addr_vec[0], addr_vec[1].parse()?);
        let address = Address { ip: ip.to_owned(), port };
        Ok(address)
    }
}

impl Address {
    pub fn new(ip: String, port: u32) -> Self {
        Self { ip, port }
    }
}

// AddressBuilder
impl AddressBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Address {
        self.address
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.address.ip = ip;
        self
    }

    pub fn port(mut self, port: u32) -> Self {
        self.address.port = port;
        self
    }
}
