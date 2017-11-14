#![cfg_attr(feature="clippy", feature(plugin))] //nightly rustc required by `clippy`
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(proc_macro)] //nightly rustc required by `galvanic-mock`
#![allow(unused_imports)] //disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
        option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
        pub_enum_variant_names, range_plus_one, result_map_unwrap_or_else, result_unwrap_used, trivial_casts,
        non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix,
        /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces, unused_qualifications,
        wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]

pub extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate galvanic_mock;
#[macro_use] extern crate galvanic_assert;
#[macro_use] extern crate galvanic_test;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;
mod arch;

use std::io::Write;
pub use error::Error;
use arch::Info;

type Result<T> = std::result::Result<T, Error>;

/// # Errors
/// Returns an error in the event that any unhandled errors arise during execution.  Rather than returning a
/// type-erased `Box<std::error::Error>` or downcastable `Fail::Error`, `run()` returns a strongly typed
/// `self::error::Error` which can be exhaustively matched or coerced into a `Fail::Error` at the caller's discretion.
///
/// # Remarks
/// This method is the library's primary entry point.
pub struct App<F: Fn() -> usize> {
    width: F,
}

impl<F: Fn() -> usize> App<F> {
    pub fn new(width: F) -> Self { Self { width } }

    pub fn run(&mut self) -> Result<String> {
        let mut buf = Vec::<u8>::new();
        writeln!(&mut buf, "Hello, {}-bit world!", (self.width)())?;
        Ok(String::from_utf8(buf)?)
    }
}

