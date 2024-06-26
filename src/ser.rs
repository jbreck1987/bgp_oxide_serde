// Definition of custom Serializer
use bytes::{BytesMut, BufMut};
use serde::{ser, Serialize};

use crate::error::{SerializerError, Result};

// Since the serialization is basic (just to bytes), will only have one public
// method; to_bytes
// the err_metadata field is used for holding metadata for returning useful
// error messages, based on the wrapper type that had a field fail serialization.
pub struct Serializer {
    output: BytesMut,
    _err_type_metadata: String,
    _err_variant_metadata: String,
    _err_field_metadata: String
}


pub fn to_bytes<T: Serialize>(in_type: T) -> Result<BytesMut> {
        // Construct a new instance of Self
        let mut serializer = Serializer {
            // Max message size is 4096 octets. BytesMut is smart,
            // giving max capacity does not mean the message is guaranteed
            // to be that long!
            output: BytesMut::with_capacity(4096),
            _err_type_metadata: String::new(),
            _err_variant_metadata: String::new(),
            _err_field_metadata: String::new(),
    };

// Try to serialize the type and return the result
        in_type.serialize(&mut serializer)?;
        Ok(serializer.output)
}

impl Serializer {
    // Function to format the metadata to use for errors.
    fn format_metadata(&self) -> Option<String> {
        let t = &self._err_type_metadata;
        let v = &self._err_variant_metadata;
        let f = &self._err_field_metadata;

        match (self._err_type_metadata.is_empty(),
               self._err_variant_metadata.is_empty(),
               self._err_field_metadata.is_empty())
        {
            (false, false, false) => {
                Some(format!("Type: \"{}\", Variant: \"{}\", Field: \"{}\"", t, v, f))
            },
            (false, false, true) => {
                Some(format!("Type: \"{}\", Variant: \"{}\"", t, v))
            },
            (false, true, false) => {
                Some(format!("Type: \"{}\", Field: \"{}\"", t, f))
            },
            (false, true, true) => {
                Some(format!("Type: \"{}\"", t))
            }
            _ => None
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    
    // Will be mutating the internal buffer, no need to return any intermediate results
    // to the caller
    type Ok = ();

    // Using our custom Error type here
    type Error = SerializerError;

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
        Err(SerializerError::UnsupportedSignedInt(self.format_metadata()))
    }
    
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(SerializerError::UnsupportedSignedInt(self.format_metadata()))
    }
    
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(SerializerError::UnsupportedSignedInt(self.format_metadata()))
    }
    
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(SerializerError::UnsupportedSignedInt(self.format_metadata()))
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
        Err(SerializerError::UnsupportedFloat(self.format_metadata()))
    }
    
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(SerializerError::UnsupportedFloat(self.format_metadata()))
    }
    
    fn serialize_char(self, _v: char) -> Result<()> {
       Err(SerializerError::UnsupportedText(self.format_metadata()))
    }
    
    fn serialize_str(self, _v: &str) -> Result<()>  {
        Err(SerializerError::UnsupportedText(self.format_metadata()))
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
        name: &'static str,
        value: &T,
    ) -> Result<()> 
    where
        T: ?Sized + ser::Serialize {
            self._err_type_metadata = String::from(name);
            self._err_field_metadata.clear();
            self._err_variant_metadata.clear();
            value.serialize(self)
    }
    
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> 
    where
        T: ?Sized + ser::Serialize {

        self._err_type_metadata = String::from(name);
        self._err_variant_metadata = String::from(variant);
        self._err_field_metadata.clear();

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
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self._err_type_metadata = String::from(name);
        self._err_field_metadata.clear();
        self._err_variant_metadata.clear();
        Ok(self)
    }
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        
        self._err_type_metadata = String::from(name);
        self._err_variant_metadata = String::from(variant);
        self._err_field_metadata.clear();
        Ok(self)
    }
    
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // No use for maps in the message formatting (for now), map serialization will be unsupported.
        Err(SerializerError::UnsupportedMap(self.format_metadata()))
    }
    
    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self._err_type_metadata = String::from(name);
        self._err_field_metadata.clear();
        self._err_variant_metadata.clear();
        Ok(self)
    }
    
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self._err_type_metadata = String::from(name);
        self._err_variant_metadata = String::from(variant);
        self._err_field_metadata.clear();
        Ok(self)
    }
    
}

// Now to define the impls that handle compound types.
// The structure of the message types are pre-defined
// and are self-describing. Most of these will be identical.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = SerializerError;

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
    type Error = SerializerError;

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
    type Error = SerializerError;

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
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                self._err_field_metadata = String::from(key);
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializerError;

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
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                self._err_field_metadata = String::from(key);
                value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Map is unsupported in the format (for now)
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                Err(SerializerError::UnsupportedMap(self.format_metadata()))

    }
    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<()>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize, {
                Err(SerializerError::UnsupportedMap(self.format_metadata()))

    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
                Err(SerializerError::UnsupportedMap(self.format_metadata()))

    }
    fn end(self) -> Result<()> {
       Err(SerializerError::UnsupportedMap(self.format_metadata()))

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    // Types used for testing error conditions
    //
    // -- Enums --
    #[derive(Serialize)]
    enum EnumHashTest {
        NewTypeVariant(HashMap<&'static str, u8>),
        StructVariant{field: HashMap<&'static str, u8>},
        TupleVariant(u8, HashMap<&'static str, u8>)
    }   
    #[derive(Serialize)]
    enum EnumSignedIntTest {
        NewTypeVariant(i8),
        StructVariant{field: i16},
        TupleVariant(u8, i32)
    }
    #[derive(Serialize)]
    enum EnumFloatTest {
        NewTypeVariant(f32),
        StructVariant{field: f64},
        TupleVariant(u8, f32)
    }
    #[derive(Serialize)]
    enum EnumTextTest {
        NewTypeVariant(char),
        StructVariant{field: String},
        TupleVariant(u8, &'static str)
    }

    // -- Structs --
    #[derive(Serialize)]
    struct NewTypeStructHash(HashMap<u8, &'static str>);
    #[derive(Serialize)]
    struct TupleStructHash(u8, HashMap<u8, &'static str>);
    #[derive(Serialize)]
    struct StructHash {
        field: HashMap<u8, &'static str>
    }

    #[derive(Serialize)]
    struct NewTypeStructFloat(f32);
    #[derive(Serialize)]
    struct TupleStructFloat(u8, f64);
    #[derive(Serialize)]
    struct StructFloat {
        field: f32
    }

    #[derive(Serialize)]
    struct NewTypeStructText(char);
    #[derive(Serialize)]
    struct TupleStructText(u8, String);
    #[derive(Serialize)]
    struct StructText {
        field: &'static str 
    }

    #[derive(Serialize)]
    struct NewTypeStructSignedInt(i8);
    #[derive(Serialize)]
    struct TupleStructSignedInt(u8, i32);
    #[derive(Serialize)]
    struct StructSignedInt {
        field: i64
    }

    #[test]
    fn test_err_enum_hash() {
        let test_ntype = EnumHashTest::NewTypeVariant(HashMap::new());
        let test_struct = EnumHashTest::StructVariant {field: HashMap::new()};
        let test_tuple = EnumHashTest::TupleVariant(42, HashMap::new());

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"EnumHashTest\", Variant: \"NewTypeVariant\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"EnumHashTest\", Variant: \"StructVariant\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"EnumHashTest\", Variant: \"TupleVariant\".")
            },
        }
    }
    
    #[test]
    fn test_err_enum_float() {
        let test_ntype = EnumFloatTest::NewTypeVariant(0.0);
        let test_struct = EnumFloatTest::StructVariant {field: 6.023e23};
        let test_tuple = EnumFloatTest::TupleVariant(42, 3.14);

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"EnumFloatTest\", Variant: \"NewTypeVariant\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"EnumFloatTest\", Variant: \"StructVariant\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"EnumFloatTest\", Variant: \"TupleVariant\".")
            },
        }
    }
    #[test]
    fn test_err_enum_text() {
        let test_ntype = EnumTextTest::NewTypeVariant('F');
        let test_struct = EnumTextTest::StructVariant {field: "In the chat".to_string()};
        let test_tuple = EnumTextTest::TupleVariant(42, "for Harambe");

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"EnumTextTest\", Variant: \"NewTypeVariant\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"EnumTextTest\", Variant: \"StructVariant\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"EnumTextTest\", Variant: \"TupleVariant\".")
            },
        }
    }
    #[test]
    fn test_err_enum_sint() {
        let test_ntype = EnumSignedIntTest::NewTypeVariant(42i8);
        let test_struct = EnumSignedIntTest::StructVariant {field: 13i16};
        let test_tuple = EnumSignedIntTest::TupleVariant(42, 0i32);

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"EnumSignedIntTest\", Variant: \"NewTypeVariant\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"EnumSignedIntTest\", Variant: \"StructVariant\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"EnumSignedIntTest\", Variant: \"TupleVariant\".")
            },
        }
    }
    
    #[test]
    fn test_err_struct_hash() {
        let test_ntype = NewTypeStructHash(HashMap::new());
        let test_struct = StructHash {field: HashMap::new()};
        let test_tuple = TupleStructHash(42, HashMap::new());

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"NewTypeStructHash\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"StructHash\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of maps unsupported. Error info - Type: \"TupleStructHash\".")
            },
        }
    }
    
    #[test]
    fn test_err_struct_float() {
        let test_ntype = NewTypeStructFloat(3.14);
        let test_struct = StructFloat {field: 6.022e23};
        let test_tuple = TupleStructFloat(42, 9.0);

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"NewTypeStructFloat\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"StructFloat\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of floats unsupported. Error info - Type: \"TupleStructFloat\".")
            },
        }
    }
    
    #[test]
    fn test_err_struct_sint() {
        let test_ntype = NewTypeStructSignedInt(-9);
        let test_struct = StructSignedInt {field: -6};
        let test_tuple = TupleStructSignedInt(42, -9);

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"NewTypeStructSignedInt\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"StructSignedInt\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of signed ints unsupported. Error info - Type: \"TupleStructSignedInt\".")
            },
        }
    }
    
    #[test]
    fn test_err_struct_text() {
        let test_ntype = NewTypeStructText('F');
        let test_struct = StructText {field: "oopsie"};
        let test_tuple = TupleStructText(42, "my bad".to_string());

        let szed_ntype = to_bytes(test_ntype);
        let szed_struct = to_bytes(test_struct);
        let szed_tuple = to_bytes(test_tuple);

        match szed_ntype {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"NewTypeStructText\".")
            },
        }
        match szed_struct {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"StructText\", Field: \"field\".")
            },
        }
        match szed_tuple {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(e.to_string(), "Serialization of text types unsupported. Error info - Type: \"TupleStructText\".")
            },
        }
    }
}