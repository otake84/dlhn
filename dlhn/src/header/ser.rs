use super::Header;
use crate::{
    big_decimal::BigDecimal, big_int::BigInt, big_uint::BigUint, date::Date, date_time::DateTime,
    prefix_varint::PrefixVarint,
};
use serde_bytes::{ByteBuf, Bytes};
use std::{
    collections::{BTreeMap, HashMap},
    io::{Result, Write},
};

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

// impl SerializeHeader for u128 {
//     fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
//         writer.write_all(&[super::UINT128_CODE])
//     }
// }

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

// impl SerializeHeader for i128 {
//     fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
//         writer.write_all(&[super::INT128_CODE])
//     }
// }

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

#[cfg(all(feature = "num-traits", feature = "num-bigint"))]
impl SerializeHeader for num_bigint::BigUint {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_UINT_CODE])
    }
}

impl SerializeHeader for BigInt {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_INT_CODE])
    }
}

#[cfg(all(feature = "num-traits", feature = "num-bigint"))]
impl SerializeHeader for num_bigint::BigInt {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_INT_CODE])
    }
}

impl SerializeHeader for BigDecimal {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::BIG_DECIMAL_CODE])
    }
}

#[cfg(feature = "bigdecimal")]
impl SerializeHeader for bigdecimal::BigDecimal {
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

#[cfg(feature = "time")]
impl SerializeHeader for time::Date {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::DATE_CODE])
    }
}

impl SerializeHeader for DateTime {
    fn serialize_header<W: Write>(writer: &mut W) -> Result<()> {
        writer.write_all(&[super::DATETIME_CODE])
    }
}

#[cfg(feature = "time")]
impl SerializeHeader for time::OffsetDateTime {
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
                    let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
                    let size = ($len as u64).encode_prefix_varint(&mut buf);
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

impl Header {
    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Header::Unit => <()>::serialize_header(writer),
            Header::Optional(inner) => {
                Self::serialize_inner_box(super::OPTIONAL_CODE, inner, writer)
            }
            Header::Boolean => bool::serialize_header(writer),
            Header::UInt8 => u8::serialize_header(writer),
            Header::UInt16 => u16::serialize_header(writer),
            Header::UInt32 => u32::serialize_header(writer),
            Header::UInt64 => u64::serialize_header(writer),
            // Header::UInt128 => u128::serialize_header(writer),
            Header::Int8 => i8::serialize_header(writer),
            Header::Int16 => i16::serialize_header(writer),
            Header::Int32 => i32::serialize_header(writer),
            Header::Int64 => i64::serialize_header(writer),
            // Header::Int128 => i128::serialize_header(writer),
            Header::Float32 => f32::serialize_header(writer),
            Header::Float64 => f64::serialize_header(writer),
            Header::BigUInt => BigUint::serialize_header(writer),
            Header::BigInt => BigInt::serialize_header(writer),
            Header::BigDecimal => BigDecimal::serialize_header(writer),
            Header::String => String::serialize_header(writer),
            Header::Binary => Bytes::serialize_header(writer),
            Header::Array(inner) => Self::serialize_inner_box(super::ARRAY_CODE, inner, writer),
            Header::Tuple(inner) => Self::serialize_inner_vec(super::TUPLE_CODE, inner, writer),
            Header::Struct(inner) => Self::serialize_inner_vec(super::STRUCT_CODE, inner, writer),
            Header::Map(inner) => Self::serialize_inner_box(super::MAP_CODE, inner, writer),
            Header::Enum(inner) => Self::serialize_inner_vec(super::ENUM_CODE, inner, writer),
            Header::Date => Date::serialize_header(writer),
            Header::DateTime => DateTime::serialize_header(writer),
            Header::Extension8(i) => Self::serialize_extension(super::EXTENSION8_CODE, *i, writer),
            Header::Extension16(i) => {
                Self::serialize_extension(super::EXTENSION16_CODE, *i, writer)
            }
            Header::Extension32(i) => {
                Self::serialize_extension(super::EXTENSION32_CODE, *i, writer)
            }
            Header::Extension64(i) => {
                Self::serialize_extension(super::EXTENSION64_CODE, *i, writer)
            }
            Header::Extension128(i) => {
                Self::serialize_extension(super::EXTENSION128_CODE, *i, writer)
            }
            Header::Extension(i) => Self::serialize_extension(super::EXTENSION_CODE, *i, writer),
        }
    }

    fn serialize_inner_box<W: Write>(code: u8, inner: &Header, writer: &mut W) -> Result<()> {
        writer.write_all(&[code])?;
        inner.serialize(writer)
    }

    fn serialize_inner_vec<W: Write>(code: u8, inner: &Vec<Header>, writer: &mut W) -> Result<()> {
        writer.write_all(&[code])?;
        let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
        let size = (inner.len() as u64).encode_prefix_varint(&mut buf);
        writer.write_all(&buf[..size])?;
        for v in inner {
            v.serialize(writer)?
        }
        Ok(())
    }

    fn serialize_extension<W: Write>(code: u8, i: u64, writer: &mut W) -> Result<()> {
        writer.write_all(&[code])?;
        let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
        let size = i.encode_prefix_varint(&mut buf);
        writer.write_all(&buf[..size])
    }
}

#[cfg(test)]
mod tests {
    use super::SerializeHeader;
    use crate::{
        big_decimal::BigDecimal, big_int::BigInt, big_uint::BigUint, date::Date,
        date_time::DateTime,
    };
    use serde_bytes::{ByteBuf, Bytes};
    use std::collections::{BTreeMap, HashMap};

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

    // #[test]
    // fn serialize_header_u128() {
    //     let mut buf = Vec::new();
    //     u128::serialize_header(&mut buf).unwrap();
    //     assert_eq!(buf, [7]);
    // }

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

    // #[test]
    // fn serialize_header_i128() {
    //     let mut buf = Vec::new();
    //     i128::serialize_header(&mut buf).unwrap();
    //     assert_eq!(buf, [12]);
    // }

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

    #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
    #[test]
    fn serialize_header_big_uint2() {
        let mut buf = Vec::new();
        num_bigint::BigUint::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [15]);
    }

    #[test]
    fn serialize_header_big_int() {
        let mut buf = Vec::new();
        BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [16]);
    }

    #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
    #[test]
    fn serialize_header_big_int2() {
        let mut buf = Vec::new();
        num_bigint::BigInt::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [16]);
    }

    #[test]
    fn serialize_header_big_decimal() {
        let mut buf = Vec::new();
        BigDecimal::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [17]);
    }

    #[cfg(feature = "bigdecimal")]
    #[test]
    fn serialize_header_big_decimal2() {
        let mut buf = Vec::new();
        bigdecimal::BigDecimal::serialize_header(&mut buf).unwrap();
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

    #[cfg(feature = "time")]
    #[test]
    fn serialize_header_date2() {
        let mut buf = Vec::new();
        time::Date::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [25]);
    }

    #[test]
    fn serialize_header_date_time() {
        let mut buf = Vec::new();
        DateTime::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [26]);
    }

    #[cfg(feature = "time")]
    #[test]
    fn serialize_header_date_time2() {
        let mut buf = Vec::new();
        time::OffsetDateTime::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [26]);
    }

    mod header {
        use crate::{
            big_decimal::BigDecimal,
            big_int::BigInt,
            big_uint::BigUint,
            date::Date,
            date_time::DateTime,
            header::{ser::SerializeHeader, Header},
        };
        use serde_bytes::ByteBuf;
        use std::collections::BTreeMap;

        #[test]
        fn serialize_unit() {
            assert_eq!(serialize(Header::Unit), serialize_header::<()>());
        }

        #[test]
        fn serialize_optional() {
            assert_eq!(
                serialize(Header::Optional(Box::new(Header::Boolean))),
                serialize_header::<Option<bool>>()
            );
        }

        #[test]
        fn serialize_bool() {
            assert_eq!(serialize(Header::Boolean), serialize_header::<bool>());
        }

        #[test]
        fn serialize_uint8() {
            assert_eq!(serialize(Header::UInt8), serialize_header::<u8>());
        }

        #[test]
        fn serialize_uint16() {
            assert_eq!(serialize(Header::UInt16), serialize_header::<u16>());
        }

        #[test]
        fn serialize_uint32() {
            assert_eq!(serialize(Header::UInt32), serialize_header::<u32>());
        }

        #[test]
        fn serialize_uint64() {
            assert_eq!(serialize(Header::UInt64), serialize_header::<u64>());
        }

        // #[test]
        // fn serialize_uint128() {
        //     assert_eq!(serialize(Header::UInt128), serialize_header::<u128>());
        // }

        #[test]
        fn serialize_int8() {
            assert_eq!(serialize(Header::Int8), serialize_header::<i8>());
        }

        #[test]
        fn serialize_int16() {
            assert_eq!(serialize(Header::Int16), serialize_header::<i16>());
        }

        #[test]
        fn serialize_int32() {
            assert_eq!(serialize(Header::Int32), serialize_header::<i32>());
        }

        #[test]
        fn serialize_int64() {
            assert_eq!(serialize(Header::Int64), serialize_header::<i64>());
        }

        // #[test]
        // fn serialize_int128() {
        //     assert_eq!(serialize(Header::Int128), serialize_header::<i128>());
        // }

        #[test]
        fn serialize_float32() {
            assert_eq!(serialize(Header::Float32), serialize_header::<f32>());
        }

        #[test]
        fn serialize_float64() {
            assert_eq!(serialize(Header::Float64), serialize_header::<f64>());
        }

        #[test]
        fn serialize_big_uint() {
            assert_eq!(serialize(Header::BigUInt), serialize_header::<BigUint>());
        }

        #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
        #[test]
        fn serialize_big_uint2() {
            assert_eq!(
                serialize(Header::BigUInt),
                serialize_header::<num_bigint::BigUint>()
            );
        }

        #[test]
        fn serialize_big_int() {
            assert_eq!(serialize(Header::BigInt), serialize_header::<BigInt>());
        }

        #[cfg(all(feature = "num-traits", feature = "num-bigint"))]
        #[test]
        fn serialize_big_int2() {
            assert_eq!(
                serialize(Header::BigInt),
                serialize_header::<num_bigint::BigInt>()
            );
        }

        #[test]
        fn serialize_big_decimal() {
            assert_eq!(
                serialize(Header::BigDecimal),
                serialize_header::<BigDecimal>()
            );
        }

        #[cfg(feature = "bigdecimal")]
        #[test]
        fn serialize_big_decimal2() {
            assert_eq!(
                serialize(Header::BigDecimal),
                serialize_header::<bigdecimal::BigDecimal>()
            );
        }

        #[test]
        fn serialize_string() {
            assert_eq!(serialize(Header::String), serialize_header::<String>());
        }

        #[test]
        fn serialize_binary() {
            assert_eq!(serialize(Header::Binary), serialize_header::<ByteBuf>());
        }

        #[test]
        fn serialize_array() {
            assert_eq!(
                serialize(Header::Array(Box::new(Header::Boolean))),
                serialize_header::<Vec<bool>>()
            );
        }

        #[test]
        fn serialize_tuple() {
            assert_eq!(
                serialize(Header::Tuple(vec![Header::Boolean, Header::UInt8])),
                serialize_header::<(bool, u8)>()
            );
        }

        #[test]
        fn serialize_struct() {
            assert_eq!(
                serialize(Header::Struct(vec![Header::Boolean, Header::UInt8])),
                [22, 2, 2, 3]
            );
        }

        #[test]
        fn serialize_map() {
            assert_eq!(
                serialize(Header::Map(Box::new(Header::Boolean))),
                serialize_header::<BTreeMap<String, bool>>()
            );
        }

        #[test]
        fn serialize_enum() {
            assert_eq!(
                serialize(Header::Enum(vec![
                    Header::Boolean,
                    Header::UInt8,
                    Header::Tuple(vec![Header::Boolean, Header::UInt8])
                ])),
                [24, 3, 2, 3, 21, 2, 2, 3]
            );
        }

        #[test]
        fn serialize_date() {
            assert_eq!(serialize(Header::Date), serialize_header::<Date>());
        }

        #[cfg(feature = "time")]
        #[test]
        fn serialize_date2() {
            assert_eq!(serialize(Header::Date), serialize_header::<time::Date>());
        }

        #[test]
        fn serialize_date_time() {
            assert_eq!(serialize(Header::DateTime), serialize_header::<DateTime>());
        }

        #[cfg(feature = "time")]
        #[test]
        fn serialize_date_time2() {
            assert_eq!(
                serialize(Header::DateTime),
                serialize_header::<time::OffsetDateTime>()
            );
        }

        #[test]
        fn serialize_extension8() {
            assert_eq!(serialize(Header::Extension8(123)), [27, 123]);
        }

        #[test]
        fn serialize_extension16() {
            assert_eq!(serialize(Header::Extension16(123)), [28, 123]);
        }

        #[test]
        fn serialize_extension32() {
            assert_eq!(serialize(Header::Extension32(123)), [29, 123]);
        }

        #[test]
        fn serialize_extension64() {
            assert_eq!(serialize(Header::Extension64(123)), [30, 123]);
        }

        #[test]
        fn serialize_extension128() {
            assert_eq!(serialize(Header::Extension128(123)), [31, 123]);
        }

        #[test]
        fn serialize_extension() {
            assert_eq!(serialize(Header::Extension(123)), [32, 123]);
        }

        fn serialize_header<T: SerializeHeader>() -> Vec<u8> {
            let mut buf = Vec::new();
            T::serialize_header(&mut buf).unwrap();
            buf
        }

        fn serialize(header: Header) -> Vec<u8> {
            let mut buf = Vec::new();
            header.serialize(&mut buf).unwrap();
            buf
        }
    }
}
