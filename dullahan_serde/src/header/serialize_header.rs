use std::io::{Result, Write};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint};
use serde_bytes::Bytes;
use time::Date;
use crate::leb128::Leb128;

const UNIT_CODE: u8 = 0;
const OPTIONAL_CODE: u8 = 1;
const BOOLEAN_CODE: u8 = 2;
const UINT8_CODE: u8 = 3;
const UINT16_CODE: u8 = 4;
const UINT32_CODE: u8 = 5;
const UINT64_CODE: u8 = 6;
const INT8_CODE: u8 = 7;
const INT16_CODE: u8 = 8;
const INT32_CODE: u8 = 9;
const INT64_CODE: u8 = 10;
const FLOAT32_CODE: u8 = 11;
const FLOAT64_CODE: u8 = 12;
const BIG_UINT_CODE: u8 = 13;
const BIG_INT_CODE: u8 = 14;
const BIG_DECIMAL_CODE: u8 = 15;
const STRING_CODE: u8 = 16;
const BINARY_CODE: u8 = 17;
const ARRAY_CODE: u8 = 18;
const TUPLE_CODE: u8 = 19;
const MAP_CODE: u8 = 20;
const DYNAMIC_MAP_CODE: u8 = 21;
const ENUM_CODE: u8 = 22;
const UNIT_ENUM_CODE: u8 = 23;
const DATE_CODE: u8 = 24;
const DATETIME_CODE: u8 = 25;
const EXTENSION8_CODE: u8 = 26;
const EXTENSION16_CODE: u8 = 27;
const EXTENSION32_CODE: u8 = 28;
const EXTENSION64_CODE: u8 = 29;
const EXTENSION_CODE: u8 = 30;

trait SerializeHeader {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()>;
}

impl SerializeHeader for () {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[UNIT_CODE])
    }
}

impl<T: SerializeHeader> SerializeHeader for Option<T> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[OPTIONAL_CODE])?;
        T::serialize_header(writer)
    }
}

impl SerializeHeader for bool {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[BOOLEAN_CODE])
    }
}

impl SerializeHeader for u8 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[UINT8_CODE])
    }
}

impl SerializeHeader for u16 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[UINT16_CODE])
    }
}

impl SerializeHeader for u32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[UINT32_CODE])
    }
}

impl SerializeHeader for u64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[UINT64_CODE])
    }
}

impl SerializeHeader for i8 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[INT8_CODE])
    }
}

impl SerializeHeader for i16 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[INT16_CODE])
    }
}

impl SerializeHeader for i32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[INT32_CODE])
    }
}

impl SerializeHeader for i64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[INT64_CODE])
    }
}

impl SerializeHeader for f32 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[FLOAT32_CODE])
    }
}

impl SerializeHeader for f64 {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[FLOAT64_CODE])
    }
}

impl SerializeHeader for BigUint {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[BIG_UINT_CODE])
    }
}

impl SerializeHeader for BigInt {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[BIG_INT_CODE])
    }
}

impl SerializeHeader for BigDecimal {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[BIG_DECIMAL_CODE])
    }
}

impl SerializeHeader for &str {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[STRING_CODE])
    }
}

impl SerializeHeader for String {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[STRING_CODE])
    }
}

impl SerializeHeader for Bytes {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[BINARY_CODE])
    }
}

impl<T: SerializeHeader> SerializeHeader for Vec<T> {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[ARRAY_CODE])?;
        T::serialize_header(writer)
    }
}

impl SerializeHeader for Date {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[DATE_CODE])
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
                    writer.write_all(&[TUPLE_CODE])?;
                    let (buf, size) = ($len as usize).encode_leb128();
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
    use bigdecimal::BigDecimal;
    use num_bigint::{BigInt, BigUint};
    use serde_bytes::Bytes;
    use time::Date;
    use super::SerializeHeader;

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
    fn serialize_header_i8() {
        let mut buf = Vec::new();
        i8::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [7]);
    }

    #[test]
    fn serialize_header_i16() {
        let mut buf = Vec::new();
        i16::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [8]);
    }

    #[test]
    fn serialize_header_i32() {
        let mut buf = Vec::new();
        i32::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [9]);
    }

    #[test]
    fn serialize_header_i64() {
        let mut buf = Vec::new();
        i64::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [10]);
    }

    #[test]
    fn serialize_header_f32() {
        let mut buf = Vec::new();
        f32::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [11]);
    }

    #[test]
    fn serialize_header_f64() {
        let mut buf = Vec::new();
        f64::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [12]);
    }

    #[test]
    fn serialize_header_big_uint() {
        let mut buf = Vec::new();
        BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [13]);
    }

    #[test]
    fn serialize_header_big_int() {
        let mut buf = Vec::new();
        BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [14]);
    }

    #[test]
    fn serialize_header_big_decimal() {
        let mut buf = Vec::new();
        BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [15]);
    }

    #[test]
    fn serialize_header_str() {
        let mut buf = Vec::new();
        <&str>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [16]);
    }

    #[test]
    fn serialize_header_string() {
        let mut buf = Vec::new();
        String::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [16]);
    }

    #[test]
    fn serialize_header_binary() {
        let mut buf = Vec::new();
        Bytes::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [17]);
    }

    #[test]
    fn serialize_header_vec() {
        let mut buf = Vec::new();
        Vec::<bool>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [18, 2]);
    }

    #[test]
    fn serialize_header_tuple() {
        let mut buf = Vec::new();
        <((), Option<()>, bool, u8)>::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [19, 4, 0, 1, 0, 2, 3]);
    }

    #[test]
    fn serialize_header_date() {
        let mut buf = Vec::new();
        Date::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [24]);
    }
}
