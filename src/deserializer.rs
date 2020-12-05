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
    fn deserialize_int() {
        assert_eq!(super::deserialize(&[Header::Int.serialize(), 0i8.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(0))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i8::MIN.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i8::MIN as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i8::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i8::MAX as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i16::MIN.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i16::MIN as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i16::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i16::MAX as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i32::MIN.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i32::MIN as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i32::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i32::MAX as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i64::MIN.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i64::MIN as i64))));
        assert_eq!(super::deserialize(&[Header::Int.serialize(), i64::MAX.encode_var_vec()].concat() as &[u8]), Ok((Header::Int, Body::Int(i64::MAX as i64))));
    }

    #[test]
    fn deserialize_int8() {
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), 0i8.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(0))));
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), i8::MIN.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MIN))));
        assert_eq!(super::deserialize(&[Header::Int8.serialize(), i8::MAX.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MAX))));
    }

    #[test]
    fn deserialize_float32() {
        assert_eq!(super::deserialize(&[Header::Float32.serialize(), 0f32.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float32, Body::Float32(0f32))));
        assert_eq!(super::deserialize(&[Header::Float32.serialize(), f32::INFINITY.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float32, Body::Float32(f32::INFINITY))));
        assert_eq!(super::deserialize(&[Header::Float32.serialize(), (-f32::INFINITY).to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float32, Body::Float32(-f32::INFINITY))));
    }

    #[test]
    fn deserialize_float64() {
        assert_eq!(super::deserialize(&[Header::Float64.serialize(), 0f64.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float64, Body::Float64(0f64))));
        assert_eq!(super::deserialize(&[Header::Float64.serialize(), f64::INFINITY.to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float64, Body::Float64(f64::INFINITY))));
        assert_eq!(super::deserialize(&[Header::Float64.serialize(), (-f64::INFINITY).to_le_bytes().to_vec()].concat() as &[u8]), Ok((Header::Float64, Body::Float64(-f64::INFINITY))));
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(super::deserialize(&[Header::String.serialize(), "test".len().encode_var_vec(), "test".as_bytes().to_vec()].concat() as &[u8]), Ok((Header::String, Body::String(String::from("test")))));
    }

    #[test]
    fn deserialize_array() {
        let body = [0u8, 1, 2, u8::MAX];
        assert_eq!(super::deserialize(&[Header::Array(Box::new(Header::UInt8)).serialize(), [body.len().encode_var_vec(), body.iter().flat_map(|v| v.to_le_bytes().to_vec()).collect()].concat()].concat() as &[u8]), Ok((Header::Array(Box::new(Header::UInt8)), Body::Array(body.iter().map(|v| Body::UInt8(*v)).collect()))));

        let body = ["aaaa", "bbbb"];
        assert_eq!(super::deserialize(&[Header::Array(Box::new(Header::String)).serialize(), [body.len().encode_var_vec(), body.iter().flat_map(|v| [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()).collect()].concat()].concat() as &[u8]), Ok((Header::Array(Box::new(Header::String)), Body::Array(body.iter().map(|v| Body::String(v.to_string())).collect()))));
    }
}
