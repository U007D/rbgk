#[cfg(test)]
mod unit_tests;
mod validators;

use self::validators::*;
use std::ops::Deref;
use super::{consts::*, Error, Game, Result};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Rolls<'a>(&'a [u8]);

impl<'a> Rolls<'a> {
    pub fn new(rolls: &'a [u8]) -> Result<Self> {
        Ok(Rolls(Self::validate(rolls)?))
    }

    fn validate(rolls: &'a [u8]) -> Result<&'a [u8]> {
        #[allow(unused_comparisons, absurd_extreme_comparisons)]
        match rolls.iter()
                   .map(|&v| Ok(v).and_then(|v| validate_roll_value(v))
                                  .and_then(|v| validate_frame(v)))
                   .filter(|v| v.is_err())
                   .nth(0) {
            None => Ok(rolls),
            Some(Err(e)) => Err(e),
            _ => unreachable!(MSG_ERR_INTERNAL_ERR_EXPECTED),
        }
    }
}

impl<'a> Deref for Rolls<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}
