// Definition of custom Serializer

use bytes::{BytesMut, BufMut};
use serde::Serialize;
use crate::error::{Error, Result};

// Since the serialization is basic (just to bytes), will only have one public
// method; to_bytes
pub struct Serializer {
    output: BytesMut
}

impl Serializer {
    pub fn to_bytes<T: Serialize>(&self, in_type: T) -> Result<BytesMut> {
        // Construct a new instance of Self
        let mut serializer = Self {
            // A little premature optimization, but
            // the minimum message size is the Message Header size at 19 octets.
            output: BytesMut::with_capacity(19)
        };

        // Try to serialize the type and return the result
        in_type.serialize(&mut serializer)?;
        Ok(serializer.output)

    }
}