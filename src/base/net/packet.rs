use crate::*;

use std::{fmt::Debug, mem::MaybeUninit};

use packet_enum::*;
use serde::{Deserialize, Serialize};

pub const fn init_buf() -> [MaybeUninit<u8>; PACKET_SIZE] {
    unsafe { MaybeUninit::uninit().assume_init() }
}

pub use socket2::SockAddr;

#[derive(Clone, Copy, Debug)]
pub struct ClientHandshake;

#[derive(Clone, Copy, Debug)]
pub struct ServerHandshake(u8);

impl ServerHandshake {
    pub const fn id(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Handshake {
    secret: [u8; 3],
    is_client: Option<u8>,
}

impl Handshake {
    // maybe generate random hash at build-time
    const SECRET: [u8; 3] = [1, 0, 1];

    const fn new(is_client: Option<u8>) -> Self {
        Self {
            secret: Self::SECRET,
            is_client,
        }
    }

    pub const fn client() -> Self {
        Self::new(None)
    }

    pub const fn server(id: u8) -> Self {
        Self::new(Some(id))
    }

    pub fn verify(&self) -> PulseResult<()> {
        if self.secret == Self::SECRET {
            Ok(())
        } else {
            Err(PulseError::Packet(PacketError::Handshake(
                HandshakeError::InvalidContent,
            )))
        }
    }

    pub const fn into_client(self) -> Option<ClientHandshake> {
        if self.is_client.is_none() {
            Some(ClientHandshake)
        } else {
            None
        }
    }

    pub const fn into_server(self) -> Option<ServerHandshake> {
        if let Some(id) = self.is_client {
            Some(ServerHandshake(id))
        } else {
            None
        }
    }
}

#[derive(PacketEnum, Serialize, Deserialize, Debug)]
pub enum Packet {
    Data(Vec<f32>),
    Ping,
}
