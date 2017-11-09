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
use quickstart_template::consts::*;
use quickstart_template::Args;
use libc::{EXIT_SUCCESS, EXIT_FAILURE};

fn main() {
    std::process::exit(match quickstart_template::run(Args::from(std::env::args_os())) {
            Ok(_) => EXIT_SUCCESS,
            Err(ref e) => {
                writeln!(&mut std::io::stderr(), "{}: {}", MSG_ERROR, e).expect(MSG_ERR_WRITING_STDERR);
                EXIT_FAILURE
            },
        }
    );
}
