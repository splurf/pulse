use super::{recv, uninit_buf};
use crate::*;

use std::{
    io::Write,
    mem::MaybeUninit,
    net::{TcpStream, ToSocketAddrs},
    sync::Arc,
};

use bincode::serialize;
use packet_enum::*;
use socket2::{Domain, Protocol, Socket, Type};

#[derive(Debug)]
pub struct UdpClient {
    inner: Socket,
}

impl UdpClient {
    pub fn new(
        local_addr: impl ToSocketAddrs,
        remote_addr: impl ToSocketAddrs,
    ) -> PulseResult<Self> {
        let inner = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        inner.bind(&local_addr.to_socket_addrs()?.next().unwrap().into())?;
        inner.connect(&remote_addr.to_socket_addrs()?.next().unwrap().into())?;

        Ok(Self { inner })
    }

    pub fn send(&mut self, packet: &impl AsPacketSend) -> PulseResult<usize> {
        let bytes = serialize(packet)?;
        let n = self.inner.send(&bytes)?;
        self.inner.flush()?;
        Ok(n)
    }

    pub fn recv<'a, K: AsPacketKind, T: AsPacketRecv<'a, K>>(
        &self,
        buf: &'a mut [MaybeUninit<u8>],
        kind: K,
    ) -> PulseResult<T> {
        let n = self.inner.recv(buf)?;
        let buf = uninit_buf(buf.as_ptr(), n);
        recv(buf, kind)
    }

    pub fn try_clone(&self) -> PulseResult<Self> {
        let inner = self.inner.try_clone()?;
        Ok(Self { inner })
    }
}

impl UdpConn for UdpClient {
    fn socket(&self) -> &Socket {
        &self.inner
    }
}

#[derive(Clone, Debug)]
pub struct TcpClient {
    inner: Arc<TcpStream>,
}

impl TcpClient {
    pub fn new(addr: impl ToSocketAddrs) -> PulseResult<Self> {
        let stream = TcpStream::connect(addr)?;
        let client = Self {
            inner: Arc::new(stream),
        };
        Ok(client)
    }
}

impl TcpConn for TcpClient {
    fn stream(&self) -> &TcpStream {
        &self.inner
    }
}
