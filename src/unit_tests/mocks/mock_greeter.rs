use greeter::Greeter;
use std::cell::RefCell;
use super::*;

pub struct MockGreeter {
    pub greet_times_called: AtomicUsize,
    pub greet_args: RefCell<Vec<Vec<String>>>,   //TODO: Upgrade to AtomicVec<Vec<String>> for concurrency safety
}

impl MockGreeter {
    pub fn new() -> Self {
        Self {
            greet_times_called: AtomicUsize::new(0),
            greet_args: RefCell::new(Vec::new()),
        }
    }
}

impl Greeter for MockGreeter {
    fn greet(&self, args: Vec<String>) -> String {
        self.greet_times_called.fetch_add(1, Ordering::Relaxed);
        self.greet_args.borrow_mut().push(args);
        String::from("Hello, this is a test greeting.")
    }
}
