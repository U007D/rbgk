pub trait WidthProvider {
    fn new() -> Self;
    fn width(&self) -> usize;
}
