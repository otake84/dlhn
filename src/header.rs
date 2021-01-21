use indexmap::IndexMap;
use integer_encoding::{VarInt, VarIntReader};
use std::io::{BufReader, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Optional(Box<Header>),
    Boolean,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int,
    Int8,
    Float32,
    Float64,
    BigInt,
    BigDecimal,
    String,
    Binary,
    Array(Box<Header>),
    Map(IndexMap<String, Header>),
    DynamicMap(Box<Header>),
    Timestamp,
    Date,
}

impl Header {
    const OPTIONAL_CODE: u8 = 0;
    const BOOLEAN_CODE: u8 = 1;
    const UINT8_CODE: u8 = 2;
    const UINT16_CODE: u8 = 3;
    const UINT32_CODE: u8 = 4;
    const UINT64_CODE: u8 = 5;
    const INT_CODE: u8 = 6;
    const INT8_CODE: u8 = 7;
    const FLOAT32_CODE: u8 = 8;
    const FLOAT64_CODE: u8 = 9;
    const BIG_INT_CODE: u8 = 10;
    const BIG_DECIMAL_CODE: u8 = 11;
    const STRING_CODE: u8 = 12;
    const BINARY_CODE: u8 = 13;
    const ARRAY_CODE: u8 = 14;
    const MAP_CODE: u8 = 15;
    const DYNAMIC_MAP_CODE: u8 = 16;
    const TIMESTAMP_CODE: u8 = 17;
    const DATE_CODE: u8 = 18;

    pub const fn body_size(&self) -> BodySize {
        match self {
            Self::Optional(_) => BodySize::Variable,
            Self::Boolean => BodySize::Fix(1),
            Self::UInt8 => BodySize::Fix(1),
            Self::UInt16 => BodySize::Variable,
            Self::UInt32 => BodySize::Variable,
            Self::UInt64 => BodySize::Variable,
            Self::Int => BodySize::Variable,
            Self::Int8 => BodySize::Fix(1),
            Self::Float32 => BodySize::Fix(4),
            Self::Float64 => BodySize::Fix(8),
            Self::BigInt => BodySize::Variable,
            Self::BigDecimal => BodySize::Variable,
            Self::String => BodySize::Variable,
            Self::Binary => BodySize::Variable,
            Self::Array(_) => BodySize::Variable,
            Self::Map(_) => BodySize::Variable,
            Self::DynamicMap(_) => BodySize::Variable,
            Self::Timestamp => BodySize::Variable,
            Self::Date => BodySize::Variable,
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Optional(inner) => vec![vec![Self::OPTIONAL_CODE], inner.serialize()].concat(),
            Self::Boolean => {
                vec![Self::Boolean.code()]
            }
            Self::UInt8 => {
                vec![Self::UInt8.code()]
            }
            Self::UInt16 => {
                vec![Self::UInt16.code()]
            }
            Self::UInt32 => {
                vec![Self::UInt32.code()]
            }
            Self::UInt64 => {
                vec![Self::UInt64.code()]
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
            Self::BigInt => {
                vec![Self::BigInt.code()]
            }
            Self::BigDecimal => {
                vec![Self::BigDecimal.code()]
            }
            Self::String => {
                vec![Self::String.code()]
            }
            Self::Binary => {
                vec![Self::Binary.code()]
            }
            Self::Array(inner) => vec![vec![Self::ARRAY_CODE], inner.serialize()].concat(),
            Self::Map(inner) => vec![
                vec![Self::MAP_CODE],
                inner.len().encode_var_vec(),
                inner
                    .iter()
                    .flat_map(|v| [Self::serialize_map_key(v.0), v.1.serialize()].concat())
                    .collect(),
            ]
            .concat(),
            Self::DynamicMap(inner) => {
                vec![vec![Self::DYNAMIC_MAP_CODE], inner.serialize()].concat()
            }
            Self::Timestamp => {
                vec![Self::Timestamp.code()]
            }
            Self::Date => {
                vec![Self::Date.code()]
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Header, ()> {
        let mut buf = [0u8; 1];
        buf_reader.read_exact(&mut buf).or(Err(()))?;

        match *buf.first().ok_or(())? {
            Self::OPTIONAL_CODE => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Self::Optional(Box::new(inner)))
            }
            Self::BOOLEAN_CODE => Ok(Self::Boolean),
            Self::UINT8_CODE => Ok(Self::UInt8),
            Self::UINT16_CODE => Ok(Self::UInt16),
            Self::UINT32_CODE => Ok(Self::UInt32),
            Self::UINT64_CODE => Ok(Self::UInt64),
            Self::INT_CODE => Ok(Self::Int),
            Self::INT8_CODE => Ok(Self::Int8),
            Self::FLOAT32_CODE => Ok(Self::Float32),
            Self::FLOAT64_CODE => Ok(Self::Float64),
            Self::BIG_INT_CODE => Ok(Self::BigInt),
            Self::BIG_DECIMAL_CODE => Ok(Self::BigDecimal),
            Self::STRING_CODE => Ok(Self::String),
            Self::BINARY_CODE => Ok(Self::Binary),
            Self::ARRAY_CODE => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Self::Array(Box::new(inner)))
            }
            Self::MAP_CODE => {
                let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                let mut index_map: IndexMap<String, Header> = IndexMap::with_capacity(size);
                for _ in 0..size {
                    index_map.insert(
                        Self::deserialize_map_key(buf_reader)?,
                        Self::deserialize(buf_reader)?,
                    );
                }
                Ok(Self::Map(index_map))
            }
            Self::DYNAMIC_MAP_CODE => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Self::DynamicMap(Box::new(inner)))
            }
            Self::TIMESTAMP_CODE => Ok(Self::Timestamp),
            Self::DATE_CODE => Ok(Self::Date),
            _ => Err(()),
        }
    }

    pub(crate) const fn code(&self) -> u8 {
        match self {
            Self::Optional(_) => Self::OPTIONAL_CODE,
            Self::Boolean => Self::BOOLEAN_CODE,
            Self::UInt8 => Self::UINT8_CODE,
            Self::UInt16 => Self::UINT16_CODE,
            Self::UInt32 => Self::UINT32_CODE,
            Self::UInt64 => Self::UINT64_CODE,
            Self::Int => Self::INT_CODE,
            Self::Int8 => Self::INT8_CODE,
            Self::Float32 => Self::FLOAT32_CODE,
            Self::Float64 => Self::FLOAT64_CODE,
            Self::BigInt => Self::BIG_INT_CODE,
            Self::BigDecimal => Self::BIG_DECIMAL_CODE,
            Self::String => Self::STRING_CODE,
            Self::Binary => Self::BINARY_CODE,
            Self::Array(_) => Self::ARRAY_CODE,
            Self::Map(_) => Self::MAP_CODE,
            Self::DynamicMap(_) => Self::DYNAMIC_MAP_CODE,
            Self::Timestamp => Self::TIMESTAMP_CODE,
            Self::Date => Self::DATE_CODE,
        }
    }

    fn serialize_map_key(v: &str) -> Vec<u8> {
        [v.len().encode_var_vec().as_ref(), v.as_bytes()].concat()
    }

    fn deserialize_map_key<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, ()> {
        let mut body_buf = vec![0u8; buf_reader.read_varint::<usize>().or(Err(()))?];
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
    use super::Header;
    use indexmap::*;
    use std::io::BufReader;

    #[test]
    fn deserialize() {
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                [Header::OPTIONAL_CODE, Header::Boolean.code()].as_ref()
            )),
            Ok(Header::Optional(Box::new(Header::Boolean)))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Boolean.code()].as_ref())),
            Ok(Header::Boolean)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::UInt8.code()].as_ref())),
            Ok(Header::UInt8)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::UInt16.code()].as_ref())),
            Ok(Header::UInt16)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::UInt32.code()].as_ref())),
            Ok(Header::UInt32)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::UInt64.code()].as_ref())),
            Ok(Header::UInt64)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Int.code()].as_ref())),
            Ok(Header::Int)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Int8.code()].as_ref())),
            Ok(Header::Int8)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Float32.code()].as_ref())),
            Ok(Header::Float32)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Float64.code()].as_ref())),
            Ok(Header::Float64)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::BigInt.code()].as_ref())),
            Ok(Header::BigInt)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::BigDecimal.code()].as_ref())),
            Ok(Header::BigDecimal)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::String.code()].as_ref())),
            Ok(Header::String)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Binary.code()].as_ref())),
            Ok(Header::Binary)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                [Header::ARRAY_CODE, Header::Boolean.code()].as_ref()
            )),
            Ok(Header::Array(Box::new(Header::Boolean)))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                [
                    vec![Header::MAP_CODE],
                    vec![Header::Boolean.code()],
                    Header::serialize_map_key("test"),
                    Header::Boolean.serialize()
                ]
                .concat()
                .as_slice()
            )),
            Ok(Header::Map(
                indexmap! {String::from("test") => Header::Boolean}
            ))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                Header::DynamicMap(Box::new(Header::Optional(Box::new(Header::String))))
                    .serialize()
                    .as_slice()
            )),
            Ok(Header::DynamicMap(Box::new(Header::Optional(Box::new(
                Header::String
            )))))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Timestamp.code()].as_ref())),
            Ok(Header::Timestamp)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new([Header::Date.code()].as_ref())),
            Ok(Header::Date)
        );
    }
}
