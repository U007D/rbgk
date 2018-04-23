use std::mem::size_of;
use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BitWidthProvider {}

const BITS_PER_BYTE: usize = 8;

impl WidthProvider for BitWidthProvider {
    fn new() -> Self {
        Self {}
    }

    fn width(&self) -> usize {
        size_of::<usize>() * BITS_PER_BYTE
    }
}
