use std::fmt;

use consts::*;

#[derive(Fail, Debug)]
pub enum Error {
    MyDiscriminant(#[cause] ::std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::MyDiscriminant(_) => { MSG_ERR_ARG_VALIDATION },
        })
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self { Error::MyDiscriminant(err) }
}
