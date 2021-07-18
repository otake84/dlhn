use std::io::{Read, Result, Error, ErrorKind};

// https://en.wikipedia.org/wiki/LEB128
// https://github.com/stoklund/varint/blob/master/leb128.cpp

pub(crate) trait Leb128<const N: usize>: Sized {
    const LEB128_BUF_SIZE: usize = N;

    fn encode_leb128(self, buf: &mut [u8; N]) -> usize;
    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self>;

    fn encode_leb128_vec(self) -> Vec<u8> {
        let mut buf = [0u8; N];
        let size = self.encode_leb128(&mut buf);
        buf[..size].to_vec()
    }
}

impl Leb128<10> for usize {
    fn encode_leb128(mut self, buf: &mut [u8; Self::LEB128_BUF_SIZE]) -> usize {
        let mut bytes = 0;
        while self > 127 {
            buf[bytes] = (self | 0x80) as u8;
            bytes += 1;
            self >>= 7;
        }
        buf[bytes] = self as u8;

        bytes + 1
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        if buf[0] < 128 {
            Ok(value)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

impl Leb128<10> for u64 {
    fn encode_leb128(mut self, buf: &mut [u8; Self::LEB128_BUF_SIZE]) -> usize {
        let mut bytes = 0;
        while self > 127 {
            buf[bytes] = (self | 0x80) as u8;
            bytes += 1;
            self >>= 7;
        }
        buf[bytes] = self as u8;

        bytes + 1
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        if buf[0] < 128 {
            Ok(value)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

impl Leb128<5> for u32 {
    fn encode_leb128(mut self, buf: &mut [u8; Self::LEB128_BUF_SIZE]) -> usize {
        let mut bytes = 0;
        while self > 127 {
            buf[bytes] = (self | 0x80) as u8;
            bytes += 1;
            self >>= 7;
        }
        buf[bytes] = self as u8;

        bytes + 1
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        if buf[0] < 128 {
            Ok(value)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

impl Leb128<3> for u16 {
    fn encode_leb128(mut self, buf: &mut [u8; Self::LEB128_BUF_SIZE]) -> usize {
        let mut bytes = 0;
        while self > 127 {
            buf[bytes] = (self | 0x80) as u8;
            bytes += 1;
            self >>= 7;
        }
        buf[bytes] = self as u8;

        bytes + 1
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        if buf[0] < 128 {
            Ok(value)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

impl Leb128<2> for u8 {
    fn encode_leb128(mut self, buf: &mut [u8; Self::LEB128_BUF_SIZE]) -> usize {
        let mut bytes = 0;
        while self > 127 {
            buf[bytes] = (self | 0x80) as u8;
            bytes += 1;
            self >>= 7;
        }
        buf[bytes] = self as u8;

        bytes + 1
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        shift += 7;
        if buf[0] < 128 {
            return Ok(value);
        }

        reader.read_exact(&mut buf)?;
        value |= (buf[0] as Self & 0x7f) << shift;
        if buf[0] < 128 {
            Ok(value)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Leb128;

    mod u8 {
        use super::*;

        #[test]
        fn decode_leb128_u8_min() {
            let mut buf = [0u8; u8::LEB128_BUF_SIZE];
            let size = u8::MIN.encode_leb128(&mut buf);
            assert_eq!(u8::MIN, u8::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u8_max() {
            let mut buf = [0u8; u8::LEB128_BUF_SIZE];
            let size = u8::MAX.encode_leb128(&mut buf);
            assert_eq!(u8::MAX, u8::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u16_max_is_err() {
            let mut buf = [0u8; u16::LEB128_BUF_SIZE];
            let size = u16::MAX.encode_leb128(&mut buf);
            assert!(u8::decode_leb128(&mut buf[..size].as_ref()).is_err());
        }

        #[test]
        fn decode_leb128_buf_0xff_2_is_err() {
            let buf = [0xffu8; 2];
            assert!(u8::decode_leb128(&mut buf.as_ref()).is_err());
        }
    }

    mod u16 {
        use super::*;

        #[test]
        fn decode_leb128_u16_min() {
            let mut buf = [0u8; u16::LEB128_BUF_SIZE];
            let size = u16::MIN.encode_leb128(&mut buf);
            assert_eq!(u16::MIN, u16::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u16_max() {
            let mut buf = [0u8; u16::LEB128_BUF_SIZE];
            let size = u16::MAX.encode_leb128(&mut buf);
            assert_eq!(u16::MAX, u16::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u32_max_is_err() {
            let mut buf = [0u8; u32::LEB128_BUF_SIZE];
            let size = u32::MAX.encode_leb128(&mut buf);
            assert!(u16::decode_leb128(&mut buf[..size].as_ref()).is_err());
        }

        #[test]
        fn decode_leb128_buf_0xff_3_is_err() {
            let buf = [0xffu8; 3];
            assert!(u16::decode_leb128(&mut buf.as_ref()).is_err());
        }
    }

    mod u32 {
        use super::*;

        #[test]
        fn decode_leb128_u32_min() {
            let mut buf = [0u8; u32::LEB128_BUF_SIZE];
            let size = u32::MIN.encode_leb128(&mut buf);
            assert_eq!(u32::MIN, u32::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u32_max() {
            let mut buf = [0u8; u32::LEB128_BUF_SIZE];
            let size = u32::MAX.encode_leb128(&mut buf);
            assert_eq!(u32::MAX, u32::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u64_max_is_err() {
            let mut buf = [0u8; u64::LEB128_BUF_SIZE];
            let size = u64::MAX.encode_leb128(&mut buf);
            assert!(u32::decode_leb128(&mut buf[..size].as_ref()).is_err());
        }

        #[test]
        fn decode_leb128_buf_0xff_5_is_err() {
            let buf = [0xffu8; 5];
            assert!(u32::decode_leb128(&mut buf.as_ref()).is_err());
        }
    }

    mod u64 {
        use super::*;

        #[test]
        fn decode_leb128_u64_min() {
            let mut buf = [0u8; u64::LEB128_BUF_SIZE];
            let size = u64::MIN.encode_leb128(&mut buf);
            assert_eq!(u64::MIN, u64::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u64_max() {
            let mut buf = [0u8; u64::LEB128_BUF_SIZE];
            let size = u64::MAX.encode_leb128(&mut buf);
            assert_eq!(u64::MAX, u64::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_buf_0xff_10_is_err() {
            let buf = [0xffu8; 10];
            assert!(u64::decode_leb128(&mut buf.as_ref()).is_err());
        }
    }
}
