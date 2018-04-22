use super::*;

use std::{ffi::OsString, fmt};

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidUtf8Arg(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}
