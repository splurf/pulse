use crate::*;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Pulse(PulseError),
}

impl<T: Into<PulseError>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Pulse(value.into())
    }
}
