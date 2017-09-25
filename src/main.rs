#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, /*trivial_casts,*/ trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate quickstart_template;
extern crate libc;

mod consts {
    pub const MSG_BACKTRACE: &'static str = "Backtrace";
}

use consts::*;
use quickstart_template::error::*;
use quickstart_template::Args;
#[allow(unused_imports)] use libc::{EXIT_SUCCESS, EXIT_FAILURE};

fn display_error_chain(e: &Error) {
    #[allow(trivial_casts)]
    println!("{}", (e as &StdError).display_chain());
    if let Some(backtrace) = e.backtrace() { println!("{}: {:?}", MSG_BACKTRACE, backtrace) }
}

pub fn main() {
    match quickstart_template::run(Args::from(std::env::args_os())) {
        Ok(r) => r,
        Err(ref e) => {
            display_error_chain(e);
            std::process::exit(EXIT_FAILURE);
        },
    }
}
