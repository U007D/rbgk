#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

#[macro_use]
extern crate error_chain;
extern crate assayer;

#[cfg(test)]
mod unit_tests;
mod consts;
pub mod error;
mod args;

pub use std::error::Error as StdError;
use assayer::MethodSyntaxValidator;
pub use args::Args;
use error::*;

type Result<T> = std::result::Result<T, Error>;

pub fn run(args: Args) -> Result<()> {
    args.validate::<Args>().chain_err(|| "invalid argument provided")?;
    println!("Hello, {}-bit world!", 0usize.count_zeros());
    Ok(())
}
