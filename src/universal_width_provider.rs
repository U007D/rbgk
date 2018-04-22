use std::mem::size_of;

use width_provider::WidthProvider;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct UniversalWidthProvider {}

impl WidthProvider for UniversalWidthProvider {
    fn new() -> Self {
        Self {}
    }

    fn width(&self) -> usize {
        size_of::<usize>() * <Self as WidthProvider>::BITS_PER_BYTE
    }
}
