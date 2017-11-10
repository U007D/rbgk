use std::fmt;

use consts::*;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    MyDiscriminant(#[cause] fmt::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::MyDiscriminant(_) => { MSG_ERR_ARG_VALIDATION },
        })
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self { Error::MyDiscriminant(err) }
}
