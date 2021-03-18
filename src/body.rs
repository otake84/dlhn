use crate::{deserialize_string, header::Header, new_dynamic_buf, serialize_string};
use bigdecimal::BigDecimal;
use integer_encoding::{VarInt, VarIntReader};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use std::{collections::BTreeMap, io::Read, mem::MaybeUninit};
use time::{Date, NumericalDuration, OffsetDateTime};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Optional(Box<Option<Body>>),
    Boolean(bool),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    VarUInt16(u16),
    VarUInt32(u32),
    VarUInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    VarInt16(i16),
    VarInt32(i32),
    VarInt64(i64),
    Float32(f32),
    Float64(f64),
    BigUInt(BigUint),
    BigInt(BigInt),
    BigDecimal(BigDecimal),
    String(String),
    Binary(Vec<u8>),
    Array(Vec<Body>),
    Map(BTreeMap<String, Body>),
    DynamicMap(BTreeMap<String, Body>),
    Date(Date),
    DateTime(OffsetDateTime),
    Extension8(u8),
    Extension16([u8; 2]),
    Extension32([u8; 4]),
    Extension64([u8; 8]),
    Extension(Vec<u8>),
}

impl Body {
    const DATE_YEAR_OFFSET: i32 = 2000;
    const DATE_ORDINAL_OFFSET: u16 = 1;

    const DATETIME_32_SIZE: u8 = 4;
    const DATETIME_64_SIZE: u8 = 8;
    const DATETIME_96_SIZE: u8 = 12;

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
            Self::UInt8(v) => Vec::from(v.to_le_bytes()),
            Self::UInt16(v) => Vec::from(v.to_le_bytes()),
            Self::UInt32(v) => Vec::from(v.to_le_bytes()),
            Self::UInt64(v) => Vec::from(v.to_le_bytes()),
            Self::VarUInt16(v) => v.encode_var_vec(),
            Self::VarUInt32(v) => v.encode_var_vec(),
            Self::VarUInt64(v) => v.encode_var_vec(),
            Self::Int8(v) => Vec::from(v.to_le_bytes()),
            Self::Int16(v) => Vec::from(v.to_le_bytes()),
            Self::Int32(v) => Vec::from(v.to_le_bytes()),
            Self::Int64(v) => Vec::from(v.to_le_bytes()),
            Self::VarInt16(v) => v.encode_var_vec(),
            Self::VarInt32(v) => v.encode_var_vec(),
            Self::VarInt64(v) => v.encode_var_vec(),
            Self::Float32(v) => Vec::from(v.to_le_bytes()),
            Self::Float64(v) => Vec::from(v.to_le_bytes()),
            Self::BigUInt(v) => {
                if v.is_zero() {
                    vec![0]
                } else {
                    let mut data = v.to_bytes_le();
                    let mut buf = data.len().encode_var_vec();
                    buf.append(&mut data);
                    buf
                }
            }
            Self::BigInt(v) => {
                if v.is_zero() {
                    vec![0]
                } else {
                    let mut data = v.to_signed_bytes_le();
                    let mut buf = data.len().encode_var_vec();
                    buf.append(&mut data);
                    buf
                }
            }
            Self::BigDecimal(v) => {
                if v.is_zero() {
                    vec![0]
                } else {
                    let (bigint, scale) = v.normalized().into_bigint_and_exponent();
                    let mut data = bigint.to_signed_bytes_le();
                    let mut buf = data.len().encode_var_vec();
                    buf.append(&mut data);
                    buf.append(&mut scale.encode_var_vec());
                    buf
                }
            }
            Self::String(v) => serialize_string(v),
            Self::Binary(v) => {
                let mut buf = v.len().encode_var_vec();
                buf.extend(v.as_slice());
                buf
            }
            Self::Array(v) => {
                let mut buf = v.len().encode_var_vec();
                v.iter().for_each(|v| buf.append(&mut v.serialize()));
                buf
            }
            Self::Map(v) => {
                let mut buf = Vec::new();
                v.values().for_each(|v| buf.append(&mut v.serialize()));
                buf
            }
            Self::DynamicMap(v) => {
                let mut buf = v.len().encode_var_vec();
                v.iter().for_each(|(k, v)| {
                    buf.append(&mut serialize_string(k));
                    buf.append(&mut v.serialize());
                });
                buf
            }
            Self::Date(v) => {
                let year = v.year() - Self::DATE_YEAR_OFFSET;
                let ordinal = v.ordinal() - Self::DATE_ORDINAL_OFFSET;
                let mut buf = new_dynamic_buf(year.required_space() + ordinal.required_space());
                year.encode_var(&mut buf);
                ordinal.encode_var(&mut buf[year.required_space()..]);
                buf
            }
            Self::DateTime(v) => {
                let kind_size = 1;

                if v.unix_timestamp() >> 34 == 0 {
                    let v = (u64::from(v.nanosecond()) << 34) | (v.unix_timestamp() as u64);

                    if v & 0xff_ff_ff_ff_00_00_00_00 == 0 {
                        let mut buf =
                            Vec::with_capacity(kind_size + Body::DATETIME_32_SIZE as usize);
                        buf.extend(&(Body::DATETIME_32_SIZE).to_le_bytes());
                        buf.extend(&(v as u32).to_le_bytes());
                        buf
                    } else {
                        let mut buf =
                            Vec::with_capacity(kind_size + Body::DATETIME_64_SIZE as usize);
                        buf.extend(&(Body::DATETIME_64_SIZE).to_le_bytes());
                        buf.extend(&v.to_le_bytes());
                        buf
                    }
                } else {
                    let mut buf = Vec::with_capacity(kind_size + Body::DATETIME_96_SIZE as usize);
                    buf.extend(&(Body::DATETIME_96_SIZE).to_le_bytes());
                    buf.extend(&v.time().nanosecond().to_le_bytes());
                    buf.extend(&v.unix_timestamp().to_le_bytes());
                    buf
                }
            }
            Self::Extension8(v) => Vec::from(v.to_le_bytes()),
            Self::Extension16(v) => Vec::from(v.as_ref()),
            Self::Extension32(v) => Vec::from(v.as_ref()),
            Self::Extension64(v) => Vec::from(v.as_ref()),
            Self::Extension(v) => {
                let mut buf = v.len().encode_var_vec();
                buf.extend(v.as_slice());
                buf
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(header: &Header, reader: &mut R) -> Result<Body, ()> {
        match header {
            Header::Optional(inner_header) => {
                let mut buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut buf).or(Err(()))?;
                match buf[0] {
                    0 => Ok(Self::Optional(Box::new(None))),
                    1 => Ok(Self::Optional(Box::new(Some(Self::deserialize(
                        inner_header,
                        reader,
                    )?)))),
                    _ => Err(()),
                }
            }
            Header::Boolean => {
                let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                match body_buf[0] {
                    0 => Ok(Self::Boolean(false)),
                    1 => Ok(Self::Boolean(true)),
                    _ => Err(()),
                }
            }
            Header::UInt8 => {
                let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::UInt8(u8::from_le_bytes(body_buf)))
            }
            Header::UInt16 => {
                let mut body_buf: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::UInt16(u16::from_le_bytes(body_buf)))
            }
            Header::UInt32 => {
                let mut body_buf: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::UInt32(u32::from_le_bytes(body_buf)))
            }
            Header::UInt64 => {
                let mut body_buf: [u8; 8] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::UInt64(u64::from_le_bytes(body_buf)))
            }
            Header::VarUInt16 => reader.read_varint::<u16>().map(Self::VarUInt16).or(Err(())),
            Header::VarUInt32 => reader.read_varint::<u32>().map(Self::VarUInt32).or(Err(())),
            Header::VarUInt64 => reader.read_varint::<u64>().map(Self::VarUInt64).or(Err(())),
            Header::Int8 => {
                let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Int8(i8::from_le_bytes(body_buf)))
            }
            Header::Int16 => {
                let mut body_buf: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Int16(i16::from_le_bytes(body_buf)))
            }
            Header::Int32 => {
                let mut body_buf: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Int32(i32::from_le_bytes(body_buf)))
            }
            Header::Int64 => {
                let mut body_buf: [u8; 8] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Int64(i64::from_le_bytes(body_buf)))
            }
            Header::VarInt16 => reader.read_varint::<i16>().map(Self::VarInt16).or(Err(())),
            Header::VarInt32 => reader.read_varint::<i32>().map(Self::VarInt32).or(Err(())),
            Header::VarInt64 => reader.read_varint::<i64>().map(Self::VarInt64).or(Err(())),
            Header::Float32 => {
                let mut body_buf: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Float32(f32::from_le_bytes(body_buf)))
            }
            Header::Float64 => {
                let mut body_buf: [u8; 8] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Float64(f64::from_le_bytes(body_buf)))
            }
            Header::BigUInt => {
                let mut body_buf = new_dynamic_buf(reader.read_varint::<usize>().or(Err(()))?);
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::BigUInt(BigUint::from_bytes_le(body_buf.as_slice())))
            }
            Header::BigInt => {
                let mut body_buf = new_dynamic_buf(reader.read_varint::<usize>().or(Err(()))?);
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::BigInt(BigInt::from_signed_bytes_le(
                    body_buf.as_slice(),
                )))
            }
            Header::BigDecimal => {
                let size = reader.read_varint::<usize>().or(Err(()))?;
                if size == 0 {
                    Ok(Self::BigDecimal(BigDecimal::from(0)))
                } else {
                    let mut body_buf = new_dynamic_buf(size);
                    reader.read_exact(&mut body_buf).or(Err(()))?;
                    Ok(Self::BigDecimal(BigDecimal::new(
                        BigInt::from_signed_bytes_le(body_buf.as_slice()),
                        reader.read_varint::<i64>().or(Err(()))?,
                    )))
                }
            }
            Header::String => deserialize_string(reader).map(Self::String),
            Header::Binary => {
                let mut body_buf = new_dynamic_buf(reader.read_varint::<usize>().or(Err(()))?);
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Binary(body_buf))
            }
            Header::Array(inner_header) => {
                let size = reader.read_varint::<usize>().or(Err(()))?;
                let mut body = Vec::with_capacity(size);
                for _ in 0..size {
                    body.push(Self::deserialize(inner_header, reader)?);
                }
                Ok(Self::Array(body))
            }
            Header::Map(inner_header) => {
                let mut body = BTreeMap::new();
                for (key, h) in inner_header.iter() {
                    body.insert(key.clone(), Self::deserialize(h, reader)?);
                }
                Ok(Self::Map(body))
            }
            Header::DynamicMap(inner_header) => {
                let size = reader.read_varint::<usize>().or(Err(()))?;
                let mut body = BTreeMap::new();
                for _ in 0..size {
                    let key = deserialize_string(reader)?;
                    let value = Self::deserialize(inner_header, reader)?;
                    body.insert(key, value);
                }
                Ok(Self::DynamicMap(body))
            }
            Header::Date => {
                let year = reader.read_varint::<i32>().or(Err(()))? + Self::DATE_YEAR_OFFSET;
                let ordinal = reader.read_varint::<u16>().or(Err(()))? + Self::DATE_ORDINAL_OFFSET;
                let date = Date::try_from_yo(year, ordinal).or(Err(()))?;

                Ok(Self::Date(date))
            }
            Header::DateTime => {
                let mut kind_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut kind_buf).or(Err(()))?;

                match u8::from_le_bytes(kind_buf) {
                    Self::DATETIME_32_SIZE => {
                        let mut second_buf: [u8; Body::DATETIME_32_SIZE as usize] =
                            unsafe { MaybeUninit::uninit().assume_init() };
                        reader.read_exact(&mut second_buf).or(Err(()))?;

                        Ok(Self::DateTime(
                            OffsetDateTime::unix_epoch() + u32::from_le_bytes(second_buf).seconds(),
                        ))
                    }
                    Self::DATETIME_64_SIZE => {
                        let mut nanosecond_and_second_buf: [u8; Body::DATETIME_64_SIZE as usize] =
                            unsafe { MaybeUninit::uninit().assume_init() };
                        reader
                            .read_exact(&mut nanosecond_and_second_buf)
                            .or(Err(()))?;

                        let value = u64::from_le_bytes(nanosecond_and_second_buf);
                        let nanosecond = value >> 34;
                        let second = value & 0x00_00_00_03_ff_ff_ff_ff;
                        Ok(Self::DateTime(
                            OffsetDateTime::from_unix_timestamp(second as i64)
                                + (nanosecond as u32).nanoseconds(),
                        ))
                    }
                    Self::DATETIME_96_SIZE => {
                        let mut nanosecond_buf: [u8; 4] =
                            unsafe { MaybeUninit::uninit().assume_init() };
                        reader.read_exact(&mut nanosecond_buf).or(Err(()))?;
                        let nanosecond = u32::from_le_bytes(nanosecond_buf);

                        let mut unix_timestamp_buf: [u8; 8] =
                            unsafe { MaybeUninit::uninit().assume_init() };
                        reader.read_exact(&mut unix_timestamp_buf).or(Err(()))?;
                        let unix_timestamp = i64::from_le_bytes(unix_timestamp_buf);

                        Ok(Self::DateTime(
                            OffsetDateTime::from_unix_timestamp(unix_timestamp)
                                + nanosecond.nanoseconds(),
                        ))
                    }
                    _ => Err(()),
                }
            }
            Header::Extension8(_) => {
                let mut body_buf: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Extension8(u8::from_le_bytes(body_buf)))
            }
            Header::Extension16(_) => {
                let mut body_buf: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Extension16(body_buf))
            }
            Header::Extension32(_) => {
                let mut body_buf: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Extension32(body_buf))
            }
            Header::Extension64(_) => {
                let mut body_buf: [u8; 8] = unsafe { MaybeUninit::uninit().assume_init() };
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Extension64(body_buf))
            }
            Header::Extension(_) => {
                let mut body_buf = new_dynamic_buf(reader.read_varint::<usize>().or(Err(()))?);
                reader.read_exact(&mut body_buf).or(Err(()))?;
                Ok(Self::Extension(body_buf))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Body;
    use crate::header::{ExtensionCode, Header};
    use bigdecimal::BigDecimal;
    use core::panic;
    use integer_encoding::VarInt;
    use num_bigint::{BigInt, BigUint};
    use std::{collections::BTreeMap, io::BufReader};
    use time::{Date, NumericalDuration, OffsetDateTime};

    #[test]
    fn serialize_uint8() {
        assert_eq!(Body::UInt8(u8::MIN).serialize(), u8::MIN.to_le_bytes());
        assert_eq!(Body::UInt8(u8::MAX).serialize(), u8::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_uint16() {
        assert_eq!(Body::UInt16(u16::MIN).serialize(), u16::MIN.to_le_bytes());
        assert_eq!(Body::UInt16(u16::MAX).serialize(), u16::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_uint32() {
        assert_eq!(Body::UInt32(u32::MIN).serialize(), u32::MIN.to_le_bytes());
        assert_eq!(Body::UInt32(u32::MAX).serialize(), u32::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_uint64() {
        assert_eq!(Body::UInt64(u64::MIN).serialize(), u64::MIN.to_le_bytes());
        assert_eq!(Body::UInt64(u64::MAX).serialize(), u64::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_var_uint16() {
        assert_eq!(Body::VarUInt16(u8::MIN as u16).serialize(), [0]);
        assert_eq!(Body::VarUInt16(u8::MAX as u16).serialize(), [255, 1]);
        assert_eq!(Body::VarUInt16(u16::MAX).serialize(), [255, 255, 3]);
    }

    #[test]
    fn serialize_var_uint32() {
        assert_eq!(Body::VarUInt32(u8::MIN as u32).serialize(), [0]);
        assert_eq!(Body::VarUInt32(u8::MAX as u32).serialize(), [255, 1]);
        assert_eq!(Body::VarUInt32(u16::MAX as u32).serialize(), [255, 255, 3]);
        assert_eq!(
            Body::VarUInt32(u32::MAX as u32).serialize(),
            [255, 255, 255, 255, 15]
        );
    }

    #[test]
    fn serialize_var_uint64() {
        assert_eq!(Body::VarUInt64(u8::MIN as u64).serialize(), [0]);
        assert_eq!(Body::VarUInt64(u8::MAX as u64).serialize(), [255, 1]);
        assert_eq!(Body::VarUInt64(u16::MAX as u64).serialize(), [255, 255, 3]);
        assert_eq!(
            Body::VarUInt64(u32::MAX as u64).serialize(),
            [255, 255, 255, 255, 15]
        );
        assert_eq!(
            Body::VarUInt64(u64::MAX).serialize(),
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 1]
        );
    }

    #[test]
    fn serialize_int8() {
        assert_eq!(Body::Int8(i8::MIN).serialize(), i8::MIN.to_le_bytes());
        assert_eq!(Body::Int8(0).serialize(), 0i8.to_le_bytes());
        assert_eq!(Body::Int8(i8::MAX).serialize(), i8::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_int16() {
        assert_eq!(Body::Int16(i16::MIN).serialize(), i16::MIN.to_le_bytes());
        assert_eq!(Body::Int16(0).serialize(), 0i16.to_le_bytes());
        assert_eq!(Body::Int16(i16::MAX).serialize(), i16::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_int32() {
        assert_eq!(Body::Int32(i32::MIN).serialize(), i32::MIN.to_le_bytes());
        assert_eq!(Body::Int32(0).serialize(), 0i32.to_le_bytes());
        assert_eq!(Body::Int32(i32::MAX).serialize(), i32::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_int64() {
        assert_eq!(Body::Int64(i64::MIN).serialize(), i64::MIN.to_le_bytes());
        assert_eq!(Body::Int64(0).serialize(), 0i64.to_le_bytes());
        assert_eq!(Body::Int64(i64::MAX).serialize(), i64::MAX.to_le_bytes());
    }

    #[test]
    fn serialize_var_int16() {
        assert_eq!(Body::VarInt16(0).serialize(), [0]);
        assert_eq!(Body::VarInt16(i8::MIN as i16).serialize(), [255, 1]);
        assert_eq!(Body::VarInt16(i8::MAX as i16).serialize(), [254, 1]);
        assert_eq!(Body::VarInt16(i16::MIN).serialize(), [255, 255, 3]);
        assert_eq!(Body::VarInt16(i16::MAX).serialize(), [254, 255, 3]);
    }

    #[test]
    fn serialize_var_int32() {
        assert_eq!(Body::VarInt32(0).serialize(), [0]);
        assert_eq!(Body::VarInt32(i8::MIN as i32).serialize(), [255, 1]);
        assert_eq!(Body::VarInt32(i8::MAX as i32).serialize(), [254, 1]);
        assert_eq!(Body::VarInt32(i16::MIN as i32).serialize(), [255, 255, 3]);
        assert_eq!(Body::VarInt32(i16::MAX as i32).serialize(), [254, 255, 3]);
        assert_eq!(
            Body::VarInt32(i32::MIN).serialize(),
            [255, 255, 255, 255, 15]
        );
        assert_eq!(
            Body::VarInt32(i32::MAX).serialize(),
            [254, 255, 255, 255, 15]
        );
    }

    #[test]
    fn serialize_var_int64() {
        assert_eq!(Body::VarInt64(0).serialize(), [0]);
        assert_eq!(Body::VarInt64(i8::MIN as i64).serialize(), [255, 1]);
        assert_eq!(Body::VarInt64(i8::MAX as i64).serialize(), [254, 1]);
        assert_eq!(Body::VarInt64(i16::MIN as i64).serialize(), [255, 255, 3]);
        assert_eq!(Body::VarInt64(i16::MAX as i64).serialize(), [254, 255, 3]);
        assert_eq!(
            Body::VarInt64(i32::MIN as i64).serialize(),
            [255, 255, 255, 255, 15]
        );
        assert_eq!(
            Body::VarInt64(i32::MAX as i64).serialize(),
            [254, 255, 255, 255, 15]
        );
        assert_eq!(
            Body::VarInt64(i64::MIN).serialize(),
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 1]
        );
        assert_eq!(
            Body::VarInt64(i64::MAX).serialize(),
            [254, 255, 255, 255, 255, 255, 255, 255, 255, 1]
        );
    }

    #[test]
    fn serialize_biguint() {
        assert_eq!(Body::BigUInt(BigUint::from(0u8)).serialize(), [0]);
        assert_eq!(Body::BigUInt(BigUint::from(u8::MAX)).serialize(), [1, 255]);
        assert_eq!(
            Body::BigUInt(BigUint::from(u16::MAX)).serialize(),
            [2, 255, 255]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u16::MAX) + 1u8).serialize(),
            [3, 0, 0, 1]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u32::MAX)).serialize(),
            [4, 255, 255, 255, 255]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u32::MAX) + 1u8).serialize(),
            [5, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u64::MAX)).serialize(),
            [8, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u64::MAX) + 1u8).serialize(),
            [9, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u128::MAX)).serialize(),
            [16, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            Body::BigUInt(BigUint::from(u128::MAX) + 1u8).serialize(),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn serialize_bigint() {
        assert_eq!(Body::BigInt(BigInt::from(0)).serialize(), [0]);

        assert_eq!(
            Body::BigInt(BigInt::from(i8::MIN)).serialize(),
            [[1], i8::MIN.to_le_bytes()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i8::MAX)).serialize(),
            [[1], i8::MAX.to_le_bytes()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i16::MIN)).serialize(),
            [vec![2], i16::MIN.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i16::MAX)).serialize(),
            [vec![2], i16::MAX.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i16::MIN) - 1).serialize(),
            [3, 255, 127, 255]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i16::MAX) + 1).serialize(),
            [3, 0, 128, 0]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i32::MIN)).serialize(),
            [vec![4], i32::MIN.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i32::MAX)).serialize(),
            [vec![4], i32::MAX.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i32::MIN) - 1).serialize(),
            [5, 255, 255, 255, 127, 255]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i32::MAX) + 1).serialize(),
            [5, 0, 0, 0, 128, 0]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i64::MIN)).serialize(),
            [vec![8], i64::MIN.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i64::MAX)).serialize(),
            [vec![8], i64::MAX.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i64::MIN) - 1).serialize(),
            [9, 255, 255, 255, 255, 255, 255, 255, 127, 255]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i64::MAX) + 1).serialize(),
            [9, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i128::MIN)).serialize(),
            [vec![16], i128::MIN.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i128::MAX)).serialize(),
            [vec![16], i128::MAX.to_le_bytes().to_vec()].concat()
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i128::MIN) - 1).serialize(),
            [
                17, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
                255
            ]
        );

        assert_eq!(
            Body::BigInt(BigInt::from(i128::MAX) + 1).serialize(),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );
    }

    #[test]
    fn serialize_bigdecimal() {
        assert_eq!(Body::BigDecimal(BigDecimal::from(0)).serialize(), [0]);

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), 0)).serialize(),
            [1, 1, 0]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), -1)).serialize(),
            [1, 1, 1]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), 1)).serialize(),
            [1, 1, 2]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(10), 0)).serialize(),
            [1, 1, 1]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), 63)).serialize(),
            [1, 1, 126]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), 64)).serialize(),
            [1, 1, 128, 1]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), -64)).serialize(),
            [1, 1, 127]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(1), -65)).serialize(),
            [1, 1, 129, 1]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(i16::MIN), 0)).serialize(),
            [2, 0, 128, 0]
        );

        assert_eq!(
            Body::BigDecimal(BigDecimal::new(BigInt::from(i16::MAX), 0)).serialize(),
            [2, 255, 127, 0]
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
    fn serialize_datetime32() {
        assert_eq!(
            Body::DateTime(OffsetDateTime::unix_epoch()).serialize(),
            [Body::DATETIME_32_SIZE, 0, 0, 0, 0]
        );
        assert_eq!(
            Body::DateTime(OffsetDateTime::from_unix_timestamp(u32::MAX as i64)).serialize(),
            [Body::DATETIME_32_SIZE, 255, 255, 255, 255]
        );
    }

    #[test]
    fn serialize_datetime64() {
        assert_eq!(
            Body::DateTime(OffsetDateTime::unix_epoch() + 1.nanoseconds()).serialize(),
            [Body::DATETIME_64_SIZE, 0, 0, 0, 0, 4, 0, 0, 0]
        );
        assert_eq!(
            Body::DateTime(
                OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                    + 999.milliseconds()
                    + 999.microseconds()
                    + 999.nanoseconds()
            )
            .serialize(),
            [
                Body::DATETIME_64_SIZE,
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
    fn serialize_datetime96() {
        assert_eq!(
            Body::DateTime(
                OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                    + 999.milliseconds()
                    + 999.microseconds()
                    + 999.nanoseconds()
                    + 1.nanoseconds()
            )
            .serialize(),
            [Body::DATETIME_96_SIZE, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0]
        );
        assert_eq!(
            Body::DateTime(OffsetDateTime::from_unix_timestamp(1 << 34)).serialize(),
            [Body::DATETIME_96_SIZE, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0]
        );
        assert_eq!(
            Body::DateTime(OffsetDateTime::unix_epoch() - 1.nanoseconds()).serialize(),
            [
                Body::DATETIME_96_SIZE,
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
    fn serialize_extension8() {
        assert_eq!(Body::Extension8(255).serialize(), [255]);
    }

    #[test]
    fn serialize_extension16() {
        assert_eq!(Body::Extension16([255, 0]).serialize(), [255, 0]);
    }

    #[test]
    fn serialize_extension32() {
        assert_eq!(
            Body::Extension32([255, 0, 255, 0]).serialize(),
            [255, 0, 255, 0]
        );
    }

    #[test]
    fn serialize_extension64() {
        assert_eq!(
            Body::Extension64([255, 0, 255, 0, 255, 0, 255, 0]).serialize(),
            [255, 0, 255, 0, 255, 0, 255, 0]
        );
    }

    #[test]
    fn serialize_extension() {
        assert_eq!(Body::Extension(vec![0, 1, 2]).serialize(), [3, 0, 1, 2]);
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
    fn deserialize_uint8() {
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt8,
                &mut BufReader::new(u8::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt8(u8::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt8,
                &mut BufReader::new(u8::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt8(u8::MAX))
        );
    }

    #[test]
    fn deserialize_uint16() {
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt16,
                &mut BufReader::new(u16::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt16(u16::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt16,
                &mut BufReader::new(u16::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt16(u16::MAX))
        );
    }

    #[test]
    fn deserialize_uint32() {
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt32,
                &mut BufReader::new(u32::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt32(u32::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt32,
                &mut BufReader::new(u32::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt32(u32::MAX))
        );
    }

    #[test]
    fn deserialize_uint64() {
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt64,
                &mut BufReader::new(u64::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt64(u64::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::UInt64,
                &mut BufReader::new(u64::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::UInt64(u64::MAX))
        );
    }

    #[test]
    fn deserialize_var_uint16() {
        let header = Header::VarUInt16;

        let body = Body::VarUInt16(u8::MIN as u16);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt16(u8::MAX as u16);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt16(u16::MAX);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_var_uint32() {
        let header = Header::VarUInt32;

        let body = Body::VarUInt32(u8::MIN as u32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt32(u8::MAX as u32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt32(u16::MAX as u32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt32(u32::MAX);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_var_uint64() {
        let header = Header::VarUInt64;

        let body = Body::VarUInt64(u8::MIN as u64);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt64(u8::MAX as u64);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt64(u16::MAX as u64);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt64(u32::MAX as u64);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarUInt64(u64::MAX);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
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
    fn deserialize_int16() {
        assert_eq!(
            super::Body::deserialize(
                &Header::Int16,
                &mut BufReader::new(i16::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::Int16(i16::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int16,
                &mut BufReader::new(0i16.to_le_bytes().as_ref())
            ),
            Ok(Body::Int16(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int16,
                &mut BufReader::new(i16::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::Int16(i16::MAX))
        );
    }

    #[test]
    fn deserialize_int32() {
        assert_eq!(
            super::Body::deserialize(
                &Header::Int32,
                &mut BufReader::new(i32::MIN.to_le_bytes().as_ref())
            ),
            Ok(Body::Int32(i32::MIN))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int32,
                &mut BufReader::new(0i32.to_le_bytes().as_ref())
            ),
            Ok(Body::Int32(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::Int32,
                &mut BufReader::new(i32::MAX.to_le_bytes().as_ref())
            ),
            Ok(Body::Int32(i32::MAX))
        );
    }

    #[test]
    fn deserialize_var_int16() {
        let header = Header::VarInt16;

        let body = Body::VarInt16(0);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt16(i8::MIN as i16);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt16(i8::MAX as i16);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt16(i16::MIN);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt16(i16::MAX);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_var_int32() {
        let header = Header::VarInt32;

        let body = Body::VarInt32(0);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i8::MIN as i32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i8::MAX as i32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i16::MIN as i32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i16::MAX as i32);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i32::MIN);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );

        let body = Body::VarInt32(i32::MAX);
        assert_eq!(
            super::Body::deserialize(&header, &mut BufReader::new(body.serialize().as_slice())),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_var_int64() {
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(0i8.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(0))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i8::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i8::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i8::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i8::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i16::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i16::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i16::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i16::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i32::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i32::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i32::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i32::MAX as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i64::MIN.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i64::MIN as i64))
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::VarInt64,
                &mut BufReader::new(i64::MAX.encode_var_vec().as_slice())
            ),
            Ok(Body::VarInt64(i64::MAX as i64))
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
    fn deserialize_biguint() {
        vec![
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(u8::MAX),
            BigUint::from(u8::MAX) + 1u8,
            BigUint::from(u16::MAX),
            BigUint::from(u16::MAX) + 1u8,
            BigUint::from(u32::MAX),
            BigUint::from(u32::MAX) + 1u8,
            BigUint::from(u64::MAX),
            BigUint::from(u64::MAX) + 1u8,
            BigUint::from(u128::MAX),
            BigUint::from(u128::MAX) + 1u8,
        ]
        .into_iter()
        .map(Body::BigUInt)
        .for_each(|body| {
            assert_eq!(
                super::Body::deserialize(
                    &Header::BigUInt,
                    &mut BufReader::new(body.serialize().as_slice())
                ),
                Ok(body)
            );
        });
    }

    #[test]
    fn deserialize_bigint() {
        let body = Body::BigInt(BigInt::from(0));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i8::MIN));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i8::MAX));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i8::MIN) - 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i8::MAX) + 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i16::MIN));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i16::MAX));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i16::MIN) - 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i16::MAX) + 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i32::MIN));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i32::MAX));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i32::MIN) - 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i32::MAX) + 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i64::MIN));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i64::MAX));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i64::MIN) - 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i64::MAX) + 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i128::MIN));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i128::MAX));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i128::MIN) - 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigInt(BigInt::from(i128::MAX) + 1);
        assert_eq!(
            super::Body::deserialize(
                &Header::BigInt,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_bigdecimal() {
        let body = Body::BigDecimal(BigDecimal::from(0));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), 0));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), -1));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), 1));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), 63));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), 64));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), -64));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(1), -65));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(i16::MIN), 0));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::BigDecimal(BigDecimal::new(BigInt::from(i16::MAX), 0));
        assert_eq!(
            super::Body::deserialize(
                &Header::BigDecimal,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
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
        let body = vec![0, 1, 2, 3, 255];
        assert_eq!(
            super::Body::deserialize(
                &Header::Binary,
                &mut BufReader::new(
                    [body.len().encode_var_vec(), body.clone()]
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
        let body = {
            let mut map = BTreeMap::new();
            map.insert(String::from("test"), Body::Boolean(true));
            map.insert(String::from("test2"), Body::UInt8(u8::MAX));
            map
        };
        assert_eq!(
            super::Body::deserialize(
                &Header::Map({
                    let mut map = BTreeMap::new();
                    map.insert(String::from("test"), Header::Boolean);
                    map.insert(String::from("test2"), Header::UInt8);
                    map
                }),
                &mut BufReader::new([1u8, u8::MAX].as_ref())
            ),
            Ok(Body::Map(body))
        );

        let body = {
            let mut map = BTreeMap::new();
            map.insert(String::from("test"), Body::String(String::from("aaaa")));
            map.insert(String::from("test2"), Body::String(String::from("bbbb")));
            map
        };
        assert_eq!(
            super::Body::deserialize(
                &Header::Map({
                    let mut map = BTreeMap::new();
                    map.insert(String::from("test"), Header::String);
                    map.insert(String::from("test2"), Header::String);
                    map
                }),
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
        let mut body = BTreeMap::new();
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

    #[test]
    fn deserialize_datetime32() {
        let body = Body::DateTime(OffsetDateTime::unix_epoch());
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::DateTime(OffsetDateTime::from_unix_timestamp(u32::MAX as i64));
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_datetime64() {
        let body = Body::DateTime(OffsetDateTime::unix_epoch() + 1.nanoseconds());
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::DateTime(
            OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                + 999.milliseconds()
                + 999.microseconds()
                + 999.nanoseconds(),
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_datetime96() {
        let body = Body::DateTime(
            OffsetDateTime::from_unix_timestamp((1 << 34) - 1)
                + 999.milliseconds()
                + 999.microseconds()
                + 999.nanoseconds()
                + 1.nanoseconds(),
        );
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::DateTime(OffsetDateTime::from_unix_timestamp(1 << 34));
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );

        let body = Body::DateTime(OffsetDateTime::unix_epoch() - 1.nanoseconds());
        assert_eq!(
            super::Body::deserialize(
                &Header::DateTime,
                &mut BufReader::new(body.serialize().as_slice())
            ),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_extension8() {
        let body = Body::Extension8(123);
        assert_eq!(
            super::Body::deserialize(&Header::Extension8(255), &mut body.serialize().as_slice()),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_extension16() {
        let body = Body::Extension16([123, 0]);
        assert_eq!(
            super::Body::deserialize(&Header::Extension16(255), &mut body.serialize().as_slice()),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_extension32() {
        let body = Body::Extension32([123, 0, 123, 0]);
        assert_eq!(
            super::Body::deserialize(&Header::Extension32(255), &mut body.serialize().as_slice()),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_extension64() {
        let body = Body::Extension64([123, 0, 123, 0, 123, 0, 123, 0]);
        assert_eq!(
            super::Body::deserialize(&Header::Extension64(255), &mut body.serialize().as_slice()),
            Ok(body)
        );
    }

    #[test]
    fn deserialize_extension() {
        let body = Body::Extension(vec![0, 1, 2, 3]);
        assert_eq!(
            super::Body::deserialize(
                &Header::Extension(ExtensionCode::Code255),
                &mut body.serialize().as_slice()
            ),
            Ok(body)
        );
    }
}
