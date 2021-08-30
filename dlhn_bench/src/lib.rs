#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Test {
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

impl Default for Test {
    fn default() -> Self {
        Self {
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
}

include!(concat!(env!("OUT_DIR"), "/proto_test.rs"));
