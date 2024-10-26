use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use clap::Parser;

const fn default_addr() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
}

#[derive(Parser, Debug)]
pub struct Config {
    /// IP address
    #[arg(long, default_value_t = default_addr())]
    addr_local: SocketAddr,

    /// IP address
    #[arg(long, default_value_t = default_addr())]
    addr_remote: SocketAddr,
}

impl Config {
    pub const fn addr_local(&self) -> SocketAddr {
        self.addr_local
    }

    pub const fn addr_remote(&self) -> SocketAddr {
        self.addr_remote
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}
