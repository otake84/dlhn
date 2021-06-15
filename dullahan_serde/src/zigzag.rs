pub(crate) trait ZigZag<T> {
    fn encode_zigzag(self) -> T;
}

impl ZigZag<u64> for isize {
    fn encode_zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }
}

impl ZigZag<u64> for i64 {
    fn encode_zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }
}

impl ZigZag<u32> for i32 {
    fn encode_zigzag(self) -> u32 {
        ((self << 1) ^ (self >> 31)) as u32
    }
}

impl ZigZag<u16> for i16 {
    fn encode_zigzag(self) -> u16 {
        ((self << 1) ^ (self >> 15)) as u16
    }
}

impl ZigZag<u8> for i8 {
    fn encode_zigzag(self) -> u8 {
        ((self << 1) ^ (self >> 7)) as u8
    }
}
