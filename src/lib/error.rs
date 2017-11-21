use super::*;
use std::fmt;
use std::io;
use std::string;

use consts::*;

#[derive(Fail, Debug, /*PartialEq*/)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
    Context(#[cause] failure::Context<&'static str>),
    SingletonViolation,
    Io(#[cause] io::Error),
    Utf8Conversion(#[cause] string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::Context(ref context) => context.get_context(),
            Error::SingletonViolation => MSG_ERR_SINGLETON_VIOLATION,
            Error::Io(_) => MSG_ERR_ARG_VALIDATION,
            Error::Utf8Conversion(_) => MSG_ERR_FROM_UTF8,
        })
    }
}

impl From<failure::Context<&'static str>> for Error {
    fn from(e: failure::Context<&'static str>) -> Self { Error::Context(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Error::Io(e) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self { Error::Utf8Conversion(e) }
}
