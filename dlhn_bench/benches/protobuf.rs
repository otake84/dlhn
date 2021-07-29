use std::collections::BTreeMap;
use iai::main;
use prost::Message;
use dlhn_bench::ProtoTest;

fn serialize_protobuf() {
    let mut buf = Vec::new();

    let test = ProtoTest {
        a: true,
        b: u8::MAX as u32,
        c: u16::MAX as u32,
        d: u32::MAX,
        e: u64::MAX,
        f: i8::MIN as i32,
        g: i16::MIN as i32,
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
    };

    buf.reserve(test.encoded_len());

    test.encode(&mut buf).unwrap();
}

fn deserialize_protobuf() {
    let buf = [8, 1, 16, 255, 1, 24, 255, 255, 3, 32, 255, 255, 255, 255, 15, 40, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 48, 255, 1, 56, 255, 255, 3, 64, 255, 255, 255, 255, 15, 72, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 85, 255, 255, 127, 127, 89, 255, 255, 255, 255, 255, 255, 239, 127, 98, 4, 116, 101, 115, 116, 106, 4, 1, 0, 1, 0, 114, 5, 10, 1, 97, 16, 1, 114, 3, 10, 1, 98, 114, 5, 10, 1, 99, 16, 1, 114, 3, 10, 1, 100];
    ProtoTest::decode(&mut buf.as_ref()).unwrap();
}

main!(
    serialize_protobuf,
    deserialize_protobuf,
);
