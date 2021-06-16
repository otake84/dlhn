use dullahan_serde::ser::Serializer;
use iai::main;
use serde::Serialize;
use serde_bytes::Bytes;

fn serialize_bool() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    true.serialize(&mut serializer).unwrap();
}

fn serialize_u8() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u8::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_u16() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u16::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_u32() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u32::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_u64() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u64::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_u128() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u128::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_i8() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i8::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_i16() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i16::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_i32() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i32::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_i64() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i64::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_i128() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i128::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_f32() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    f32::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_f64() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    f64::MAX.serialize(&mut serializer).unwrap();
}

fn serialize_char() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    'a'.serialize(&mut serializer).unwrap();
}

fn serialize_bytes() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Bytes::new(&[0u8, 1, 2, 3, 255]);
    body.serialize(&mut serializer).unwrap();
}

fn serialize_seq() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = vec![true, false, true];
    body.serialize(&mut serializer).unwrap();
}

main!(
    serialize_bool,
    serialize_u8,
    serialize_u16,
    serialize_u32,
    serialize_u64,
    serialize_u128,
    serialize_i8,
    serialize_i16,
    serialize_i32,
    serialize_i64,
    serialize_i128,
    serialize_f32,
    serialize_f64,
    serialize_char,
    serialize_bytes,
    serialize_seq,
);
