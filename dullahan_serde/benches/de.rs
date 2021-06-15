use std::collections::BTreeMap;
use dullahan_serde::{de::Deserializer, ser::Serializer};
use iai::main;
use serde::{Deserialize, Serialize};

fn deserialize_u8() -> u8 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u8::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u8::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u16() -> u16 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u16::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u16::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u32() -> u32 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u32::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u32::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u64() -> u64 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u64::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u64::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u128() -> u128 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    u128::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u128::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i8() -> i8 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i8::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i8::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i16() -> i16 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i16::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i16::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i32() -> i32 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i32::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i32::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i64() -> i64 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i64::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i64::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i128() -> i128 {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    i128::MAX.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i128::deserialize(&mut deserializer).unwrap()
}

fn deserialize_seq() -> Vec<bool> {
    let buf = serialize(vec![true, false, true]);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    Vec::<bool>::deserialize(&mut deserializer).unwrap()
}

fn deserialize_map() -> BTreeMap<String, bool> {
    let buf = serialize({
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), true);
        map.insert("b".to_string(), false);
        map.insert("c".to_string(), true);
        map.insert("1".to_string(), false);
        map
    });
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    BTreeMap::<String, bool>::deserialize(&mut deserializer).unwrap()
}

fn serialize<T: Serialize>(v: T) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    v.serialize(&mut serializer).unwrap();
    buf
}

main!(
    deserialize_u8,
    deserialize_u16,
    deserialize_u32,
    deserialize_u64,
    deserialize_u128,
    deserialize_i8,
    deserialize_i16,
    deserialize_i32,
    deserialize_i64,
    deserialize_i128,
    deserialize_seq,
    deserialize_map,
);
