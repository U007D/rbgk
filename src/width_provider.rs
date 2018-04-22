pub trait WidthProvider {
    const BITS_PER_BYTE: usize = 8;

    fn new() -> Self;
    fn width(&self) -> usize;
}
