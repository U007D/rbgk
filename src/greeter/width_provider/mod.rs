mod bit_width_provider;

pub use self::bit_width_provider::BitWidthProvider;

pub trait WidthProvider {
    fn new() -> Self;
    fn width(&self) -> usize;
}
