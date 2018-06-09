use std::{ffi::OsString, fmt, option::NoneError};
use super::*;

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
#[allow(pub_enum_variant_names)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    NoneError,
    NoRolls,
    InvalidRoll(u8),
    InvalidFrame(Vec<Option<u8>>),
    InvalidGame(Vec<u8>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::NoneError => MSG_ERR_NONE_ERROR.to_string(),
            Error::NoRolls => MSG_ERR_NO_ROLLS.to_string(),
            Error::InvalidRoll(v) => format!("{}: {}", MSG_ERR_INVALID_ROLL, v),
            Error::InvalidFrame(ref v) => format!("{}: {:?}", MSG_ERR_INVALID_FRAME, v),
            Error::InvalidGame(ref v) => format!("{}: {:?}", MSG_ERR_INVALID_GAME, v),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}
