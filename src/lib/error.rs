use std::fmt;
use std::io;
use std::string;

use consts::*;

#[derive(Fail, Debug, /*PartialEq*/)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
    IoError(#[cause] io::Error),
    FromUtf8Error(#[cause] string::FromUtf8Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::IoError(_) => MSG_ERR_ARG_VALIDATION,
            Error::FromUtf8Error(_) => MSG_ERR_FROM_UTF8,
        })
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { Error::IoError(err) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self { Error::FromUtf8Error(err) }
}
