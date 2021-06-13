pub(crate) trait Leb128<const N: usize> {
    fn encode_leb128(&self) -> ([u8; N], usize);
    fn encode_leb128_vec(&self) -> Vec<u8>;
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
}
