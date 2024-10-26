use crate::*;

pub type PulseResult<T = (), E = PulseError> = std::result::Result<T, E>;

#[derive(ThisError, Debug)]
pub enum PacketError {
    #[error("Handshake({0})")]
    Handshake(#[from] HandshakeError),

    #[error("Expected {lhs:?}, found {rhs:?}")]
    Unexpected { lhs: String, rhs: String },
}

#[derive(ThisError, Debug, Display)]
pub enum SyncError {
    Send,
    TrySend,
    Recv,
    TryRecv,
    Disconnected,
}

impl<T> From<SendError<T>> for SyncError {
    fn from(_: SendError<T>) -> Self {
        Self::Send
    }
}

impl<T> From<TrySendError<T>> for SyncError {
    fn from(value: TrySendError<T>) -> Self {
        if let TrySendError::Disconnected(..) = value {
            Self::Disconnected
        } else {
            Self::TrySend
        }
    }
}

impl From<RecvError> for SyncError {
    fn from(_: RecvError) -> Self {
        Self::Recv
    }
}

impl From<TryRecvError> for SyncError {
    fn from(value: TryRecvError) -> Self {
        if let TryRecvError::Disconnected = value {
            Self::Disconnected
        } else {
            Self::TryRecv
        }
    }
}

#[derive(thiserror::Error, Debug, Display)]
pub enum HandshakeError {
    InvalidContent,
    InvalidType,
    Unknown,
}

#[derive(thiserror::Error, Debug)]
pub enum PulseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] Box<bincode::ErrorKind>),

    #[error(transparent)]
    Packet(#[from] PacketError),

    #[error(transparent)]
    Sync(SyncError),

    #[error("Misc: {0}")]
    Misc(String),

    #[error("Unknown: An unexpected error has ocurred")]
    Unknown,
}

impl<T: Into<SyncError>> From<T> for PulseError {
    fn from(value: T) -> Self {
        Self::Sync(value.into())
    }
}

impl From<Box<dyn std::any::Any + Send>> for PulseError {
    fn from(value: Box<dyn std::any::Any + Send>) -> Self {
        match value.downcast::<&str>() {
            Ok(s) => Self::Misc(s.to_string()),
            Err(_) => Self::Unknown,
        }
    }
}
