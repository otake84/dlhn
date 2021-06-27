use std::io::{Result, Write};

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
    fn serialize_header<W: Write>(writer: W) -> Result<()>;
}

impl SerializeHeader for () {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[UNIT_CODE])
    }
}

impl<T: SerializeHeader> SerializeHeader for Option<T> {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[OPTIONAL_CODE])?;
        T::serialize_header(writer)
    }
}

impl SerializeHeader for bool {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[BOOLEAN_CODE])
    }
}

impl SerializeHeader for u8 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[UINT8_CODE])
    }
}

impl SerializeHeader for u16 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[UINT16_CODE])
    }
}

impl SerializeHeader for u32 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[UINT32_CODE])
    }
}

impl SerializeHeader for u64 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[UINT64_CODE])
    }
}

impl SerializeHeader for i8 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[INT8_CODE])
    }
}

impl SerializeHeader for i16 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[INT16_CODE])
    }
}

impl SerializeHeader for i32 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[INT32_CODE])
    }
}

impl SerializeHeader for i64 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[INT64_CODE])
    }
}

impl SerializeHeader for f32 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[FLOAT32_CODE])
    }
}

impl SerializeHeader for f64 {
    fn serialize_header<W: Write>(mut writer: W) -> Result<()> {
        writer.write_all(&[FLOAT64_CODE])
    }
}

#[cfg(test)]
mod tests {
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
}
