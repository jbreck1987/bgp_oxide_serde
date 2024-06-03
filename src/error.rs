// Defines the errors used by both Serializer and Deserializer

use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, SerializerError>;

#[derive(Debug)]
pub enum SerializerError {
    // To stay generic, will have a variant that deliver
    // generic error messages. Will add more variants as their
    // need arises.
    CustomMsg(String),
    UnsupportedSignedInt(Option<String>),
    UnsupportedFloat(Option<String>),
    UnsupportedMap(Option<String>),
    UnsupportedText(Option<String>)
}

impl std::error::Error for SerializerError {}

impl Display for SerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializerError::CustomMsg(msg) => f.write_str(msg),
            SerializerError::UnsupportedSignedInt(Some(msg)) => {
                f.write_str(&format!("Serialization of signed ints unsupported. Error info - {}.", msg))
            },
            SerializerError::UnsupportedSignedInt(None) => {
                f.write_str("Serialization of signed ints unsupported.")
            },
            SerializerError::UnsupportedFloat(Some(msg)) => {
                f.write_str(&format!("Serialization of floats unsupported. Error info - {}.", msg))
            },
            SerializerError::UnsupportedFloat(None) => {
                f.write_str("Serialization of floats unsupported.")
            },
            SerializerError::UnsupportedMap(Some(msg)) => {
                f.write_str(&format!("Serialization of maps unsupported. Error info - {}.", msg))
            },
            SerializerError::UnsupportedMap(None) => {
                f.write_str("Serialization of maps unsupported.")
            },
            SerializerError::UnsupportedText(Some(msg)) => {
                f.write_str(&format!("Serialization of text types unsupported. Error info - {}", msg))
            },
            SerializerError::UnsupportedText(None) => {
                f.write_str("Serialization of text types unsupported.")
            },
            _ => f.write_str("Undefined metadata")
        }
    }
}
impl ser::Error for SerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        SerializerError::CustomMsg(msg.to_string())
    }
}

impl de::Error for SerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        SerializerError::CustomMsg(msg.to_string())
    }
}