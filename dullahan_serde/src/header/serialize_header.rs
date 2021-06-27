use std::io::{Result, Write};

const UNIT_CODE: u8 = 0;
const OPTIONAL_CODE: u8 = 1;
const BOOLEAN_CODE: u8 = 2;
const UINT8_CODE: u8 = 3;
const UINT16_CODE: u8 = 4;
const UINT32_CODE: u8 = 5;
const UINT64_CODE: u8 = 6;
const VAR_UINT16_CODE: u8 = 7;
const VAR_UINT32_CODE: u8 = 8;
const VAR_UINT64_CODE: u8 = 9;
const INT8_CODE: u8 = 10;
const INT16_CODE: u8 = 11;
const INT32_CODE: u8 = 12;
const INT64_CODE: u8 = 13;
const VAR_INT16_CODE: u8 = 14;
const VAR_INT32_CODE: u8 = 15;
const VAR_INT64_CODE: u8 = 16;
const FLOAT32_CODE: u8 = 17;
const FLOAT64_CODE: u8 = 18;
const BIG_UINT_CODE: u8 = 19;
const BIG_INT_CODE: u8 = 20;
const BIG_DECIMAL_CODE: u8 = 21;
const STRING_CODE: u8 = 22;
const BINARY_CODE: u8 = 23;
const ARRAY_CODE: u8 = 24;
const TUPLE_CODE: u8 = 25;
const MAP_CODE: u8 = 26;
const DYNAMIC_MAP_CODE: u8 = 27;
const ENUM_CODE: u8 = 28;
const UNIT_ENUM_CODE: u8 = 29;
const DATE_CODE: u8 = 30;
const DATETIME_CODE: u8 = 31;
const EXTENSION8_CODE: u8 = 32;
const EXTENSION16_CODE: u8 = 33;
const EXTENSION32_CODE: u8 = 34;
const EXTENSION64_CODE: u8 = 35;
const EXTENSION_CODE: u8 = 36;

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
    fn serialize_header_16() {
        let mut buf = Vec::new();
        u16::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [4]);
    }
}
