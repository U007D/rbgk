#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer,
        nonminimal_bool, option_map_unwrap_or, option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used,
        /*print_stdout,*/ pub_enum_variant_names, range_plus_one, result_map_unwrap_or_else, result_unwrap_used,
        trivial_casts, non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc,
        unseparated_literal_suffix, /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces,
        unused_qualifications, wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]

extern crate quickstart_template;
extern crate libc;

use std::io::Write;
use libc::{EXIT_SUCCESS, EXIT_FAILURE};
use quickstart_template::failure::Fail;
use quickstart_template::Args;
use quickstart_template::consts::*;


/// Returns a string listing the cause(s), if any, of a `Fail`ure.
///
/// Note `causes` is recursive, and rustc (as of v1.23.0) does not support tail-call optimization.  This presents a
/// (small) risk of a stack overflow in the event of a (very) deep error chain.  Given that causes() is called by main()
/// at the end of program execution, the risk of a panic instead of an error message is deemed acceptably low, with few
/// imagined consequences if untrue.  This risk will disappear entirely with
/// [tail call optimization](https://github.com/rust-lang/rfcs/pull/1888)
fn causes(cause: Option<&Fail>) -> String {
    match cause {
        None => String::new(),
        Some(c) => format!("  {}: {}\n", MSG_CAUSED_BY, c) + causes(c.cause()).as_str(),
    }
}

/// This crate is structured as a library with `main.rs` defining a small, optional command-line application driver.
/// Use `cargo run` to invoke this entry point.
///
/// main() gathers provided command-line arguments, if any, and passes them to the `quickstart_template::run()` method
/// as a `Vec<String>` (`Args`).  Note a type conversion is required because the underlying OS may not support UTF-8
/// strings, and thus may not be convertible to Unicode.  In such an event, the invalid character will be represented in
/// the resulting string by the Unicode replacement character � (U+FFFD).
///
/// The `run()` method returns a `Result<(), quickstart_template::error::Error>`.  If the result is `Ok`, then the
/// program exits with a success code (0).  If the result is `Err`, the error is dumped to `stderr`, along with its
/// underlying cause(s), if any.
fn main() {
    std::process::exit(
        match quickstart_template::run(Args::from(std::env::args_os())) {
            Ok(_) => EXIT_SUCCESS,
            Err(ref e) => {
                writeln!(&mut std::io::stderr(), "{}: {}", MSG_ERROR, causes(e.cause())).expect(MSG_ERR_WRITING_STDERR);
                EXIT_FAILURE
            },
        }
    );
}
