#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
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
#[macro_use] extern crate derive_fail;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;
mod validate;

pub use error::Error;
use validate::Validate;

type Result<T> = std::result::Result<T, Error>;

/// # Arguments
/// `args`: a `Vec<String>` of zero or more arguments to be passed into the library.
///
/// # Errors
/// Returns an error in the event that any unhandled errors arise during execution.  Rather than returning a
/// type-erased `Box<std::error::Error>` or downcastable `Fail::Error`, `run()` returns a strongly typed
/// `self::error::Error` which can be exhaustively matched or coerced into a `Fail::Error` at the caller's discretion.
///
/// # Examples
/// See `app/main.rs` `main()` for a robust example use case.
///
/// # Remarks
/// This method is the library's primary entry point.
#[allow(needless_pass_by_value, unused_variables)]
pub fn run(args: Vec<String>) -> Result<()> {
    (&args).validate()?;
    println!("Hello, {}-bit world!", 0_usize.count_zeros());
    Ok(())
}
