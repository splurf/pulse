use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use clap::Parser;

const fn default_addr() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
}

#[derive(Parser, Debug)]
pub struct Config {
    /// IP address
    #[arg(long, default_value_t = default_addr())]
    local_addr: SocketAddr,

    /// IP address
    #[arg(long, default_value_t = default_addr())]
    remote_addr: SocketAddr,
}

impl Config {
    pub const fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub const fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}
