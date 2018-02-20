use std::fmt;
use std::ffi::OsString;
#[allow(unused_imports)]
use super::*;

#[derive(Fail, Debug, PartialEq)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
    //#[fail(display = "{}", MSG_ERROR)]
    ArgInvalidUtf8(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::ArgInvalidUtf8(_) => MSG_ERR_ARG_INVALID_UTF8,
        })
    }
}

impl From<OsString> for Error {
    fn from(arg: OsString) -> Self {
        Error::ArgInvalidUtf8(arg)
    }
}
