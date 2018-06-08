use std::fmt::Debug;

const ALL_PINS_HIT: u8 = 10;
const FRAMES_PER_GAME: usize = 10;
const STRIKE_SCORING_ROLLS: usize = 3;
const SPARE_OR_OPEN_SCORING_ROLLS: usize = 2;
const STRIKE_ROLLS: usize = 1;
const SPARE_OR_OPEN_ROLLS: usize = 2;

#[derive(Clone, Default, Debug)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self, rolls: &[u8]) -> u16 {
        enum Frame {
            Strike,
            SpareOrOpen,
        }

        #[allow(integer_arithmetic, indexing_slicing)]
        fn inner_score(rolls: &[u8], curr_frame: usize) -> u16 {
            match curr_frame > FRAMES_PER_GAME {
                true => return None,
                false => rolls.iter()
                              .take(match rolls.iter()
                                               .take(2) {
                                        v if v >= 10 => 3,
                                        _ => 2,
                                    })
                              .map(|&v| u16::from(v))
                              .sum::<u16>() +
                         match rolls.get(0) {
                            Some(v) =>i nner_score()
            }
            }

            let (frame_type, rolls_to_start_of_next_frame) = match rolls.get(0) {
                Some(&ALL_PINS_HIT) => (Frame::Strike, STRIKE_ROLLS),
                _ => (Frame::SpareOrOpen, SPARE_OR_OPEN_ROLLS),
            };

            Some(rolls.iter()
                      .take(match frame_type {
                          Frame::Strike => STRIKE_SCORING_ROLLS,
                          Frame::SpareOrOpen => SPARE_OR_OPEN_SCORING_ROLLS,
                      })
                      .map(|&v| u16::from(v))
                      .sum::<u16>() +
                match rolls.get(rolls_to_start_of_next_frame..) {
                    Some(s) => inner_score(s, curr_frame + 1).unwrap_or(0),
                    None => 0,
                })
        }

        inner_score(rolls, 1)
    }
}


