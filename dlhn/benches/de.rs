use dlhn::{Deserializer, Serializer};
use iai::main;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

fn deserialize_u8() -> u8 {
    let buf = serialize(u8::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u8::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u16() -> u16 {
    let buf = serialize(u16::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u16::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u32() -> u32 {
    let buf = serialize(u32::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u32::deserialize(&mut deserializer).unwrap()
}

fn deserialize_u64() -> u64 {
    let buf = serialize(u64::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    u64::deserialize(&mut deserializer).unwrap()
}

// fn deserialize_u128() -> u128 {
//     let buf = serialize(u128::MAX);
//     let mut reader = buf.as_slice();
//     let mut deserializer = Deserializer::new(&mut reader);
//     u128::deserialize(&mut deserializer).unwrap()
// }

fn deserialize_i8() -> i8 {
    let buf = serialize(i8::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i8::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i16() -> i16 {
    let buf = serialize(i16::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i16::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i32() -> i32 {
    let buf = serialize(i32::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i32::deserialize(&mut deserializer).unwrap()
}

fn deserialize_i64() -> i64 {
    let buf = serialize(i64::MAX);
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    i64::deserialize(&mut deserializer).unwrap()
}

// fn deserialize_i128() -> i128 {
//     let buf = serialize(i128::MAX);
//     let mut reader = buf.as_slice();
//     let mut deserializer = Deserializer::new(&mut reader);
//     i128::deserialize(&mut deserializer).unwrap()
// }

fn deserialize_char() -> char {
    let buf = serialize('a');
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    char::deserialize(&mut deserializer).unwrap()
}

fn deserialize_string() -> String {
    let buf = serialize("test");
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    String::deserialize(&mut deserializer).unwrap()
}

fn deserialize_byte_buf() -> ByteBuf {
    let buf = serialize(ByteBuf::from(vec![0u8, 1, 2, 3, 255]));
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    ByteBuf::deserialize(&mut deserializer).unwrap()
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
    // deserialize_u128,
    deserialize_i8,
    deserialize_i16,
    deserialize_i32,
    deserialize_i64,
    // deserialize_i128,
    deserialize_char,
    deserialize_string,
    deserialize_byte_buf,
    deserialize_seq,
    deserialize_map,
);
