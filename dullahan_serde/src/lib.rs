use std::{fmt::{self, Display}, io::Write};
use dullahan::{body::Body, serializer::serialize_body};
use serde::{de, ser};
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

    type SerializeMap = Self;

    type SerializeStruct = Self;

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
        todo!()
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
        T: serde::Serialize {
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
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
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

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
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

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use dullahan::{body::Body, serializer::serialize_body};
    use crate::Serializer;

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
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = 'a';
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::String(String::from('a'))));
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
    }

    #[test]
    fn serialize_tuple() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = (true, 123u8, "test");
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())])));
    }
}
