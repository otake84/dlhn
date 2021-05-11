use std::{collections::BTreeMap, fmt::{self, Display}, io::Write};
use dullahan::{body::Body, serializer::serialize_body};
use serde::{Serialize, de, ser::{self, Impossible}};
use integer_encoding::VarInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Write,
    Syntax,
    UnknownSeqSize,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Syntax => formatter.write_str("syntax error"),
            Error::Write => formatter.write_str("write error"),
            Error::UnknownSeqSize => formatter.write_str("unknown seq size"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Serializer<W: Write> {
    output: W,
}

impl<W: Write> Serializer<W> {
    pub fn new(output: W) -> Self {
        Self {
            output,
        }
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = StructSerializer<'a, W>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::Boolean(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::Int8(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarInt16(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarInt32(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarInt64(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::UInt8(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarUInt16(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarUInt32(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::VarUInt64(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::Float32(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::Float64(v)).as_slice()).or(Err(Error::Write))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::String(v.to_string())).as_slice()).or(Err(Error::Write))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(serialize_body(&Body::String(v.to_string())).as_slice()).or(Err(Error::Write))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(v.len().encode_var_vec().as_slice()).or(Err(Error::Write))?;
        self.output.write_all(v).or(Err(Error::Write))?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(&[0u8]).or(Err(Error::Write))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
            self.output.write_all(&[1u8]).or(Err(Error::Write))?;
            value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.output.write_all(len.encode_var_vec().as_slice()).or(Err(Error::Write))?;
            Ok(self)
        } else {
            Err(Error::UnknownSeqSize)
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            output: &mut self.output,
            buf: BTreeMap::new(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            output: &mut self.output,
            buf: BTreeMap::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct StructSerializer<'a, W: Write> {
    output: &'a mut W,
    buf: BTreeMap<String, Vec<u8>>,
}

impl<'a, W: Write> ser::SerializeStruct for StructSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let mut buf = Vec::new();
        value.serialize(&mut Serializer::new(&mut buf))?;
        self.buf.insert(key.to_string(), buf);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        for value in self.buf.values() {
            self.output.write_all(value).or(Err(Error::Write))?;
        }
        Ok(())
    }
}

pub struct MapSerializer<'a, W: Write> {
    output: &'a mut W,
    buf: BTreeMap<String, Vec<u8>>,
}

impl<'a, W: Write> ser::SerializeMap for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize, {
            let mut key_buf = Vec::new();
            key.serialize(MapKeySerializer {
                output: &mut key_buf,
            })?;
            let mut value_buf = Vec::new();
            value.serialize(&mut Serializer::new(&mut value_buf))?;
            self.buf.insert(String::from_utf8(key_buf).or(Err(Error::Write))?, value_buf);
            Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(self.buf.len().encode_var_vec().as_slice()).or(Err(Error::Write))?;

        for (k, v) in self.buf.into_iter() {
            self.output.write_all(serialize_body(&Body::String(k)).as_slice()).or(Err(Error::Write))?;
            self.output.write_all(v.as_slice()).or(Err(Error::Write))?;
        }
        Ok(())
    }
}

pub struct MapKeySerializer<'a, W: Write> {
    output: &'a mut W,
}

impl<'a, W: Write> ser::Serializer for MapKeySerializer<'a, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(v.as_bytes()).or(Err(Error::Write))?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use serde::Serialize;
    use dullahan::{body::Body, serializer::serialize_body};
    use serde_bytes::Bytes;
    use super::Serializer;

    #[test]
    fn serialize_bool() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);

            let body = true;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Boolean(true)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);

            let body = false;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Boolean(false)));
        }
    }

    #[test]
    fn serialize_i8() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i8::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Int8(i8::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i8::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Int8(i8::MAX)));
        }
    }

    #[test]
    fn serialize_i16() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i16::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt16(i16::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i16::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt16(i16::MAX)));
        }
    }

    #[test]
    fn serialize_i32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i32::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt32(i32::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i32::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt32(i32::MAX)));
        }
    }

    #[test]
    fn serialize_i64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i64::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt64(i64::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i64::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarInt64(i64::MAX)));
        }
    }

    #[test]
    fn serialize_u8() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u8::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::UInt8(u8::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u8::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::UInt8(u8::MAX)));
        }
    }

    #[test]
    fn serialize_u16() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u16::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt16(u16::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u16::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt16(u16::MAX)));
        }
    }

    #[test]
    fn serialize_u32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u32::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt32(u32::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u32::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt32(u32::MAX)));
        }
    }

    #[test]
    fn serialize_u64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u64::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt64(u64::MIN)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u64::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::VarUInt64(u64::MAX)));
        }
    }

    #[test]
    fn serialize_f32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 1.2f32;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Float32(1.2)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = f32::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Float32(f32::INFINITY)));
        }
    }

    #[test]
    fn serialize_f64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 1.2f64;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Float64(1.2)));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = f64::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Float64(f64::INFINITY)));
        }
    }

    #[test]
    fn serialize_char() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 'a';
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::String(String::from('a'))));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 'あ';
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::String(String::from('あ'))));
        }
    }

    #[test]
    fn serialize_str() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = "";
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::String("".to_string())));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = "test";
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::String("test".to_string())));
        }
    }

    #[test]
    fn serialize_none() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body: Option<bool> = None;
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Optional(None)));
    }

    #[test]
    fn serialize_some() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Some(123u8);
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Optional(Some(Box::new(Body::UInt8(123))))));
    }

    #[test]
    fn serialize_unit() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = ();
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Unit));
    }

    #[test]
    fn serialize_unit_struct() {
        #[derive(Debug, PartialEq, Serialize)]
        struct Test;

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test{};
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Unit));
    }

    #[test]
    fn serialize_newtype_struct() {
        {
            #[derive(Debug, PartialEq, Serialize)]
            struct Inner {
                c: String,
                a: bool,
                b: u8,
            }
            #[derive(Debug, PartialEq, Serialize)]
            struct Test(Inner);

            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            Test(Inner {
                c: "test".to_string(),
                a: true,
                b: 123,
            }).serialize(&mut serializer).unwrap();

            assert_eq!(buf, serialize_body(&Body::Map({
                let mut map = BTreeMap::new();
                map.insert("c".to_string(), Body::String("test".to_string()));
                map.insert("a".to_string(), Body::Boolean(true));
                map.insert("b".to_string(), Body::UInt8(123));
                map
            })));
        }

        {
            #[derive(Debug, PartialEq, Serialize)]
            struct Inner(u8);
            #[derive(Debug, PartialEq, Serialize)]
            struct Test {
                c: String,
                a: Inner,
                b: bool,
            }

            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            Test {
                c: "test".to_string(),
                a: Inner(123),
                b: true,
            }.serialize(&mut serializer).unwrap();

            assert_eq!(buf, serialize_body(&Body::Map({
                let mut map = BTreeMap::new();
                map.insert("c".to_string(), Body::String("test".to_string()));
                map.insert("a".to_string(), Body::Tuple(vec![Body::UInt8(123)]));
                map.insert("b".to_string(), Body::Boolean(true));
                map
            })));
        }
    }

    #[test]
    fn serialize_seq() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body: Vec<u8> = Vec::new();
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Array(Vec::new())));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = vec![123u8];
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Array(vec![Body::UInt8(123)])));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = [1u8].repeat(128);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Array([1u8].repeat(128).into_iter().map(Body::UInt8).collect())));
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = vec![true, false, true];
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, serialize_body(&Body::Array(vec![Body::Boolean(true), Body::Boolean(false), Body::Boolean(true)])));
        }
    }

    #[test]
    fn serialize_tuple() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = (true, 123u8, "test");
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())])));
    }

    #[test]
    fn serialize_tuple_struct() {
        #[derive(Serialize)]
        struct Test(bool, u8, String);

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test(true, 123u8, "test".to_string());
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())])));
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct Test {
            c: String,
            a: bool,
            b: u8,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            c: "test".to_string(),
            a: true,
            b: 123,
        };
        body.serialize(&mut serializer).unwrap();

        assert_eq!(buf, serialize_body(&Body::Map({
            let mut map = BTreeMap::new();
            map.insert("a".to_string(), Body::Boolean(true));
            map.insert("b".to_string(), Body::UInt8(123));
            map.insert("c".to_string(), Body::String("test".to_string()));
            map
        })));
    }

    #[test]
    fn serialize_map() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = {
            let mut map = BTreeMap::new();
            map.insert("a".to_string(), 0u8);
            map.insert("b".to_string(), 123u8);
            map.insert("c".to_string(), 255u8);
            map
        };
        body.serialize(&mut serializer).unwrap();

        assert_eq!(buf, serialize_body(&Body::DynamicMap({
            let mut map = BTreeMap::new();
            map.insert("a".to_string(), Body::UInt8(0));
            map.insert("b".to_string(), Body::UInt8(123));
            map.insert("c".to_string(), Body::UInt8(255));
            map
        })));
    }

    #[test]
    fn serialize_bytes() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Bytes::new(&[0u8, 1, 2, 3, 255]);
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Binary(vec![0, 1, 2, 3, 255])));
    }
}
