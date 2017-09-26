use std::env::ArgsOs;
use std::ops::{Deref, DerefMut};
use assayer::Error as AssayerError;
use assayer::Validator;

type Result<T> = ::std::result::Result<T, AssayerError>;

//Newtype Args wrapper around Vec<String> necessary to implement Validator<Args>.
#[derive(Clone, Debug)]
pub struct Args(Vec<String>);

impl Args {
    pub fn new(args: Vec<String>) -> Self { Args(args) }
}

impl Deref for Args {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Args {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<ArgsOs> for Args {
    fn from(args_os: ArgsOs) -> Self {
        //converts any non-unicode-representable character encoding -> � (U+FFFD replacement character)
        Args(args_os.map(|arg_os| arg_os.to_string_lossy()
                                        .to_string())
                    .collect::<Vec<_>>())
    }
}

impl Validator<Args> for Args {
    fn validate(value: Args) -> Result<Self> {
        Ok(value)
        // Err(AssayerError::ValueInvalid(format!("{:?}", value)))
    }
}
