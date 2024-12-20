use crate::*;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Display)]
pub enum CpalError {
    HostUnavailable,
    DeviceName,
    DevicesError,
    SupportedConfig,
    DefaultConfig,
    BuildStream,
    InputUnavailable,
    OutputUnavailable,
    Unexpected,
}

impl From<cpal::HostUnavailable> for CpalError {
    fn from(_: cpal::HostUnavailable) -> Self {
        Self::HostUnavailable
    }
}

impl From<cpal::DeviceNameError> for CpalError {
    fn from(_: cpal::DeviceNameError) -> Self {
        Self::DeviceName
    }
}

impl From<cpal::DevicesError> for CpalError {
    fn from(_: cpal::DevicesError) -> Self {
        Self::DevicesError
    }
}

impl From<cpal::SupportedStreamConfigsError> for CpalError {
    fn from(_: cpal::SupportedStreamConfigsError) -> Self {
        Self::SupportedConfig
    }
}

impl From<cpal::DefaultStreamConfigError> for CpalError {
    fn from(_: cpal::DefaultStreamConfigError) -> Self {
        Self::DefaultConfig
    }
}

impl From<cpal::BuildStreamError> for CpalError {
    fn from(_: cpal::BuildStreamError) -> Self {
        Self::BuildStream
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Cpal(#[from] CpalError),

    #[error(transparent)]
    Pulse(PulseError),
}

impl<T: Into<PulseError>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Pulse(value.into())
    }
}
