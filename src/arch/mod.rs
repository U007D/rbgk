#[cfg(test)] mod unit_tests;

pub trait Info {
    /// # Remarks
    /// Return the native width of the current architecture, in bits
    fn width(&self) -> usize;
}

#[allow(dead_code)] //Undetected use in galvanic unit test
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Arch;

impl Arch {
    #[allow(dead_code)] //Undetected use in galvanic unit test
    pub fn new() -> Self { Self {} }
}

impl Info for Arch {
    fn width(&self) -> usize { 0_usize.count_zeros() as usize }
}
