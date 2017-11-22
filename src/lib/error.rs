#[allow(unused_imports)] use super::*;
use std::fmt;

use consts::*;

#[derive(Fail, Debug, /*PartialEq*/)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
//    Context(#[cause] failure::Context<&'static str>),
    SingletonViolation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
//            Error::Context(ref context) => context.get_context(),
            Error::SingletonViolation => MSG_ERR_SINGLETON_VIOLATION,
        })
    }
}

//impl From<failure::Context<&'static str>> for Error {
//    fn from(e: failure::Context<&'static str>) -> Self { Error::Context(e) }
//}
