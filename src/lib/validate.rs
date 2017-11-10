type Result<T> = ::std::result::Result<T, super::Error>;

pub trait Validate {
    fn validate(&self) -> Result<()>;
}

impl Validate for Vec<String> {
    fn validate(&self) -> Result<()> {
        Ok(self).and_then(|_input| Ok(()))
    }
}
