#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]
#![recursion_limit = "1024"] // `error_chain!` can recurse deeply

#[macro_use] extern crate error_chain;
extern crate assayer;

mod consts {
    pub const MSG_ERR: &'static str = "Error";
    pub const MSG_CAUSED_BY: &'static str = "caused by";
}

pub mod error {
    use super::consts::*;
    pub use std::error::Error as StdError;

    pub trait ErrorExt {
        fn display_chain(&self) -> String;
    }

    impl<'a> ErrorExt for &'a StdError {
        fn display_chain(&self) -> String {
            format!("{}: {}\n", MSG_ERR, self) +
                &match self.cause() {
                    None => String::new(),
                    Some(cause) => MSG_CAUSED_BY.to_string() + ": " + &cause.display_chain(),
                }
        }
    }

    error_chain! {}
}

mod args {
    use std::env::ArgsOs;
    use std::ops::{Deref, DerefMut};
    use assayer::Error as AssayerError;
    use assayer::Validator;

    //Newtype Args wrapper around Vec<String> necessary to implement Validator<Args>.
    #[derive(Clone, Debug)]
    pub struct Args(Vec<String>);

    impl Args {
        pub fn new(args: Vec<String>) -> Self { Args(args) }
    }

    impl Deref for Args {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl DerefMut for Args {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }

    impl From<ArgsOs> for Args {
        fn from(args_os: ArgsOs) -> Self {
            //converts any non-unicode-representable character encoding -> � (U+FFFD replacement character)
            Args(args_os.map(|arg_os| arg_os.to_string_lossy()
                                            .to_string())
                        .collect::<Vec<_>>())
        }
    }

    impl Validator<Args> for Args {
        fn validate(value: Args) -> Result<Self, AssayerError> {
            Err(AssayerError::ValueInvalid(format!("{:?}", value)))
        }
    }
}

use assayer::MethodSyntaxValidator;
pub use args::Args;
pub use error::*;

type Result<T> = std::result::Result<T, Error>;

pub fn run(args: Args) -> Result<()> {
    args.validate::<Args>().chain_err(|| "invalid argument provided")?;
    println!("Hello, {}-bit world!", 0usize.count_zeros());
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
