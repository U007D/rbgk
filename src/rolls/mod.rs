#[cfg(test)]
mod unit_tests;

use super::{consts::*, Error, Game, Result};
use std::ops::{Deref, Generator, GeneratorState};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
//#[allow(needless_borrow)] //suppress false-positive from Clippy; does not repro on Playground :( http://play.rust-lang.org/?gist=d708c668e1adaf67d7640512e5840198&version=nightly&mode=debug&edition=2015
pub struct Rolls<'a>(&'a [u8]);

impl<'a> Rolls<'a> {
    pub fn new(rolls: &'a [u8]) -> Result<Self> {
        Ok(Rolls(Self::validate(rolls)?))
    }

    fn validate(rolls: &'a [u8]) -> Result<&'a [u8]> {
        #[allow(unused_comparisons, absurd_extreme_comparisons)]
            match rolls.iter()
                       .map(|v|
                           // check absolute roll value
                           match *v >= Game::FRAME_MIN_VALUE && *v <= Game::FRAME_MAX_VALUE {
                               true => Ok(v),
                               false => Err(Error::InvalidRoll(*v)),
                           }

                           // check roll value within context of bowling frame (stateful validation)
                           // (e.g. a 5 followed by a 7 in a single frame is not possible; nor is 17 frames in one game)
                           .and_then(|v| {
                               let mut frame_validator = || -> Error {
                                   for frame in Game::FIRST_FRAME..=Game::LAST_FRAME {
                                       let first_roll = *v;
                                       yield Ok(first_roll);

                                       // this frame has a second roll if it's the last frame OR if first roll was not a strike
                                       if frame == Game::LAST_FRAME || first_roll < Game::FRAME_MAX_VALUE {
                                           let second_roll = *v;
                                           match first_roll + second_roll <= Game::FRAME_MAX_VALUE {
                                               true => yield Ok(second_roll),
                                               false => yield Err(Error::InvalidFrame(frame, vec![first_roll, second_roll])),
                                           }

                                           // this frame has a third roll iff it's the last frame AND is a strike or spare
                                           if frame == Game::LAST_FRAME && first_roll + second_roll >= Game::FRAME_MAX_VALUE {
                                               let third_roll = *v;
                                               yield Ok(third_roll);
                                           }
                                       }
                                   };
                                   Error::TooManyRolls
                               };

                               match unsafe { frame_validator.resume() } {
                                   GeneratorState::Yielded(Ok(v)) => Ok(v),
                                   GeneratorState::Yielded(Err(e)) => Err(e),
                                   GeneratorState::Complete(e) => Err(e),
                               }
                           })
                       )
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
