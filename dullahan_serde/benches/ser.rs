use dullahan_serde::ser::Serializer;
use iai::main;
use serde::Serialize;
use serde_bytes::Bytes;

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
    serialize_u8,
    serialize_u16,
    serialize_u32,
    serialize_u64,
    serialize_u128,
    serialize_bytes,
    serialize_seq,
);
