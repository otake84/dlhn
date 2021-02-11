use dullahan::{body::Body, serializer::serialize_body};
use iai::main;

fn serialize_body_uint8() -> Vec<u8> {
    serialize_body(&Body::UInt8(u8::MAX))
}

main!(serialize_body_uint8,);
