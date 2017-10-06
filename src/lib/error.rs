use consts::*;
use super::StdError;
use assayer::Error as AssayerError;

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

impl From<AssayerError> for StdError {
    fn from(e: AssayerError) -> Self {
        unimplemented!()
    }
}

error_def! Error {
    Message { #[from] cause: Box<Error>, } => "quickstart_template::Error::Message",
    Validation { #[from] cause: AssayerError, } => "quickstart_template::Error::Validation",
}
