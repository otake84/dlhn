use std::{collections::BTreeMap, io::Read};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize, ser::{SerializeMap, SerializeSeq, SerializeTuple}};
use serde_bytes::ByteBuf;
use time::{Date, OffsetDateTime};
use crate::{de::{Deserializer, Error}, format, header::Header};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Unit,
    Optional(Option<Box<Body>>),
    Boolean(bool),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    BigUInt(BigUint),
    BigInt(BigInt),
    BigDecimal(BigDecimal),
    String(String),
    Binary(Vec<u8>),
    Array(Vec<Body>),
    Tuple(Vec<Body>),
    Struct(Vec<Body>),
    Map(BTreeMap<String, Body>),
    Enum(u32, Box<Body>),
    Date(Date),
    DateTime(OffsetDateTime),
    Extension8((u64, u8)),
    Extension16((u64, [u8; 2])),
    Extension32((u64, [u8; 4])),
    Extension64((u64, [u8; 8])),
    Extension((u64, Vec<u8>)),
}

impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Body::Unit => serializer.serialize_unit(),
            Body::Optional(v) => {
                if let Some(v) = v {
                    serializer.serialize_some(v)
                } else {
                    serializer.serialize_none()
                }
            }
            Body::Boolean(v) => serializer.serialize_bool(*v),
            Body::UInt8(v) => serializer.serialize_u8(*v),
            Body::UInt16(v) => serializer.serialize_u16(*v),
            Body::UInt32(v) => serializer.serialize_u32(*v),
            Body::UInt64(v) => serializer.serialize_u64(*v),
            Body::Int8(v) => serializer.serialize_i8(*v),
            Body::Int16(v) => serializer.serialize_i16(*v),
            Body::Int32(v) => serializer.serialize_i32(*v),
            Body::Int64(v) => serializer.serialize_i64(*v),
            Body::Float32(v) => serializer.serialize_f32(*v),
            Body::Float64(v) => serializer.serialize_f64(*v),
            Body::BigUInt(v) => format::big_uint::serialize(v, serializer),
            Body::BigInt(v) => format::big_int::serialize(v, serializer),
            Body::BigDecimal(v) => format::big_decimal::serialize(v, serializer),
            Body::String(v) => serializer.serialize_str(v),
            Body::Binary(v) => serializer.serialize_bytes(v),
            Body::Array(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for body in v {
                    seq.serialize_element(body)?;
                }
                seq.end()
            }
            Body::Tuple(v) => {
                let mut tuple = serializer.serialize_tuple(v.len())?;
                for value in v {
                    tuple.serialize_element(value)?
                }
                tuple.end()
            }
            Body::Struct(v) => {
                let mut tuple = serializer.serialize_tuple(v.len())?;
                for value in v.iter() {
                    tuple.serialize_element(value)?;
                }
                tuple.end()
            }
            Body::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (key, value) in v {
                    map.serialize_entry(key, value)?
                }
                map.end()
            }
            Body::Enum(i, v) => {
                serializer.serialize_newtype_variant("", *i, "", v)
            }
            Body::Date(v) => format::date::serialize(v, serializer),
            Body::DateTime(v) => format::date_time::serialize(v, serializer),
            Body::Extension8((i, v)) => {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element(i)?;
                seq.serialize_element(v)?;
                seq.end()
            }
            Body::Extension16((i, v)) => {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element(i)?;
                seq.serialize_element(v)?;
                seq.end()
            }
            Body::Extension32((i, v)) => {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element(i)?;
                seq.serialize_element(v)?;
                seq.end()
            }
            Body::Extension64((i, v)) => {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element(i)?;
                seq.serialize_element(v)?;
                seq.end()
            }
            Body::Extension((i, v)) => {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element(i)?;
                seq.serialize_element(v)?;
                seq.end()
            }
        }
    }
}

impl Body {
    pub fn deserialize<R: Read>(header: &Header, deserializer: &mut Deserializer<R>) -> Result<Self, crate::de::Error> {
        match header {
            Header::Unit => Ok(Self::Unit),
            Header::Optional(inner) => {
                if bool::deserialize(&mut *deserializer)? {
                    Ok(Self::Optional(Some(Box::new(Self::deserialize(inner, deserializer)?))))
                } else {
                    Ok(Self::Optional(None))
                }
            }
            Header::Boolean => bool::deserialize(deserializer).map(Self::Boolean),
            Header::UInt8 => u8::deserialize(deserializer).map(Self::UInt8),
            Header::UInt16 => u16::deserialize(deserializer).map(Self::UInt16),
            Header::UInt32 => u32::deserialize(deserializer).map(Self::UInt32),
            Header::UInt64 => u64::deserialize(deserializer).map(Self::UInt64),
            Header::Int8 => i8::deserialize(deserializer).map(Self::Int8),
            Header::Int16 => i16::deserialize(deserializer).map(Self::Int16),
            Header::Int32 => i32::deserialize(deserializer).map(Self::Int32),
            Header::Int64 => i64::deserialize(deserializer).map(Self::Int64),
            Header::Float32 => f32::deserialize(deserializer).map(Self::Float32),
            Header::Float64 => f64::deserialize(deserializer).map(Self::Float64),
            Header::BigUInt => format::big_uint::deserialize(deserializer).map(Self::BigUInt),
            Header::BigInt => format::big_int::deserialize(deserializer).map(Self::BigInt),
            Header::BigDecimal => format::big_decimal::deserialize(deserializer).map(Self::BigDecimal),
            Header::String => String::deserialize(deserializer).map(Self::String),
            Header::Binary => ByteBuf::deserialize(deserializer).map(|v| Self::Binary(v.into_vec())),
            Header::Array(inner) => {
                let len = u64::deserialize(&mut *deserializer)?;
                let mut buf = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(Self::deserialize(inner, deserializer)?);
                }
                Ok(Self::Array(buf))
            }
            Header::Tuple(inner) => {
                let mut buf = Vec::with_capacity(inner.len());
                for inner in inner.iter() {
                    buf.push(Self::deserialize(inner, deserializer)?);
                }
                Ok(Self::Tuple(buf))
            }
            Header::Struct(inner) => {
                let mut buf = Vec::with_capacity(inner.len());
                for inner in inner.iter() {
                    buf.push(Self::deserialize(inner, deserializer)?);
                }
                Ok(Self::Struct(buf))
            }
            Header::Map(inner) => {
                let len = u64::deserialize(&mut *deserializer)?;
                let mut buf = BTreeMap::new();
                for _ in 0..len {
                    buf.insert(String::deserialize(&mut *deserializer)?, Self::deserialize(inner, deserializer)?);
                }
                Ok(Self::Map(buf))
            }
            Header::Enum(inner) => {
                let i = u32::deserialize(&mut *deserializer)?;
                let inner = inner.get(i as usize).ok_or(Error::Read)?;
                Ok(Self::Enum(i, Box::new(Self::deserialize(inner, deserializer)?)))
            }
            Header::Date => format::date::deserialize(deserializer).map(Self::Date),
            Header::DateTime => format::date_time::deserialize(deserializer).map(Self::DateTime),
            Header::Extension8(i) => Ok(Body::Extension8((*i, u8::deserialize(deserializer)?))),
            Header::Extension16(_) => todo!(),
            Header::Extension32(_) => todo!(),
            Header::Extension64(_) => todo!(),
            Header::Extension(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use crate::{body::Body, ser::Serializer};

    #[test]
    fn serialize_unit() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        ().serialize(&mut serializer).unwrap();

        let mut buf2 = Vec::new();
        let mut serializer2 = Serializer::new(&mut buf2);
        Body::Unit.serialize(&mut serializer2).unwrap();

        assert_eq!(buf, buf2);
    }

    #[test]
    fn serialize_bool() {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        true.serialize(&mut serializer).unwrap();

        let mut buf2 = Vec::new();
        let mut serializer2 = Serializer::new(&mut buf2);
        Body::Boolean(true).serialize(&mut serializer2).unwrap();

        assert_eq!(buf, buf2);
    }

    mod deserialize {
        use std::{array::IntoIter, collections::BTreeMap};
        use bigdecimal::BigDecimal;
        use num_bigint::{BigInt, BigUint};
        use serde::Serialize;
        use time::{Date, OffsetDateTime};
        use crate::{body::Body, de::Deserializer, header::Header, ser::Serializer};

        #[test]
        fn deserialize_unit() {
            let buf = [];
            assert_eq!(Body::deserialize(&Header::Unit, &mut Deserializer::new(&mut buf.as_ref())).unwrap(), Body::Unit);
        }

        #[test]
        fn deserialize_option() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                Some(true).serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Optional(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Optional(Some(Box::new(Body::Boolean(true)))));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                None::<Option<bool>>.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Optional(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Optional(None));
            }
        }

        #[test]
        fn deserialize_bool() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                true.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Boolean, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Boolean(true));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                false.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Boolean, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Boolean(false));
            }
        }

        #[test]
        fn deserialize_u8() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                0u8.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt8(0));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                u8::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt8(u8::MAX));
            }
        }

        #[test]
        fn deserialize_u16() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                0u16.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt16(0));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                u16::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt16(u16::MAX));
            }
        }

        #[test]
        fn deserialize_u32() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                0u32.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt32(0));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                u32::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt32(u32::MAX));
            }
        }

        #[test]
        fn deserialize_u64() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                0u64.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt64(0));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                u64::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::UInt64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt64(u64::MAX));
            }
        }

        #[test]
        fn deserialize_i8() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i8::MIN.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int8(i8::MIN));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i8::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int8(i8::MAX));
            }
        }

        #[test]
        fn deserialize_i16() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i16::MIN.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int16(i16::MIN));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i16::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int16(i16::MAX));
            }
        }

        #[test]
        fn deserialize_i32() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i32::MIN.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int32(i32::MIN));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i32::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int32(i32::MAX));
            }
        }

        #[test]
        fn deserialize_i64() {
            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i64::MIN.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int64(i64::MIN));
            }

            {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                i64::MAX.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Int64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int64(i64::MAX));
            }
        }

        #[test]
        fn deserialize_f32() {
            IntoIter::new([-f32::INFINITY, f32::MIN, 0f32, f32::MAX, f32::INFINITY]).for_each(|v| {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                v.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Float32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Float32(v));
            });
        }

        #[test]
        fn deserialize_f64() {
            IntoIter::new([-f64::INFINITY, f64::MIN, 0f64, f64::MAX, f64::INFINITY]).for_each(|v| {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                v.serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::Float64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Float64(v));
            });
        }

        #[test]
        fn deserialize_big_uint() {
            IntoIter::new([
                BigUint::from(0u8),
                BigUint::from(u8::MAX),
                BigUint::from(u16::MAX),
                BigUint::from(u16::MAX) + 1u8,
                BigUint::from(u32::MAX),
                BigUint::from(u32::MAX) + 1u8,
                BigUint::from(u64::MAX),
                BigUint::from(u64::MAX) + 1u8,
                BigUint::from(u128::MAX),
                BigUint::from(u128::MAX) + 1u8,
            ]).for_each(|v| {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                Body::BigUInt(v.clone()).serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::BigUInt, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::BigUInt(v));
            });
        }

        #[test]
        fn deserialize_big_int() {
            IntoIter::new([
                BigInt::from(0),
                BigInt::from(i8::MIN),
                BigInt::from(i8::MAX),
                BigInt::from(i8::MIN) - 1,
                BigInt::from(i8::MAX) + 1,
                BigInt::from(i16::MIN),
                BigInt::from(i16::MAX),
                BigInt::from(i16::MIN) - 1,
                BigInt::from(i16::MAX) + 1,
                BigInt::from(i32::MIN),
                BigInt::from(i32::MAX),
                BigInt::from(i32::MIN) - 1,
                BigInt::from(i32::MAX) + 1,
                BigInt::from(i64::MIN),
                BigInt::from(i64::MAX),
                BigInt::from(i64::MIN) - 1,
                BigInt::from(i64::MAX) + 1,
                BigInt::from(i128::MIN),
                BigInt::from(i128::MAX),
                BigInt::from(i128::MIN) - 1,
                BigInt::from(i128::MAX) + 1,
            ]).for_each(|v| {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                Body::BigInt(v.clone()).serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::BigInt, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::BigInt(v));
            });
        }

        #[test]
        fn deserialize_big_decimal() {
            IntoIter::new([
                BigDecimal::from(0),
                BigDecimal::new(BigInt::from(1), 0),
                BigDecimal::new(BigInt::from(1), -1),
                BigDecimal::new(BigInt::from(1), 1),
                BigDecimal::new(BigInt::from(1), 63),
                BigDecimal::new(BigInt::from(1), 64),
                BigDecimal::new(BigInt::from(1), -64),
                BigDecimal::new(BigInt::from(1), -65),
                BigDecimal::new(BigInt::from(i16::MIN), 0),
                BigDecimal::new(BigInt::from(i16::MAX), 0),
            ]).for_each(|v| {
                let mut buf = Vec::new();
                let mut serializer = Serializer::new(&mut buf);
                Body::BigDecimal(v.clone()).serialize(&mut serializer).unwrap();
                assert_eq!(Body::deserialize(&Header::BigDecimal, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::BigDecimal(v));
            });
        }

        #[test]
        fn deserialize_string() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::String("test".to_string());
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::String, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_binary() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Binary(vec![0, 1, 2, 3]);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Binary, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_array() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Array(vec![Body::Boolean(true), Body::Boolean(false), Body::Boolean(true)]);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Array(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_tuple() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())]);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Tuple(vec![Header::Boolean, Header::UInt8, Header::String]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_struct() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Struct(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())]);
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Struct(vec![Header::Boolean, Header::UInt8, Header::String]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_map() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Map({
                let mut buf = BTreeMap::new();
                buf.insert("a".to_string(), Body::Boolean(true));
                buf.insert("b".to_string(), Body::Boolean(false));
                buf.insert("c".to_string(), Body::Boolean(true));
                buf
            });
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Map(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_enum() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Enum(1, Box::new(Body::UInt8(123)));
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Enum(vec![Header::Boolean, Header::UInt8]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_date() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::Date(Date::try_from_ymd(1970, 1, 1).unwrap());
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::Date, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_date_time() {
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            let body = Body::DateTime(OffsetDateTime::unix_epoch());
            body.serialize(&mut serializer).unwrap();
            assert_eq!(Body::deserialize(&Header::DateTime, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }
    }
}
