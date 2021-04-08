use bigdecimal::BigDecimal;
use dullahan::{body::Body, deserializer::deserialize, header::Header, serializer::serialize};
use iai::main;
use num_bigint::{BigInt, BigUint};
use std::collections::BTreeMap;
use time::{NumericalDuration, OffsetDateTime};

fn deserialize_optional() -> Result<(Header, Body), ()> {
    deserialize([0u8, 1, 1, 0].as_ref())
}

fn deserialize_boolean() -> Result<(Header, Body), ()> {
    deserialize([1u8, 0].as_ref())
}

fn deserialize_uint8() -> Result<(Header, Body), ()> {
    deserialize([2u8, 255].as_ref())
}

fn deserialize_uint16() -> Result<(Header, Body), ()> {
    deserialize([3u8, 255, 255].as_ref())
}

fn deserialize_uint32() -> Result<(Header, Body), ()> {
    deserialize([4u8, 255, 255, 255, 255].as_ref())
}

fn deserialize_uint64() -> Result<(Header, Body), ()> {
    deserialize([5u8, 255, 255, 255, 255, 255, 255, 255, 255].as_ref())
}

fn deserialize_int8() -> Result<(Header, Body), ()> {
    deserialize([9u8, 255].as_ref())
}

fn deserialize_float32() -> Result<(Header, Body), ()> {
    deserialize(
        [vec![13u8], 1.1f32.to_le_bytes().to_vec()]
            .concat()
            .as_slice(),
    )
}

fn deserialize_float64() -> Result<(Header, Body), ()> {
    deserialize(
        [vec![14u8], 1.1f64.to_le_bytes().to_vec()]
            .concat()
            .as_slice(),
    )
}

fn deserialize_biguint() -> Result<(Header, Body), ()> {
    deserialize(
        serialize(&Header::BigUInt, &Body::BigUInt(BigUint::from(u128::MAX)))
            .unwrap()
            .as_slice(),
    )
}

fn deserialize_bigint() -> Result<(Header, Body), ()> {
    deserialize(
        serialize(&Header::BigInt, &Body::BigInt(BigInt::from(i128::MAX)))
            .unwrap()
            .as_slice(),
    )
}

fn deserialize_bigdecimal() -> Result<(Header, Body), ()> {
    deserialize(
        serialize(
            &Header::BigDecimal,
            &Body::BigDecimal(BigDecimal::new(BigInt::from(i128::MAX), 0)),
        )
        .unwrap()
        .as_slice(),
    )
}

fn deserialize_string() -> Result<(Header, Body), ()> {
    let body = Body::String(String::from("test"));
    deserialize(serialize(&Header::String, &body).unwrap().as_slice())
}

fn deserialize_binary() -> Result<(Header, Body), ()> {
    let body = vec![0, 1, 2, 3, 255];
    deserialize(
        serialize(&Header::Binary, &Body::Binary(body))
            .unwrap()
            .as_slice(),
    )
}

fn deserialize_map() -> Result<(Header, Body), ()> {
    let header = Header::Map({
        let mut map = BTreeMap::new();
        map.insert(String::from("key1"), Header::Boolean);
        map.insert(String::from("key2"), Header::UInt8);
        map
    });

    let body = Body::Map({
        let mut map = BTreeMap::new();
        map.insert(String::from("key1"), Body::Boolean(true));
        map.insert(String::from("key2"), Body::UInt8(u8::MAX));
        map
    });

    deserialize(serialize(&header, &body).unwrap().as_slice())
}

fn deserialize_dynamic_map() -> Result<(Header, Body), ()> {
    let header = Header::DynamicMap(Box::new(Header::Boolean));

    let body = Body::DynamicMap({
        let mut map = BTreeMap::new();
        map.insert(String::from("key1"), Body::Boolean(true));
        map.insert(String::from("key2"), Body::Boolean(false));
        map
    });

    deserialize(serialize(&header, &body).unwrap().as_slice())
}

fn deserialize_datetime96() -> Result<(Header, Body), ()> {
    let body = Body::DateTime(OffsetDateTime::unix_epoch() - 1.nanoseconds());
    deserialize(serialize(&Header::DateTime, &body).unwrap().as_slice())
}

main!(
    deserialize_optional,
    deserialize_boolean,
    deserialize_uint8,
    deserialize_uint16,
    deserialize_uint32,
    deserialize_uint64,
    deserialize_int8,
    deserialize_float32,
    deserialize_float64,
    deserialize_biguint,
    deserialize_bigint,
    deserialize_bigdecimal,
    deserialize_string,
    deserialize_binary,
    deserialize_map,
    deserialize_dynamic_map,
    deserialize_datetime96,
);
