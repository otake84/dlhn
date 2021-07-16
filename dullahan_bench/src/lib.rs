use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

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

#[test]
fn json() {
    let buf = [123, 34, 97, 34, 58, 116, 114, 117, 101, 44, 34, 98, 34, 58, 50, 53, 53, 44, 34, 99, 34, 58, 54, 53, 53, 51, 53, 44, 34, 100, 34, 58, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 44, 34, 101, 34, 58, 49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 53, 44, 34, 102, 34, 58, 45, 49, 50, 56, 44, 34, 103, 34, 58, 45, 51, 50, 55, 54, 56, 44, 34, 104, 34, 58, 45, 50, 49, 52, 55, 52, 56, 51, 54, 52, 56, 44, 34, 105, 34, 58, 45, 57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56, 44, 34, 106, 34, 58, 51, 46, 52, 48, 50, 56, 50, 51, 53, 101, 51, 56, 44, 34, 107, 34, 58, 49, 46, 55, 57, 55, 54, 57, 51, 49, 51, 52, 56, 54, 50, 51, 49, 53, 55, 101, 51, 48, 56, 44, 34, 108, 34, 58, 34, 116, 101, 115, 116, 34, 44, 34, 109, 34, 58, 91, 116, 114, 117, 101, 44, 102, 97, 108, 115, 101, 44, 116, 114, 117, 101, 44, 102, 97, 108, 115, 101, 93, 44, 34, 110, 34, 58, 123, 34, 97, 34, 58, 116, 114, 117, 101, 44, 34, 98, 34, 58, 102, 97, 108, 115, 101, 44, 34, 99, 34, 58, 116, 114, 117, 101, 44, 34, 100, 34, 58, 102, 97, 108, 115, 101, 125, 125];
    let v: Test = serde_json::from_reader(buf.as_ref()).unwrap();
    println!("{:?}", v);
}
