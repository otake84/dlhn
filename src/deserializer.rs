use std::io::{BufReader, Read};
use crate::{body::Body, header::Header};

pub fn deserialize<R: Read>(read: R) -> Result<(Header, Body), ()> {
    let mut buf_reader = BufReader::new(read);

    if let Ok(header) = Header::deserialize(&mut buf_reader) {
        if let Ok(body) = Body::deserialize(&header, &mut buf_reader) {
            Ok((header, body))
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use integer_encoding::VarInt;
    use crate::{body::Body, header::Header};

    #[test]
    fn deserialize_boolean() {
        assert_eq!(super::deserialize([Header::Boolean.serialize(), vec![0]].concat().as_ref() as &[u8]), Ok((Header::Boolean, Body::Boolean(false))));
        assert_eq!(super::deserialize([Header::Boolean.serialize(), vec![1]].concat().as_ref() as &[u8]), Ok((Header::Boolean, Body::Boolean(true))));
    }

    #[test]
    fn deserialize_uint() {
        assert_eq!(super::deserialize(&[Header::UInt.serialize(), 0u8.encode_var_vec()].concat() as &[u8]), Ok((Header::UInt, Body::UInt(0))));
        assert_eq!(super::deserialize(&[Header::UInt.serialize(), u8::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::UInt, Body::UInt(u8::MAX as u64))));
        assert_eq!(super::deserialize(&[Header::UInt.serialize(), u16::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::UInt, Body::UInt(u16::MAX as u64))));
        assert_eq!(super::deserialize(&[Header::UInt.serialize(), u32::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::UInt, Body::UInt(u32::MAX as u64))));
        assert_eq!(super::deserialize(&[Header::UInt.serialize(), u64::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::UInt, Body::UInt(u64::MAX as u64))));
    }

    #[test]
    fn deserialize_uint8() {
        assert_eq!(super::deserialize(&[Header::UInt8.serialize(), 0u8.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::UInt8, Body::UInt8(0))));
        assert_eq!(super::deserialize(&[Header::UInt8.serialize(), 255u8.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::UInt8, Body::UInt8(255))));
    }

    #[test]
    fn deserialize_int8() {
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), 0i8.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(0))));
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), i8::MIN.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MIN))));
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), i8::MAX.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MAX))));
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(super::deserialize(&[Header::String.serialize(), "test".len().encode_var_vec(), "test".as_bytes().to_vec()].concat() as &[u8]), Ok((Header::String, Body::String(String::from("test")))));
    }
}
