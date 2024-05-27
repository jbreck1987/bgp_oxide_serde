// BGP message serialization and deserialization using serde

mod de;
mod error;
mod ser;

pub use de::Deserializer;
pub use error::{Error, Result};
pub use ser::Serializer;

#[cfg(test)]
mod tests {}
