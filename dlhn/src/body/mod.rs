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
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
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
            Body::UInt128(v) => serializer.serialize_u128(*v),
            Body::Int8(v) => serializer.serialize_i8(*v),
            Body::Int16(v) => serializer.serialize_i16(*v),
            Body::Int32(v) => serializer.serialize_i32(*v),
            Body::Int64(v) => serializer.serialize_i64(*v),
            Body::Int128(v) => serializer.serialize_i128(*v),
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
            Header::UInt128 => u128::deserialize(deserializer).map(Self::UInt128),
            Header::Int8 => i8::deserialize(deserializer).map(Self::Int8),
            Header::Int16 => i16::deserialize(deserializer).map(Self::Int16),
            Header::Int32 => i32::deserialize(deserializer).map(Self::Int32),
            Header::Int64 => i64::deserialize(deserializer).map(Self::Int64),
            Header::Int128 => i128::deserialize(deserializer).map(Self::Int128),
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

    pub fn validate(&self, header: &Header) -> bool {
        match (header, self) {
            (Header::Unit, Body::Unit) => true,
            (Header::Optional(inner_header), Body::Optional(inner_body)) => {
                if let Some(v) = inner_body {
                    v.validate(inner_header)
                } else {
                    true
                }
            }
            (Header::Boolean, Body::Boolean(_)) => true,
            (Header::UInt8, Body::UInt8(_)) => true,
            (Header::UInt16, Body::UInt16(_)) => true,
            (Header::UInt32, Body::UInt32(_)) => true,
            (Header::UInt64, Body::UInt64(_)) => true,
            (Header::Int8, Body::Int8(_)) => true,
            (Header::Int16, Body::Int16(_)) => true,
            (Header::Int32, Body::Int32(_)) => true,
            (Header::Int64, Body::Int64(_)) => true,
            (Header::Float32, Body::Float32(_)) => true,
            (Header::Float64, Body::Float64(_)) => true,
            (Header::BigUInt, Body::BigUInt(_)) => true,
            (Header::BigInt, Body::BigInt(_)) => true,
            (Header::BigDecimal, Body::BigDecimal(_)) => true,
            (Header::String, Body::String(_)) => true,
            (Header::Binary, Body::Binary(_)) => true,
            (Header::Array(inner_header), Body::Array(inner_body)) => {
                inner_body.iter().all(|v| v.validate(inner_header))
            }
            (Header::Tuple(inner_headers), Body::Tuple(inner_bodies)) => {
                inner_headers.len() == inner_bodies.len()
                    && inner_headers
                        .iter()
                        .zip(inner_bodies)
                        .all(|(header, body)| body.validate(header))
            }
            (Header::Struct(inner_header), Body::Struct(inner_body)) => {
                inner_header.len() == inner_body.len()
                    && inner_header.iter().zip(inner_body).all(|(header, body)| {
                        body.validate(header)
                    })
            }
            (Header::Map(inner_header), Body::Map(inner_body)) => {
                inner_body
                    .values()
                    .all(|value| value.validate(inner_header))
            }
            (Header::Enum(inner_header), Body::Enum(i, v)) => {
                if let Some(header) = inner_header.get(*i as usize) {
                    v.validate(header)
                } else {
                    false
                }
            }
            (Header::Date, Body::Date(_)) => true,
            (Header::DateTime, Body::DateTime(_)) => true,
            (Header::Extension8(_), Body::Extension8(_)) => todo!(),
            (Header::Extension16(_), Body::Extension16(_)) => todo!(),
            (Header::Extension32(_), Body::Extension32(_)) => todo!(),
            (Header::Extension64(_), Body::Extension64(_)) => todo!(),
            (Header::Extension(_), Body::Extension(_)) => todo!(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use crate::{body::Body, ser::Serializer};

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }

    fn serialize_body(v: Body) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer2 = Serializer::new(&mut buf);
        v.serialize(&mut serializer2).unwrap();
        buf
    }

    mod serialize {
        use std::{array::IntoIter, collections::BTreeMap};
        use bigdecimal::BigDecimal;
        use num_bigint::{BigInt, BigUint};
        use serde_bytes::ByteBuf;
        use time::{Date, OffsetDateTime, Month};
        use super::*;

        #[test]
        fn serialize_unit() {
            assert_eq!(serialize_body(Body::Unit), serialize(()));
        }

        #[test]
        fn serialize_optional() {
            assert_eq!(serialize_body(Body::Optional(Some(Box::new(Body::Boolean(true))))), serialize(Some(true)));
            assert_eq!(serialize_body(Body::Optional(None)), serialize(None::<Option<bool>>));
        }

        #[test]
        fn serialize_bool() {
            assert_eq!(serialize_body(Body::Boolean(true)), serialize(true));
            assert_eq!(serialize_body(Body::Boolean(false)), serialize(false));
            assert_ne!(serialize_body(Body::Boolean(false)), serialize(true));
        }

        #[test]
        fn serialize_uint8() {
            assert_eq!(serialize_body(Body::UInt8(0)), serialize(0u8));
            assert_eq!(serialize_body(Body::UInt8(u8::MAX)), serialize(u8::MAX));
            assert_ne!(serialize_body(Body::UInt8(u8::MAX)), serialize(true));
        }

        #[test]
        fn serialize_uint16() {
            assert_eq!(serialize_body(Body::UInt16(0)), serialize(0u16));
            assert_eq!(serialize_body(Body::UInt16(u16::MAX)), serialize(u16::MAX));
            assert_ne!(serialize_body(Body::UInt16(u16::MAX)), serialize(true));
        }

        #[test]
        fn serialize_uint32() {
            assert_eq!(serialize_body(Body::UInt32(0)), serialize(0u32));
            assert_eq!(serialize_body(Body::UInt32(u32::MAX)), serialize(u32::MAX));
            assert_ne!(serialize_body(Body::UInt32(u32::MAX)), serialize(true));
        }

        #[test]
        fn serialize_uint64() {
            assert_eq!(serialize_body(Body::UInt64(0)), serialize(0u64));
            assert_eq!(serialize_body(Body::UInt64(u64::MAX)), serialize(u64::MAX));
            assert_ne!(serialize_body(Body::UInt64(u64::MAX)), serialize(true));
        }

        #[test]
        fn serialize_uint128() {
            assert_eq!(serialize_body(Body::UInt128(0)), serialize(0u128));
            assert_eq!(serialize_body(Body::UInt128(u128::MAX)), serialize(u128::MAX));
            assert_ne!(serialize_body(Body::UInt128(u128::MAX)), serialize(true));
        }

        #[test]
        fn serialize_int8() {
            assert_eq!(serialize_body(Body::Int8(i8::MIN)), serialize(i8::MIN));
            assert_eq!(serialize_body(Body::Int8(0)), serialize(0i8));
            assert_eq!(serialize_body(Body::Int8(i8::MAX)), serialize(i8::MAX));
            assert_ne!(serialize_body(Body::Int8(i8::MAX)), serialize(true));
        }

        #[test]
        fn serialize_int16() {
            assert_eq!(serialize_body(Body::Int16(i16::MIN)), serialize(i16::MIN));
            assert_eq!(serialize_body(Body::Int16(0)), serialize(0i16));
            assert_eq!(serialize_body(Body::Int16(i16::MAX)), serialize(i16::MAX));
            assert_ne!(serialize_body(Body::Int16(i16::MAX)), serialize(true));
        }

        #[test]
        fn serialize_int32() {
            assert_eq!(serialize_body(Body::Int32(i32::MIN)), serialize(i32::MIN));
            assert_eq!(serialize_body(Body::Int32(0)), serialize(0i32));
            assert_eq!(serialize_body(Body::Int32(i32::MAX)), serialize(i32::MAX));
            assert_ne!(serialize_body(Body::Int32(i32::MAX)), serialize(true));
        }

        #[test]
        fn serialize_int64() {
            assert_eq!(serialize_body(Body::Int64(i64::MIN)), serialize(i64::MIN));
            assert_eq!(serialize_body(Body::Int64(0)), serialize(0i64));
            assert_eq!(serialize_body(Body::Int64(i64::MAX)), serialize(i64::MAX));
            assert_ne!(serialize_body(Body::Int64(i64::MAX)), serialize(true));
        }

        #[test]
        fn serialize_int128() {
            assert_eq!(serialize_body(Body::Int128(i128::MIN)), serialize(i128::MIN));
            assert_eq!(serialize_body(Body::Int128(0)), serialize(0i128));
            assert_eq!(serialize_body(Body::Int128(i128::MAX)), serialize(i128::MAX));
            assert_ne!(serialize_body(Body::Int128(i128::MAX)), serialize(true));
        }


        #[test]
        fn serialize_f32() {
            assert_eq!(serialize_body(Body::Float32(0f32)), serialize(0f32));
            assert_eq!(serialize_body(Body::Float32(1.1f32)), serialize(1.1f32));
            assert_eq!(serialize_body(Body::Float32(-1.1f32)), serialize(-1.1f32));
            assert_eq!(serialize_body(Body::Float32(f32::INFINITY)), serialize(f32::INFINITY));
            assert_eq!(serialize_body(Body::Float32(-f32::INFINITY)), serialize(-f32::INFINITY));
            assert_eq!(serialize_body(Body::Float32(f32::NAN)), serialize(f32::NAN));
            assert_eq!(serialize_body(Body::Float32(-f32::NAN)), serialize(-f32::NAN));
        }

        #[test]
        fn serialize_f64() {
            assert_eq!(serialize_body(Body::Float64(0f64)), serialize(0f64));
            assert_eq!(serialize_body(Body::Float64(1.1f64)), serialize(1.1f64));
            assert_eq!(serialize_body(Body::Float64(-1.1f64)), serialize(-1.1f64));
            assert_eq!(serialize_body(Body::Float64(f64::INFINITY)), serialize(f64::INFINITY));
            assert_eq!(serialize_body(Body::Float64(-f64::INFINITY)), serialize(-f64::INFINITY));
            assert_eq!(serialize_body(Body::Float64(f64::NAN)), serialize(f64::NAN));
            assert_eq!(serialize_body(Body::Float64(-f64::NAN)), serialize(-f64::NAN));
        }

        #[test]
        fn serialize_big_uint() {
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
                crate::format::big_uint::serialize(&v, &mut serializer).unwrap();
                assert_eq!(serialize_body(Body::BigUInt(v)), buf);
            });
        }

        #[test]
        fn serialize_big_int() {
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
                crate::format::big_int::serialize(&v, &mut serializer).unwrap();
                assert_eq!(serialize_body(Body::BigInt(v)), buf);
            });
        }

        #[test]
        fn serialize_big_decimal() {
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
                crate::format::big_decimal::serialize(&v, &mut serializer).unwrap();
                assert_eq!(serialize_body(Body::BigDecimal(v)), buf);
            });
        }

        #[test]
        fn serialize_string() {
            IntoIter::new([
                "test",
                "テスト",
            ]).for_each(|v| {
                assert_eq!(serialize_body(Body::String(v.to_string())), serialize(v));
            });
        }

        #[test]
        fn serialize_binary() {
            assert_eq!(serialize_body(Body::Binary(vec![0, 1, 2, 3])), serialize(ByteBuf::from(vec![0 , 1, 2, 3])));
        }

        #[test]
        fn serialize_array() {
            assert_eq!(serialize_body(Body::Array(vec![Body::Boolean(true), Body::Boolean(false)])), serialize(vec![true, false]));
        }

        #[test]
        fn serialize_tuple() {
            assert_eq!(serialize_body(Body::Tuple(vec![Body::Unit, Body::Boolean(false)])), serialize(((), false)));
        }

        #[test]
        fn serialize_struct() {
            #[derive(Serialize)]
            struct Test {
                a: (),
                b: bool,
            }
            assert_eq!(serialize_body(Body::Struct(vec![Body::Unit, Body::Boolean(false)])), serialize(Test { a: (), b: false }));
        }

        #[test]
        fn serialize_map() {
            assert_eq!(serialize_body(Body::Map({
                let mut v = BTreeMap::new();
                v.insert("a".to_string(), Body::Boolean(true));
                v.insert("b".to_string(), Body::Boolean(false));
                v.insert("c".to_string(), Body::Boolean(true));
                v
            })), serialize({
                let mut v = BTreeMap::new();
                v.insert("a".to_string(), Body::Boolean(true));
                v.insert("b".to_string(), Body::Boolean(false));
                v.insert("c".to_string(), Body::Boolean(true));
                v
            }));
        }

        #[test]
        fn serialize_enum() {
            #[allow(dead_code)]
            #[derive(Serialize)]
            enum Test {
                A,
                B(bool),
                C(bool, u8),
            }
            assert_eq!(serialize_body(Body::Enum(2, Box::new(Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123)])))), serialize(Test::C(true, 123)));
        }

        #[test]
        fn serialize_date() {
            let v = Date::from_calendar_date(1970, Month::January, 1).unwrap();
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            crate::format::date::serialize(&v, &mut serializer).unwrap();
            assert_eq!(serialize_body(Body::Date(v)), buf);
        }

        #[test]
        fn serialize_date_time() {
            let v = OffsetDateTime::UNIX_EPOCH;
            let mut buf = Vec::new();
            let mut serializer = Serializer::new(&mut buf);
            crate::format::date_time::serialize(&v, &mut serializer).unwrap();
            assert_eq!(serialize_body(Body::DateTime(v)), buf);
        }
    }

    mod deserialize {
        use super::*;
        use std::{array::IntoIter, collections::BTreeMap};
        use bigdecimal::BigDecimal;
        use num_bigint::{BigInt, BigUint};
        use serde::Serialize;
        use time::{Date, Month, OffsetDateTime};
        use crate::{body::Body, de::Deserializer, header::Header, ser::Serializer};

        #[test]
        fn deserialize_unit() {
            let buf = serialize(());
            assert_eq!(Body::deserialize(&Header::Unit, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Unit);
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
                let buf = serialize(None::<Option<bool>>);
                assert_eq!(Body::deserialize(&Header::Optional(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Optional(None));
            }
        }

        #[test]
        fn deserialize_bool() {
            {
                let buf = serialize(true);
                assert_eq!(Body::deserialize(&Header::Boolean, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Boolean(true));
            }

            {
                let buf = serialize(false);
                assert_eq!(Body::deserialize(&Header::Boolean, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Boolean(false));
            }
        }

        #[test]
        fn deserialize_u8() {
            {
                let buf = serialize(0u8);
                assert_eq!(Body::deserialize(&Header::UInt8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt8(0));
            }

            {
                let buf = serialize(u8::MAX);
                assert_eq!(Body::deserialize(&Header::UInt8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt8(u8::MAX));
            }
        }

        #[test]
        fn deserialize_u16() {
            {
                let buf = serialize(0u16);
                assert_eq!(Body::deserialize(&Header::UInt16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt16(0));
            }

            {
                let buf = serialize(u16::MAX);
                assert_eq!(Body::deserialize(&Header::UInt16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt16(u16::MAX));
            }
        }

        #[test]
        fn deserialize_u32() {
            {
                let buf = serialize(0u32);
                assert_eq!(Body::deserialize(&Header::UInt32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt32(0));
            }

            {
                let buf = serialize(u32::MAX);
                assert_eq!(Body::deserialize(&Header::UInt32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt32(u32::MAX));
            }
        }

        #[test]
        fn deserialize_u64() {
            {
                let buf = serialize(0u64);
                assert_eq!(Body::deserialize(&Header::UInt64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt64(0));
            }

            {
                let buf = serialize(u64::MAX);
                assert_eq!(Body::deserialize(&Header::UInt64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt64(u64::MAX));
            }
        }

        #[test]
        fn deserialize_u128() {
            {
                let buf = serialize(0u128);
                assert_eq!(Body::deserialize(&Header::UInt128, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt128(0));
            }

            {
                let buf = serialize(u128::MAX);
                assert_eq!(Body::deserialize(&Header::UInt128, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::UInt128(u128::MAX));
            }
        }

        #[test]
        fn deserialize_i8() {
            {
                let buf = serialize(i8::MIN);
                assert_eq!(Body::deserialize(&Header::Int8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int8(i8::MIN));
            }

            {
                let buf = serialize(0i8);
                assert_eq!(Body::deserialize(&Header::Int8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int8(0i8));
            }

            {
                let buf = serialize(i8::MAX);
                assert_eq!(Body::deserialize(&Header::Int8, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int8(i8::MAX));
            }
        }

        #[test]
        fn deserialize_i16() {
            {
                let buf = serialize(i16::MIN);
                assert_eq!(Body::deserialize(&Header::Int16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int16(i16::MIN));
            }

            {
                let buf = serialize(0i16);
                assert_eq!(Body::deserialize(&Header::Int16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int16(0i16));
            }

            {
                let buf = serialize(i16::MAX);
                assert_eq!(Body::deserialize(&Header::Int16, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int16(i16::MAX));
            }
        }

        #[test]
        fn deserialize_i32() {
            {
                let buf = serialize(i32::MIN);
                assert_eq!(Body::deserialize(&Header::Int32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int32(i32::MIN));
            }

            {
                let buf = serialize(0i32);
                assert_eq!(Body::deserialize(&Header::Int32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int32(0i32));
            }

            {
                let buf = serialize(i32::MAX);
                assert_eq!(Body::deserialize(&Header::Int32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int32(i32::MAX));
            }
        }

        #[test]
        fn deserialize_i64() {
            {
                let buf = serialize(i64::MIN);
                assert_eq!(Body::deserialize(&Header::Int64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int64(i64::MIN));
            }

            {
                let buf = serialize(0i64);
                assert_eq!(Body::deserialize(&Header::Int64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int64(0i64));
            }

            {
                let buf = serialize(i64::MAX);
                assert_eq!(Body::deserialize(&Header::Int64, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int64(i64::MAX));
            }
        }

        #[test]
        fn deserialize_i128() {
            {
                let buf = serialize(i128::MIN);
                assert_eq!(Body::deserialize(&Header::Int128, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int128(i128::MIN));
            }

            {
                let buf = serialize(0i128);
                assert_eq!(Body::deserialize(&Header::Int128, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int128(0i128));
            }

            {
                let buf = serialize(i128::MAX);
                assert_eq!(Body::deserialize(&Header::Int128, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Int128(i128::MAX));
            }
        }

        #[test]
        fn deserialize_f32() {
            IntoIter::new([-f32::INFINITY, f32::MIN, 0f32, f32::MAX, f32::INFINITY]).for_each(|v| {
                let buf = serialize(v);
                assert_eq!(Body::deserialize(&Header::Float32, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::Float32(v));
            });
        }

        #[test]
        fn deserialize_f64() {
            IntoIter::new([-f64::INFINITY, f64::MIN, 0f64, f64::MAX, f64::INFINITY]).for_each(|v| {
                let buf = serialize(v);
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
                let buf = serialize_body(Body::BigUInt(v.clone()));
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
                let buf = serialize_body(Body::BigInt(v.clone()));
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
                let buf = serialize_body(Body::BigDecimal(v.clone()));
                assert_eq!(Body::deserialize(&Header::BigDecimal, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), Body::BigDecimal(v));
            });
        }

        #[test]
        fn deserialize_string() {
            {
                let body = Body::String("test".to_string());
                let buf = serialize_body(body.clone());
                assert_eq!(Body::deserialize(&Header::String, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
            }

            {
                let body = Body::String("テスト".to_string());
                let buf = serialize_body(body.clone());
                assert_eq!(Body::deserialize(&Header::String, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
            }
        }

        #[test]
        fn deserialize_binary() {
            let body = Body::Binary(vec![0, 1, 2, 3]);
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Binary, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_array() {
            let body = Body::Array(vec![Body::Boolean(true), Body::Boolean(false), Body::Boolean(true)]);
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Array(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_tuple() {
            let body = Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())]);
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Tuple(vec![Header::Boolean, Header::UInt8, Header::String]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_struct() {
            let body = Body::Struct(vec![Body::Boolean(true), Body::UInt8(123), Body::String("test".to_string())]);
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Struct(vec![Header::Boolean, Header::UInt8, Header::String]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_map() {
            let body = Body::Map({
                let mut buf = BTreeMap::new();
                buf.insert("a".to_string(), Body::Boolean(true));
                buf.insert("b".to_string(), Body::Boolean(false));
                buf.insert("c".to_string(), Body::Boolean(true));
                buf
            });
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Map(Box::new(Header::Boolean)), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_enum() {
            let body = Body::Enum(1, Box::new(Body::UInt8(123)));
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Enum(vec![Header::Boolean, Header::UInt8]), &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_date() {
            let body = Body::Date(Date::from_calendar_date(1970, Month::January, 1).unwrap());
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::Date, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }

        #[test]
        fn deserialize_date_time() {
            let body = Body::DateTime(OffsetDateTime::UNIX_EPOCH);
            let buf = serialize_body(body.clone());
            assert_eq!(Body::deserialize(&Header::DateTime, &mut Deserializer::new(&mut buf.as_slice().as_ref())).unwrap(), body);
        }
    }

    mod validate {
        use std::collections::BTreeMap;
        use bigdecimal::BigDecimal;
        use num_bigint::{BigInt, BigUint};
        use time::{Date, Month, OffsetDateTime};
        use crate::header::Header;
        use super::*;

        #[test]
        fn validate_unit() {
            let header = Header::Unit;
            assert!(Body::Unit.validate(&header));
            assert!(!Body::Boolean(true).validate(&header));
        }

        #[test]
        fn validate_optional() {
            let header = Header::Optional(Box::new(Header::Boolean));
            assert!(Body::Optional(Some(Box::new(Body::Boolean(true)))).validate(&header));
            assert!(!Body::Optional(Some(Box::new(Body::Unit))).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_boolean() {
            let header = Header::Boolean;
            assert!(Body::Boolean(true).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_uint8() {
            let header = Header::UInt8;
            assert!(Body::UInt8(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_uint16() {
            let header = Header::UInt16;
            assert!(Body::UInt16(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_uint32() {
            let header = Header::UInt32;
            assert!(Body::UInt32(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_uint64() {
            let header = Header::UInt64;
            assert!(Body::UInt64(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_int8() {
            let header = Header::Int8;
            assert!(Body::Int8(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_int16() {
            let header = Header::Int16;
            assert!(Body::Int16(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_int32() {
            let header = Header::Int32;
            assert!(Body::Int32(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_int64() {
            let header = Header::Int64;
            assert!(Body::Int64(123).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_float32() {
            let header = Header::Float32;
            assert!(Body::Float32(1.1).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_float64() {
            let header = Header::Float64;
            assert!(Body::Float64(1.1).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_big_uint() {
            let header = Header::BigUInt;
            assert!(Body::BigUInt(BigUint::from(123u8)).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_big_int() {
            let header = Header::BigInt;
            assert!(Body::BigInt(BigInt::from(123)).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_big_decimal() {
            let header = Header::BigDecimal;
            assert!(Body::BigDecimal(BigDecimal::from(123)).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_string() {
            let header = Header::String;
            assert!(Body::String("test".to_string()).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_binary() {
            let header = Header::Binary;
            assert!(Body::Binary(vec![0, 1, 2, 3]).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_array() {
            let header = Header::Array(Box::new(Header::Boolean));
            assert!(Body::Array(vec![Body::Boolean(true), Body::Boolean(false), Body::Boolean(true)]).validate(&header));
            assert!(!Body::Array(vec![Body::Unit]).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_tuple() {
            let header = Header::Tuple(vec![Header::Boolean, Header::UInt8]);
            assert!(Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123)]).validate(&header));
            assert!(!Body::Tuple(vec![Body::Boolean(true), Body::Boolean(true)]).validate(&header));
            assert!(!Body::Tuple(vec![Body::Boolean(true), Body::UInt8(123), Body::UInt8(123)]).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_struct() {
            let header = Header::Struct(vec![Header::Boolean, Header::UInt8]);
            assert!(Body::Struct(vec![Body::Boolean(true), Body::UInt8(123)]).validate(&header));
            assert!(!Body::Struct(vec![Body::Boolean(true), Body::Boolean(true)]).validate(&header));
            assert!(!Body::Struct(vec![Body::Boolean(true), Body::UInt8(123), Body::UInt8(123)]).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_map() {
            let header = Header::Map(Box::new(Header::Boolean));
            assert!(Body::Map({
                let mut buf = BTreeMap::new();
                buf.insert("a".to_string(), Body::Boolean(true));
                buf.insert("b".to_string(), Body::Boolean(false));
                buf.insert("c".to_string(), Body::Boolean(true));
                buf
            }).validate(&header));

            assert!(!Body::Map({
                let mut buf = BTreeMap::new();
                buf.insert("a".to_string(), Body::Unit);
                buf.insert("b".to_string(), Body::Unit);
                buf.insert("c".to_string(), Body::Unit);
                buf
            }).validate(&header));

            assert!(!Body::Map({
                let mut buf = BTreeMap::new();
                buf.insert("a".to_string(), Body::Boolean(true));
                buf.insert("b".to_string(), Body::Unit);
                buf.insert("c".to_string(), Body::Unit);
                buf
            }).validate(&header));

            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_enum() {
            let header = Header::Enum(vec![Header::Unit, Header::Boolean]);
            assert!(Body::Enum(0, Box::new(Body::Unit)).validate(&header));
            assert!(Body::Enum(1, Box::new(Body::Boolean(true))).validate(&header));
            assert!(!Body::Enum(0, Box::new(Body::Boolean(true))).validate(&header));
            assert!(!Body::Enum(1, Box::new(Body::Unit)).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_date() {
            let header = Header::Date;
            assert!(Body::Date(Date::from_calendar_date(1970, Month::January, 1).unwrap()).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }

        #[test]
        fn validate_date_time() {
            let header = Header::DateTime;
            assert!(Body::DateTime(OffsetDateTime::UNIX_EPOCH).validate(&header));
            assert!(!Body::Unit.validate(&header));
        }
    }
}
