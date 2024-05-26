// Defines the errors used by both Serializer and Deserializer

use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // To stay generic, will have a variant that deliver
    // generic error messages. Will add more variants as their
    // need arises.
    CustomMsg(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CustomMsg(msg) => f.write_str(msg)
        }
    }
}
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::CustomMsg(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::CustomMsg(msg.to_string())
    }
}