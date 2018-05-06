#![cfg_attr(feature = "clippy", feature(plugin))] // nightly rustc required by `clippy`
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(match_bool)] // disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, clone_on_ref_ptr,
        decimal_literal_representation, doc_markdown, else_if_without_else, empty_enum, enum_glob_use,
        expl_impl_clone_on_copy, fallible_impl_from, filter_map,
        float_cmp_const /* use for safety-critical applications only */,
        if_not_else,
        indexing_slicing /* use for safety-critical applications only */,
        inline_always, integer_arithmetic, invalid_upcast_comparisons, int_plus_one, invalid_upcast_comparisons,
        items_after_statements, linkedlist, match_same_arms, maybe_infinite_iter, mem_forget,
        /* multiple_crate_versions /* enable for release to discover and reconcile multiple crate versions */, */
        mut_mut, missing_debug_implementations, mut_mut, mutex_integer, needless_borrow, needless_continue,
        nonminimal_bool, option_map_unwrap_or, option_map_unwrap_or_else, option_map_unwrap_or_else,
        option_unwrap_used, pub_enum_variant_names, range_plus_one, replace_consts, redundant_closure,
        result_map_unwrap_or_else, result_unwrap_used, shadow_unrelated, similar_names, stutter, trivial_casts,
        non_camel_case_types, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix, use_self,
        used_underscore_binding, unused_import_braces, unnecessary_mut_passed, unused_qualifications,
        wrong_pub_self_convention)]
#![forbid(overflowing_literals, unused_must_use)]

extern crate qst;

use qst::{Container, Greeter, GreeterContainer, Result};

fn args() -> Result<Vec<String>> {
    Ok(std::env::args_os()
                .map(|oss| oss.into_string())
                .collect::<std::result::Result<Vec<_>, _>>()?)
}

fn main() -> Result<()> {
    println!("{}", GreeterContainer::build()
                                    .resolve_greeter()
                                    .greet(args()?));
    Ok(())
}
