use std::io::{Read, Result, Error, ErrorKind};

// https://en.wikipedia.org/wiki/LEB128
// https://github.com/stoklund/varint/blob/master/leb128.cpp

pub(crate) trait Leb128<const N: usize>: Sized {
    fn encode_leb128(&self) -> ([u8; N], usize);
    fn encode_leb128_vec(&self) -> Vec<u8>;
    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self>;
}

struct Buf<const N: usize> {
    buf: [u8; N],
    bytes: usize,
}

impl<const N: usize> Buf<N> {
    fn new() -> Self {
        Self {
            buf: [0; N],
            bytes: 0,
        }
    }

    fn write(&mut self, byte: u8) {
        self.buf[self.bytes] = byte;
        self.bytes += 1;
    }
}

impl Leb128<10> for usize {
    fn encode_leb128(&self) -> ([u8; 10], usize) {
        let mut value = *self;
        let mut buf = Buf::new();
        while value > 127 {
            buf.write((value | 0x80) as u8);
            value >>= 7;
        }
        buf.write(value as u8);

        (buf.buf, buf.bytes)
    }

    fn encode_leb128_vec(&self) -> Vec<u8> {
        let (buf, size) = self.encode_leb128();
        buf[0..size].to_vec()
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;
        let mut i = 0;

        while {
            if i >= 10 {
                Err(Error::new(ErrorKind::InvalidData, "Invalid data"))?;
            }
            reader.read_exact(&mut buf)?;
            value |= (buf[0] as Self & 0x7f) << shift;
            shift += 7;
            i += 1;
            buf[0] >= 128
        } {}

        Ok(value)
    }
}

impl Leb128<10> for u64 {
    fn encode_leb128(&self) -> ([u8; 10], usize) {
        let mut value = *self;
        let mut buf = Buf::new();
        while value > 127 {
            buf.write((value | 0x80) as u8);
            value >>= 7;
        }
        buf.write(value as u8);

        (buf.buf, buf.bytes)
    }

    fn encode_leb128_vec(&self) -> Vec<u8> {
        let (buf, size) = self.encode_leb128();
        buf[0..size].to_vec()
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;
        let mut i = 0;

        while {
            if i >= 10 {
                Err(Error::new(ErrorKind::InvalidData, "Invalid data"))?;
            }
            reader.read_exact(&mut buf)?;
            value |= (buf[0] as Self & 0x7f) << shift;
            shift += 7;
            i += 1;
            buf[0] >= 128
        } {}

        Ok(value)
    }
}

impl Leb128<5> for u32 {
    fn encode_leb128(&self) -> ([u8; 5], usize) {
        let mut value = *self;
        let mut buf = Buf::new();
        while value > 127 {
            buf.write((value | 0x80) as u8);
            value >>= 7;
        }
        buf.write(value as u8);

        (buf.buf, buf.bytes)
    }

    fn encode_leb128_vec(&self) -> Vec<u8> {
        let (buf, size) = self.encode_leb128();
        buf[0..size].to_vec()
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;
        let mut i = 0;

        while {
            if i >= 5 {
                Err(Error::new(ErrorKind::InvalidData, "Invalid data"))?;
            }
            reader.read_exact(&mut buf)?;
            value |= (buf[0] as Self & 0x7f) << shift;
            shift += 7;
            i += 1;
            buf[0] >= 128
        } {}

        Ok(value)
    }
}

impl Leb128<3> for u16 {
    fn encode_leb128(&self) -> ([u8; 3], usize) {
        let mut value = *self;
        let mut buf = Buf::new();
        while value > 127 {
            buf.write((value | 0x80) as u8);
            value >>= 7;
        }
        buf.write(value as u8);

        (buf.buf, buf.bytes)
    }

    fn encode_leb128_vec(&self) -> Vec<u8> {
        let (buf, size) = self.encode_leb128();
        buf[0..size].to_vec()
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;
        let mut i = 0;

        while {
            if i >= 3 {
                Err(Error::new(ErrorKind::InvalidData, "Invalid data"))?;
            }
            reader.read_exact(&mut buf)?;
            value |= (buf[0] as Self & 0x7f) << shift;
            shift += 7;
            i += 1;
            buf[0] >= 128
        } {}

        Ok(value)
    }
}

impl Leb128<2> for u8 {
    fn encode_leb128(&self) -> ([u8; 2], usize) {
        let mut value = *self;
        let mut buf = Buf::new();
        while value > 127 {
            buf.write((value | 0x80) as u8);
            value >>= 7;
        }
        buf.write(value as u8);

        (buf.buf, buf.bytes)
    }

    fn encode_leb128_vec(&self) -> Vec<u8> {
        let (buf, size) = self.encode_leb128();
        buf[0..size].to_vec()
    }

    fn decode_leb128<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        let mut value: Self = 0;
        let mut shift = 0;
        let mut i = 0;

        while {
            if i >= 2 {
                Err(Error::new(ErrorKind::InvalidData, "Invalid data"))?;
            }
            reader.read_exact(&mut buf)?;
            value |= (buf[0] as Self & 0x7f) << shift;
            shift += 7;
            i += 1;
            buf[0] >= 128
        } {}

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Leb128;

    mod u8 {
        use super::*;

        #[test]
        fn decode_leb128_u8_min() {
            let (buf, size) = u8::MIN.encode_leb128();
            assert_eq!(u8::MIN, u8::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u8_max() {
            let (buf, size) = u8::MAX.encode_leb128();
            assert_eq!(u8::MAX, u8::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u16_max_is_err() {
            let (buf, size) = u16::MAX.encode_leb128();
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
            let (buf, size) = u16::MIN.encode_leb128();
            assert_eq!(u16::MIN, u16::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u16_max() {
            let (buf, size) = u16::MAX.encode_leb128();
            assert_eq!(u16::MAX, u16::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u32_max_is_err() {
            let (buf, size) = u32::MAX.encode_leb128();
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
            let (buf, size) = u32::MIN.encode_leb128();
            assert_eq!(u32::MIN, u32::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u32_max() {
            let (buf, size) = u32::MAX.encode_leb128();
            assert_eq!(u32::MAX, u32::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u64_max_is_err() {
            let (buf, size) = u64::MAX.encode_leb128();
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
            let (buf, size) = u64::MIN.encode_leb128();
            assert_eq!(u64::MIN, u64::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_u64_max() {
            let (buf, size) = u64::MAX.encode_leb128();
            assert_eq!(u64::MAX, u64::decode_leb128(&mut buf[..size].as_ref()).unwrap());
        }

        #[test]
        fn decode_leb128_buf_0xff_10_is_err() {
            let buf = [0xffu8; 10];
            assert!(u64::decode_leb128(&mut buf.as_ref()).is_err());
        }
    }
}
