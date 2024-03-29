use super::Header;
use crate::PrefixVarint;
use std::io::{ErrorKind, Read, Result};

pub trait DeserializeHeader<R: Read> {
    fn deserialize_header(&mut self) -> Result<Header>;
}

impl<R: Read> DeserializeHeader<R> for R {
    fn deserialize_header(&mut self) -> Result<Header> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;

        match buf[0] {
            super::UNIT_CODE => Ok(Header::Unit),
            super::OPTIONAL_CODE => {
                let inner = self.deserialize_header()?;
                Ok(Header::Optional(Box::new(inner)))
            }
            super::BOOLEAN_CODE => Ok(Header::Boolean),
            super::UINT8_CODE => Ok(Header::UInt8),
            super::UINT16_CODE => Ok(Header::UInt16),
            super::UINT32_CODE => Ok(Header::UInt32),
            super::UINT64_CODE => Ok(Header::UInt64),
            // super::UINT128_CODE => Ok(Header::UInt128),
            super::INT8_CODE => Ok(Header::Int8),
            super::INT16_CODE => Ok(Header::Int16),
            super::INT32_CODE => Ok(Header::Int32),
            super::INT64_CODE => Ok(Header::Int64),
            // super::INT128_CODE => Ok(Header::Int128),
            super::FLOAT32_CODE => Ok(Header::Float32),
            super::FLOAT64_CODE => Ok(Header::Float64),
            super::BIG_UINT_CODE => Ok(Header::BigUInt),
            super::BIG_INT_CODE => Ok(Header::BigInt),
            super::BIG_DECIMAL_CODE => Ok(Header::BigDecimal),
            super::STRING_CODE => Ok(Header::String),
            super::BINARY_CODE => Ok(Header::Binary),
            super::ARRAY_CODE => {
                let inner = self.deserialize_header()?;
                Ok(Header::Array(Box::new(inner)))
            }
            super::TUPLE_CODE => {
                let size = u16::decode_prefix_varint(self)?;
                let mut vec = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    vec.push(self.deserialize_header()?);
                }
                Ok(Header::Tuple(vec))
            }
            // super::STRUCT_CODE => {
            //     let size = u16::decode_prefix_varint(self)?;
            //     let mut buf = Vec::with_capacity(size as usize);
            //     for _ in 0..size {
            //         buf.push(self.deserialize_header()?);
            //     }
            //     Ok(Header::Struct(buf))
            // }
            super::MAP_CODE => {
                let inner = self.deserialize_header()?;
                Ok(Header::Map(Box::new(inner)))
            }
            super::ENUM_CODE => {
                let size = u16::decode_prefix_varint(self)?;
                let mut buf = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    buf.push(self.deserialize_header()?);
                }
                Ok(Header::Enum(buf))
            }
            super::DATE_CODE => Ok(Header::Date),
            super::DATETIME_CODE => Ok(Header::DateTime),
            code => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("invalid header code: {}", code),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeserializeHeader;
    use crate::{
        big_decimal::BigDecimal, big_int::BigInt, big_uint::BigUint, date::Date,
        date_time::DateTime, Header, SerializeHeader,
    };
    use serde_bytes::Bytes;
    use std::{collections::BTreeMap, io::Cursor};

    #[test]
    fn deserialize_header_unit() {
        let mut buf = Vec::new();
        <()>::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Unit);
    }

    #[test]
    fn deserialize_header_optional() {
        let mut buf = Vec::new();
        Option::<()>::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Optional(Box::new(Header::Unit))
        );
    }

    #[test]
    fn deserialize_header_boolean() {
        let mut buf = Vec::new();
        bool::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Boolean
        );
    }

    #[test]
    fn deserialize_header_uint8() {
        let mut buf = Vec::new();
        u8::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::UInt8
        );
    }

    #[test]
    fn deserialize_header_uint16() {
        let mut buf = Vec::new();
        u16::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::UInt16
        );
    }

    #[test]
    fn deserialize_header_uint32() {
        let mut buf = Vec::new();
        u32::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::UInt32
        );
    }

    #[test]
    fn deserialize_header_uint64() {
        let mut buf = Vec::new();
        u64::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::UInt64
        );
    }

    // #[test]
    // fn deserialize_header_uint128() {
    //     let mut buf = Vec::new();
    //     u128::serialize_header(&mut buf).unwrap();
    //     assert_eq!(
    //         Cursor::new(buf).deserialize_header().unwrap(),
    //         Header::UInt128
    //     );
    // }

    #[test]
    fn deserialize_header_int8() {
        let mut buf = Vec::new();
        i8::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Int8);
    }

    #[test]
    fn deserialize_header_int16() {
        let mut buf = Vec::new();
        i16::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Int16
        );
    }

    #[test]
    fn deserialize_header_int32() {
        let mut buf = Vec::new();
        i32::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Int32
        );
    }

    #[test]
    fn deserialize_header_int64() {
        let mut buf = Vec::new();
        i64::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Int64
        );
    }

    // #[test]
    // fn deserialize_header_int128() {
    //     let mut buf = Vec::new();
    //     i128::serialize_header(&mut buf).unwrap();
    //     assert_eq!(
    //         Cursor::new(buf).deserialize_header().unwrap(),
    //         Header::Int128
    //     );
    // }

    #[test]
    fn deserialize_header_float32() {
        let mut buf = Vec::new();
        f32::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Float32
        );
    }

    #[test]
    fn deserialize_header_float64() {
        let mut buf = Vec::new();
        f64::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Float64
        );
    }

    #[test]
    fn deserialize_header_big_uint() {
        let mut buf = Vec::new();
        BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigUInt
        );
    }

    #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
    #[test]
    fn deserialize_header_big_uint2() {
        let mut buf = Vec::new();
        num_bigint::BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigUInt
        );
    }

    #[test]
    fn deserialize_header_big_int() {
        let mut buf = Vec::new();
        BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigInt
        );
    }

    #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
    #[test]
    fn deserialize_header_big_int2() {
        let mut buf = Vec::new();
        num_bigint::BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigInt
        );
    }

    #[test]
    fn deserialize_header_big_decimal() {
        let mut buf = Vec::new();
        BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigDecimal
        );
    }

    #[cfg(feature = "bigdecimal")]
    #[test]
    fn deserialize_header_big_decimal2() {
        let mut buf = Vec::new();
        bigdecimal::BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::BigDecimal
        );
    }

    #[test]
    fn deserialize_header_string() {
        {
            let mut buf = Vec::new();
            String::serialize_header(&mut buf).unwrap();
            assert_eq!(
                Cursor::new(buf).deserialize_header().unwrap(),
                Header::String
            );
        }

        {
            let mut buf = Vec::new();
            <&str>::serialize_header(&mut buf).unwrap();
            assert_eq!(
                Cursor::new(buf).deserialize_header().unwrap(),
                Header::String
            );
        }
    }

    #[test]
    fn deserialize_header_binary() {
        let mut buf = Vec::new();
        Bytes::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Binary
        );
    }

    #[test]
    fn deserialize_header_array() {
        let mut buf = Vec::new();
        Vec::<()>::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Array(Box::new(Header::Unit))
        );
    }

    #[test]
    fn deserialize_header_tuple() {
        let mut buf = Vec::new();
        <((), bool, u8)>::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Tuple(vec![Header::Unit, Header::Boolean, Header::UInt8])
        );
    }

    #[test]
    fn deserialize_header_map() {
        let mut buf = Vec::new();
        BTreeMap::<String, bool>::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::Map(Box::new(Header::Boolean))
        );
    }

    #[test]
    fn deserialize_header_date() {
        let mut buf = Vec::new();
        Date::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Date);
    }

    #[cfg(feature = "time")]
    #[test]
    fn deserialize_header_date2() {
        let mut buf = Vec::new();
        time::Date::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Date);
    }

    #[test]
    fn deserialize_header_date_time() {
        let mut buf = Vec::new();
        DateTime::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::DateTime
        );
    }

    #[cfg(feature = "time")]
    #[test]
    fn deserialize_header_date_time2() {
        let mut buf = Vec::new();
        time::OffsetDateTime::serialize_header(&mut buf).unwrap();
        assert_eq!(
            Cursor::new(buf).deserialize_header().unwrap(),
            Header::DateTime
        );
    }
}
