// BGP message serialization and deserialization using serde

mod de;
mod error;
mod ser;

pub use de::Deserializer;
pub use error::{SerializerError, Result};
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod tests {}
