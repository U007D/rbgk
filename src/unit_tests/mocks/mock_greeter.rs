use greeter::Greeter;
use super::*;

pub struct MockGreeter {
    pub greet_times_called: AtomicUsize,
}

impl MockGreeter {
    pub fn new() -> Self {
        Self {
            greet_times_called: AtomicUsize::new(0),
        }
    }
}

impl Greeter for MockGreeter {
    fn greet(&self) -> String {
        self.greet_times_called.fetch_add(1, Ordering::Relaxed);
        String::from("Hello, this is a test greeting.")
    }
}
