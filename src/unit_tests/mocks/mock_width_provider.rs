use super::*;
use self::greeter::WidthProvider;

pub struct MockWidthProvider {
    pub width_times_called: AtomicUsize,
}

impl WidthProvider for MockWidthProvider {
    fn new() -> Self {
        Self {
            width_times_called: AtomicUsize::new(0),
        }
    }

    fn width(&self) -> usize {
        self.width_times_called.fetch_add(1, Ordering::Relaxed);
        42
    }
}
