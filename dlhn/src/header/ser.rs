use crate::leb128::Leb128;
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint};
use serde_bytes::{ByteBuf, Bytes};
use std::{
    collections::{BTreeMap, HashMap},
    io::{Result, Write},
};
use time::{Date, OffsetDateTime};

pub trait SerializeHeader {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()>;
}

impl SerializeHeader for () {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UNIT_CODE])
    }
}

impl<T: SerializeHeader> SerializeHeader for Option<T> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::OPTIONAL_CODE])?;
        T::serialize_header(writer)
    }
}

impl SerializeHeader for bool {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BOOLEAN_CODE])
    }
}

impl SerializeHeader for u8 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UINT8_CODE])
    }
}

impl SerializeHeader for u16 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UINT16_CODE])
    }
}

impl SerializeHeader for u32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UINT32_CODE])
    }
}

impl SerializeHeader for u64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UINT64_CODE])
    }
}

impl SerializeHeader for u128 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::UINT128_CODE])
    }
}

impl SerializeHeader for i8 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::INT8_CODE])
    }
}

impl SerializeHeader for i16 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::INT16_CODE])
    }
}

impl SerializeHeader for i32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::INT32_CODE])
    }
}

impl SerializeHeader for i64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::INT64_CODE])
    }
}

impl SerializeHeader for i128 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::INT128_CODE])
    }
}

impl SerializeHeader for f32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::FLOAT32_CODE])
    }
}

impl SerializeHeader for f64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::FLOAT64_CODE])
    }
}

impl SerializeHeader for BigUint {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_UINT_CODE])
    }
}

impl SerializeHeader for BigInt {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_INT_CODE])
    }
}

impl SerializeHeader for BigDecimal {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_DECIMAL_CODE])
    }
}

impl SerializeHeader for &str {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::STRING_CODE])
    }
}

impl SerializeHeader for String {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::STRING_CODE])
    }
}

impl SerializeHeader for Bytes {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BINARY_CODE])
    }
}

impl SerializeHeader for ByteBuf {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BINARY_CODE])
    }
}

impl<T: SerializeHeader> SerializeHeader for Vec<T> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::ARRAY_CODE])?;
        T::serialize_header(writer)
    }
}

impl SerializeHeader for Date {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::DATE_CODE])
    }
}

impl SerializeHeader for OffsetDateTime {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::DATETIME_CODE])
    }
}

impl<K: AsRef<str>, V: SerializeHeader> SerializeHeader for BTreeMap<K, V> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::MAP_CODE])?;
        V::serialize_header(writer)
    }
}

impl<K: AsRef<str>, V: SerializeHeader> SerializeHeader for HashMap<K, V> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::MAP_CODE])?;
        V::serialize_header(writer)
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($name:ident)+))+) => {
        $(
            impl<$($name),+> SerializeHeader for ($($name,)+)
            where
                $($name: SerializeHeader,)+
            {
                fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
                    writer.write_all(&[super::TUPLE_CODE])?;
                    let mut buf = [0u8; usize::LEB128_BUF_SIZE];
                    let size = ($len as usize).encode_leb128(&mut buf);
                    writer.write_all(&buf[..size])?;
                    $(
                        $name::serialize_header(writer)?;
                    )+
                    Ok(())
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (T0)
    2 => (T0 T1)
    3 => (T0 T1 T2)
    4 => (T0 T1 T2 T3)
    5 => (T0 T1 T2 T3 T4)
    6 => (T0 T1 T2 T3 T4 T5)
    7 => (T0 T1 T2 T3 T4 T5 T6)
    8 => (T0 T1 T2 T3 T4 T5 T6 T7)
    9 => (T0 T1 T2 T3 T4 T5 T6 T7 T8)
    10 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9)
    11 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10)
    12 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11)
    13 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    14 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    15 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    16 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
}

#[cfg(test)]
mod tests {
    use super::SerializeHeader;
    use bigdecimal::BigDecimal;
    use num_bigint::{BigInt, BigUint};
    use serde_bytes::{ByteBuf, Bytes};
    use std::collections::{BTreeMap, HashMap};
    use time::{Date, OffsetDateTime};

    #[test]
    fn serialize_header_unit() {
        let mut buf = Vec::new();
        <()>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [0]);
    }

    #[test]
    fn serialize_header_option() {
        let mut buf = Vec::new();
        Option::<()>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [1, 0]);
    }

    #[test]
    fn serialize_header_bool() {
        let mut buf = Vec::new();
        bool::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [2]);
    }

    #[test]
    fn serialize_header_u8() {
        let mut buf = Vec::new();
        u8::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [3]);
    }

    #[test]
    fn serialize_header_u16() {
        let mut buf = Vec::new();
        u16::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [4]);
    }

    #[test]
    fn serialize_header_u32() {
        let mut buf = Vec::new();
        u32::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [5]);
    }

    #[test]
    fn serialize_header_u64() {
        let mut buf = Vec::new();
        u64::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [6]);
    }

    #[test]
    fn serialize_header_u128() {
        let mut buf = Vec::new();
        u128::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [7]);
    }

    #[test]
    fn serialize_header_i8() {
        let mut buf = Vec::new();
        i8::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [8]);
    }

    #[test]
    fn serialize_header_i16() {
        let mut buf = Vec::new();
        i16::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [9]);
    }

    #[test]
    fn serialize_header_i32() {
        let mut buf = Vec::new();
        i32::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [10]);
    }

    #[test]
    fn serialize_header_i64() {
        let mut buf = Vec::new();
        i64::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [11]);
    }

    #[test]
    fn serialize_header_i128() {
        let mut buf = Vec::new();
        i128::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [12]);
    }

    #[test]
    fn serialize_header_f32() {
        let mut buf = Vec::new();
        f32::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [13]);
    }

    #[test]
    fn serialize_header_f64() {
        let mut buf = Vec::new();
        f64::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [14]);
    }

    #[test]
    fn serialize_header_big_uint() {
        let mut buf = Vec::new();
        BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [15]);
    }

    #[test]
    fn serialize_header_big_int() {
        let mut buf = Vec::new();
        BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [16]);
    }

    #[test]
    fn serialize_header_big_decimal() {
        let mut buf = Vec::new();
        BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [17]);
    }

    #[test]
    fn serialize_header_str() {
        let mut buf = Vec::new();
        <&str>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [18]);
    }

    #[test]
    fn serialize_header_string() {
        let mut buf = Vec::new();
        String::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [18]);
    }

    #[test]
    fn serialize_header_binary() {
        {
            let mut buf = Vec::new();
            Bytes::serialize_header(&mut buf).unwrap();
            assert_eq!(buf, [19]);
        }

        {
            let mut buf = Vec::new();
            ByteBuf::serialize_header(&mut buf).unwrap();
            assert_eq!(buf, [19]);
        }
    }

    #[test]
    fn serialize_header_vec() {
        let mut buf = Vec::new();
        Vec::<bool>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [20, 2]);
    }

    #[test]
    fn serialize_header_tuple() {
        let mut buf = Vec::new();
        <((), Option<()>, bool, u8)>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [21, 4, 0, 1, 0, 2, 3]);
    }

    #[test]
    fn serialize_header_map() {
        {
            let mut buf = Vec::new();
            BTreeMap::<String, bool>::serialize_header(&mut buf).unwrap();
            assert_eq!(buf, [23, 2]);
        }

        {
            let mut buf = Vec::new();
            HashMap::<String, bool>::serialize_header(&mut buf).unwrap();
            assert_eq!(buf, [23, 2]);
        }
    }

    #[test]
    fn serialize_header_date() {
        let mut buf = Vec::new();
        Date::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [25]);
    }

    #[test]
    fn serialize_header_date_time() {
        let mut buf = Vec::new();
        OffsetDateTime::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [26]);
    }
}
