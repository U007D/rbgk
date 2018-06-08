use super::{Error, Result};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(integer_arithmetic)]
    pub fn score(&self, rolls: &[u8]) -> Result<u16> {
        rolls.iter()
             .map(|v| u16::from(*v))
             .fold(Err(Error::NoRolls), |acc, el| Ok(acc.unwrap_or(0) + el))
    }
}
