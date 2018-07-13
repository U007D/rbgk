#[cfg(test)]
mod unit_tests;

use super::{Result, Rolls};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {}

impl Game {
    pub(super) const FRAME_MIN_VALUE: u8 = 0;
    pub(super) const FRAME_MAX_VALUE: u8 = 10;
    pub(super) const FIRST_FRAME: usize = 1;
    pub(super)const LAST_FRAME: usize = 10;

    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self, rolls: &Rolls) -> Result<Option<u16>> {
        match rolls.len() {
            0 => Ok(None),
            _ => Ok(Some(rolls.iter()
                              .map(|v| u16::from(*v))
                              .sum())),
        }
    }
}
