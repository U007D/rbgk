use super::{Error, Result};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    // Compare unsigned type >= 0 to protect against a future refactor to a signed type
    #[allow(unused_comparisons, absurd_extreme_comparisons)]
    #[allow(integer_arithmetic)]
    pub fn score(&self, rolls: &[u8]) -> Result<u16> {
        fn validate(rolls: &[u8]) -> Result<&[u8]> {
            Ok(rolls).and_then(|r| match r.iter()
                                          .find(|el| !(**el >= 0 && **el <= 10)) {
                                      Some(el) => Err(Error::InvalidRoll(*el)),
                                      _ => Ok(r),
                                  })
        }
        validate(rolls)?.iter()
                        .map(|v| u16::from(*v))
                        .fold(Err(Error::NoRolls), |acc, el| Ok(acc.unwrap_or(0) + el))
    }
}
