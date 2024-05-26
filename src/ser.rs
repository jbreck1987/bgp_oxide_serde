// Definition of custom Serializer

use bytes::{BytesMut, BufMut};
use serde::{ser, Serialize};

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
            // Max message size is 4096 octets. BytesMut is smart,
            // giving max capacity does not mean the message is guaranteed
            // to be that long!
            output: BytesMut::with_capacity(4096)
        };

        // Try to serialize the type and return the result
        in_type.serialize(&mut serializer)?;
        Ok(serializer.output)

    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    
    // Will be mutating the internal buffer, no need to return any intermediate results
    // to the caller
    type Ok = ();

    // Using our custom Error type here
    type Error = Error;

    // These will all be implemented within the Serializer type since
    // this is a simple data format.
    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    

    fn serialize_bool(self, v: bool) -> Result<()> {
        match v {
            true => self.output.put_u8(0u8),
            false => self.output.put_u8(1u8)
        }
        Ok(())
    }
    
    // BGP4 doesn't support signed integers
    fn serialize_i8(self, v: i8) -> Result<()> {
        Err(Error::UnsupportedSignedInt)
    }
    
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i8(0)
    }
    
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i8(0)
    }
    
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_i8(0)
    }
    
    fn serialize_u8(self, v: u8) -> Result<()> {
       todo!() 
    }
    
    fn serialize_u16(self, v: u16) -> Result<()> {
       todo!() 
    }
    
    fn serialize_u32(self, v: u32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_u64(self, v: u64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_f32(self, v: f32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_f64(self, v: f64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_char(self, v: char) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_str(self, v: &str) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_bytes(self, v: &[u8]) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_none(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_some<T>(self, value: &T) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }
    
    fn serialize_unit(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_unit_struct(self, name: &'static str) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }
    
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }
    
    fn serialize_seq(self, len: Option<usize>) -> std::prelude::v1::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple(self, len: usize) -> std::prelude::v1::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }
    
    fn serialize_map(self, len: Option<usize>) -> std::prelude::v1::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }
    
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }
    
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
    
    fn serialize_i128(self, v: i128) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(ser::Error::custom("i128 is not supported"))
    }
    
    fn serialize_u128(self, v: u128) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(ser::Error::custom("u128 is not supported"))
    }
    
    fn collect_seq<I>(self, iter: I) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let mut iter = std::iter.into_iter();
        let mut serializer = tri!(self.serialize_seq(iterator_len_hint(&iter)));
        tri!(iter.try_for_each(|item| serializer.serialize_element(&item)));
        serializer.end()
    }
    
    fn collect_map<K, V, I>(self, iter: I) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = std::iter.into_iter();
        let mut serializer = tri!(self.serialize_map(iterator_len_hint(&iter)));
        tri!(iter.try_for_each(|(key, value)| serializer.serialize_entry(&key, &value)));
        serializer.end()
    }
    
    fn collect_str<T>(self, value: &T) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + std::fmt::Display,
    {
        self.serialize_str(&value.to_string())
    }
    
    fn is_human_readable(&self) -> bool {
        true
    }

}