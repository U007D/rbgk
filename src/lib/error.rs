use std::fmt;

#[allow(unused_imports)]
use consts::*;

#[allow(empty_enum)]
#[derive(Fail, Debug)]
pub enum Error {
}

#[allow(unreachable_code)]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
        })
    }
}
