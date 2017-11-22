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

extern crate quickstart_template;
extern crate libc;

use std::io::{self, Write};

use quickstart_template::failure::Fail as Fail;
use quickstart_template::consts::*;
use libc::{EXIT_SUCCESS, EXIT_FAILURE};

type Result<T> = std::result::Result<T, quickstart_template::Error>;

/// This crate is structured as a library with `bin/main.rs` defining a small, optional command-line application driver.
/// Use `cargo run` to invoke this entry point which will call the crate's main library entry point (`run()`) and report
/// on returned errors, if any.
fn main() {
    std::process::exit(
        match run() {
            Ok(ref msg) => {
                writeln!(&mut io::stdout(), "{}", msg).expect(MSG_ERR_WRITING_STDOUT);
                EXIT_SUCCESS
            },
            Err(ref err) => {
                writeln!(&mut io::stderr(), "{}: {}", MSG_ERROR, causes(err.cause())).expect(MSG_ERR_WRITING_STDERR);
                EXIT_FAILURE
            },
        }
    )
}

fn run() -> Result<String> {
    //DI registration and resolution
    let app_state = quickstart_template::concurrency_primitives::AppSystemWideSingleton::new();
    let arch_info = quickstart_template::arch::Arch::new();

    let app = quickstart_template::App::new(&app_state, arch_info)?;
    app.run()
}

/// Returns a string listing the causes, if any, of a `Fail`.
/// Note: [tail call optimization](https://github.com/rust-lang/rfcs/pull/1888) should convert this to a simple loop.
fn causes(cause: Option<&Fail>) -> String {
    match cause {
        None => String::new(),
        Some(err) => format!("  {}: {}\n", MSG_CAUSED_BY, err) + causes(err.cause()).as_str(),
    }
}
