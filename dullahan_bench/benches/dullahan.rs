use std::collections::BTreeMap;

use dullahan_serde::{de::Deserializer, ser::Serializer};
use iai::main;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Test {
    a: bool,
    b: u8,
    c: u16,
    d: u32,
    e: u64,
    f: i8,
    g: i16,
    h: i32,
    i: i64,
    j: f32,
    k: f64,
    l: String,
    m: Vec<bool>,
    n: BTreeMap<String, bool>,
}

fn serialize() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    testdata().serialize(&mut serializer).unwrap();
}

fn deserialize() {
    let buf = [1u8, 255, 255, 255, 3, 255, 255, 255, 255, 15, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 128, 255, 255, 3, 255, 255, 255, 255, 15, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 255, 255, 127, 127, 255, 255, 255, 255, 255, 255, 239, 127, 4, 116, 101, 115, 116, 4, 1, 0, 1, 0, 4, 1, 97, 1, 1, 98, 0, 1, 99, 1, 1, 100, 0];
    let mut reader = buf.as_ref();
    let mut deserializer = Deserializer::new(&mut reader);
    Test::deserialize(&mut deserializer).unwrap();
}

fn testdata() -> Test {
    Test {
        a: true,
        b: u8::MAX,
        c: u16::MAX,
        d: u32::MAX,
        e: u64::MAX,
        f: i8::MIN,
        g: i16::MIN,
        h: i32::MIN,
        i: i64::MIN,
        j: f32::MAX,
        k: f64::MAX,
        l: "test".to_string(),
        m: vec![true, false, true, false],
        n: {
            let mut buf = BTreeMap::new();
            buf.insert("a".to_string(), true);
            buf.insert("b".to_string(), false);
            buf.insert("c".to_string(), true);
            buf.insert("d".to_string(), false);
            buf
        },
    }
}

main!(
    serialize,
    deserialize,
);
