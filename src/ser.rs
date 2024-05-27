// Definition of custom Serializer

use bytes::{BytesMut, BufMut};
use serde::{ser, Serialize};

use crate::error::{Error, Result};

// Since the serialization is basic (just to bytes), will only have one public
// method; to_bytes
pub struct Serializer {
    output: BytesMut
}


pub fn to_bytes<T: Serialize>(in_type: T) -> Result<BytesMut> {
        // Construct a new instance of Self
        let mut serializer = Serializer {
            // Max message size is 4096 octets. BytesMut is smart,
            // giving max capacity does not mean the message is guaranteed
            // to be that long!
            output: BytesMut::with_capacity(4096)
    };

// Try to serialize the type and return the result
        in_type.serialize(&mut serializer)?;
        Ok(serializer.output)
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
            true => self.output.put_u8(1u8),
            false => self.output.put_u8(0u8)
        }
        Ok(())
    }
    
    // BGP4 doesn't support signed integers
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Error::UnsupportedSignedInt)
    }
    
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::UnsupportedSignedInt)
    }
    
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::UnsupportedSignedInt)
    }
    
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::UnsupportedSignedInt)
    }
    
    fn serialize_u8(self, v: u8) -> Result<()> {
       self.output.put_u8(v);
       Ok(())
    }
    // BytesMut put_x methods store multi-byte
    // values in network byte order by default.
    fn serialize_u16(self, v: u16) -> Result<()> {
       self.output.put_u16(v);
       Ok(())
    }
    
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.put_u32(v);
        Ok(())
    }
    
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.put_u64(v);
        Ok(())
    }
    
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::UnsupportedFloat)
    }
    
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::UnsupportedFloat)
    }
    
    fn serialize_char(self, _v: char) -> Result<()> {
       Err(Error::UnsupportedText) 
    }
    
    fn serialize_str(self, _v: &str) -> Result<()>  {
        Err(Error::UnsupportedText)
    }
    
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.put_slice(v);
        Ok(())
    }
    
    fn serialize_none(self) -> Result<()> {
        // If None, do nothing.
        Ok(())
    }
    
   fn serialize_some<T>(self, value: &T) -> Result<()>
       where
           T: ?Sized + Serialize 
    {
       // Serialize the inner
       value.serialize(self)
    } 
    
    fn serialize_unit(self) -> Result<()> {
        // Do nothing for these, but no need to error
        Ok(())
    }
    
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        // Do nothing here, no need to error.
        Ok(())
    }
    
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        // Do nothing with these, no need to error.
        Ok(())
    }
    
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> 
    where
        T: ?Sized + ser::Serialize {
            // Serialize the inner
            value.serialize(self)
    }
    
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()> 
    where
        T: ?Sized + ser::Serialize {
        // Will only serialize the inner, no use (for now) for the
        // variant metadata.
        value.serialize(self)
    }
    
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        // Nothing special about initializing sequences, the protocol is binary and self-describing.
        Ok(self)
    }
    
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        // Tuples are the same a sequences in the protocol, no special init setup necessary.
        Ok(self)
    }
    
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        // Same as serialize tuple, the protocol doesnt care about the tuple name.
        Ok(self)
    }
    
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // Same as tuple struct, no use for metadata.
        Ok(self)
    }
    
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // No use for maps in the message formatting (for now), map serialization will be unsupported.
        // The message types have pre-defined structure, so can't see a need for these arising in the future
        Err(Error::UnsupportedMap)
    }
    
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        // No use for struct metadata
        Ok(self)
    }
    
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // No use for struct variant metadata
        Ok(self)
    }
    
}

// Now to define the impls that handle compound types.
// The structure of the message types are pre-defined
// and are self-describing. Most of these will be identical.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                // Since format is binary, no special handling
                // between elements. Just stick them in the buffer
                // in order.
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        // Again, no special closing character in the
        // format, nothing special for the end.
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                // Implementation no different for sequences and tuples.
                // Format is fixed and/or self-describing
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        // Same as sequence, nothing special for the end.
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Map is unsupported in the format (for now)
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                Err(Error::UnsupportedMap)
    }
    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<()>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize, {
                Err(Error::UnsupportedMap)
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                Err(Error::UnsupportedMap)
    }
    fn end(self) -> Result<()> {
       Err(Error::UnsupportedMap) 
    }
}