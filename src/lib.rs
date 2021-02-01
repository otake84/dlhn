use integer_encoding::{VarInt, VarIntReader};
use std::io::{BufReader, Read};

pub mod binary;
pub mod body;
pub mod deserializer;
pub mod header;
pub mod message;
pub mod serializer;

#[inline]
fn serialize_string(v: &str) -> Vec<u8> {
    let mut buf = v.len().encode_var_vec();
    buf.extend(v.as_bytes());
    buf
}

#[inline]
fn deserialize_string<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, ()> {
    let mut body_buf = vec![0u8; buf_reader.read_varint::<usize>().or(Err(()))?];
    buf_reader.read_exact(&mut body_buf).or(Err(()))?;
    String::from_utf8(body_buf).or(Err(()))
}
