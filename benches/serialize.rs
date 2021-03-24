use bigdecimal::BigDecimal;
use dullahan::{body::Body, header::Header, serializer::serialize};
use iai::main;
use num_bigint::{BigInt, BigUint};
use std::collections::BTreeMap;
use time::{Date, NumericalDuration, OffsetDateTime};

fn serialize_optional() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::Optional(Box::new(Header::Boolean)),
        &Body::Optional(Some(Box::new(Body::Boolean(true)))),
    )
}

fn serialize_uint8() -> Result<Vec<u8>, ()> {
    serialize(&Header::UInt8, &Body::UInt8(u8::MAX))
}

fn serialize_int8() -> Result<Vec<u8>, ()> {
    serialize(&Header::Int8, &Body::Int8(i8::MAX))
}

fn serialize_float32() -> Result<Vec<u8>, ()> {
    serialize(&Header::Float32, &Body::Float32(1.23456))
}

fn serialize_float64() -> Result<Vec<u8>, ()> {
    serialize(&Header::Float64, &Body::Float64(1.23456))
}

fn serialize_string() -> Result<Vec<u8>, ()> {
    serialize(&Header::String, &Body::String(String::from("test")))
}

fn serialize_biguint() -> Result<Vec<u8>, ()> {
    serialize(&Header::BigUInt, &Body::BigUInt(BigUint::from(u128::MAX)))
}

fn serialize_bigint() -> Result<Vec<u8>, ()> {
    serialize(&Header::BigInt, &Body::BigInt(BigInt::from(i128::MAX)))
}

fn serialize_bigdecimal() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::BigDecimal,
        &Body::BigDecimal(BigDecimal::from(i64::MAX)),
    )
}

fn serialize_binary() -> Result<Vec<u8>, ()> {
    serialize(&Header::Binary, &Body::Binary(vec![1, 2, 3, 4, 5]))
}

fn serialize_array() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::Array(Box::new(Header::Boolean)),
        &Body::Array(vec![
            Body::Boolean(true),
            Body::Boolean(false),
            Body::Boolean(true),
            Body::Boolean(false),
            Body::Boolean(true),
            Body::Boolean(false),
        ]),
    )
}

fn serialize_map() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::Map({
            let mut map = BTreeMap::new();
            map.insert(String::from("key1"), Header::Boolean);
            map.insert(String::from("key2"), Header::UInt8);
            map
        }),
        &Body::Map({
            let mut map = BTreeMap::new();
            map.insert(String::from("key1"), Body::Boolean(true));
            map.insert(String::from("key2"), Body::UInt8(u8::MAX));
            map
        }),
    )
}

fn serialize_dynamic_map() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::DynamicMap(Box::new(Header::Boolean)),
        &Body::DynamicMap({
            let mut map = BTreeMap::new();
            map.insert(String::from("key1"), Body::Boolean(true));
            map.insert(String::from("key2"), Body::Boolean(false));
            map
        }),
    )
}

fn serialize_date() -> Result<Vec<u8>, ()> {
    serialize(
        &Header::Date,
        &Body::Date(Date::try_from_ymd(1970, 1, 1).unwrap()),
    )
}

fn serialize_datetime96() -> Result<Vec<u8>, ()> {
    let body = Body::DateTime(OffsetDateTime::unix_epoch() - 1.nanoseconds());
    serialize(&Header::DateTime, &body)
}

fn serialize_extension8() -> Result<Vec<u8>, ()> {
    let body = Body::Extension8((255, 123));
    serialize(&Header::Extension8(255), &body)
}

fn serialize_extension16() -> Result<Vec<u8>, ()> {
    let body = Body::Extension16([123, 0]);
    serialize(&Header::Extension16(255), &body)
}

fn serialize_extension32() -> Result<Vec<u8>, ()> {
    let body = Body::Extension32([123, 0, 123, 0]);
    serialize(&Header::Extension32(255), &body)
}

fn serialize_extension64() -> Result<Vec<u8>, ()> {
    let body = Body::Extension64([123, 0, 123, 0, 123, 0, 123, 0]);
    serialize(&Header::Extension64(255), &body)
}

main!(
    serialize_optional,
    serialize_uint8,
    serialize_int8,
    serialize_float32,
    serialize_float64,
    serialize_string,
    serialize_biguint,
    serialize_bigint,
    serialize_bigdecimal,
    serialize_binary,
    serialize_array,
    serialize_map,
    serialize_dynamic_map,
    serialize_date,
    serialize_datetime96,
    serialize_extension8,
    serialize_extension16,
    serialize_extension32,
    serialize_extension64,
);
