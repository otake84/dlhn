use dullahan::{body::Body, header::Header, serializer::serialize_without_validate};
use iai::main;

fn serialize_uint8() -> Vec<u8> {
    serialize_without_validate(&Header::UInt8, &Body::UInt8(u8::MAX))
}

main!(serialize_uint8,);
