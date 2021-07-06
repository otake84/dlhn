use std::io::{Read, Result};
use super::Header;

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
            super::INT8_CODE => Ok(Header::Int8),
            super::INT16_CODE => Ok(Header::Int16),
            super::INT32_CODE => Ok(Header::Int32),
            super::INT64_CODE => Ok(Header::Int64),
            super::FLOAT32_CODE => Ok(Header::Float32),
            super::FLOAT64_CODE => Ok(Header::Float64),
            super::BIG_UINT_CODE => Ok(Header::BigUInt),
            super::BIG_INT_CODE => Ok(Header::BigInt),
            super::BIG_DECIMAL_CODE => Ok(Header::BigDecimal),
            super::STRING_CODE => Ok(Header::String),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use bigdecimal::BigDecimal;
    use num_bigint::{BigInt, BigUint};

    use crate::header::{Header, serialize_header::SerializeHeader};
    use super::DeserializeHeader;

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
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Optional(Box::new(Header::Unit)));
    }

    #[test]
    fn deserialize_header_boolean() {
        let mut buf = Vec::new();
        bool::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Boolean);
    }

    #[test]
    fn deserialize_header_uint8() {
        let mut buf = Vec::new();
        u8::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::UInt8);
    }

    #[test]
    fn deserialize_header_uint16() {
        let mut buf = Vec::new();
        u16::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::UInt16);
    }

    #[test]
    fn deserialize_header_uint32() {
        let mut buf = Vec::new();
        u32::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::UInt32);
    }

    #[test]
    fn deserialize_header_uint64() {
        let mut buf = Vec::new();
        u64::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::UInt64);
    }

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
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Int16);
    }

    #[test]
    fn deserialize_header_int32() {
        let mut buf = Vec::new();
        i32::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Int32);
    }

    #[test]
    fn deserialize_header_int64() {
        let mut buf = Vec::new();
        i64::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Int64);
    }

    #[test]
    fn deserialize_header_float32() {
        let mut buf = Vec::new();
        f32::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Float32);
    }

    #[test]
    fn deserialize_header_float64() {
        let mut buf = Vec::new();
        f64::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Float64);
    }

    #[test]
    fn deserialize_header_big_uint() {
        let mut buf = Vec::new();
        BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::BigUInt);
    }

    #[test]
    fn deserialize_header_big_int() {
        let mut buf = Vec::new();
        BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::BigInt);
    }

    #[test]
    fn deserialize_header_big_decimal() {
        let mut buf = Vec::new();
        BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::BigDecimal);
    }

    #[test]
    fn deserialize_header_string() {
        {
            let mut buf = Vec::new();
            String::serialize_header(&mut buf).unwrap();
            assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::String);
        }

        {
            let mut buf = Vec::new();
            <&str>::serialize_header(&mut buf).unwrap();
            assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::String);
        }
    }
}
