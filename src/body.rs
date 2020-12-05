use std::{convert::TryInto, io::{BufReader, Read}};
use integer_encoding::{VarInt, VarIntReader};
use crate::header::{BodySize, Header};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
    UInt(u64),
    UInt8(u8),
    Int(i64),
    Int8(i8),
    Float32(f32),
    Float64(f64),
    String(String),
    Array(Vec<Body>),
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
            Body::Float32(v) => {
                v.to_le_bytes().to_vec()
            }
            Body::Float64(v) => {
                v.to_le_bytes().to_vec()
            }
            Body::String(v) => {
                [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()
            }
            Body::Array(v) => {
                let items = v.iter().flat_map(|v| v.serialize()).collect::<Vec<u8>>();
                [v.len().encode_var_vec(), items].concat()
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
                Header::Float32 => {
                    let bytes = body_buf.try_into().or(Err(()))?;
                    Ok(Body::Float32(f32::from_le_bytes(bytes)))
                }
                Header::Float64 => {
                    let bytes = body_buf.try_into().or(Err(()))?;
                    Ok(Body::Float64(f64::from_le_bytes(bytes)))
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
                Header::Array(inner_header) => {
                    let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                    let mut body = Vec::with_capacity(size);
                    for _ in 0..size {
                        body.push(Self::deserialize(inner_header, buf_reader)?);
                    }
                    Ok(Body::Array(body))
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
    fn deserialize_boolean() {
        assert_eq!(super::Body::deserialize(&Header::Boolean, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::Boolean(false)));
        assert_eq!(super::Body::deserialize(&Header::Boolean, &mut BufReader::new(&[1u8] as &[u8])), Ok(Body::Boolean(true)));
    }

    #[test]
    fn deserialize_uint() {
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(0u8.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(0)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u8::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u8::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u16::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u16::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u32::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u32::MAX as u64)));
        assert_eq!(super::Body::deserialize(&Header::UInt, &mut BufReader::new(u64::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::UInt(u64::MAX as u64)));
    }

    #[test]
    fn deserialize_uint8() {
        assert_eq!(super::Body::deserialize(&Header::UInt8, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::UInt8(0)));
        assert_eq!(super::Body::deserialize(&Header::UInt8, &mut BufReader::new(&[255u8] as &[u8])), Ok(Body::UInt8(255)));
    }

    #[test]
    fn deserialize_int() {
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(0i8.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(0)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i8::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i8::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i8::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i8::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i16::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i16::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i16::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i16::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i32::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i32::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i32::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i32::MAX as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i64::MIN.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i64::MIN as i64)));
        assert_eq!(super::Body::deserialize(&Header::Int, &mut BufReader::new(i64::MAX.encode_var_vec().as_ref() as &[u8])), Ok(Body::Int(i64::MAX as i64)));
    }

    #[test]
    fn deserialize_int8() {
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&[0u8] as &[u8])), Ok(Body::Int8(0)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&(-1i8).to_le_bytes() as &[u8])), Ok(Body::Int8(-1)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&i8::MIN.to_le_bytes() as &[u8])), Ok(Body::Int8(i8::MIN)));
        assert_eq!(super::Body::deserialize(&Header::Int8, &mut BufReader::new(&i8::MAX.to_le_bytes() as &[u8])), Ok(Body::Int8(i8::MAX)));
    }

    #[test]
    fn deserialize_float32() {
        assert_eq!(super::Body::deserialize(&Header::Float32, &mut BufReader::new(&0f32.to_le_bytes() as &[u8])), Ok(Body::Float32(0f32)));
        assert_eq!(super::Body::deserialize(&Header::Float32, &mut BufReader::new(&1.1f32.to_le_bytes() as &[u8])), Ok(Body::Float32(1.1f32)));
        assert_eq!(super::Body::deserialize(&Header::Float32, &mut BufReader::new(&(-1.1f32).to_le_bytes() as &[u8])), Ok(Body::Float32(-1.1f32)));
        assert_eq!(super::Body::deserialize(&Header::Float32, &mut BufReader::new(&f32::INFINITY.to_le_bytes() as &[u8])), Ok(Body::Float32(f32::INFINITY)));
        assert_eq!(super::Body::deserialize(&Header::Float32, &mut BufReader::new(&(-f32::INFINITY).to_le_bytes() as &[u8])), Ok(Body::Float32(-f32::INFINITY)));
    }

    #[test]
    fn deserialize_float64() {
        assert_eq!(super::Body::deserialize(&Header::Float64, &mut BufReader::new(&0f64.to_le_bytes() as &[u8])), Ok(Body::Float64(0f64)));
        assert_eq!(super::Body::deserialize(&Header::Float64, &mut BufReader::new(&1.1f64.to_le_bytes() as &[u8])), Ok(Body::Float64(1.1f64)));
        assert_eq!(super::Body::deserialize(&Header::Float64, &mut BufReader::new(&(-1.1f64).to_le_bytes() as &[u8])), Ok(Body::Float64(-1.1f64)));
        assert_eq!(super::Body::deserialize(&Header::Float64, &mut BufReader::new(&f64::INFINITY.to_le_bytes() as &[u8])), Ok(Body::Float64(f64::INFINITY)));
        assert_eq!(super::Body::deserialize(&Header::Float64, &mut BufReader::new(&(-f64::INFINITY).to_le_bytes() as &[u8])), Ok(Body::Float64(-f64::INFINITY)));
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(super::Body::deserialize(&Header::String, &mut BufReader::new(["test".len().encode_var_vec(), "test".as_bytes().to_vec()].concat().as_ref() as &[u8])), Ok(Body::String(String::from("test"))));
        assert_eq!(super::Body::deserialize(&Header::String, &mut BufReader::new(["テスト".len().encode_var_vec(), "テスト".as_bytes().to_vec()].concat().as_ref() as &[u8])), Ok(Body::String(String::from("テスト"))));
    }

    #[test]
    fn deserialize_array() {
        let body = [0u8, 1, 2, u8::MAX];
        assert_eq!(super::Body::deserialize(&Header::Array(Box::new(Header::UInt8)), &mut BufReader::new([body.len().encode_var_vec(), body.iter().flat_map(|v| v.to_le_bytes().to_vec()).collect()].concat().as_ref() as &[u8])), Ok(Body::Array(vec![Body::UInt8(0), Body::UInt8(1), Body::UInt8(2), Body::UInt8(u8::MAX)])));

        let body = ["aaaa", "bbbb"];
        assert_eq!(super::Body::deserialize(&Header::Array(Box::new(Header::String)), &mut BufReader::new([body.len().encode_var_vec(), body.iter().flat_map(|v| [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()).collect()].concat().as_ref() as &[u8])), Ok(Body::Array(vec![Body::String(String::from("aaaa")), Body::String(String::from("bbbb"))])));
    }
}
