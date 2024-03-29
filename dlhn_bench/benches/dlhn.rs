use dlhn::{Deserializer, Serializer};
use dlhn_bench::Test;
use iai::main;
use serde::{Deserialize, Serialize};

fn serialize_dlhn() {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    Test::default().serialize(&mut serializer).unwrap();
}

fn deserialize_dlhn() {
    let buf = [
        1u8, 255, 254, 255, 0, 247, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        128, 254, 255, 0, 247, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 127, 127, 255, 255, 255, 255, 255, 255, 239, 127, 4, 116, 101, 115, 116, 4, 1, 0, 1,
        0, 4, 1, 97, 1, 1, 98, 0, 1, 99, 1, 1, 100, 0,
    ];
    let mut reader = buf.as_ref();
    let mut deserializer = Deserializer::new(&mut reader);
    Test::deserialize(&mut deserializer).unwrap();
}

main!(serialize_dlhn, deserialize_dlhn,);
