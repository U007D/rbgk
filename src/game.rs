#[derive(Clone, Default, Debug)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self, rolls: &[u8]) -> Option<u16> {
        match rolls.len() {
            0 => None,
            _ => Some(rolls.iter()
                           .map(|v| u16::from(*v))
                           .sum()),
        }
    }
}
