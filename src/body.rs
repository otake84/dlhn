use crate::{
    binary::Binary,
    header::{BodySize, Header},
};
use indexmap::IndexMap;
use integer_encoding::{VarInt, VarIntReader};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    io::{BufReader, Read, Write},
};
use time::{Date, NumericalDuration, OffsetDateTime};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Optional(Box<Option<Body>>),
    Boolean(bool),
    UInt(u64),
    UInt8(u8),
    Int(i64),
    Int8(i8),
    Float32(f32),
    Float64(f64),
    String(String),
    Binary(Binary),
    Array(Vec<Body>),
    Map(IndexMap<String, Body>),
    DynamicMap(HashMap<String, Body>),
    Timestamp(OffsetDateTime),
    Date(Date),
}

impl Body {
    const DATE_OFFSET: i32 = 2000;

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Optional(v) => {
                if let Some(v) = &**v {
                    vec![[1u8].as_ref(), v.serialize().as_slice()].concat()
                } else {
                    vec![0]
                }
            }
            Self::Boolean(v) => {
                if *v {
                    vec![1]
                } else {
                    vec![0]
                }
            }
            Self::UInt(v) => v.encode_var_vec(),
            Self::UInt8(v) => v.to_le_bytes().to_vec(),
            Self::Int(v) => v.encode_var_vec(),
            Self::Int8(v) => v.to_le_bytes().to_vec(),
            Self::Float32(v) => v.to_le_bytes().to_vec(),
            Self::Float64(v) => v.to_le_bytes().to_vec(),
            Self::String(v) => Self::serialize_string(v),
            Self::Binary(v) => [v.0.len().encode_var_vec().as_ref(), v.0.as_slice()].concat(),
            Self::Array(v) => {
                let items = v.iter().flat_map(|v| v.serialize()).collect::<Vec<u8>>();
                [v.len().encode_var_vec(), items].concat()
            }
            Self::Map(v) => v.iter().flat_map(|v| v.1.serialize()).collect::<Vec<u8>>(),
            Self::DynamicMap(v) => [
                v.len().encode_var_vec(),
                v.iter()
                    .flat_map(|(k, v)| [Self::serialize_string(k), v.serialize()].concat())
                    .collect(),
            ]
            .concat(),
            Self::Timestamp(v) => {
                let kind_size = 1;

                if v.unix_timestamp() >> 34 == 0 {
                    let v = (u64::from(v.nanosecond()) << 34) | (v.unix_timestamp() as u64);

                    if v & 0xff_ff_ff_ff_00_00_00_00 == 0 {
                        let mut buf =
                            Vec::with_capacity(kind_size + TimestampSize::Timestamp32 as usize);
                        buf.write(&(TimestampSize::Timestamp32 as u8).to_le_bytes())
                            .unwrap();
                        buf.write(&(v as u32).to_le_bytes()).unwrap();
                        buf
                    } else {
                        let mut buf =
                            Vec::with_capacity(kind_size + TimestampSize::Timestamp64 as usize);
                        buf.write(&(TimestampSize::Timestamp64 as u8).to_le_bytes())
                            .unwrap();
                        buf.write(&v.to_le_bytes()).unwrap();
                        buf
                    }
                } else {
                    let mut buf =
                        Vec::with_capacity(kind_size + TimestampSize::Timestamp96 as usize);
                    buf.write(&(TimestampSize::Timestamp96 as u8).to_le_bytes())
                        .unwrap();
                    buf.write(&v.time().nanosecond().to_le_bytes()).unwrap();
                    buf.write(&v.unix_timestamp().to_le_bytes()).unwrap();
                    buf
                }
            }
            Self::Date(v) => [
                (v.year() - Self::DATE_OFFSET).encode_var_vec(),
                (v.ordinal() - 1).encode_var_vec(),
            ]
            .concat(),
        }
    }

    pub(crate) fn deserialize<R: Read>(
        header: &Header,
        buf_reader: &mut BufReader<R>,
    ) -> Result<Body, ()> {
        if let BodySize::Fix(size) = header.body_size() {
            let mut body_buf = vec![0u8; size];
            buf_reader.read_exact(&mut body_buf).or(Err(()))?;

            match header {
                Header::Boolean => match *body_buf.first().unwrap() {
                    0 => Ok(Self::Boolean(false)),
                    1 => Ok(Self::Boolean(true)),
                    _ => Err(()),
                },
                Header::UInt8 => Ok(Self::UInt8(u8::from_le_bytes([*body_buf.first().unwrap()]))),
                Header::Int8 => Ok(Self::Int8(i8::from_le_bytes([*body_buf.first().unwrap()]))),
                Header::Float32 => {
                    let bytes = body_buf.try_into().or(Err(()))?;
                    Ok(Self::Float32(f32::from_le_bytes(bytes)))
                }
                Header::Float64 => {
                    let bytes = body_buf.try_into().or(Err(()))?;
                    Ok(Self::Float64(f64::from_le_bytes(bytes)))
                }
                _ => Err(()),
            }
        } else {
            match header {
                Header::Optional(inner_header) => {
                    let mut buf = [0u8; 1];
                    buf_reader.read_exact(&mut buf).or(Err(()))?;
                    if buf.first() == Some(&1) {
                        Ok(Self::Optional(Box::new(Some(Self::deserialize(
                            inner_header,
                            buf_reader,
                        )?))))
                    } else {
                        Ok(Self::Optional(Box::new(None)))
                    }
                }
                Header::UInt => buf_reader
                    .read_varint::<u64>()
                    .map(|v| Self::UInt(v.into()))
                    .or(Err(())),
                Header::Int => buf_reader
                    .read_varint::<i64>()
                    .map(|v| Self::Int(v.into()))
                    .or(Err(())),
                Header::String => Self::deserialize_string(buf_reader).map(Self::String),
                Header::Binary => {
                    let mut body_buf = vec![0u8; buf_reader.read_varint::<usize>().or(Err(()))?];
                    buf_reader.read_exact(&mut body_buf).or(Err(()))?;
                    Ok(Self::Binary(Binary(body_buf)))
                }
                Header::Array(inner_header) => {
                    let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                    let mut body = Vec::with_capacity(size);
                    for _ in 0..size {
                        body.push(Self::deserialize(inner_header, buf_reader)?);
                    }
                    Ok(Self::Array(body))
                }
                Header::Map(inner_header) => {
                    let mut body: IndexMap<String, Body> =
                        IndexMap::with_capacity(inner_header.len());
                    for (key, h) in inner_header.iter() {
                        body.insert(key.clone(), Self::deserialize(h, buf_reader)?);
                    }
                    Ok(Self::Map(body))
                }
                Header::DynamicMap(inner_header) => {
                    let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                    let mut body = HashMap::with_capacity(size);
                    for _ in 0..size {
                        let key = Self::deserialize_string(buf_reader)?;
                        let value = Self::deserialize(inner_header, buf_reader)?;
                        body.insert(key, value);
                    }
                    Ok(Self::DynamicMap(body))
                }
                Header::Timestamp => {
                    let mut kind_buf = [0u8; 1];
                    buf_reader.read_exact(&mut kind_buf).or(Err(()))?;

                    match TimestampSize::try_from(u8::from_le_bytes(kind_buf)) {
                        Ok(TimestampSize::Timestamp32) => {
                            let mut second_buf = [0u8; TimestampSize::Timestamp32 as usize];
                            buf_reader.read_exact(&mut second_buf).or(Err(()))?;

                            Ok(Self::Timestamp(
                                OffsetDateTime::unix_epoch()
                                    + u32::from_le_bytes(second_buf).seconds(),
                            ))
                        }
                        Ok(TimestampSize::Timestamp64) => {
                            let mut nanosecond_and_second_buf =
                                [0u8; TimestampSize::Timestamp64 as usize];
                            buf_reader
                                .read_exact(&mut nanosecond_and_second_buf)
                                .or(Err(()))?;

                            let value = u64::from_le_bytes(nanosecond_and_second_buf);
                            let nanosecond = value >> 34;
                            let second = value & 0x00_00_00_03_ff_ff_ff_ff;
                            Ok(Self::Timestamp(
                                OffsetDateTime::from_unix_timestamp(second as i64)
                                    + (nanosecond as u32).nanoseconds(),
                            ))
                        }
                        Ok(TimestampSize::Timestamp96) => {
                            let mut nanosecond_buf = [0u8; 4];
                            buf_reader.read_exact(&mut nanosecond_buf).or(Err(()))?;
                            let nanosecond = u32::from_le_bytes(nanosecond_buf);

                            let mut unix_timestamp_buf = [0u8; 8];
                            buf_reader.read_exact(&mut unix_timestamp_buf).or(Err(()))?;
                            let unix_timestamp = i64::from_le_bytes(unix_timestamp_buf);

                            Ok(Self::Timestamp(
                                OffsetDateTime::from_unix_timestamp(unix_timestamp)
                                    + nanosecond.nanoseconds(),
                            ))
                        }
                        Err(_) => Err(()),
                    }
                }
                Header::Date => {
                    let year = buf_reader.read_varint::<i32>().or(Err(()))? + Self::DATE_OFFSET;
                    let ordinal = buf_reader.read_varint::<u16>().or(Err(()))? + 1;
                    let date = Date::try_from_yo(year, ordinal).or(Err(()))?;

                    Ok(Self::Date(date))
                }
                _ => Err(()),
            }
        }
    }

    fn serialize_string(v: &str) -> Vec<u8> {
        [v.len().encode_var_vec().as_ref(), v.as_bytes()].concat()
    }

    fn deserialize_string<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, ()> {
        let mut body_buf = vec![0u8; buf_reader.read_varint::<usize>().or(Err(()))?];
        buf_reader.read_exact(&mut body_buf).or(Err(()))?;
        String::from_utf8(body_buf).or(Err(()))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TimestampSize {
    Timestamp32 = 4,
    Timestamp64 = 8,
    Timestamp96 = 12,
}

impl TryFrom<u8> for TimestampSize {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == TimestampSize::Timestamp32 as u8 => Ok(TimestampSize::Timestamp32),
            v if v == TimestampSize::Timestamp64 as u8 => Ok(TimestampSize::Timestamp64),
            v if v == TimestampSize::Timestamp96 as u8 => Ok(TimestampSize::Timestamp96),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Body, TimestampSize};
    use crate::{binary::Binary, header::Header};
    use core::panic;
    use indexmap::*;
    use integer_encoding::VarInt;
    use std::{collections::HashMap, io::BufReader};
    use time::{Date, NumericalDuration, OffsetDateTime};

    #[test]
    fn serialize_timestamp32() {
        assert_eq!(
            Body::Timestamp(OffsetDateTime::unix_epoch()).serialize(),
            [TimestampSize::Timestamp32 as u8, 0, 0, 0, 0]
        );
        assert_eq!(
            Body::Timestamp(OffsetDateTime::from_unix_timestamp(u32::MAX as i64)).serialize(),
            [TimestampSize::Timestamp32 as u8, 255, 255, 255, 255]
        );
    }

    #[test]
    fn serialize_timestamp64() {
        assert_eq!(
            Body::Timestamp(OffsetDateTime::unix_epoch() + 1.nanoseconds()).serialize(),
            [TimestampSize::Timestamp64 as u8, 0, 0, 0, 0, 4, 0, 0, 0]
        );
        assert_eq!(
            Body::Timestamp(
                OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                    + 999.milliseconds()
                    + 999.microseconds()
                    + 999.nanoseconds()
            )
            .serialize(),
            [
                TimestampSize::Timestamp64 as u8,
                255,
                255,
                255,
                255,
                255,
                39,
                107,
                238
            ]
        );
    }

    #[test]
    fn serialize_timestamp96() {
        assert_eq!(
            Body::Timestamp(
                OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                    + 999.milliseconds()
                    + 999.microseconds()
                    + 999.nanoseconds()
                    + 1.nanoseconds()
            )
            .serialize(),
            [
                TimestampSize::Timestamp96 as u8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                4,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            Body::Timestamp(OffsetDateTime::from_unix_timestamp(1 << 34)).serialize(),
            [
                TimestampSize::Timestamp96 as u8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                4,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            Body::Timestamp(OffsetDateTime::unix_epoch() - 1.nanoseconds()).serialize(),
            [
                TimestampSize::Timestamp96 as u8,
                255,
                201,
                154,
                59,
                255,
                255,
                255,
                255,
                255,
                255,
                255,
                255
            ]
        );
    }

    #[test]
    fn serialize_date() {
        assert_eq!(
            Body::Date(Date::try_from_yo(2000, 1).unwrap()).serialize(),
            [0, 0]
        );
        assert_eq!(
            Body::Date(Date::try_from_yo(1936, 1).unwrap()).serialize(),
            [127, 0]
        );
        assert_eq!(
            Body::Date(Date::try_from_yo(1935, 1).unwrap()).serialize(),
            [129, 1, 0]
        );
        assert_eq!(
            Body::Date(Date::try_from_yo(2063, 128).unwrap()).serialize(),
            [126, 127]
        );
        assert_eq!(
            Body::Date(Date::try_from_yo(2064, 129).unwrap()).serialize(),
            [128, 1, 128, 1]
        );
        assert_eq!(
            Body::Date(Date::try_from_yo(2000, 366).unwrap()).serialize(),
            [0, 237, 2]
        );
    }

    #[test]
    fn deserialize_optional() {
        let body = Body::Optional(Box::new(None));
        assert_eq!(
            super::Body::deserialize(
                &Header::Optional(Box::new(Header::Boolean)),
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Optional(Box::new(Some(Body::Boolean(true))));
        assert_eq!(
            super::Body::deserialize(
                &Header::Optional(Box::new(Header::Boolean)),
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Optional(Box::new(Some(Body::String(String::from("test")))));
        assert_eq!(
            super::Body::deserialize(
                &Header::Optional(Box::new(Header::String)),
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_boolean() {
        assert_eq!(
            super::Body::deserialize(&Header::Boolean, &mut BufReader::new([0u8].as_ref())),
            Ok(Body::Boolean(false))
        );
        assert_eq!(
            super::Body::deserialize(&Header::Boolean, &mut BufReader::new([1u8].as_ref())),
            Ok(Body::Boolean(true))
        );
    }

    #[test]
    fn deserialize_uint() {
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt,
                &mut BufReader::new(0u8.encode_var_vec().as_slice())
            ),
            Ok(Body::UInt(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt,
                &mut BufReader::new(u8::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::UInt(u8::MAX as u64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt,
                &mut BufReader::new(u16::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::UInt(u16::MAX as u64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt,
                &mut BufReader::new(u32::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::UInt(u32::MAX as u64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt,
                &mut BufReader::new(u64::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::UInt(u64::MAX as u64))
        );
    }

    #[test]
    fn deserialize_uint8() {
        assert_eq!(
            super::Body::deserialize(&Header::UInt8, &mut BufReader::new([0u8].as_ref())),
            Ok(Body::UInt8(0))
        );
        assert_eq!(
            super::Body::deserialize(&Header::UInt8, &mut BufReader::new([255u8].as_ref())),
            Ok(Body::UInt8(255))
        );
    }

    #[test]
    fn deserialize_int() {
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(0i8.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i8::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i8::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i8::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i8::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i16::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i16::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i16::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i16::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i32::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i32::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i32::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i32::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i64::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i64::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int,
                &mut BufReader::new(i64::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::Int(i64::MAX as i64))
        );
    }

    #[test]
    fn deserialize_int8() {
        assert_eq!(
            super::Body::deserialize(&Header::Int8, &mut BufReader::new([0u8].as_ref())),
            Ok(Body::Int8(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int8,
                &mut BufReader::new((-1i8).to_le_bytes().as_ref())
            ),
            Ok(Body::Int8(-1))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int8,
                &mut BufReader::new(i8::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::Int8(i8::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int8,
                &mut BufReader::new(i8::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::Int8(i8::MAX))
        );
    }

    #[test]
    fn deserialize_float32() {
        assert_eq!(
            super::Body::deserialize(
                &Header::Float32,
                &mut BufReader::new(0f32.to_le_bytes().as_ref())
            ),
            Ok(Body::Float32(0f32))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float32,
                &mut BufReader::new(1.1f32.to_le_bytes().as_ref())
            ),
            Ok(Body::Float32(1.1f32))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float32,
                &mut BufReader::new((-1.1f32).to_le_bytes().as_ref())
            ),
            Ok(Body::Float32(-1.1f32))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float32,
                &mut BufReader::new(f32::INFINITY.to_le_bytes().as_ref())
            ),
            Ok(Body::Float32(f32::INFINITY))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float32,
                &mut BufReader::new((-f32::INFINITY).to_le_bytes().as_ref())
            ),
            Ok(Body::Float32(-f32::INFINITY))
        );
    }

    #[test]
    fn deserialize_float64() {
        assert_eq!(
            super::Body::deserialize(
                &Header::Float64,
                &mut BufReader::new(0f64.to_le_bytes().as_ref())
            ),
            Ok(Body::Float64(0f64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float64,
                &mut BufReader::new(1.1f64.to_le_bytes().as_ref())
            ),
            Ok(Body::Float64(1.1f64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float64,
                &mut BufReader::new((-1.1f64).to_le_bytes().as_ref())
            ),
            Ok(Body::Float64(-1.1f64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float64,
                &mut BufReader::new(f64::INFINITY.to_le_bytes().as_ref())
            ),
            Ok(Body::Float64(f64::INFINITY))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Float64,
                &mut BufReader::new((-f64::INFINITY).to_le_bytes().as_ref())
            ),
            Ok(Body::Float64(-f64::INFINITY))
        );
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(
            super::Body::deserialize(
                &Header::String,
                &mut BufReader::new(
                    ["test".len().encode_var_vec(), "test".as_bytes().to_vec()]
                        .concat()
                        .as_slice()
                )
            ),
            Ok(Body::String(String::from("test")))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::String,
                &mut BufReader::new(
                    [
                        "テスト".len().encode_var_vec(),
                        "テスト".as_bytes().to_vec()
                    ]
                    .concat()
                    .as_slice()
                )
            ),
            Ok(Body::String(String::from("テスト")))
        );
    }

    #[test]
    fn deserialize_binary() {
        let body = Binary(vec![0, 1, 2, 3, 255]);
        assert_eq!(
            super::Body::deserialize(
                &Header::Binary,
                &mut BufReader::new(
                    [body.0.len().encode_var_vec(), body.0.clone()]
                        .concat()
                        .as_slice()
                )
            ),
            Ok(Body::Binary(body))
        );
    }

    #[test]
    fn deserialize_array() {
        let body = [0u8, 1, 2, u8::MAX];
        assert_eq!(
            super::Body::deserialize(
                &Header::Array(Box::new(Header::UInt8)),
                &mut BufReader::new(
                    [
                        body.len().encode_var_vec(),
                        body.iter().flat_map(|v| v.to_le_bytes().to_vec()).collect()
                    ]
                    .concat()
                    .as_slice()
                )
            ),
            Ok(Body::Array(vec![
                Body::UInt8(0),
                Body::UInt8(1),
                Body::UInt8(2),
                Body::UInt8(u8::MAX)
            ]))
        );

        let body = ["aaaa", "bbbb"];
        assert_eq!(super::Body::deserialize(&Header::Array(Box::new(Header::String)), &mut BufReader::new([body.len().encode_var_vec(), body.iter().flat_map(|v| [v.len().encode_var_vec(), v.as_bytes().to_vec()].concat()).collect()].concat().as_slice())), Ok(Body::Array(vec![Body::String(String::from("aaaa")), Body::String(String::from("bbbb"))])));
    }

    #[test]
    fn deserialize_map() {
        let body: IndexMap<String, Body> = indexmap! { String::from("test") => Body::Boolean(true), String::from("test2") => Body::UInt8(u8::MAX) };
        assert_eq!(
            super::Body::deserialize(
                &Header::Map(
                    indexmap! { String::from("test") => Header::Boolean, String::from("test2") => Header::UInt8 }
                ),
                &mut BufReader::new([1u8, u8::MAX].as_ref())
            ),
            Ok(Body::Map(body))
        );

        let body: IndexMap<String, Body> = indexmap! { String::from("test") => Body::String(String::from("aaaa")), String::from("test2") =>Body::String(String::from("bbbb")) };
        assert_eq!(
            super::Body::deserialize(
                &Header::Map(
                    indexmap! { String::from("test") => Header::String, String::from("test2") => Header::String }
                ),
                &mut BufReader::new(
                    body.iter()
                        .flat_map(|v| if let Body::String(value) = v.1 {
                            [value.len().encode_var_vec(), value.as_bytes().to_vec()].concat()
                        } else {
                            panic!();
                        })
                        .collect::<Vec<u8>>()
                        .as_slice()
                )
            ),
            Ok(Body::Map(body))
        );
    }

    #[test]
    fn deserialize_dynamic_map() {
        let mut body = HashMap::new();
        body.insert(String::from("test"), Body::Boolean(true));
        assert_eq!(
            super::Body::deserialize(
                &Header::DynamicMap(Box::new(Header::Boolean)),
                &mut BufReader::new(Body::DynamicMap(body.clone()).serialize().as_slice())
            ),
            Ok(Body::DynamicMap(body))
        );
    }

    #[test]
    fn deserialize_timestamp32() {
        let body = Body::Timestamp(OffsetDateTime::unix_epoch());
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Timestamp(OffsetDateTime::from_unix_timestamp(u32::MAX as i64));
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_timestamp64() {
        let body = Body::Timestamp(OffsetDateTime::unix_epoch() + 1.nanoseconds());
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Timestamp(
            OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                + 999.milliseconds()
                + 999.microseconds()
                + 999.nanoseconds(),
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_timestamp96() {
        let body = Body::Timestamp(
            OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                + 999.milliseconds()
                + 999.microseconds()
                + 999.nanoseconds()
                + 1.nanoseconds(),
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Timestamp(OffsetDateTime::from_unix_timestamp(1 << 34));
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Timestamp(OffsetDateTime::unix_epoch() - 1.nanoseconds());
        assert_eq!(
            super::Body::deserialize(
                &Header::Timestamp,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_date() {
        let body = Body::Date(Date::try_from_yo(2000, 1).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Date(Date::try_from_yo(1936, 1).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Date(Date::try_from_yo(1935, 1).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Date(Date::try_from_yo(2063, 128).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Date(Date::try_from_yo(2064, 129).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::Date(Date::try_from_yo(2000, 366).unwrap());
        assert_eq!(
            super::Body::deserialize(
                &Header::Date,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }
}
