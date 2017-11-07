#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate failure;
#[macro_use] extern crate derive_fail;
extern crate assayer;

#[cfg(test)]
mod unit_tests;
mod consts;
mod error;
mod args;

pub use std::error::Error as StdError;
pub use error::Error;
pub use failure::ResultExt;

use assayer::MethodSyntaxValidator;
pub use args::Args;

type Result<T> = std::result::Result<T, self::error::Error>;

pub fn run(args: Args) -> Result<()> {
    args.validate::<Args>()?;
    println!("Hello, {}-bit world!", 0usize.count_zeros());
    Ok(())
}
