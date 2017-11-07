use std::fmt;

use consts::*;
use assayer::Error as AssayerError;

#[derive(Fail, Debug)]
pub enum Error {
    ArgValidation(#[cause] AssayerError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::ArgValidation(_) => { MSG_ERR_ARG_VALIDATION },
        })
    }
}

impl From<AssayerError> for Error {
    fn from(err: AssayerError) -> Error { Error::ArgValidation(err) }
}
