use std::{fmt::{self, Display}, io::Read, mem::MaybeUninit};
use serde::de;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Read,
    Syntax,
    UnknownSeqSize,
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read => formatter.write_str("read error"),
            Error::Syntax => formatter.write_str("syntax error"),
            Error::UnknownSeqSize => formatter.write_str("unknown seq size"),
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
        let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
        self.reader.read_exact(&mut body_buf).or(Err(Error::Read))?;
        visitor.visit_u8(u8::from_le_bytes(body_buf))
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
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::array::IntoIter;
    use serde::{Deserialize, Serialize};
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
    fn deserialize_u8() {
        IntoIter::new([u8::MIN, u8::MAX]).for_each(|v| {
            let buf = serialize(v);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(v, Deserialize::deserialize(&mut deserializer).unwrap());
        });
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
