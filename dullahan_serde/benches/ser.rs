use dullahan_serde::ser::Serializer;
use iai::main;
use serde::Serialize;

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

main!(
    serialize_u8,
    serialize_u16,
    serialize_u32,
    serialize_u64,
    serialize_u128,
);
