use crate::{PrefixVarint, ZigZag};
use serde::{de, Deserialize};
use std::{
    cmp::min,
    fmt::{self, Display},
    io::Read,
    slice::Iter,
    vec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Read,
    CharSize,
    UnsupportedKeyType,
    Message(String),
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Expected for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read => formatter.write_str("Read error"),
            Error::CharSize => formatter.write_str("The size of the char is more than 32bit"),
            Error::UnsupportedKeyType => formatter.write_str("Unsupported Key Type"),
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read => formatter.write_str("Read error"),
            Error::CharSize => formatter.write_str("The size of the char is more than 32bit"),
            Error::UnsupportedKeyType => formatter.write_str("Unsupported Key Type"),
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

pub struct Deserializer<'de, R: Read> {
    reader: &'de mut R,
}

impl<'de, R: Read> Deserializer<'de, R> {
    pub fn new(reader: &'de mut R) -> Self {
        Deserializer { reader }
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        match buf[0] {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(Error::Read),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        visitor.visit_i8(i8::from_le_bytes(buf))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(
            u16::decode_prefix_varint(self.reader)
                .map(i16::decode_zigzag)
                .or(Err(Error::Read))?,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(
            u32::decode_prefix_varint(self.reader)
                .map(i32::decode_zigzag)
                .or(Err(Error::Read))?,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(
            u64::decode_prefix_varint(self.reader)
                .map(i64::decode_zigzag)
                .or(Err(Error::Read))?,
        )
    }

    // fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    // where
    //     V: de::Visitor<'de>,
    // {
    //     visitor.visit_i128(
    //         u128::decode_leb128(self.reader)
    //             .map(i128::decode_zigzag)
    //             .or(Err(Error::Read))?,
    //     )
    // }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        visitor.visit_u8(u8::from_le_bytes(buf))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(u16::decode_prefix_varint(self.reader).or(Err(Error::Read))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(u32::decode_prefix_varint(self.reader).or(Err(Error::Read))?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(u64::decode_prefix_varint(self.reader).or(Err(Error::Read))?)
    }

    // fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    // where
    //     V: de::Visitor<'de>,
    // {
    //     visitor.visit_u128(u128::decode_leb128(self.reader).or(Err(Error::Read))?)
    // }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(
            String::deserialize(self)?
                .chars()
                .into_iter()
                .next()
                .ok_or(Error::CharSize)?,
        )
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = u64::decode_prefix_varint(self.reader).or(Err(Error::Read))?;
        const MAX_SIZE: u64 = 128;
        if len < MAX_SIZE {
            let mut body_buf = [0; MAX_SIZE as usize];
            self.reader
                .read_exact(&mut body_buf[..(len as usize)])
                .or(Err(Error::Read))?;
            visitor.visit_string(
                String::from_utf8(body_buf[..(len as usize)].to_vec()).or(Err(Error::Read))?,
            )
        } else {
            let mut s = String::new();
            if self
                .reader
                .take(len as u64)
                .read_to_string(&mut s)
                .or(Err(Error::Read))?
                != len as usize
            {
                return Err(Error::Read);
            };
            visitor.visit_string(s)
        }
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = u64::decode_prefix_varint(self.reader).or(Err(Error::Read))?;
        const MAX_SIZE: u64 = 4096;
        if len > MAX_SIZE {
            let mut result = Vec::new();
            let mut buf = vec![0; MAX_SIZE as usize];
            let mut pos = 0;
            while result.len() < len as usize {
                self.reader
                    .read_exact(&mut buf[..(min(MAX_SIZE, len - pos)) as usize])
                    .or(Err(Error::Read))?;
                result.extend_from_slice(&buf[..(min(MAX_SIZE, len - pos)) as usize]);
                pos += min(MAX_SIZE, len - pos);
            }
            visitor.visit_byte_buf(result)
        } else {
            let mut buf = vec![0; len as usize];
            self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
            visitor.visit_byte_buf(buf)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if bool::deserialize(&mut *self)? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let count = u64::decode_prefix_varint(self.reader).or(Err(Error::Read))?;
        visitor.visit_seq(SeqDeserializer::new(&mut self, count as usize))
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let count = u64::decode_prefix_varint(self.reader).or(Err(Error::Read))?;
        visitor.visit_map(MapDeserializer::new(&mut self, count as usize))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(StructDeserializer::new(self, fields))
    }

    fn deserialize_enum<V>(
        mut self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(VariantDeserializer::new(&mut self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u16(visitor)
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SeqDeserializer<'a, 'de: 'a, R: Read> {
    deserializer: &'a mut Deserializer<'de, R>,
    count: usize,
}

impl<'a, 'de: 'a, R: Read> SeqDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, count: usize) -> Self {
        Self {
            deserializer,
            count,
        }
    }
}

impl<'a, 'de: 'a, R: Read> de::SeqAccess<'de> for SeqDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.count > 0 {
            self.count -= 1;
            seed.deserialize(&mut *self.deserializer).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct MapDeserializer<'a, 'de: 'a, R: Read> {
    deserializer: &'a mut Deserializer<'de, R>,
    count: usize,
}

impl<'a, 'de: 'a, R: Read> MapDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, count: usize) -> Self {
        Self {
            deserializer,
            count,
        }
    }
}

impl<'a, 'de: 'a, R: Read> de::MapAccess<'de> for MapDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.count > 0 {
            self.count -= 1;
            seed.deserialize(&mut *self.deserializer).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

struct StructDeserializer<'a, 'de: 'a, R: Read> {
    deserializer: &'a mut Deserializer<'de, R>,
    keys: Iter<'a, &'static str>,
}

impl<'a, 'de: 'a, R: Read> StructDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, keys: &'static [&'static str]) -> Self {
        Self {
            deserializer,
            keys: keys.iter(),
        }
    }
}

impl<'a, 'de: 'a, R: Read> de::MapAccess<'de> for StructDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.keys.next() {
            Some(&key) => seed
                .deserialize(serde::de::value::BorrowedStrDeserializer::new(key))
                .map(Some),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

struct VariantDeserializer<'de, 'a, R: Read> {
    de: &'a mut Deserializer<'de, R>,
}

impl<'de, 'a, R: Read> VariantDeserializer<'de, 'a, R> {
    fn new(de: &'a mut Deserializer<'de, R>) -> Self {
        VariantDeserializer { de }
    }
}

impl<'de, 'a, R: Read> de::EnumAccess<'de> for VariantDeserializer<'de, 'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'de, 'a, R: Read> de::VariantAccess<'de> for VariantDeserializer<'de, 'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{de::Deserializer, ser::Serializer};
    use serde::{Deserialize, Serialize};
    use serde_bytes::ByteBuf;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn deserialize_bool() {
        IntoIterator::into_iter([true, false]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i8() {
        IntoIterator::into_iter([i8::MIN, 0, i8::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i16() {
        IntoIterator::into_iter([i16::MIN, 0, i16::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i16::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i32() {
        IntoIterator::into_iter([i32::MIN, 0, i32::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i32::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i64() {
        IntoIterator::into_iter([i64::MIN, 0, i64::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i64::deserialize(&mut deserializer).unwrap());
        });
    }

    // #[test]
    // fn deserialize_i128() {
    //     IntoIterator::into_iter([i128::MIN, 0, i128::MAX]).for_each(|v| {
    //         let buf = serialize(v);
    //         let mut reader = buf.as_slice();
    //         let mut deserializer = Deserializer::new(&mut reader);
    //         assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
    //     });
    // }

    #[test]
    fn deserialize_u8() {
        IntoIterator::into_iter([u8::MIN, u8::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u16() {
        IntoIterator::into_iter([u16::MIN, u16::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u16::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u32() {
        IntoIterator::into_iter([u32::MIN, u32::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u32::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u64() {
        IntoIterator::into_iter([u64::MIN, u64::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u64::deserialize(&mut deserializer).unwrap());
        });
    }

    // #[test]
    // fn deserialize_u128() {
    //     IntoIterator::into_iter([u128::MIN, u128::MAX]).for_each(|v| {
    //         let buf = serialize(v);
    //         let mut reader = buf.as_slice();
    //         let mut deserializer = Deserializer::new(&mut reader);
    //         assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
    //     })
    // }

    #[test]
    fn deserialize_f32() {
        IntoIterator::into_iter([-f32::INFINITY, f32::MIN, 0f32, f32::MAX, f32::INFINITY])
            .for_each(|v| {
                let buf = serialize(v);
                let mut reader = buf.as_slice();
                let mut deserializer = Deserializer::new(&mut reader);
                assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
            });
    }

    #[test]
    fn deserialize_f64() {
        IntoIterator::into_iter([-f64::INFINITY, f64::MIN, 0f64, f64::MAX, f64::INFINITY])
            .for_each(|v| {
                let buf = serialize(v);
                let mut reader = buf.as_slice();
                let mut deserializer = Deserializer::new(&mut reader);
                assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
            });
    }

    #[test]
    fn deserialize_char() {
        {
            let buf = serialize('a');
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = char::deserialize(&mut deserializer).unwrap();
            assert_eq!('a', result)
        }

        {
            let buf = serialize('あ');
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = char::deserialize(&mut deserializer).unwrap();
            assert_eq!('あ', result)
        }
    }

    #[test]
    fn deserialize_string() {
        let buf = serialize("test".to_string());
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = String::deserialize(&mut deserializer).unwrap();
        assert_eq!("test".to_string(), result);
    }

    #[test]
    fn deserialize_string129_issue() {
        // Thanks @caibear and @udoprog
        // https://github.com/otake84/dlhn/issues/14
        // https://github.com/otake84/dlhn/issues/15
        let original = " ".repeat(129);
        let mut serialized = vec![];
        original
            .serialize(&mut Serializer::new(&mut serialized))
            .unwrap();

        let deserialized =
            String::deserialize(&mut Deserializer::new(&mut serialized.as_slice())).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn deserialize_byte_buf() {
        let buf = serialize(ByteBuf::from(vec![0u8, 1, 2, 3, 255].repeat(1000)));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = ByteBuf::deserialize(&mut deserializer).unwrap();
        assert_eq!([0u8, 1, 2, 3, 255].repeat(1000), result.as_slice());
    }

    #[test]
    fn deserialize_byte_buf_4097() {
        let buf = serialize(ByteBuf::from(vec![0u8].repeat(4097)));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = ByteBuf::deserialize(&mut deserializer).unwrap();
        assert_eq!([0u8].repeat(4097), result.as_slice());
    }

    #[test]
    fn deserialize_byte_buf_100000() {
        let buf = serialize(ByteBuf::from(vec![0u8].repeat(100000)));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = ByteBuf::deserialize(&mut deserializer).unwrap();
        assert_eq!([0u8].repeat(100000), result.as_slice());
    }

    #[test]
    fn deserialize_option() {
        {
            let buf = serialize(Option::<u8>::None);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = <Option<u8>>::deserialize(&mut deserializer).unwrap();
            assert_eq!(None, result);
        }

        {
            let buf = serialize(Some(255u8));
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = <Option<u8>>::deserialize(&mut deserializer).unwrap();
            assert_eq!(Some(255), result);
        }
    }

    #[test]
    fn deserialize_unit() {
        let buf = serialize(());
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = <()>::deserialize(&mut deserializer).unwrap();
        assert_eq!((), result);
    }

    #[test]
    fn deserialize_unit_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test;

        let buf = serialize(Test);
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = Test::deserialize(&mut deserializer).unwrap();
        assert_eq!(Test, result);
    }

    #[test]
    fn deserialize_newtype_struct() {
        {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            struct Inner {
                c: String,
                a: bool,
                b: u8,
            }
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            struct Test(Inner);
            let buf = serialize(Test(Inner {
                c: "test".to_string(),
                a: true,
                b: 123,
            }));
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                Test(Inner {
                    c: "test".to_string(),
                    a: true,
                    b: 123,
                }),
                result
            );
        }

        {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            struct Inner(u8);
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            struct Test {
                c: String,
                a: Inner,
                b: bool,
            }
            let buf = serialize(Test {
                c: "test".to_string(),
                a: Inner(123),
                b: true,
            });
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                Test {
                    c: "test".to_string(),
                    a: Inner(123),
                    b: true,
                },
                result
            );
        }
    }

    #[test]
    fn deserialize_seq() {
        {
            let buf = serialize(Vec::<bool>::new());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Vec::<bool>::deserialize(&mut deserializer).unwrap();
            assert_eq!(Vec::<bool>::new(), result);
        }

        {
            let buf = serialize(vec![true, false, true]);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Vec::<bool>::deserialize(&mut deserializer).unwrap();
            assert_eq!(vec![true, false, true], result);
        }

        {
            let buf = serialize(vec![0u8, 1, 2, 3, 255]);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Vec::<u8>::deserialize(&mut deserializer).unwrap();
            assert_eq!(vec![0, 1, 2, 3, 255], result);
        }

        {
            let buf = serialize(vec!['a', 'b', 'c']);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Vec::<char>::deserialize(&mut deserializer).unwrap();
            assert_eq!(vec!['a', 'b', 'c'], result);
        }
    }

    #[test]
    fn deserialize_tuple() {
        let buf = serialize((true, 123u8, 'a'));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = <(bool, u8, char)>::deserialize(&mut deserializer).unwrap();
        assert_eq!((true, 123, 'a'), result);
    }

    #[test]
    fn deserialize_tuple_struct() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Test(bool, u8, char);

        let buf = serialize(Test(true, 123, 'a'));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = Test::deserialize(&mut deserializer).unwrap();
        assert_eq!(Test(true, 123, 'a'), result);
    }

    #[test]
    fn deserialize_map() {
        {
            let buf = serialize({
                let mut map = BTreeMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            });
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = BTreeMap::<String, bool>::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                {
                    let mut map = BTreeMap::new();
                    map.insert("a".to_string(), true);
                    map.insert("b".to_string(), false);
                    map.insert("c".to_string(), true);
                    map.insert("1".to_string(), false);
                    map
                },
                result
            );
        }

        {
            let buf = serialize({
                let mut map = BTreeMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            });
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = HashMap::<String, bool>::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                {
                    let mut map = HashMap::new();
                    map.insert("a".to_string(), true);
                    map.insert("b".to_string(), false);
                    map.insert("c".to_string(), true);
                    map.insert("1".to_string(), false);
                    map
                },
                result
            );
        }

        {
            let buf = serialize({
                let mut map = HashMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            });
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = BTreeMap::<String, bool>::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                {
                    let mut map = BTreeMap::new();
                    map.insert("a".to_string(), true);
                    map.insert("b".to_string(), false);
                    map.insert("c".to_string(), true);
                    map.insert("1".to_string(), false);
                    map
                },
                result
            );
        }
    }

    #[test]
    fn deserialize_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            c: String,
            a: bool,
            b: u8,
        }

        let buf = serialize(Test {
            c: "test".to_string(),
            a: true,
            b: 123,
        });
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = Test::deserialize(&mut deserializer).unwrap();

        assert_eq!(
            Test {
                c: "test".to_string(),
                a: true,
                b: 123,
            },
            result
        );
    }

    #[test]
    fn deserialize_enum() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        enum Test {
            A,
            B(String),
            C(bool, u8, String),
            D { a: bool, b: u8, c: String },
        }

        {
            let buf = serialize(Test::A);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(Test::A, result);
        }

        {
            let buf = serialize(Test::B("test".to_string()));
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(Test::B("test".to_string()), result);
        }

        {
            let buf = serialize(Test::C(true, 123, "test".to_string()));
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(Test::C(true, 123, "test".to_string()), result);
        }

        {
            let buf = serialize(Test::D {
                a: true,
                b: 123,
                c: "test".to_string(),
            });
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();

            assert_eq!(
                Test::D {
                    a: true,
                    b: 123,
                    c: "test".to_string()
                },
                result
            );
        }
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
