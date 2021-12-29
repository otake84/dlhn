use crate::{leb128::Leb128, prefix_varint::PrefixVarint, zigzag::ZigZag};
use serde::{
    ser::{self, Impossible},
    Serialize,
};
use std::{
    fmt::{self, Display},
    io::Write,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Write,
    UnknownMapSize,
    UnsupportedKeyType,
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Write => formatter.write_str("write error"),
            Error::UnknownMapSize => formatter.write_str("unknown map size"),
            Error::UnsupportedKeyType => formatter.write_str("unsupported key type"),
            Error::Message(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

pub struct Serializer<W: Write> {
    output: W,
}

impl<W: Write> Serializer<W> {
    pub fn new(output: W) -> Self {
        Self { output }
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
        if v {
            self.output.write_all(&[1]).or(Err(Error::Write))
        } else {
            self.output.write_all(&[0]).or(Err(Error::Write))
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(&v.to_le_bytes())
            .or(Err(Error::Write))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
        let size = v.encode_zigzag().encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
        let size = v.encode_zigzag().encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
        let size = v.encode_zigzag().encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u128::LEB128_BUF_SIZE];
        let size = v.encode_zigzag().encode_leb128(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(&v.to_le_bytes())
            .or(Err(Error::Write))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u16::LEB128_BUF_SIZE];
        let size = v.encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u32::LEB128_BUF_SIZE];
        let size = v.encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
        let size = v.encode_prefix_varint(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u128::LEB128_BUF_SIZE];
        let size = v.encode_leb128(&mut buf);
        self.output.write_all(&buf[..size]).or(Err(Error::Write))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(&v.to_le_bytes())
            .or(Err(Error::Write))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(&v.to_le_bytes())
            .or(Err(Error::Write))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        v.to_string().serialize(self)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.len().serialize(&mut *self)?;
        self.output.write_all(v.as_bytes()).or(Err(Error::Write))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        v.len().serialize(&mut *self)?;
        self.output.write_all(v).or(Err(Error::Write))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.output.write_all(&[0u8]).or(Err(Error::Write))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
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
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        variant_index.serialize(&mut *self)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            len.serialize(&mut *self)?;
        }
        Ok(self)
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
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        variant_index.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if let Some(len) = len {
            len.serialize(&mut *self)?;
            Ok(self)
        } else {
            Err(Error::UnknownMapSize)
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        variant_index.serialize(&mut *self)?;
        Ok(self)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
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
        T: serde::Serialize,
    {
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
        T: serde::Serialize,
    {
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
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        key.serialize(MapKeySerializer::new(self))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct MapKeySerializer<'a, W: Write> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W: 'a + Write> MapKeySerializer<'a, W> {
    fn new(ser: &'a mut Serializer<W>) -> Self {
        Self { ser }
    }
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

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedKeyType)
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use crate::{leb128::Leb128, ser::Error, zigzag::ZigZag};
    use serde::Serialize;
    use serde_bytes::Bytes;
    use std::collections::BTreeMap;

    #[test]
    fn serialize_bool() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);

            let body = true;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [1]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);

            let body = false;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }
    }

    #[test]
    fn serialize_i8() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i8::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i8::MIN.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0i8;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i8::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i8::MAX.to_le_bytes());
        }
    }

    #[test]
    fn serialize_i16() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i16::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i16::MIN.encode_zigzag().encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0i16;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i16::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i16::MAX.encode_zigzag().encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_i32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i32::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i32::MIN.encode_zigzag().encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0i32;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i32::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i32::MAX.encode_zigzag().encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_i64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i64::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i64::MIN.encode_zigzag().encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0i64;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i64::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i64::MAX.encode_zigzag().encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_i128() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i128::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i128::MIN.encode_zigzag().encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = i128::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, i128::MAX.encode_zigzag().encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_u8() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u8::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u8::MIN.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u8::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u8::MAX.to_le_bytes());
        }
    }

    #[test]
    fn serialize_u16() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u16::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u16::MIN.encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u16::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u16::MAX.encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_u32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u32::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u32::MIN.encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u32::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u32::MAX.encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_u64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u64::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u64::MIN.encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u64::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u64::MAX.encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_u128() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u128::MIN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u128::MIN.encode_leb128_vec());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = u128::MAX;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, u128::MAX.encode_leb128_vec());
        }
    }

    #[test]
    fn serialize_f32() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0f32;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, 0f32.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 1.1f32;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, 1.1f32.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = -1.1f32;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, (-1.1f32).to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = f32::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, f32::INFINITY.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = -f32::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, (-f32::INFINITY).to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = f32::NAN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, f32::NAN.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = -f32::NAN;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, (-f32::NAN).to_le_bytes());
        }
    }

    #[test]
    fn serialize_f64() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 0f64;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, 0f64.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 1.1f64;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, 1.1f64.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = -1.1f64;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, (-1.1f64).to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = f64::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, f64::INFINITY.to_le_bytes());
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = -f64::INFINITY;
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, (-f64::INFINITY).to_le_bytes());
        }
    }

    #[test]
    fn serialize_char() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 'a';
            body.serialize(&mut serializer).unwrap();
            assert_eq!(
                buf,
                [
                    "a".as_bytes().len().encode_leb128_vec().as_slice(),
                    "a".as_bytes()
                ]
                .concat()
            );
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = 'あ';
            body.serialize(&mut serializer).unwrap();
            assert_eq!(
                buf,
                [
                    "あ".as_bytes().len().encode_leb128_vec().as_slice(),
                    "あ".as_bytes()
                ]
                .concat()
            );
        }
    }

    #[test]
    fn serialize_str() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = "";
            body.serialize(&mut serializer).unwrap();
            assert_eq!(
                buf,
                ["".len().encode_leb128_vec().as_slice(), "".as_bytes()].concat()
            );
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = "test";
            body.serialize(&mut serializer).unwrap();
            assert_eq!(
                buf,
                [
                    "test".as_bytes().len().encode_leb128_vec().as_slice(),
                    "test".as_bytes()
                ]
                .concat()
            );
        }
    }

    #[test]
    fn serialize_none() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body: Option<bool> = None;
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, [0]);
    }

    #[test]
    fn serialize_some() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Some(123u8);
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, [1, 123]);
    }

    #[test]
    fn serialize_unit() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = ();
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, []);
    }

    #[test]
    fn serialize_unit_struct() {
        #[derive(Debug, PartialEq, Serialize)]
        struct Test;

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test;
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, []);
    }

    #[test]
    fn serialize_unit_variant() {
        #[derive(Debug, PartialEq, Serialize)]
        enum Test {
            A,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test::A;
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, [0]);
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
            })
            .serialize(&mut serializer)
            .unwrap();

            assert_eq!(
                buf,
                [
                    [4].as_ref(),
                    "test".as_bytes(),
                    [1].as_ref(),
                    [123].as_ref()
                ]
                .concat()
            );
        }

        {
            #[derive(Debug, PartialEq, Serialize)]
            struct Inner(u8);
            #[derive(Debug, PartialEq, Serialize)]
            struct Test {
                c: String,
                a: bool,
                b: Inner,
            }

            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            Test {
                c: "test".to_string(),
                a: true,
                b: Inner(123),
            }
            .serialize(&mut serializer)
            .unwrap();

            assert_eq!(
                buf,
                [
                    [4].as_ref(),
                    "test".as_bytes(),
                    [1].as_ref(),
                    [123].as_ref()
                ]
                .concat()
            );
        }
    }

    #[test]
    fn serialize_newtype_variant() {
        #[allow(dead_code)]
        #[derive(Serialize)]
        enum Test {
            A,
            B(String),
            C,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test::B("test".to_string());
        body.serialize(&mut serializer).unwrap();
        assert_eq!(
            buf,
            [[1u8].as_ref(), [4u8].as_ref(), "test".as_bytes()].concat()
        );
    }

    #[test]
    fn serialize_seq() {
        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body: Vec<u8> = Vec::new();
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [0]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = vec![123u8];
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [1, 123]);
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = [1u8].repeat(128);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(
                buf,
                [128usize.encode_leb128_vec(), [1u8].repeat(128)].concat()
            );
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = vec![true, false, true];
            body.serialize(&mut serializer).unwrap();
            assert_eq!(buf, [3, 1, 0, 1]);
        }
    }

    #[test]
    fn serialize_tuple() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = (true, 123u8, "test");
        body.serialize(&mut serializer).unwrap();
        assert_eq!(
            buf,
            [
                &[1],
                &[123],
                "test".len().encode_leb128_vec().as_slice(),
                "test".as_bytes()
            ]
            .concat()
        );
    }

    #[test]
    fn serialize_tuple_struct() {
        #[derive(Serialize)]
        struct Test(bool, u8, String);

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test(true, 123u8, "test".to_string());
        body.serialize(&mut serializer).unwrap();
        assert_eq!(
            buf,
            [
                &[1],
                &[123],
                "test".len().encode_leb128_vec().as_slice(),
                "test".as_bytes()
            ]
            .concat()
        );
    }

    #[test]
    fn serialize_tuple_variant() {
        #[allow(dead_code)]
        #[derive(Serialize)]
        enum Test {
            A,
            B(bool, u8, String),
            C,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test::B(true, 123, "test".to_string());
        body.serialize(&mut serializer).unwrap();
        assert_eq!(
            buf,
            [
                [1u8].as_ref(),
                [1].as_ref(),
                [123].as_ref(),
                [4u8].as_ref(),
                "test".as_bytes()
            ]
            .concat()
        );
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

        assert_eq!(
            buf,
            [
                [4].as_ref(),
                "test".as_bytes(),
                [1].as_ref(),
                [123].as_ref()
            ]
            .concat()
        );
    }

    #[test]
    fn serialize_struct_variant() {
        #[allow(dead_code)]
        #[derive(Serialize)]
        enum Test {
            A,
            B { a: bool, b: u8, c: String },
            C,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test::B {
            a: true,
            b: 123,
            c: "test".to_string(),
        };
        body.serialize(&mut serializer).unwrap();

        assert_eq!(buf, [[1, 1, 123, 4].as_ref(), "test".as_bytes()].concat());
    }

    #[test]
    fn serialize_map() {
        {
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

            assert_eq!(
                buf,
                [
                    &[3],
                    &[1],
                    "a".as_bytes(),
                    &[0],
                    &[1],
                    "b".as_bytes(),
                    &[123],
                    &[1],
                    "c".as_bytes(),
                    &[255]
                ]
                .concat()
            );
        }

        {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = {
                let mut map = BTreeMap::new();
                map.insert(1, 0u8);
                map.insert(2, 123u8);
                map.insert(3, 255u8);
                map
            };

            assert_eq!(
                body.serialize(&mut serializer),
                Err(Error::UnsupportedKeyType)
            );
        }
    }

    #[test]
    fn serialize_bytes() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Bytes::new(&[0u8, 1, 2, 3, 255]);
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, [5, 0, 1, 2, 3, 255]);
    }
}
