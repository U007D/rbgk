use consts::*;
use super::StdError;

pub trait ErrorExt {
    fn trace(&self) -> String;
}

impl<'a> ErrorExt for &'a StdError {
    fn trace(&self) -> String {
        format!("{}: {}\n", MSG_ERR, self) +
            &match self.cause() {
                None => String::new(),
                Some(cause) => MSG_CAUSED_BY.to_string() + ": " + &cause.trace(),
            }
    }
}

error_chain! {}
