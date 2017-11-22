use std::sync::atomic::{AtomicBool, Ordering};

// Concurrency primitive store for AppSystemWideSingleton
static APP_IS_INSTANTIATED: AtomicBool = AtomicBool::new(false);

pub trait SingletonPrimitive {
    fn is_already_instantiated(&self) -> bool;
}

#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct AppSystemWideSingleton;

impl AppSystemWideSingleton {
    pub fn new() -> Self { Self {} }
}

impl SingletonPrimitive for AppSystemWideSingleton {
    fn is_already_instantiated(&self) -> bool {
        APP_IS_INSTANTIATED.compare_and_swap(false, true, Ordering::Relaxed)
    }
}

