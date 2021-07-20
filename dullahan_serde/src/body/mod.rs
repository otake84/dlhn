use std::collections::BTreeMap;
use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num_bigint::{BigInt, BigUint};
use serde::{Serialize, ser::{SerializeMap, SerializeSeq, SerializeTuple}};
use time::{Date, OffsetDateTime};
use crate::format;

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
    Struct(IndexMap<String, Body>),
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
                for value in v.values() {
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
}
