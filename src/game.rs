use super::{consts::*, Error, Result};

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
            Ok(rolls)
                // No roll exceeds 10
                .and_then(|rolls| match rolls.iter().find(|el| !(**el >= 0 && **el <= 10)) {
                    Some(el) => Err(Error::InvalidRoll(*el)),
                    _ => Ok(rolls),
                })

                // No frame exceeds 10
                .and_then(|rolls| {
                    let mut it = rolls.iter();
                    loop {
                        let ball_1 = it.next();
                        match ball_1 {
                            None => break,
                            Some(&10) => continue,
                            _ => {
                                let ball_2 = it.next();
                                //#neverpanics since `ball_1 == None` would invoke `None` arm, above
                                match ball_1.expect(MSG_ERR_INTERNAL_ROLL_NONE) + ball_2.unwrap_or(&0) {
                                    v if v >= 0 && v <= 10 => continue,
                                    _ => Err(Error::InvalidFrame(vec![ball_1.cloned(),
                                                                      ball_2.cloned()]))?,
                                }
                            },
                        }
                    }
                    Ok(rolls)
                })

                // Number of rolls does not exceed that of a valid game
                .and_then(|rolls| {
                    match rolls.len() <= 20 {
                        true => Ok(rolls),
                        false => Err(Error::InvalidGame(rolls.to_owned()))?,
                    }
                })
        }
        validate(rolls)?.iter()
                        .map(|v| u16::from(*v))
                        .fold(Err(Error::NoRolls), |acc, el| Ok(acc.unwrap_or(0) + el))
    }
}
