#![cfg_attr(feature="clippy", feature(plugin))] //nightly rustc required by `clippy`
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(proc_macro, galvanic_mock_integration, const_atomic_bool_new)] //req'd by galvanic-mock, galvanic-test + galvanic-mock
#![allow(unused_imports, redundant_closure /*galvanic*/)] //disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
        option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
        pub_enum_variant_names, range_plus_one, result_map_unwrap_or_else, result_unwrap_used, /*trivial_casts, //galvanic*/
        non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix,
        /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces, /*unused_qualifications, //galvanic*/
        wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]

pub extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate lazy_static;
extern crate galvanic_mock;
#[macro_use] extern crate galvanic_assert;
#[macro_use] extern crate galvanic_test;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;
mod arch;

use std::io::Write;
//pub use error::Error;
use std::sync::atomic::AtomicBool;
use failure::ResultExt;
use consts::*;
use arch::Info;

type Result<T> = std::result::Result<T, failure::Error>;

// System-wide mutex
static mut APP_INSTANTIATED: AtomicBool = AtomicBool::new(false);

/// # Errors
/// Returns an error in the event that any unhandled errors arise during execution.  Rather than returning a
/// type-erased `Box<std::error::Error>` or downcastable `Fail::Error`, `run()` returns a strongly typed
/// `self::error::Error` which can be exhaustively matched or coerced into a `Fail::Error` at the caller's discretion.
///
/// # Remarks
/// This method is the library's primary entry point.
#[derive(Debug, Clone, PartialEq)]
pub struct App<T: Info> {
    info: T,
}

impl<T: Info> App<T> {
    pub fn new(info: T) -> Result<Self> {
        match unsafe { APP_INSTANTIATED.get_mut() } {
            ref mut v if !**v => {
                **v = true;
                Ok(Self { info })
            },
            _ => Err(failure::Context::new(MSG_ERR_SINGLETON_VIOLATION))?,
        }
    }

    pub fn run(&self) -> Result<String> {
        Ok(format!("Hello, {}-bit world!", self.info.width()))
    }
}

