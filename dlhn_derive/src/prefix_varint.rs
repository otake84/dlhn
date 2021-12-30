// https://chromium.googlesource.com/chromiumos/third_party/libtextclassifier/+/adbbad2e0138453af45cc08cb3d04317ae2b8ba1/utils/base/prefixvarint.h

use std::io::{Read, Result};

pub(crate) trait PrefixVarint<const N: usize>: Sized {
    const PREFIX_VARINT_BUF_SIZE: usize = N;

    fn encode_prefix_varint(self, buf: &mut [u8; N]) -> usize;
    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self>;

    fn encode_prefix_varint_vec(self) -> Vec<u8> {
        let mut buf = [0u8; N];
        let size = self.encode_prefix_varint(&mut buf);
        buf[..size].to_vec()
    }
}

fn decode_prefix(reader: &mut impl Read) -> Result<u8> {
    let mut prefix_buf = [0u8; 1];
    reader.read_exact(&mut prefix_buf)?;
    Ok(prefix_buf[0])
}

impl PrefixVarint<2> for u8 {
    const PREFIX_VARINT_BUF_SIZE: usize = 2;

    fn encode_prefix_varint(self, buf: &mut [u8; 2]) -> usize {
        if self < (1 << 7) {
            buf[0] = self;
            1
        } else {
            buf[0] = 0b_1000_0000;
            buf[1] = self;
            2
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)?;

        if prefix < 128 {
            Ok(prefix)
        } else {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            Ok(buf[0])
        }
    }
}

impl PrefixVarint<3> for u16 {
    const PREFIX_VARINT_BUF_SIZE: usize = 3;

    fn encode_prefix_varint(self, buf: &mut [u8; 3]) -> usize {
        let mut value = self;

        if value < 1 << 7 {
            buf[0] = value as u8;
            1
        } else if value < (1 << 14) {
            value <<= 2;
            buf[0] = (value as u8 >> 2) | 128;
            buf[1] = (value >> 8) as u8;
            2
        } else {
            buf[0] = 0b_1100_0000;
            buf[1..].copy_from_slice(value.to_le_bytes().as_slice());
            3
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)? as u16;

        if prefix < 128 {
            Ok(prefix)
        } else if prefix < 192 {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x3f) | ((buf[0] as u16) << 6))
        } else {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            Ok(u16::from_le_bytes(buf))
        }
    }
}

impl PrefixVarint<5> for u32 {
    const PREFIX_VARINT_BUF_SIZE: usize = 5;

    fn encode_prefix_varint(self, buf: &mut [u8; 5]) -> usize {
        let mut value = self;

        if value < (1 << 7) {
            buf[0] = value as u8;
            1
        } else if value < (1 << 14) {
            value <<= 2;
            buf[0] = (value as u8 >> 2) | 128;
            buf[1] = (value >> 8) as u8;
            2
        } else if value < (1 << 21) {
            value <<= 3;
            buf[0] = (value as u8 >> 3) | 192;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            3
        } else if value < (1 << 28) {
            value <<= 4;
            buf[0] = (value as u8 >> 4) | 224;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            4
        } else {
            buf[0] = 0b_1111_0000;
            buf[1..].copy_from_slice(value.to_le_bytes().as_slice());
            5
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)? as u32;

        if prefix < 128 {
            Ok(prefix)
        } else if prefix < 192 {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x3f) | ((buf[0] as u32) << 6))
        } else if prefix < 224 {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x1f) | ((u16::from_le_bytes(buf) as u32) << 5))
        } else if prefix < 240 {
            let mut buf = [0u8; 3];
            reader.read_exact(&mut buf)?;
            let mut v = buf[2] as u32;
            v = (v << 16) | (u16::from_le_bytes([buf[0], buf[1]]) as u32);
            Ok((prefix & 0x0f) | (v << 4))
        } else {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok(u32::from_le_bytes(buf))
        }
    }
}

impl PrefixVarint<9> for u64 {
    const PREFIX_VARINT_BUF_SIZE: usize = 9;

    fn encode_prefix_varint(self, buf: &mut [u8; 9]) -> usize {
        let mut value = self;

        if value < (1 << 7) {
            buf[0] = value as u8;
            1
        } else if value < (1 << 14) {
            value <<= 2;
            buf[0] = (value as u8 >> 2) | 128;
            buf[1] = (value >> 8) as u8;
            2
        } else if value < (1 << 21) {
            value <<= 3;
            buf[0] = (value as u8 >> 3) | 192;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            3
        } else if value < (1 << 28) {
            value <<= 4;
            buf[0] = (value as u8 >> 4) | 224;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            4
        } else if value < (1 << 35) {
            value <<= 5;
            buf[0] = (value as u8 >> 5) | 240;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            buf[4] = (value >> 32) as u8;
            5
        } else if value < (1 << 42) {
            value <<= 6;
            buf[0] = (value as u8 >> 6) | 248;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            buf[4] = (value >> 32) as u8;
            buf[5] = (value >> 40) as u8;
            6
        } else if value < (1 << 49) {
            value <<= 7;
            buf[0] = (value as u8 >> 7) | 252;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            buf[4] = (value >> 32) as u8;
            buf[5] = (value >> 40) as u8;
            buf[6] = (value >> 48) as u8;
            7
        } else if value < (1 << 56) {
            buf[0] = 254;
            buf[1] = value as u8;
            buf[2] = (value >> 8) as u8;
            buf[3] = (value >> 16) as u8;
            buf[4] = (value >> 24) as u8;
            buf[5] = (value >> 32) as u8;
            buf[6] = (value >> 40) as u8;
            buf[7] = (value >> 48) as u8;
            8
        } else {
            buf[0] = 255;
            buf[1..].copy_from_slice(&value.to_le_bytes());
            9
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)? as u64;

        if prefix < 128 {
            Ok(prefix)
        } else if prefix < 192 {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x3f) | ((buf[0] as u64) << 6))
        } else if prefix < 224 {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x1f) | ((u16::from_le_bytes(buf) as u64) << 5))
        } else if prefix < 240 {
            let mut buf = [0u8; 3];
            reader.read_exact(&mut buf)?;
            let mut v = buf[2] as u64;
            v = (v << 16) | (u16::from_le_bytes([buf[0], buf[1]]) as u64);
            Ok((prefix & 0x0f) | (v << 4))
        } else if prefix < 248 {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok((prefix & 0x07) | ((u32::from_le_bytes(buf) as u64) << 3))
        } else if prefix < 252 {
            let mut buf = [0u8; 5];
            reader.read_exact(&mut buf)?;
            let mut v = buf[4] as u64;
            v = (v << 32) | (u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64);
            Ok((prefix & 0x03) | (v << 2))
        } else if prefix < 254 {
            let mut buf = [0u8; 6];
            reader.read_exact(&mut buf)?;
            let mut v = u16::from_le_bytes([buf[4], buf[5]]) as u64;
            v = (v << 32) | (u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as u64);
            Ok((prefix & 0x01) | (v << 1))
        } else if prefix < 255 {
            let mut buf = [0u8; 8];
            buf[0] = prefix as u8;
            reader.read_exact(&mut buf[1..8])?;
            Ok(u64::from_le_bytes(buf) >> 8)
        } else {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixVarint;
    use std::array::IntoIter;

    #[test]
    fn decode_u8() {
        IntoIter::new([0u8, (1 << 7) - 1, (1 << 7), u8::MAX]).for_each(|v| {
            let mut buf = [0u8; u8::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u8::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u16() {
        IntoIter::new([
            0u16,
            (1 << 7) - 1,
            (1 << 7),
            (1 << 14) - 1,
            1 << 14,
            u16::MAX,
        ])
        .for_each(|v| {
            let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u16::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u32() {
        IntoIter::new([
            0u32,
            (1 << 7) - 1,
            (1 << 7),
            (1 << 14) - 1,
            1 << 14,
            (1 << 21) - 1,
            1 << 21,
            (1 << 28) - 1,
            1 << 28,
            u32::MAX,
        ])
        .for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u32::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u64() {
        IntoIter::new([
            0u64,
            (1 << 7) - 1,
            (1 << 7),
            (1 << 14) - 1,
            1 << 14,
            (1 << 21) - 1,
            1 << 21,
            (1 << 28) - 1,
            1 << 28,
            (1 << 35) - 1,
            1 << 35,
            (1 << 42) - 1,
            1 << 42,
            (1 << 49) - 1,
            1 << 49,
            (1 << 56) - 1,
            1 << 56,
            u64::MAX,
        ])
        .for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u64::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }
}
