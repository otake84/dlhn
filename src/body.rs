use std::io::{BufReader, Read};
use integer_encoding::{VarInt, VarIntReader};
use crate::header::{BodySize, Header};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
    UInt(u64),
    UInt8(u8),
    Int(i64),
    Int8(i8),
    String(String),
}

impl Body {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Body::Boolean(v) => {
                if *v {
                    vec![1]
                } else {
                    vec![0]
                }
            }
            Body::UInt(v) => {
                v.encode_var_vec()
            }
            Body::UInt8(v) => {
                v.to_le_bytes().to_vec()
            }
            Body::Int(v) => {
                v.encode_var_vec()
            }
            Body::Int8(v) => {
                v.to_le_bytes().to_vec()
            }
            Body::String(v) => {
                [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(header: &Header, buf_reader: &mut BufReader<R>) -> Result<Body, ()> {
        if let BodySize::Fix(size) = header.body_size() {
            let mut body_buf = Vec::with_capacity(size);
            unsafe { body_buf.set_len(size); }
            buf_reader.read_exact(&mut body_buf).or(Err(()))?;

            match header {
                Header::Boolean => {
                    match *body_buf.first().unwrap() {
                        0 => Ok(Body::Boolean(false)),
                        1 => Ok(Body::Boolean(true)),
                        _ => Err(()),
                    }
                }
                Header::UInt8 => {
                    Ok(Body::UInt8(*body_buf.first().unwrap()))
                }
                Header::Int8 => {
                    Ok(Body::Int8(i8::from_le_bytes([*body_buf.first().unwrap()])))
                }
                _ => Err(())
            }
        } else {
            match header {
                Header::UInt => {
                    buf_reader.read_varint::<u64>().map(|v| Body::UInt(v.into())).or(Err(()))
                }
                Header::Int => {
                    buf_reader.read_varint::<i64>().map(|v| Body::Int(v.into())).or(Err(()))
                }
                Header::String => {
                    let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                    let mut body_buf = Vec::with_capacity(size);
                    unsafe { body_buf.set_len(size); }
                    buf_reader.read_exact(&mut body_buf).or(Err(()))?;
                    String::from_utf8(body_buf).map(Body::String).or(Err(()))
                }
                _ => Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use integer_encoding::VarInt;
    use crate::header::Header;
    use super::Body;

    #[test]
    fn deserialize() {
        assert_eq!(super::Body::deserialize(&Header::Boolean, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::Boolean(false)));
        assert_eq!(super::Body::deserialize(&Header::Boolean, &mut BufReader::new(&[1u8] as &[u8])), Ok(Body::Boolean(true)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(0u8.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(0)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u8::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u8::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u16::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u16::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u32::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u32::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u64::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u64::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt8, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::UInt8(0)));
        assert_eq!(super::Body::deserialize(&Header::UInt8, &mut BufReader::new(&[255u8] as &[u8])), Ok(Body::UInt8(255)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(0i8.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(0)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i8::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i8::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i8::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i8::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i16::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i16::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i16::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i16::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i32::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i32::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i32::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i32::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i64::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i64::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i64::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i64::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::Int8(0)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&(-1i8).to_le_bytes() as &[u8])), Ok(Body::Int8(-1)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&i8::MIN.to_le_bytes() as &[u8])), Ok(Body::Int8(i8::MIN)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&i8::MAX.to_le_bytes() as &[u8])), Ok(Body::Int8(i8::MAX)));
        assert_eq!(super::Body::deserialize(&Header::String, &mut BufReader::new(["test".len().encode_var_vec(), "test".as_bytes().to_vec()].concat().as_ref() as &[u8])), Ok(Body::String(String::from("test"))));
        assert_eq!(super::Body::deserialize(&Header::String, &mut BufReader::new(["テスト".len().encode_var_vec(), "テスト".as_bytes().to_vec()].concat().as_ref() as &[u8])), Ok(Body::String(String::from("テスト"))));
    }
}
