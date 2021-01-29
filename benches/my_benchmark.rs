use dullahan::{body::Body, header::Header, serializer::serialize};
use iai::main;

fn serialize_uint8() -> Result<Vec<u8>, ()> {
    serialize(&Header::UInt8, &Body::Int8(i8::MAX))
}

fn serialize_int8() -> Result<Vec<u8>, ()> {
    serialize(&Header::Int8, &Body::Int8(i8::MAX))
}

fn serialize_float32() -> Result<Vec<u8>, ()> {
    serialize(&Header::Float32, &Body::Float32(1.23456))
}

fn serialize_float64() -> Result<Vec<u8>, ()> {
    serialize(&Header::Float64, &Body::Float64(1.23456))
}

fn serialize_string() -> Result<Vec<u8>, ()> {
    serialize(&Header::String, &Body::String(String::from("test")))
}

main!(
    serialize_uint8,
    serialize_int8,
    serialize_float32,
    serialize_float64,
    serialize_string
);
