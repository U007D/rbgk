#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, /*trivial_casts,*/ trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate quickstart_template;
extern crate libc;

use quickstart_template::{StdError, Args, ErrorExt};
use libc::{EXIT_SUCCESS, EXIT_FAILURE};

pub fn main() {
    std::process::exit(
        match quickstart_template::run(Args::from(std::env::args_os())) {
            Ok(_) => EXIT_SUCCESS,
            Err(ref e) => {
                writeln!(&mut std::io::stderr(), "{}", e).expect(MSG_ERROR_WRITING_STDERR);
                EXIT_FAILURE
            },
        }
    );
}
