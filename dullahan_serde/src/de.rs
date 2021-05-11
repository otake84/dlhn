use std::{collections::VecDeque, fmt::{self, Display}, io::Read, mem::MaybeUninit};
use integer_encoding::VarIntReader;
use serde::de;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Read,
    Syntax,
    UnknownSeqSize,
    CharSize,
    CharCode,
    Message(String),
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read => formatter.write_str("Read error"),
            Error::Syntax => formatter.write_str("Syntax error"),
            Error::UnknownSeqSize => formatter.write_str("Unknown seq size"),
            Error::CharSize => formatter.write_str("The size of the char is more than 32bit"),
            Error::CharCode => formatter.write_str("Incorrect character encoding"),
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
        Deserializer {
            reader,
        }
    }

    #[inline]
    fn new_dynamic_buf(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.reader.read_varint::<usize>().or(Err(Error::Read))?;
        let mut buf = Vec::<u8>::with_capacity(len);
        unsafe {
            buf.set_len(len);
        }
        Ok(buf)
    }
}

impl<'de , 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_bool(match body_buf[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Syntax),
        }?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
            let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
            self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
            visitor.visit_i8(i8::from_le_bytes(body_buf))
        }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_i16(self.reader.read_varint::<i16>().or(Err(Error::Read))?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_i32(self.reader.read_varint::<i32>().or(Err(Error::Read))?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_i64(self.reader.read_varint::<i64>().or(Err(Error::Read))?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_u8(u8::from_le_bytes(body_buf))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_u16(self.reader.read_varint::<u16>().or(Err(Error::Read))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_u32(self.reader.read_varint::<u32>().or(Err(Error::Read))?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_u64(self.reader.read_varint::<u64>().or(Err(Error::Read))?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut body_buf: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_f32(f32::from_le_bytes(body_buf))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut body_buf: [u8; 8] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_f64(f64::from_le_bytes(body_buf))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
            let mut body_buf = self.new_dynamic_buf()?;
            self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
            let s = String::from_utf8(body_buf).or(Err(Error::CharCode))?;
            visitor.visit_char(s.chars().into_iter().next().ok_or(Error::CharSize)?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
            let mut body_buf = self.new_dynamic_buf()?;
            self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
            visitor.visit_string(String::from_utf8(body_buf).or(Err(Error::Read))?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut body_buf = self.new_dynamic_buf()?;
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_byte_buf(body_buf)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut buf).or(Err(Error::Read))?;
        match buf[0] {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::Read),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let count = self.reader.read_varint::<usize>().or(Err(Error::Read))?;
        visitor.visit_seq(SeqDeserializer::new(&mut self, count))
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let count = self.reader.read_varint::<usize>().or(Err(Error::Read))?;
        visitor.visit_map(MapDeserializer::new(&mut self, count))
    }

    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        let mut keys = fields.iter().map(|v| v.to_string()).collect::<VecDeque<String>>();
        keys.make_contiguous().sort();
        visitor.visit_map(StructDeserializer::new(&mut self, keys))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
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
        T: de::DeserializeSeed<'de> {
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
        K: de::DeserializeSeed<'de> {
            if self.count > 0 {
                self.count -= 1;
                seed.deserialize(&mut *self.deserializer).map(Some)
            } else {
                Ok(None)
            }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de> {
            seed.deserialize(&mut *self.deserializer)
    }
}

struct StructDeserializer<'a, 'de: 'a, R: Read> {
    deserializer: &'a mut Deserializer<'de, R>,
    keys: VecDeque<String>,
}

impl<'a, 'de: 'a, R: Read> StructDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, keys: VecDeque<String>) -> Self {
        Self {
            deserializer,
            keys,
        }
    }
}

impl<'a, 'de: 'a, R: Read> de::MapAccess<'de> for StructDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de> {
            if let Some(key) = self.keys.pop_front() {
                seed.deserialize(StructKey::new(key)).map(Some)
            } else {
                Ok(None)
            }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de> {
            seed.deserialize(&mut *self.deserializer)
    }
}

struct StructKey {
    key: String,
}

impl StructKey {
    pub fn new(key: String) -> Self {
        Self {
            key,
        }
    }
}

impl<'de> de::Deserializer<'de> for StructKey {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        visitor.visit_string(self.key)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{array::IntoIter, collections::{BTreeMap, HashMap}};
    use serde::{Deserialize, Serialize};
    use serde_bytes::ByteBuf;
    use crate::{de::Deserializer, ser::Serializer};

    #[test]
    fn deserialize_bool() {
        IntoIter::new([true, false]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i8() {
        IntoIter::new([i8::MIN, 0, i8::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i16() {
        IntoIter::new([i16::MIN, 0, i16::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i16::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i32() {
        IntoIter::new([i32::MIN, 0, i32::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i32::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_i64() {
        IntoIter::new([i64::MIN, 0, i64::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, i64::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u8() {
        IntoIter::new([u8::MIN, u8::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u16() {
        IntoIter::new([u16::MIN, u16::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u16::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u32() {
        IntoIter::new([u32::MIN, u32::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u32::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_u64() {
        IntoIter::new([u64::MIN, u64::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, u64::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_f32() {
        IntoIter::new([-f32::INFINITY, f32::MIN, 0f32, f32::MAX, f32::INFINITY]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    #[test]
    fn deserialize_f64() {
        IntoIter::new([-f64::INFINITY, f64::MIN, 0f64, f64::MAX, f64::INFINITY]).for_each(|v| {
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
    fn deserialize_byte_buf() {
        let buf = serialize(ByteBuf::from(vec![0u8, 1, 2, 3, 255]));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = ByteBuf::deserialize(&mut deserializer).unwrap();
        assert_eq!([0u8, 1, 2, 3, 255], result.as_slice());
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

            assert_eq!(Test(Inner {
                c: "test".to_string(),
                a: true,
                b: 123,
            }), result);
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

            assert_eq!(Test {
                c: "test".to_string(),
                a: Inner(123),
                b: true,
            }, result);
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

            assert_eq!({
                let mut map = BTreeMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            }, result);
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

            assert_eq!({
                let mut map = HashMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            }, result);
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

            assert_eq!({
                let mut map = BTreeMap::new();
                map.insert("a".to_string(), true);
                map.insert("b".to_string(), false);
                map.insert("c".to_string(), true);
                map.insert("1".to_string(), false);
                map
            }, result);
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

        assert_eq!(Test {
            c: "test".to_string(),
            a: true,
            b: 123,
        }, result);
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
