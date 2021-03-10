use integer_encoding::{VarInt, VarIntReader};
use std::io::Read;

pub mod body;
pub mod deserializer;
pub mod header;
pub mod message;
pub mod serializer;
pub mod stream;

#[inline]
fn serialize_string(v: &str) -> Vec<u8> {
    let mut buf = v.len().encode_var_vec();
    buf.extend(v.as_bytes());
    buf
}

#[inline]
fn deserialize_string<R: Read>(reader: &mut R) -> Result<String, ()> {
    let mut body_buf = new_dynamic_buf(reader.read_varint::<usize>().or(Err(()))?);
    reader.read_exact(&mut body_buf).or(Err(()))?;
    String::from_utf8(body_buf).or(Err(()))
}

#[inline]
fn new_dynamic_buf(len: usize) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(len);
    unsafe {
        buf.set_len(len);
    }
    buf
}
