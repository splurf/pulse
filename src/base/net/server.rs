use socket2::{Domain, Protocol, Socket, Type};

use crate::*;

use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct UdpServer {
    inner: Socket,
}

impl UdpServer {
    pub fn new(addr: impl ToSocketAddrs) -> PulseResult<Self> {
        let inner = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        inner.bind(&addr.to_socket_addrs()?.next().unwrap().into())?;
        Ok(Self { inner })
    }

    pub fn try_clone(&self) -> PulseResult<Self> {
        let inner = self.inner.try_clone()?;
        Ok(Self { inner })
    }
}

impl UdpConn for UdpServer {
    fn socket(&self) -> &Socket {
        &self.inner
    }
}
