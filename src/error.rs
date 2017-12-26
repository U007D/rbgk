use std::fmt;
use consts::*;
#[allow(unused_imports)] use super::*;

#[derive(Fail, Debug, PartialEq)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
    //#[fail(display = "{}", MSG_ERROR)]
    Uninhabited,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::Uninhabited => MSG_ERROR,
        })
    }
}
