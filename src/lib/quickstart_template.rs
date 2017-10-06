#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate assayer;

#[cfg(test)]
mod unit_tests;
mod consts;
mod error;
mod args;

pub use std::error::Error as StdError;
pub use error::{Error, ErrorExt};

use assayer::MethodSyntaxValidator;
pub use args::Args;

type Result<T> = std::result::Result<T, Error>;

pub fn run(args: Args) -> Result<()> {
    args.validate::<Args>().map_err(|e| Error::Message { cause: Box::new(&e) })?;
    println!("Hello, {}-bit world!", 0usize.count_zeros());
    Ok(())
}
