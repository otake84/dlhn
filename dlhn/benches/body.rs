use dlhn::{body::Body, ser::Serializer};
use iai::main;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

fn serialize_unit() -> Vec<u8> {
    serialize(Body::Unit)
}

fn serialize_optional() -> Vec<u8> {
    serialize(Body::Optional(Some(Box::new(Body::UInt8(u8::MAX)))))
}

fn serialize_bool() -> Vec<u8> {
    serialize(Body::Boolean(true))
}

fn serialize_u8() -> Vec<u8> {
    serialize(Body::UInt8(u8::MAX))
}

fn serialize_u16() -> Vec<u8> {
    serialize(Body::UInt16(u16::MAX))
}

fn serialize_u32() -> Vec<u8> {
    serialize(Body::UInt32(u32::MAX))
}

fn serialize_u64() -> Vec<u8> {
    serialize(Body::UInt64(u64::MAX))
}

// fn serialize_u128() -> Vec<u8> {
//     serialize(Body::UInt128(u128::MAX))
// }

fn serialize_i8() -> Vec<u8> {
    serialize(Body::Int8(i8::MAX))
}

fn serialize_i16() -> Vec<u8> {
    serialize(Body::Int16(i16::MAX))
}

fn serialize_i32() -> Vec<u8> {
    serialize(Body::Int32(i32::MAX))
}

fn serialize_i64() -> Vec<u8> {
    serialize(Body::Int64(i64::MAX))
}

// fn serialize_i128() -> Vec<u8> {
//     serialize(Body::Int128(i128::MAX))
// }

fn serialize_array() -> Vec<u8> {
    serialize(Body::Array(vec![Body::Boolean(true), Body::Boolean(false)]))
}

fn serialize_tuple() -> Vec<u8> {
    serialize(Body::Tuple(vec![Body::Boolean(true), Body::Boolean(false)]))
}

fn serialize_struct() -> Vec<u8> {
    serialize(Body::Struct(vec![
        Body::Boolean(true),
        Body::Boolean(false),
    ]))
}

fn serialize_map() -> Vec<u8> {
    serialize(Body::Map({
        let mut buf = BTreeMap::new();
        buf.insert("a".to_string(), Body::Boolean(true));
        buf.insert("b".to_string(), Body::Boolean(false));
        buf
    }))
}

fn serialize_enum() -> Vec<u8> {
    serialize(Body::Enum(0, Box::new(Body::Boolean(true))))
}

fn serialize_binary() -> Vec<u8> {
    serialize(Body::Binary(ByteBuf::from((0..255).collect::<Vec<u8>>())))
}

fn serialize<T: Serialize>(v: T) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    v.serialize(&mut serializer).unwrap();
    buf
}

main!(
    serialize_unit,
    serialize_optional,
    serialize_bool,
    serialize_u8,
    serialize_u16,
    serialize_u32,
    serialize_u64,
    // serialize_u128,
    serialize_i8,
    serialize_i16,
    serialize_i32,
    serialize_i64,
    // serialize_i128,
    serialize_array,
    serialize_tuple,
    serialize_struct,
    serialize_map,
    serialize_enum,
    serialize_binary,
);
