use integer_encoding::VarInt;

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
