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
extern crate assayer;

#[cfg(test)]
mod unit_tests;
pub mod consts;
mod error;
mod args;

pub use error::Error;

use assayer::MethodSyntaxValidator;
pub use args::Args;

type Result<T> = std::result::Result<T, Error>;

pub fn run(args: Args) -> Result<()> {
    args.validate::<Args>()?;
    println!("Hello, {}-bit world!", 0_usize.count_zeros());
    Ok(())
}
