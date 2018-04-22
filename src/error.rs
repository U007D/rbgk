use super::*;

use std::{ffi::OsString, fmt, option::NoneError};

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    NoneError,
    InvalidUtf8Arg(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::NoneError => MSG_ERR_NONE_ERROR.to_string(),
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
        })
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}
