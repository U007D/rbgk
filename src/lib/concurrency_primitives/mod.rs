#[cfg(test)] mod unit_tests;

use std::sync::atomic::{AtomicBool, Ordering};
use galvanic_mock::mockable;

// Concurrency primitive store for AppSystemWideSingleton
static APP_ALREADY_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[mockable]
pub trait Singleton {
    /// # Remarks
    /// Ensure exactly one caller (no more, no less) in a multiple-caller scenario can initialize the trait's
    /// implementation
    fn already_initialized(&self) -> bool;
}

#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct AppState;

impl AppState {
    pub fn new() -> Self { Self {} }
}

impl Singleton for AppState {
    fn already_initialized(&self) -> bool {
        APP_ALREADY_INITIALIZED.compare_and_swap(false, true, Ordering::SeqCst)
    }
}

