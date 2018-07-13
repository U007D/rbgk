use super::*;
use std::ops::{Generator, GeneratorState};

////////////////////////////////////////////////////////////////////////////
// ensure each individual roll is between the min and max permissible values
#[inline]
pub(super) fn validate_roll_value(roll: u8) -> Result<u8> {
    match roll >= Game::FRAME_MIN_VALUE && roll <= Game::FRAME_MAX_VALUE {
        true => Ok(roll),
        false => Err(Error::InvalidRoll(roll)),
    }
}

/////////////////////////////////////////////////////////////////////////
// check roll value within context of bowling frame (stateful validation)
// (e.g. a 5 followed by a 7 in a single frame is not possible; nor is 17 frames in one game)
#[inline]
pub(super) fn validate_frame(roll: u8) -> Result<u8> {
    let mut frame_validator = || -> Error {
        for frame in Game::FIRST_FRAME..=Game::LAST_FRAME {
            let first_roll = roll;
            yield Ok(first_roll);

            // this frame has a second roll if it's the last frame OR if first roll was not a strike
            if frame == Game::LAST_FRAME || first_roll < Game::FRAME_MAX_VALUE {
                let second_roll = roll;
                match first_roll + second_roll <= Game::FRAME_MAX_VALUE {
                    true => yield Ok(second_roll),
                    false => yield Err(Error::InvalidFrame(frame, vec![first_roll, second_roll])),
                }

                // this frame has a third roll iff it's the last frame AND is a strike or spare
                if frame == Game::LAST_FRAME && first_roll + second_roll >= Game::FRAME_MAX_VALUE {
                    let third_roll = roll;
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
}
