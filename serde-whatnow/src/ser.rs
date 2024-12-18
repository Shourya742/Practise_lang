
use serde::{ser, Serialize};

use crate::error::{Error, Result};

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized
    output: String
}

/// By convention, the public API of a Serde serializer is one or more `to_abc`
/// functions such as `to_string`, `to_bytes` or `to_writer` depending on what Rust 
/// types the serializer is able to produce as output.
/// 
/// This basic serializer supports only `to_string`
pub fn to_string<T> (value: &T) -> Result<String> where T: Serialize{
    let mut serializer = Serializer {
        output: String::new()
    };
    value.serialize(&mut serializer);
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    /// The output type produced by this `Serializer` during successful
    /// serializer. Most serializers that produce text or binary output should
    /// set `Ok = ()` and serialize into an `io::Write` or buffer contained
    /// withing the `Serializer` instance, as happens here. Serializers that build
    /// in-memory data structures may be simplified by using `OK` to propagate the
    /// data structure around.
    type Ok = ();

    /// The error type when some error occurs during serialization.
    type Error = Error;

    /// Associated types for keeping track of additional state while serializing
    /// compound data structures like sequences and maps, IN this case no additional
    /// state is required beyond what is already stored in the Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleVariant = Self;
    type SerializeTupleStruct = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;


    /// Here we go with the simple methods. The following 12 methods receive one
    /// of the primitive types of the data model and map it to JSON by appending
    /// into the output_string
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v {"true"} else {"false"};
        Ok(())
    }

    /// JSON does not distinguish between different sizes of integers, so all signed integers will be
    /// serialized the same and all unsigned integers
    /// will be serialized the same. Other formats, especially compact binary
    /// formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    /// A more proformant apporach would be to use the itoa crate
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output = v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    /// A more proformant apporach would be to use the itoa crate
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output = v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    /// A more proformant apporach would be to use the itoa crate
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output = v.to_string();
        Ok(())
    }

    /// Serialize a char as a single-character string,. Other formats may
    /// represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    /// This only works for strings that don't require escape sequences but you
    /// get the idea. For example it would emit invalid JSON if the input string
    /// contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    /// Serialize a byte array as an array of bytes. Could also use a base64
    /// string here. Binary formats will typically represent byte arrays more compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte);
        }
        seq.end()
    }

    /// An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    /// A present optional is represented as just the contained value, Note that
    /// this is a lossy representation. For example the values `Some(())` and `None`
    /// both serialize as just `null`. Unfortunately this is typically
    /// what people expect when working with JSON. Other formats are encouraged
    /// to behave more intelligently if possible
    fn serialize_some<T>(self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }

    /// In serde, unit means an anonymous value containing no data. Map this to JSON as `null`
    fn serialize_unit(self) -> Result<()> {
        self.output += "null";
        Ok(())
    }


    /// Unit struct means a named value containing no data. Again, since there is
    /// no data, map this to JSON as `null`. There is no need to serialize the
    /// name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    /// When serializing a unit variant (or any kind of variant), formats
    /// can choose whether to keep track of it by index or by name. Binary 
    /// formats typically use the index of the variant and human-readable formats
    /// typically use the name
    fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<()> {
        self.serialize_str(variant)
    }

    /// As is done here, serializers are encouraged to treat newtype structs as
    /// insignificant wrappers around that data they contain.
    fn serialize_newtype_struct<T>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }

    /// Note taht newtype variant (and all of the other variant serialization
    /// methods) refer exclusively to the "externally tagged" enum
    /// representation
    /// 
    /// Serialize this to JSON in externally tagged form as `{NAME: VALUE}`
    fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<()>
        where
            T: ?Sized + Serialize {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":";
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    /// Now we get the serialization of compound types.
    /// 
    /// The start of the sequence, each value and the end are three separate
    /// method calls. This
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "[";
        Ok(self)
    }
    
    /// Tuples look just like sequences in JSON. Some formats may be able to represent
    /// tuples more efficiently by omitting the length, since typle means that the corresponding 
    /// `Deserialize implementation will know the length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    /// Tuple structs look just like seqyences in JSON
    fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    /// Tuple variants are represented in JSON as `{NAME: [DATA...]}`. Again
    /// this method is only responsible for extenrally tagged representation.
    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V}`
    fn serialize_map(self, len: Option<usize>) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.output += "{";
        Ok(self)
    }

    /// Structs look just like maps in JSON. In particular, JSON requires taht we
    /// serialize the field names of the struct, Other formats may be able to 
    /// omit the field names when serializing structs because the correspoding
    /// Deserialize impleementation is required to know waht the keys are without looking at the serialized data.
    fn serialize_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    /// Struct variants are represented in JSON as `{NAME: {K:V,.....}}`.
    /// This is the externally tagged representation
    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}

/// The following 7 impls deal with the serialization of compound types like
/// sequence and maps. Serialization of such types is begun by a Serializer
/// method and followed by zero or more calls to serialize individual elements of the compound 
/// type and on call to end the compound type.
/// 
/// This iml is SerializeSeq so these methods are called after `serialize_seq` is called on the
/// Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

/// Same thing but for tuple
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}

// Same thing byt for typle struct
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}

/// Tuple variants are a liitle different. Refer back to the 
/// `serialize_tuple_variant` method above:
/// 
/// self.output += "{"
/// variant.serialize(&mut *self)?;
/// self,output += ":[";
/// 
/// So the `end` method in this impl is respondible for closing both the ']' and the `}`;
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "]}";
        Ok(())
    }
}


/// Some `Serialize` types are not able to hold a key and value in memory at the same time 
/// so `SerializeMap` implementations are required to support
/// `serialize_key` and `serialize_value` individually.
/// 
/// There is a third optional method on the `SerializeMap` trait. The
/// `serialize_entry` method allows serializers to optimize for the case where
/// key and value are both avialable simulaneously. In JSON it doesn't make a difference
/// so the default behavior for `serialize_entry` is fine
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    /// The Serde data model allows map keys to be any serializable type. JSON
    /// only allows string keys so the implementation below will produce invalid
    /// JSON if the key serializes as something other than a string.
    /// 
    /// A real JSON serializer would need to validate that map keys are strings.
    /// This can be done by using a different Serializer to serialize the key
    /// (instead of  &mut **self) having that other serializer only
    /// implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with("{") {
            self.output +=",";
        }
        key.serialize(&mut **self)
    }

    /// It doesn't make a difference whether the colon is printed at the end of `serialize_key` or
    /// at the beginning of `serialize_value`. IN this case the code is bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}";
        Ok(())
    }
}


impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with("{") {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}";
        Ok(())
    }
}


impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if !self.output.ends_with("{") {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}}";
        Ok(())
    }
}
