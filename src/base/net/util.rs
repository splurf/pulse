use std::mem::MaybeUninit;

use crate::*;

use bincode::deserialize;
use packet_enum::{AsPacketKind, AsPacketRecv};

pub const fn uninit_buf<'a>(buf: *const MaybeUninit<u8>, size: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(buf as *const u8, size) }
}

pub fn recv<'a, K: AsPacketKind, T: AsPacketRecv<'a, K>>(buf: &'a [u8], kind: K) -> PulseResult<T> {
    let packet = deserialize::<T>(buf)?;

    if !kind.contains(packet.kind()) {
        return Err(PulseError::Packet(PacketError::Unexpected {
            lhs: format!("{:?}", kind),
            rhs: format!("{:?}", packet.kind()),
        }));
    }
    Ok(packet)
}
