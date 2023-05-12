use std::{
    fmt,
    net::Ipv4Addr,
    num::ParseIntError,
    str::FromStr,
};

use derive_builder::Builder;

/// Stores device ip address and port
#[derive(Builder)]
#[builder(default)]
pub struct Address {
    pub ip: String,
    pub port: u32,
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
