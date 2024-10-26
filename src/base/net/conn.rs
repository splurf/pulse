use super::{recv, uninit_buf};
use crate::*;

use std::{
    io::{Read, Write},
    mem::MaybeUninit,
    net::TcpStream,
};

use bincode::serialize;
use packet_enum::*;
use socket2::{SockAddr, Socket};

pub trait UdpConn {
    fn socket(&self) -> &Socket;

    fn send_to(&self, packet: &impl AsPacketSend, addr: SockAddr) -> PulseResult<usize> {
        let bytes = serialize(packet)?;
        self.socket().send_to(&bytes, &addr).map_err(Into::into)
    }

    fn recv_from<'a, K: AsPacketKind, T: AsPacketRecv<'a, K>>(
        &self,
        buf: &'a mut [MaybeUninit<u8>],
        kind: K,
    ) -> PulseResult<(T, SockAddr)> {
        let (n, addr) = self.socket().recv_from(buf)?;
        let buf = uninit_buf(buf.as_ptr(), n);
        let packet = recv(buf, kind)?;
        Ok((packet, addr))
    }
}

pub trait TcpConn {
    fn stream(&self) -> &TcpStream;

    fn send(&self, packet: &impl AsPacketSend) -> PulseResult<()> {
        let bytes = serialize(packet)?;
        self.stream().write_all(&bytes).map_err(Into::into)
    }

    fn recv<'a, K: AsPacketKind, T: AsPacketRecv<'a, K>, const N: usize>(
        &self,
        buf: &'a mut [u8; N],
        kind: K,
    ) -> PulseResult<T> {
        let bytes = self.stream().read(buf)?;
        recv(&buf[..bytes], kind)
    }
}
