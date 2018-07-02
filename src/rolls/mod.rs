#[cfg(test)]
mod unit_tests;

use super::{consts::*, Error, Result};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
//#[allow(needless_borrow)] //suppress false-positive from Clippy; does not repro on Playground :( http://play.rust-lang.org/?gist=d708c668e1adaf67d7640512e5840198&version=nightly&mode=debug&edition=2015
pub struct Rolls<'a>(&'a [u8]);

impl<'a> Rolls<'a> {
    const MIN_VALUE: u8 = 0;
    const MAX_VALUE: u8 = 10;

    pub fn new(rolls: &'a [u8]) -> Result<Self> {
        Ok(Rolls(Self::validate(rolls)?))
    }

    fn validate(rolls: &'a [u8]) -> Result<&'a [u8]> {
        #[allow(unused_comparisons, absurd_extreme_comparisons)]
            match rolls.iter()
                       .map(|v| match *v >= Self::MIN_VALUE && *v <= Self::MAX_VALUE {
                           true => Ok(v),
                           false => Err(Error::InvalidRoll(*v)),
                       })
                       .filter(|v| v.is_err())
                       .nth(0) {
            None => Ok(rolls),
            Some(e) => Err(e.expect_err(MSG_ERR_INTERNAL_ERR_EXPECTED)),
        }
    }
}

impl<'a> Deref for Rolls<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}
