use std::fmt::Debug;

use packet_enum::*;
use serde::{Deserialize, Serialize};

pub use socket2::SockAddr;

#[derive(PacketEnum, Serialize, Deserialize, Debug)]
pub enum Packet {
    #[serde(with = "serde_bytes")]
    Data(Vec<u8>),
    Ping,
}
