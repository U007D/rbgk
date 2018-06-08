#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self, rolls: &[u8]) -> Option<u16> {
        rolls.get(0)
             .map(|&v| u16::from(v))
    }
}
