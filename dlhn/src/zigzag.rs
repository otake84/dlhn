// https://developers.google.com/protocol-buffers/docs/encoding#signed_integers
// https://stackoverflow.com/questions/2210923/zig-zag-decoding

pub(crate) trait ZigZag<T> {
    fn encode_zigzag(self) -> T;
    fn decode_zigzag(value: T) -> Self;
}

impl ZigZag<u64> for isize {
    fn encode_zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }

    fn decode_zigzag(value: u64) -> Self {
        (value >> 1) as isize ^ (-(value as isize & 1))
    }
}

impl ZigZag<u128> for i128 {
    fn encode_zigzag(self) -> u128 {
        ((self << 1) ^ (self >> 127)) as u128
    }

    fn decode_zigzag(value: u128) -> Self {
        (value >> 1) as i128 ^ (-(value as i128 & 1))
    }
}

impl ZigZag<u64> for i64 {
    fn encode_zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }

    fn decode_zigzag(value: u64) -> Self {
        (value >> 1) as i64 ^ (-(value as i64 & 1))
    }
}

impl ZigZag<u32> for i32 {
    fn encode_zigzag(self) -> u32 {
        ((self << 1) ^ (self >> 31)) as u32
    }

    fn decode_zigzag(value: u32) -> Self {
        (value >> 1) as i32 ^ (-(value as i32 & 1))
    }
}

impl ZigZag<u16> for i16 {
    fn encode_zigzag(self) -> u16 {
        ((self << 1) ^ (self >> 15)) as u16
    }

    fn decode_zigzag(value: u16) -> Self {
        (value >> 1) as i16 ^ (-(value as i16 & 1))
    }
}

impl ZigZag<u8> for i8 {
    fn encode_zigzag(self) -> u8 {
        ((self << 1) ^ (self >> 7)) as u8
    }

    fn decode_zigzag(value: u8) -> Self {
        (value >> 1) as i8 ^ (-(value as i8 & 1))
    }
}
