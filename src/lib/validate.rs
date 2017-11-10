use std::marker::Sized;

type Result<T> = ::std::result::Result<T, super::Error>;

pub trait Validatable<F: FnOnce(&Self) -> Result<()>> where Self: Sized {
    fn validate_ref(&self, f: F) -> Result<&Self> where Self: Sized;
    fn validate_mut(&mut self, f: F) -> Result<&mut Self> where Self: Sized;
    fn validate(self, f: F) -> Result<Self> where Self: Sized;
}

impl<F: FnOnce(&Self) -> Result<()>, T> Validatable<F> for T where Self: Sized {
    fn validate_ref(&self, f: F) -> Result<&Self> where Self: Sized {
        f(self)?;
        Ok(self)
    }

    fn validate_mut(&mut self, f: F) -> Result<&mut Self> where Self: Sized {
        f(self)?;
        Ok(self)
    }

    fn validate(self, f: F) -> Result<Self> {
        f(&self)?;
        Ok(self)
    }
}
