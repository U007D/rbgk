#[cfg(test)] mod unit_tests;

use std::sync::atomic::{AtomicBool, Ordering};
use galvanic_mock::mockable;

static SYSTEM_WIDE_SINGLETON_SENTINEL: AtomicBool = AtomicBool::new(false);

#[mockable]
pub trait Singleton {
    /// # Remarks
    /// Ensure exactly one caller (no more, no less) in a multiple-caller scenario can initialize the trait's
    /// implementation
    fn already_instantiated(&self) -> bool;
}

#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct SystemWideSingleton;

impl SystemWideSingleton {
    pub fn new() -> Self { Self {} }
}

impl Singleton for SystemWideSingleton {
    fn already_instantiated(&self) -> bool {
        SYSTEM_WIDE_SINGLETON_SENTINEL.compare_and_swap(false, true, Ordering::Relaxed)
    }
}

