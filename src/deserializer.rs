use crate::{body::Body, header::Header};
use std::io::{BufReader, Read};

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
    use crate::{binary::Binary, body::Body, header::Header, serializer::serialize};
    use core::panic;
    use indexmap::*;
    use integer_encoding::VarInt;
    use std::{collections::HashMap, iter};
    use time::{Date, OffsetDateTime};

    #[test]
    fn deserialize_optional() {
        let (header, body) = (
            Header::Optional(Box::new(Header::Boolean)),
            Body::Optional(Box::new(Some(Body::Boolean(true)))),
        );
        assert_eq!(
            super::deserialize(serialize(&header, &body).unwrap().as_slice()),
            Ok((header, body))
        );

        let (header, body) = (
            Header::Optional(Box::new(Header::Boolean)),
            Body::Optional(Box::new(None)),
        );
        assert_eq!(
            super::deserialize(serialize(&header, &body).unwrap().as_slice()),
            Ok((header, body))
        );

        let (header, body) = (
            Header::Optional(Box::new(Header::String)),
            Body::Optional(Box::new(Some(Body::String(String::from("test"))))),
        );
        assert_eq!(
            super::deserialize(serialize(&header, &body).unwrap().as_slice()),
            Ok((header, body))
        );
    }

    #[test]
    fn deserialize_boolean() {
        assert_eq!(
            super::deserialize([Header::Boolean.serialize(), vec![0]].concat().as_slice()),
            Ok((Header::Boolean, Body::Boolean(false)))
        );
        assert_eq!(
            super::deserialize([Header::Boolean.serialize(), vec![1]].concat().as_slice()),
            Ok((Header::Boolean, Body::Boolean(true)))
        );
    }

    #[test]
    fn deserialize_uint() {
        assert_eq!(
            super::deserialize(
                [Header::UInt.serialize(), 0u8.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt, Body::UInt(0)))
        );
        assert_eq!(
            super::deserialize(
                [Header::UInt.serialize(), u8::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt, Body::UInt(u8::MAX as u64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::UInt.serialize(), u16::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt, Body::UInt(u16::MAX as u64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::UInt.serialize(), u32::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt, Body::UInt(u32::MAX as u64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::UInt.serialize(), u64::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt, Body::UInt(u64::MAX as u64)))
        );
    }

    #[test]
    fn deserialize_uint8() {
        assert_eq!(
            super::deserialize(
                [Header::UInt8.serialize(), 0u8.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt8, Body::UInt8(0)))
        );
        assert_eq!(
            super::deserialize(
                [Header::UInt8.serialize(), 255u8.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::UInt8, Body::UInt8(255)))
        );
    }

    #[test]
    fn deserialize_int() {
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), 0i8.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(0)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i8::MIN.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i8::MIN as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i8::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i8::MAX as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i16::MIN.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i16::MIN as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i16::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i16::MAX as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i32::MIN.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i32::MIN as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i32::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i32::MAX as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i64::MIN.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i64::MIN as i64)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int.serialize(), i64::MAX.encode_var_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int, Body::Int(i64::MAX as i64)))
        );
    }

    #[test]
    fn deserialize_int8() {
        assert_eq!(
            super::deserialize(
                [Header::Int8.serialize(), 0i8.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int8, Body::Int8(0)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int8.serialize(), i8::MIN.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int8, Body::Int8(i8::MIN)))
        );
        assert_eq!(
            super::deserialize(
                [Header::Int8.serialize(), i8::MAX.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Int8, Body::Int8(i8::MAX)))
        );
    }

    #[test]
    fn deserialize_float32() {
        assert_eq!(
            super::deserialize(
                [Header::Float32.serialize(), 0f32.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Float32, Body::Float32(0f32)))
        );
        assert_eq!(
            super::deserialize(
                [
                    Header::Float32.serialize(),
                    f32::INFINITY.to_le_bytes().to_vec()
                ]
                .concat()
                .as_slice()
            ),
            Ok((Header::Float32, Body::Float32(f32::INFINITY)))
        );
        assert_eq!(
            super::deserialize(
                [
                    Header::Float32.serialize(),
                    (-f32::INFINITY).to_le_bytes().to_vec()
                ]
                .concat()
                .as_slice()
            ),
            Ok((Header::Float32, Body::Float32(-f32::INFINITY)))
        );
    }

    #[test]
    fn deserialize_float64() {
        assert_eq!(
            super::deserialize(
                [Header::Float64.serialize(), 0f64.to_le_bytes().to_vec()]
                    .concat()
                    .as_slice()
            ),
            Ok((Header::Float64, Body::Float64(0f64)))
        );
        assert_eq!(
            super::deserialize(
                [
                    Header::Float64.serialize(),
                    f64::INFINITY.to_le_bytes().to_vec()
                ]
                .concat()
                .as_slice()
            ),
            Ok((Header::Float64, Body::Float64(f64::INFINITY)))
        );
        assert_eq!(
            super::deserialize(
                [
                    Header::Float64.serialize(),
                    (-f64::INFINITY).to_le_bytes().to_vec()
                ]
                .concat()
                .as_slice()
            ),
            Ok((Header::Float64, Body::Float64(-f64::INFINITY)))
        );
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(
            super::deserialize(
                [
                    Header::String.serialize(),
                    "test".len().encode_var_vec(),
                    "test".as_bytes().to_vec()
                ]
                .concat()
                .as_slice()
            ),
            Ok((Header::String, Body::String(String::from("test"))))
        );
    }

    #[test]
    fn deserialize_binary() {
        assert_eq!(
            super::deserialize(
                serialize(
                    &Header::Binary,
                    &Body::Binary(Binary(vec![0, 1, 2, 3, 255]))
                )
                .unwrap()
                .as_slice()
            ),
            Ok((Header::Binary, Body::Binary(Binary(vec![0, 1, 2, 3, 255]))))
        );
        assert_eq!(
            super::deserialize(
                serialize(
                    &Header::Binary,
                    &Body::Binary(Binary(
                        iter::repeat(255u8).take(u16::MAX as usize).collect()
                    ))
                )
                .unwrap()
                .as_slice()
            ),
            Ok((
                Header::Binary,
                Body::Binary(Binary(
                    iter::repeat(255u8).take(u16::MAX as usize).collect()
                ))
            ))
        );
    }

    #[test]
    fn deserialize_array() {
        let body = [0u8, 1, 2, u8::MAX];
        assert_eq!(
            super::deserialize(
                [
                    Header::Array(Box::new(Header::UInt8)).serialize(),
                    [
                        body.len().encode_var_vec(),
                        body.iter().flat_map(|v| v.to_le_bytes().to_vec()).collect()
                    ]
                    .concat()
                ]
                .concat()
                .as_slice()
            ),
            Ok((
                Header::Array(Box::new(Header::UInt8)),
                Body::Array(body.iter().map(|v| Body::UInt8(*v)).collect())
            ))
        );

        let body = ["aaaa", "bbbb"];
        assert_eq!(super::deserialize([Header::Array(Box::new(Header::String)).serialize(), [body.len().encode_var_vec(), body.iter().flat_map(|v| [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()).collect()].concat()].concat().as_slice()), Ok((Header::Array(Box::new(Header::String)), Body::Array(body.iter().map(|v| Body::String(v.to_string())).collect()))));
    }

    #[test]
    fn deserialize_map() {
        let header = Header::Map(
            indexmap! { String::from("test") => Header::String, String::from("test2") => Header::Boolean },
        );
        let body: IndexMap<String, Body> = indexmap! { String::from("test") => Body::String(String::from("aaaa")), String::from("test2") => Body::Boolean(true) };
        assert_eq!(
            super::deserialize(
                [
                    header.serialize(),
                    body.iter()
                        .flat_map(|v| [if let Body::String(v) = v.1 {
                            [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()
                        } else if let Body::Boolean(v) = v.1 {
                            if *v {
                                vec![1u8]
                            } else {
                                vec![0u8]
                            }
                        } else {
                            panic!()
                        }]
                        .concat())
                        .collect()
                ]
                .concat()
                .as_slice()
            ),
            Ok((header, Body::Map(body)))
        );
    }

    #[test]
    fn deserialize_dynamic_map() {
        let header = Header::DynamicMap(Box::new(Header::Boolean));
        let body = Body::DynamicMap({
            let mut body = HashMap::new();
            body.insert(String::from("test"), Body::Boolean(true));
            body
        });
        assert_eq!(
            super::deserialize(serialize(&header, &body).unwrap().as_slice()),
            Ok((header, body))
        );
    }

    #[test]
    fn deserialize_timestamp() {
        let body = OffsetDateTime::unix_epoch();
        assert_eq!(
            super::deserialize(
                serialize(&Header::Timestamp, &Body::Timestamp(body))
                    .unwrap()
                    .as_slice()
            ),
            Ok((Header::Timestamp, Body::Timestamp(body)))
        );
    }

    #[test]
    fn deserialize_date() {
        let body = Date::try_from_yo(2000, 1).unwrap();
        assert_eq!(
            super::deserialize(
                serialize(&Header::Date, &Body::Date(body))
                    .unwrap()
                    .as_slice()
            ),
            Ok((Header::Date, Body::Date(body)))
        );
    }
}
