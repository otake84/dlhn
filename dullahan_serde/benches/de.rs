use dullahan_serde::{de::Deserializer, ser::Serializer};
use iai::main;
use serde::{Deserialize, Serialize};

fn decode_leb128_u8() -> u8 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u8::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u8::deserialize(&mut deserializer).unwrap()
}

fn decode_leb128_u16() -> u16 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u16::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u16::deserialize(&mut deserializer).unwrap()
}

fn decode_leb128_u32() -> u32 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u32::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u32::deserialize(&mut deserializer).unwrap()
}

fn decode_leb128_u64() -> u64 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u64::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u64::deserialize(&mut deserializer).unwrap()
}

main!(
    decode_leb128_u8,
    decode_leb128_u16,
    decode_leb128_u32,
    decode_leb128_u64,
);
