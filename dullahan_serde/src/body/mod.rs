use std::collections::BTreeMap;
use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num_bigint::{BigInt, BigUint};
use time::{Date, OffsetDateTime};

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
    Enum(u64, Box<Body>),
    Date(Date),
    DateTime(OffsetDateTime),
    Extension8((u64, u8)),
    Extension16((u64, [u8; 2])),
    Extension32((u64, [u8; 4])),
    Extension64((u64, [u8; 8])),
    Extension((u64, Vec<u8>)),
}
