use std::{fmt, result};
use rusb;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Io,
    InvalidParam,
    Access,
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    BadDescriptor,
    Other,
    Parse,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match self {
            Error::Io => rusb::Error::Io.to_string(),
            Error::InvalidParam => rusb::Error::InvalidParam.to_string(),
            Error::Access => rusb::Error::Access.to_string(),
            Error::NoDevice => rusb::Error::NoDevice.to_string(),
            Error::NotFound => rusb::Error::NotFound.to_string(),
            Error::Busy => rusb::Error::Busy.to_string(),
            Error::Timeout => rusb::Error::Timeout.to_string(),
            Error::Overflow => rusb::Error::Overflow.to_string(),
            Error::Pipe => rusb::Error::Pipe.to_string(),
            Error::Interrupted => rusb::Error::Interrupted.to_string(),
            Error::NoMem => rusb::Error::NoMem.to_string(),
            Error::NotSupported => rusb::Error::NotSupported.to_string(),
            Error::BadDescriptor => rusb::Error::BadDescriptor.to_string(),
            Error::Other => rusb::Error::Other.to_string(),
            Error::Parse => "Failed to parse".to_string(),
        })
    }
}

impl std::error::Error for Error {}

pub(crate) fn from_rusb_error(err: rusb::Error) -> Error {
    match err {
        rusb::Error::Io => Error::Io,
        rusb::Error::InvalidParam => Error::InvalidParam,
        rusb::Error::Access => Error::Access,
        rusb::Error::NoDevice => Error::NoDevice,
        rusb::Error::NotFound => Error::NotFound,
        rusb::Error::Busy => Error::Busy,
        rusb::Error::Timeout => Error::Timeout,
        rusb::Error::Overflow => Error::Overflow,
        rusb::Error::Pipe => Error::Pipe,
        rusb::Error::Interrupted => Error::Interrupted,
        rusb::Error::NoMem => Error::NoMem,
        rusb::Error::NotSupported => Error::NotSupported,
        rusb::Error::BadDescriptor => Error::BadDescriptor,
        rusb::Error::Other => Error::Other,
    }
}
