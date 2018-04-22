use std::sync::atomic::{AtomicUsize, Ordering};
use super::*;

mod mock_greeter;
mod mock_width_provider;

pub use self::mock_greeter::MockGreeter;
pub use self::mock_width_provider::MockWidthProvider;
