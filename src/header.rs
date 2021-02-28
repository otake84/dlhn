use crate::{deserialize_string, serialize_string};
use indexmap::IndexMap;
use integer_encoding::{VarInt, VarIntReader};
use std::io::{BufReader, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Optional(Box<Header>),
    Boolean,
    UInt8,
    VarUInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    BigUInt,
    BigInt,
    BigDecimal,
    String,
    Binary,
    Array(Box<Header>),
    Map(IndexMap<String, Header>),
    DynamicMap(Box<Header>),
    Date,
    DateTime,
}

impl Header {
    const OPTIONAL_CODE: u8 = 0;
    const BOOLEAN_CODE: u8 = 1;
    const UINT8_CODE: u8 = 2;
    const VAR_UINT16_CODE: u8 = 3;
    const UINT32_CODE: u8 = 4;
    const UINT64_CODE: u8 = 5;
    const INT8_CODE: u8 = 6;
    const INT16_CODE: u8 = 7;
    const INT32_CODE: u8 = 8;
    const INT64_CODE: u8 = 9;
    const FLOAT32_CODE: u8 = 10;
    const FLOAT64_CODE: u8 = 11;
    const BIG_UINT_CODE: u8 = 12;
    const BIG_INT_CODE: u8 = 13;
    const BIG_DECIMAL_CODE: u8 = 14;
    const STRING_CODE: u8 = 15;
    const BINARY_CODE: u8 = 16;
    const ARRAY_CODE: u8 = 17;
    const MAP_CODE: u8 = 18;
    const DYNAMIC_MAP_CODE: u8 = 19;
    const DATE_CODE: u8 = 20;
    const DATETIME_CODE: u8 = 21;

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Optional(inner) => {
                let mut buf = vec![Self::OPTIONAL_CODE];
                buf.append(&mut inner.serialize());
                buf
            }
            Self::Boolean => {
                vec![Self::Boolean.code()]
            }
            Self::UInt8 => {
                vec![Self::UInt8.code()]
            }
            Self::VarUInt16 => {
                vec![Self::VarUInt16.code()]
            }
            Self::UInt32 => {
                vec![Self::UInt32.code()]
            }
            Self::UInt64 => {
                vec![Self::UInt64.code()]
            }
            Self::Int8 => {
                vec![Self::Int8.code()]
            }
            Self::Int16 => {
                vec![Self::Int16.code()]
            }
            Self::Int32 => {
                vec![Self::Int32.code()]
            }
            Self::Int64 => {
                vec![Self::Int64.code()]
            }
            Self::Float32 => {
                vec![Self::Float32.code()]
            }
            Self::Float64 => {
                vec![Self::Float64.code()]
            }
            Self::BigUInt => {
                vec![Self::BigUInt.code()]
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
            Self::Array(inner) => {
                let mut buf = vec![Self::ARRAY_CODE];
                buf.append(&mut inner.serialize());
                buf
            }
            Self::Map(inner) => {
                let mut buf = vec![Self::MAP_CODE];
                buf.append(&mut inner.len().encode_var_vec());
                inner.iter().for_each(|(k, v)| {
                    buf.append(&mut serialize_string(k));
                    buf.append(&mut v.serialize());
                });
                buf
            }
            Self::DynamicMap(inner) => {
                let mut buf = vec![Self::DYNAMIC_MAP_CODE];
                buf.append(&mut inner.serialize());
                buf
            }
            Self::Date => {
                vec![Self::Date.code()]
            }
            Self::DateTime => {
                vec![Self::DateTime.code()]
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
            Self::VAR_UINT16_CODE => Ok(Self::VarUInt16),
            Self::UINT32_CODE => Ok(Self::UInt32),
            Self::UINT64_CODE => Ok(Self::UInt64),
            Self::INT8_CODE => Ok(Self::Int8),
            Self::INT16_CODE => Ok(Self::Int16),
            Self::INT32_CODE => Ok(Self::Int32),
            Self::INT64_CODE => Ok(Self::Int64),
            Self::FLOAT32_CODE => Ok(Self::Float32),
            Self::FLOAT64_CODE => Ok(Self::Float64),
            Self::BIG_UINT_CODE => Ok(Self::BigUInt),
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
                        deserialize_string(buf_reader)?,
                        Self::deserialize(buf_reader)?,
                    );
                }
                Ok(Self::Map(index_map))
            }
            Self::DYNAMIC_MAP_CODE => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Self::DynamicMap(Box::new(inner)))
            }
            Self::DATE_CODE => Ok(Self::Date),
            Self::DATETIME_CODE => Ok(Self::DateTime),
            _ => Err(()),
        }
    }

    pub(crate) const fn code(&self) -> u8 {
        match self {
            Self::Optional(_) => Self::OPTIONAL_CODE,
            Self::Boolean => Self::BOOLEAN_CODE,
            Self::UInt8 => Self::UINT8_CODE,
            Self::VarUInt16 => Self::VAR_UINT16_CODE,
            Self::UInt32 => Self::UINT32_CODE,
            Self::UInt64 => Self::UINT64_CODE,
            Self::Int8 => Self::INT8_CODE,
            Self::Int16 => Self::INT16_CODE,
            Self::Int32 => Self::INT32_CODE,
            Self::Int64 => Self::INT64_CODE,
            Self::Float32 => Self::FLOAT32_CODE,
            Self::Float64 => Self::FLOAT64_CODE,
            Self::BigUInt => Self::BIG_UINT_CODE,
            Self::BigInt => Self::BIG_INT_CODE,
            Self::BigDecimal => Self::BIG_DECIMAL_CODE,
            Self::String => Self::STRING_CODE,
            Self::Binary => Self::BINARY_CODE,
            Self::Array(_) => Self::ARRAY_CODE,
            Self::Map(_) => Self::MAP_CODE,
            Self::DynamicMap(_) => Self::DYNAMIC_MAP_CODE,
            Self::Date => Self::DATE_CODE,
            Self::DateTime => Self::DATETIME_CODE,
        }
    }
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
                Header::Optional(Box::new(Header::Boolean))
                    .serialize()
                    .as_slice()
            )),
            Ok(Header::Optional(Box::new(Header::Boolean)))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Boolean.serialize().as_slice())),
            Ok(Header::Boolean)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::UInt8.serialize().as_slice())),
            Ok(Header::UInt8)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                Header::VarUInt16.serialize().as_slice()
            )),
            Ok(Header::VarUInt16)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::UInt32.serialize().as_slice())),
            Ok(Header::UInt32)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::UInt64.serialize().as_slice())),
            Ok(Header::UInt64)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Int8.serialize().as_slice())),
            Ok(Header::Int8)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Int16.serialize().as_slice())),
            Ok(Header::Int16)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Int32.serialize().as_slice())),
            Ok(Header::Int32)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Int64.serialize().as_slice())),
            Ok(Header::Int64)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Float32.serialize().as_slice())),
            Ok(Header::Float32)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Float64.serialize().as_slice())),
            Ok(Header::Float64)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::BigUInt.serialize().as_slice())),
            Ok(Header::BigUInt)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::BigInt.serialize().as_slice())),
            Ok(Header::BigInt)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                Header::BigDecimal.serialize().as_slice()
            )),
            Ok(Header::BigDecimal)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::String.serialize().as_slice())),
            Ok(Header::String)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::Binary.serialize().as_slice())),
            Ok(Header::Binary)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                Header::Array(Box::new(Header::Boolean))
                    .serialize()
                    .as_slice()
            )),
            Ok(Header::Array(Box::new(Header::Boolean)))
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(
                Header::Map({
                    let mut map = IndexMap::new();
                    map.insert(String::from("test"), Header::Boolean);
                    map
                })
                .serialize()
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
            Header::deserialize(&mut BufReader::new(Header::Date.serialize().as_slice())),
            Ok(Header::Date)
        );
        assert_eq!(
            Header::deserialize(&mut BufReader::new(Header::DateTime.serialize().as_slice())),
            Ok(Header::DateTime)
        );
    }
}
