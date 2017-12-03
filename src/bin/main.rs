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
use libc::{EXIT_SUCCESS, EXIT_FAILURE};

use quickstart_template::failure::Fail as Fail;
use quickstart_template::consts::*;

type Result<T> = std::result::Result<T, quickstart_template::Error>;

/// This crate is structured as a library with `bin/main.rs` defining a small, optional command-line application driver.
/// Use `cargo run` to invoke this entry point which will return the crate's main library entry point's (`run()`) result
/// or returned errors, if any.
fn main() {
    std::process::exit(
        match run() {
            Ok(ref msg) => {
                writeln!(&mut io::stdout(), "{}", msg).expect(MSG_ERR_WRITING_STDOUT);
                EXIT_SUCCESS
            },
            Err(ref err) => {
                writeln!(&mut io::stderr(), "{}: {}", MSG_ERROR,
                         err.causes()
                            .fold(String::new(), |acc, cause| acc + &format!("  {}: {}\n", MSG_CAUSED_BY, cause)))
                    .expect(MSG_ERR_WRITING_STDERR);
                EXIT_FAILURE
            },
        }
    )
}

/// "Inner main()" function with a `Result` return type to simplify propagation of `Result`s from function calls.
/// Rust's native main will support `Result` when [RFC1937](https://github.com/rust-lang/rfcs/pull/1937) lands, making
/// this function unnecessary.
fn run() -> Result<String> {
    let container = di::Container::build(); // TODO: Determine if `code_gen` `registration` can `build()` the `Container` and own the instance
    quickstart_template::App::new(
            container.resolve_ref_concurrency_primitives_singleton(),
            container.resolve_ref_arch_info())?
        .run()
}

// TODO: Auto-generate `Container` using `di::registration` module and `code_gen`.
mod di {
    use quickstart_template::{self, concurrency_primitives, arch};

    pub struct Container {
        concurrency_primitives_system_wide_singleton: concurrency_primitives::SystemWideSingleton,
        arch_info: arch::Arch,
    }

    impl Container {
        pub fn build() -> Self {
            Self {
                concurrency_primitives_system_wide_singleton: quickstart_template::concurrency_primitives::SystemWideSingleton::new(),
                arch_info: quickstart_template::arch::Arch::new(),
            }
        }

        pub fn resolve_ref_concurrency_primitives_singleton(&self) -> &concurrency_primitives::SystemWideSingleton {
            &self.concurrency_primitives_system_wide_singleton
        }

        pub fn resolve_ref_arch_info(&self) -> &arch::Arch {
            &self.arch_info
        }
    }
}
