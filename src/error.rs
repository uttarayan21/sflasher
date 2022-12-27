use std::backtrace::Backtrace;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub backtrace: Backtrace,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "backtrace"))]
        {
            writeln!(f, "{}", self.kind)
        }
        #[cfg(feature = "backtrace")]
        {
            writeln!(f, "{}", self.kind)?;
            writeln!(f, "Backtrace: {:#?}", self.backtrace)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Hid(#[from] hidapi::HidError),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    TryIntoError(#[from] std::array::TryFromSliceError),

    #[error("The device was not found")]
    DeviceNotFound,
    #[error("No devices were found")]
    NoDevicesFound,
    #[error("Invalid identifier {0}")]
    InvalidIdentifier(String),
    #[error("Device was not specified")]
    UnspecifiedDevice,
    #[error("Invalid Firmware")]
    InvalidFirmware,
    #[error("Invalid Report Length {0}")]
    InvalidReportLength(usize),
    #[error("Invalid Response")]
    InvalidResponse,
    #[error("Failed to initialize")]
    FailedToInitialize,
    #[error("Failed to write {0:?}")]
    FailedToWrite(WriteFailure),
    #[error("Invalid Device")]
    InvalidDevice,
}

#[derive(Debug)]
pub enum WriteFailure {
    InvalidCommand,
    InvalidStatus,
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(e: E) -> Self {
        Self {
            kind: e.into(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::convert::Infallible> for ErrorKind {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
