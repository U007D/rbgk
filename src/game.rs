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
            Ok(rolls).and_then(|r| match r.iter().find(|el| !(**el >= 0 && **el <= 10)) {
                                      Some(el) => Err(Error::InvalidRoll(*el)),
                                      _ => Ok(r),
                                  })
                     .and_then(|r| {
                         let mut it = r.iter();
                         loop {
                             let ball_1 = it.next();
                             match ball_1 {
                                 None => break,
                                 Some(&10) => continue,
                                 _ => {
                                     let ball_2 = it.next();
                                     match ball_1.expect("ball_1 encountered unexpected None") + ball_2.unwrap_or(&0) {
                                         v if v >= 0 && v <= 10 => continue,
                                         _ => Err(Error::InvalidFrame(vec![ball_1, ball_2]))?,
                                     }
                                 }
                             }
                             Ok(r)
                         }
                     })
        }
        validate(rolls)?.iter()
                        .map(|v| u16::from(*v))
                        .fold(Err(Error::NoRolls), |acc, el| Ok(acc.unwrap_or(0) + el))
    }
}
