#[cfg(test)]
mod unit_tests;

use super::{Error, Result, Rolls};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {}

impl Game {
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
