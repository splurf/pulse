use crate::*;

use std::mem::MaybeUninit;

use bincode::deserialize;
use packet_enum::{AsPacketKind, AsPacketRecv};

pub const fn init_buf() -> [MaybeUninit<u8>; PACKET_SIZE] {
    unsafe { MaybeUninit::uninit().assume_init() }
}

pub const fn uninit_buf<'a>(buf: *const MaybeUninit<u8>, size: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(buf as *const u8, size) }
}

pub fn recv<'a, K: AsPacketKind, T: AsPacketRecv<'a, K>>(buf: &'a [u8], kind: K) -> PulseResult<T> {
    let packet = deserialize::<T>(buf)?;

    if !kind.contains(packet.kind()) {
        return Err(PulseError::Packet(PacketError::unexpected(
            packet.kind(),
            kind,
        )));
    }
    Ok(packet)
}
