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

    pub fn score(&self, rolls: &[u8]) -> Option<u16> {
        enum Frame {
            Strike,
            SpareOrOpen,
        }

        #[allow(integer_arithmetic, indexing_slicing)]
        fn inner_score<I>(rolls: &I, curr_frame: usize) -> Option<u16> where I: Clone + Debug + Iterator {
            if curr_frame > FRAMES_PER_GAME {
                return None;
            }

            let frame_type = match rolls.clone().next() {
                Some(&ALL_PINS_HIT) => Frame::Strike,
                _ => Frame::SpareOrOpen,
            };
            let rolls_to_start_of_next_frame = match frame_type {
                Frame::Strike => STRIKE_ROLLS,
                Frame::SpareOrOpen => SPARE_OR_OPEN_ROLLS,
            };

            println!("***{:?}***", rolls);
            Some(rolls.clone()
                      .take(match frame_type {
                          Frame::Strike => STRIKE_SCORING_ROLLS,
                          Frame::SpareOrOpen => SPARE_OR_OPEN_SCORING_ROLLS,
                      })
                      .map(|&v| u16::from(v))
                      .sum::<u16>()
                + inner_score(rolls, curr_frame + 1).unwrap_or(0))
        }

        let foo = inner_score(&rolls.iter(), 1);
        println!("*** result ***");
        foo
    }
}


