use dullahan::{body::Body, deserializer::deserialize, header::Header, serializer::serialize};
use iai::main;
use indexmap::IndexMap;
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

fn deserialize_string() -> Result<(Header, Body), ()> {
    let body = Body::String(String::from("test"));
    deserialize(serialize(&Header::String, &body).unwrap().as_slice())
}

fn deserialize_map() -> Result<(Header, Body), ()> {
    let header = Header::Map({
        let mut map = IndexMap::new();
        map.insert(String::from("key1"), Header::Boolean);
        map.insert(String::from("key2"), Header::UInt8);
        map
    });

    let body = Body::Map({
        let mut map = IndexMap::new();
        map.insert(String::from("key1"), Body::Boolean(true));
        map.insert(String::from("key2"), Body::UInt8(u8::MAX));
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
    deserialize_string,
    deserialize_map,
    deserialize_datetime96,
);
