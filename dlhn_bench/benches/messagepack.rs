use dlhn_bench::Test;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use iai::main;

fn serialize_messagepack() {
    let mut buf = Vec::new();
    Test::default().serialize(&mut Serializer::new(&mut buf).with_struct_map()).unwrap();
}

fn deserialize_messagepack() {
    let buf = [142, 161, 97, 195, 161, 98, 204, 255, 161, 99, 205, 255, 255, 161, 100, 206, 255, 255, 255, 255, 161, 101, 207, 255, 255, 255, 255, 255, 255, 255, 255, 161, 102, 208, 128, 161, 103, 209, 128, 0, 161, 104, 210, 128, 0, 0, 0, 161, 105, 211, 128, 0, 0, 0, 0, 0, 0, 0, 161, 106, 202, 127, 127, 255, 255, 161, 107, 203, 127, 239, 255, 255, 255, 255, 255, 255, 161, 108, 164, 116, 101, 115, 116, 161, 109, 148, 195, 194, 195, 194, 161, 110, 132, 161, 97, 195, 161, 98, 194, 161, 99, 195, 161, 100, 194];
    Test::deserialize(&mut Deserializer::new(buf.as_ref())).unwrap();
}
main!(
    serialize_messagepack,
    deserialize_messagepack,
);
