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

        match value.leading_zeros() {
            0 | 1 => {
                buf[0] = 0b_1100_0000;
                buf[1..].copy_from_slice(value.to_le_bytes().as_slice());
                3
            }
            2..=8 => {
                value <<= 2;
                buf[0] = (value as u8 >> 2) | 128;
                buf[1] = (value >> 8) as u8;
                2
            }
            _ => {
                buf[0] = value as u8;
                1
            }
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)?;

        match prefix.leading_ones() as u8 {
            0 => Ok(prefix as u16),
            1 => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u16 & 0x3f) | ((buf[0] as u16) << 6))
            }
            _ => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                Ok(u16::from_le_bytes(buf))
            }
        }
    }
}

impl PrefixVarint<5> for u32 {
    const PREFIX_VARINT_BUF_SIZE: usize = 5;

    fn encode_prefix_varint(self, buf: &mut [u8; 5]) -> usize {
        let mut value = self;

        match value.leading_zeros() {
            0..=3 => {
                buf[0] = 0b_1111_0000;
                buf[1..].copy_from_slice(value.to_le_bytes().as_slice());
                5
            }
            4..=10 => {
                value <<= 4;
                buf[0] = (value as u8 >> 4) | 224;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                buf[3] = (value >> 24) as u8;
                4
            }
            11..=17 => {
                value <<= 3;
                buf[0] = (value as u8 >> 3) | 192;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                3
            }
            18..=24 => {
                value <<= 2;
                buf[0] = (value as u8 >> 2) | 128;
                buf[1] = (value >> 8) as u8;
                2
            }
            _ => {
                buf[0] = value as u8;
                1
            }
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)?;

        match prefix.leading_ones() {
            0 => Ok(prefix as u32),
            1 => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u32 & 0x3f) | ((buf[0] as u32) << 6))
            }
            2 => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u32 & 0x1f) | ((u16::from_le_bytes(buf) as u32) << 5))
            }
            3 => {
                let mut buf = [0u8; 3];
                reader.read_exact(&mut buf)?;
                let mut v = buf[2] as u32;
                v = (v << 16) | (u16::from_le_bytes([buf[0], buf[1]]) as u32);
                Ok((prefix as u32 & 0x0f) | (v << 4))
            }
            _ => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(u32::from_le_bytes(buf))
            }
        }
    }
}

impl PrefixVarint<9> for u64 {
    const PREFIX_VARINT_BUF_SIZE: usize = 9;

    fn encode_prefix_varint(self, buf: &mut [u8; 9]) -> usize {
        let mut value = self;

        match value.leading_zeros() {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                buf[0] = 255;
                buf[1..].copy_from_slice(&value.to_le_bytes());
                9
            }
            8 | 9 | 10 | 11 | 12 | 13 | 14 => {
                buf[0] = 254;
                buf[1] = value as u8;
                buf[2] = (value >> 8) as u8;
                buf[3] = (value >> 16) as u8;
                buf[4] = (value >> 24) as u8;
                buf[5] = (value >> 32) as u8;
                buf[6] = (value >> 40) as u8;
                buf[7] = (value >> 48) as u8;
                8
            }
            15 | 16 | 17 | 18 | 19 | 20 | 21 => {
                value <<= 7;
                buf[0] = (value as u8 >> 7) | 252;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                buf[3] = (value >> 24) as u8;
                buf[4] = (value >> 32) as u8;
                buf[5] = (value >> 40) as u8;
                buf[6] = (value >> 48) as u8;
                7
            }
            22 | 23 | 24 | 25 | 26 | 27 | 28 => {
                value <<= 6;
                buf[0] = (value as u8 >> 6) | 248;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                buf[3] = (value >> 24) as u8;
                buf[4] = (value >> 32) as u8;
                buf[5] = (value >> 40) as u8;
                6
            }
            29 | 30 | 31 | 32 | 33 | 34 | 35 => {
                value <<= 5;
                buf[0] = (value as u8 >> 5) | 240;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                buf[3] = (value >> 24) as u8;
                buf[4] = (value >> 32) as u8;
                5
            }
            36 | 37 | 38 | 39 | 40 | 41 | 42 => {
                value <<= 4;
                buf[0] = (value as u8 >> 4) | 224;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                buf[3] = (value >> 24) as u8;
                4
            }
            43 | 44 | 45 | 46 | 47 | 48 | 49 => {
                value <<= 3;
                buf[0] = (value as u8 >> 3) | 192;
                buf[1] = (value >> 8) as u8;
                buf[2] = (value >> 16) as u8;
                3
            }
            50 | 51 | 52 | 53 | 54 | 55 | 56 => {
                value <<= 2;
                buf[0] = (value as u8 >> 2) | 128;
                buf[1] = (value >> 8) as u8;
                2
            }
            _ => {
                buf[0] = value as u8;
                1
            }
        }
    }

    fn decode_prefix_varint(reader: &mut impl Read) -> Result<Self> {
        let prefix = decode_prefix(reader)?;

        match prefix.leading_ones() as u8 {
            0 => Ok(prefix as u64),
            1 => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u64 & 0x3f) | ((buf[0] as u64) << 6))
            }
            2 => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u64 & 0x1f) | ((u16::from_le_bytes(buf) as u64) << 5))
            }
            3 => {
                let mut buf = [0u8; 3];
                reader.read_exact(&mut buf)?;
                let mut v = buf[2] as u64;
                v = (v << 16) | (u16::from_le_bytes([buf[0], buf[1]]) as u64);
                Ok((prefix as u64 & 0x0f) | (v << 4))
            }
            4 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok((prefix as u64 & 0x07) | ((u32::from_le_bytes(buf) as u64) << 3))
            }
            5 => {
                let mut buf = [0u8; 5];
                reader.read_exact(&mut buf)?;
                let mut v = buf[4] as u64;
                v = (v << 32) | (u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64);
                Ok((prefix as u64 & 0x03) | (v << 2))
            }
            6 => {
                let mut buf = [0u8; 6];
                reader.read_exact(&mut buf)?;
                let mut v = u16::from_le_bytes([buf[4], buf[5]]) as u64;
                v = (v << 32) | (u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as u64);
                Ok((prefix as u64 & 0x01) | (v << 1))
            }
            7 => {
                let mut buf = [0u8; 8];
                buf[0] = prefix as u8;
                reader.read_exact(&mut buf[1..8])?;
                Ok(u64::from_le_bytes(buf) >> 8)
            }
            _ => {
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf)?;
                Ok(u64::from_le_bytes(buf))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixVarint;

    #[test]
    fn encode_u8() {
        IntoIterator::into_iter([
            0u8,
            1u8,
            1u8 << 1,
            1u8 << 2,
            1u8 << 3,
            1u8 << 4,
            1u8 << 5,
            1u8 << 6,
            (1u8 << 7) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u8::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 1);
        });

        IntoIterator::into_iter([
            1u8 << 7,
            u8::MAX,
        ]).for_each(|v| {
            let mut buf = [0u8; u8::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 2);
        });
    }

    #[test]
    fn encode_u16() {
        IntoIterator::into_iter([
            0u16,
            1u16,
            1u16 << 1,
            1u16 << 2,
            1u16 << 3,
            1u16 << 4,
            1u16 << 5,
            1u16 << 6,
            (1u16 << 7) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 1);
        });

        IntoIterator::into_iter([
            1u16 << 7,
            1u16 << 8,
            1u16 << 9,
            1u16 << 10,
            1u16 << 11,
            1u16 << 12,
            1u16 << 13,
            (1u16 << 14) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 2);
        });

        IntoIterator::into_iter([
            1u16 << 14,
            1u16 << 15,
            u16::MAX,
        ]).for_each(|v| {
            let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 3);
        });
    }

    #[test]
    fn encode_u32() {
        IntoIterator::into_iter([
            0u32,
            1u32,
            1u32 << 1,
            1u32 << 2,
            1u32 << 3,
            1u32 << 4,
            1u32 << 5,
            1u32 << 6,
            (1u32 << 7) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 1);
        });

        IntoIterator::into_iter([
            1u32 << 7,
            1u32 << 8,
            1u32 << 9,
            1u32 << 10,
            1u32 << 11,
            1u32 << 12,
            1u32 << 13,
            (1u32 << 14) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 2);
        });

        IntoIterator::into_iter([
            1u32 << 14,
            1u32 << 15,
            1u32 << 16,
            1u32 << 17,
            1u32 << 18,
            1u32 << 19,
            1u32 << 20,
            (1u32 << 21) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 3);
        });

        IntoIterator::into_iter([
            1u32 << 21,
            1u32 << 22,
            1u32 << 23,
            1u32 << 24,
            1u32 << 25,
            1u32 << 26,
            1u32 << 27,
            (1u32 << 28) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 4);
        });

        IntoIterator::into_iter([
            1u32 << 28,
            1u32 << 29,
            1u32 << 30,
            1u32 << 31,
            u32::MAX,
        ]).for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 5);
        });
    }

    #[test]
    fn encode_u64() {
        IntoIterator::into_iter([
            0u64,
            1u64,
            1u64 << 1,
            1u64 << 2,
            1u64 << 3,
            1u64 << 4,
            1u64 << 5,
            1u64 << 6,
            (1u64 << 7) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 1);
        });

        IntoIterator::into_iter([
            1u64 << 7,
            1u64 << 8,
            1u64 << 9,
            1u64 << 10,
            1u64 << 11,
            1u64 << 12,
            1u64 << 13,
            (1u64 << 14) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 2);
        });

        IntoIterator::into_iter([
            1u64 << 14,
            1u64 << 15,
            1u64 << 16,
            1u64 << 17,
            1u64 << 18,
            1u64 << 19,
            1u64 << 20,
            (1u64 << 21) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 3);
        });

        IntoIterator::into_iter([
            1u64 << 21,
            1u64 << 22,
            1u64 << 23,
            1u64 << 24,
            1u64 << 25,
            1u64 << 26,
            1u64 << 27,
            (1u64 << 28) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 4);
        });

        IntoIterator::into_iter([
            1u64 << 28,
            1u64 << 29,
            1u64 << 30,
            1u64 << 31,
            1u64 << 32,
            1u64 << 33,
            1u64 << 34,
            (1u64 << 35) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 5);
        });

        IntoIterator::into_iter([
            1u64 << 35,
            1u64 << 36,
            1u64 << 37,
            1u64 << 38,
            1u64 << 39,
            1u64 << 40,
            1u64 << 41,
            (1u64 << 42) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 6);
        });

        IntoIterator::into_iter([
            1u64 << 42,
            1u64 << 43,
            1u64 << 44,
            1u64 << 45,
            1u64 << 46,
            1u64 << 47,
            1u64 << 48,
            (1u64 << 49) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 7);
        });

        IntoIterator::into_iter([
            1u64 << 49,
            1u64 << 50,
            1u64 << 51,
            1u64 << 52,
            1u64 << 53,
            1u64 << 54,
            1u64 << 55,
            (1u64 << 56) - 1,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 8);
        });

        IntoIterator::into_iter([
            1u64 << 56,
            1u64 << 57,
            1u64 << 58,
            1u64 << 59,
            1u64 << 60,
            1u64 << 61,
            1u64 << 62,
            1u64 << 63,
            u64::MAX,
        ]).for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            assert_eq!(v.encode_prefix_varint(&mut buf), 9);
        });
    }

    #[test]
    fn decode_u8() {
        IntoIterator::into_iter([0u8, (1 << 7) - 1, (1 << 7), u8::MAX]).for_each(|v| {
            let mut buf = [0u8; u8::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u8::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u16() {
        IntoIterator::into_iter(
            [
                vec![
                    0u16,
                    (1 << 7) - 1,
                    (1 << 7),
                    (1 << 14) - 1,
                    1 << 14,
                    u16::MAX,
                ],
                (0..16).into_iter().map(|v| 1u16 << v).collect(),
            ]
            .concat(),
        )
        .for_each(|v| {
            let mut buf = [0u8; u16::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u16::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u32() {
        IntoIterator::into_iter(
            [
                vec![
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
                ],
                (0..32).into_iter().map(|v| 1u32 << v).collect(),
            ]
            .concat(),
        )
        .for_each(|v| {
            let mut buf = [0u8; u32::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u32::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }

    #[test]
    fn decode_u64() {
        IntoIterator::into_iter(
            [
                vec![
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
                ],
                (0..64).into_iter().map(|v| 1u64 << v).collect(),
            ]
            .concat(),
        )
        .for_each(|v| {
            let mut buf = [0u8; u64::PREFIX_VARINT_BUF_SIZE];
            v.encode_prefix_varint(&mut buf);
            assert_eq!(v, u64::decode_prefix_varint(&mut buf.as_ref()).unwrap());
        });
    }
}
