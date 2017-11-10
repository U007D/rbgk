use std::marker::Sized;

type Result<T> = ::std::result::Result<T, super::Error>;

pub trait Validatable<F: FnOnce() -> Result<()>> {
    fn validate_ref(&self, f: F) -> Result<&Self> where Self: Sized;
    fn validate_mut(&mut self, f: F) -> Result<&mut Self> where Self: Sized;
    fn validate(self, f: F) -> Result<Self> where Self: Sized;
}

impl<F: FnOnce() -> Result<()>, T> Validatable<F> for T {
    fn validate_ref(&self, f: F) -> Result<&Self> where Self: Sized {
        f()?;
        Ok(self)
    }

    fn validate_mut(&mut self, f: F) -> Result<&mut Self> where Self: Sized {
        f()?;
        Ok(self)
    }

    fn validate(self, f: F) -> Result<Self> {
        f()?;
        Ok(self)
    }
}
