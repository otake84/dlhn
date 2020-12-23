use std::io::{BufReader, Read};

use indexmap::IndexMap;
use integer_encoding::{VarInt, VarIntReader};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Optional(Box<Header>),
    Boolean,
    UInt,
    UInt8,
    Int,
    Int8,
    Float32,
    Float64,
    String,
    Binary,
    Array(Box<Header>),
    Map(IndexMap<String, Header>),
}

impl Header {
    const OPTIONAL_CODE: u8 = 0;
    const BOOLEAN_CODE: u8 = 1;
    const UINT_CODE: u8 = 2;
    const UINT8_CODE: u8 = 3;
    const INT_CODE: u8 = 4;
    const INT8_CODE: u8 = 5;
    const FLOAT32_CODE: u8 = 6;
    const FLOAT64_CODE: u8 = 7;
    const STRING_CODE: u8 = 8;
    const BINARY_CODE: u8 = 9;
    const ARRAY_CODE: u8 = 10;
    const MAP_CODE: u8 = 11;

    pub const fn body_size(&self) -> BodySize {
        match self {
            Header::Optional(_) => BodySize::Variable,
            Header::Boolean => BodySize::Fix(1),
            Header::UInt => BodySize::Variable,
            Header::UInt8 => BodySize::Fix(1),
            Header::Int => BodySize::Variable,
            Header::Int8 => BodySize::Fix(1),
            Header::Float32 => BodySize::Fix(4),
            Header::Float64 => BodySize::Fix(8),
            Header::String => BodySize::Variable,
            Header::Binary => BodySize::Variable,
            Header::Array(_) => BodySize::Variable,
            Header::Map(_) => BodySize::Variable,
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Optional(inner) => {
                vec![vec![Self::OPTIONAL_CODE], inner.serialize()].concat()
            }
            Self::Boolean => {
                vec![Self::Boolean.code()]
            }
            Self::UInt => {
                vec![Self::UInt.code()]
            }
            Self::UInt8 => {
                vec![Self::UInt8.code()]
            }
            Self::Int => {
                vec![Self::Int.code()]
            }
            Self::Int8 => {
                vec![Self::Int8.code()]
            }
            Self::Float32 => {
                vec![Self::Float32.code()]
            }
            Self::Float64 => {
                vec![Self::Float64.code()]
            }
            Self::String => {
                vec![Self::String.code()]
            }
            Self::Binary => {
                vec![Self::Binary.code()]
            }
            Self::Array(inner) => {
                vec![vec![Self::ARRAY_CODE], inner.serialize()].concat()
            }
            Self::Map(inner) => {
                vec![vec![Self::MAP_CODE], inner.len().encode_var_vec(), inner.iter().flat_map(|v| [Self::serialize_map_key(v.0), v.1.serialize()].concat()).collect()].concat()
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Header, ()> {
        let mut buf = [0u8; 1];
        buf_reader.read_exact(&mut buf).or(Err(()))?;

        match buf.first() {
            Some(&Self::OPTIONAL_CODE) => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Header::Optional(Box::new(inner)))
            }
            Some(&Self::BOOLEAN_CODE) => Ok(Header::Boolean),
            Some(&Self::UINT_CODE) => Ok(Header::UInt),
            Some(&Self::UINT8_CODE) => Ok(Header::UInt8),
            Some(&Self::INT_CODE) => Ok(Header::Int),
            Some(&Self::INT8_CODE) => Ok(Header::Int8),
            Some(&Self::FLOAT32_CODE) => Ok(Header::Float32),
            Some(&Self::FLOAT64_CODE) => Ok(Header::Float64),
            Some(&Self::STRING_CODE) => Ok(Header::String),
            Some(&Self::BINARY_CODE) => Ok(Header::Binary),
            Some(&Self::ARRAY_CODE) => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Header::Array(Box::new(inner)))
            }
            Some(&Self::MAP_CODE) => {
                let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                let mut index_map: IndexMap<String, Header> = IndexMap::with_capacity(size);
                for _ in 0..size {
                    index_map.insert(Self::deserialize_map_key(buf_reader)?, Self::deserialize(buf_reader)?);
                }
                Ok(Header::Map(index_map))
            }
            _ => Err(())
        }
    }

    pub(crate) const fn code(&self) -> u8 {
        match self {
            Self::Optional(_) => Self::OPTIONAL_CODE,
            Self::Boolean => Self::BOOLEAN_CODE,
            Self::UInt => Self::UINT_CODE,
            Self::UInt8 => Self::UINT8_CODE,
            Self::Int => Self::INT_CODE,
            Self::Int8 => Self::INT8_CODE,
            Self::Float32 => Self::FLOAT32_CODE,
            Self::Float64 => Self::FLOAT64_CODE,
            Self::String => Self::STRING_CODE,
            Self::Binary => Self::BINARY_CODE,
            Self::Array(_) => Self::ARRAY_CODE,
            Self::Map(_) => Self::MAP_CODE,
        }
    }

    fn serialize_map_key(v: &str) -> Vec<u8> {
        [v.len().encode_var_vec().as_ref(), v.as_bytes()].concat()
    }

    fn deserialize_map_key<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, ()> {
        let size = buf_reader.read_varint::<usize>().or(Err(()))?;
        let mut body_buf = Vec::with_capacity(size);
        unsafe { body_buf.set_len(size); }
        buf_reader.read_exact(&mut body_buf).or(Err(()))?;
        String::from_utf8(body_buf).or(Err(()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BodySize {
    Fix(usize),
    Variable,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::Header;
    use indexmap::*;

    #[test]
    fn deserialize() {
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::OPTIONAL_CODE, Header::Boolean.code()].as_ref())), Ok(Header::Optional(Box::new(Header::Boolean))));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Boolean.code()].as_ref())), Ok(Header::Boolean));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::UInt.code()].as_ref())), Ok(Header::UInt));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::UInt8.code()].as_ref())), Ok(Header::UInt8));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Int.code()].as_ref())), Ok(Header::Int));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Int8.code()].as_ref())), Ok(Header::Int8));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Float32.code()].as_ref())), Ok(Header::Float32));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Float64.code()].as_ref())), Ok(Header::Float64));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::String.code()].as_ref())), Ok(Header::String));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::Binary.code()].as_ref())), Ok(Header::Binary));
        assert_eq!(Header::deserialize(&mut BufReader::new([Header::ARRAY_CODE, Header::Boolean.code()].as_ref())), Ok(Header::Array(Box::new(Header::Boolean))));
        assert_eq!(Header::deserialize(&mut BufReader::new([vec![Header::MAP_CODE], vec![Header::Boolean.code()], Header::serialize_map_key("test"), Header::Boolean.serialize()].concat().as_slice())), Ok(Header::Map(indexmap!{String::from("test") => Header::Boolean})));
    }
}
