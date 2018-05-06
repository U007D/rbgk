use consts::*;
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
        size_of::<usize>().checked_mul(BITS_PER_BYTE)
                          .expect(MSG_ERR_INTERNAL_ARCH_WIDTH_BITS_EXCEEDS_USIZE)
    }
}
