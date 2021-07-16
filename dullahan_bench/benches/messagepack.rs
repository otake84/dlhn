use dullahan_bench::Test;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use iai::main;

fn serialize_messagepack() {
    let mut buf = Vec::new();
    Test::default().serialize(&mut Serializer::new(&mut buf)).unwrap();
}

fn deserialize_messagepack() {
    let buf = [158, 195, 204, 255, 205, 255, 255, 206, 255, 255, 255, 255, 207, 255, 255, 255, 255, 255, 255, 255, 255, 208, 128, 209, 128, 0, 210, 128, 0, 0, 0, 211, 128, 0, 0, 0, 0, 0, 0, 0, 202, 127, 127, 255, 255, 203, 127, 239, 255, 255, 255, 255, 255, 255, 164, 116, 101, 115, 116, 148, 195, 194, 195, 194, 132, 161, 97, 195, 161, 98, 194, 161, 99, 195, 161, 100, 194];
    Test::deserialize(&mut Deserializer::new(buf.as_ref())).unwrap();
}
main!(
    serialize_messagepack,
    deserialize_messagepack,
);
