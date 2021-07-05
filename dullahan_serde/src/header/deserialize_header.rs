use std::io::{Read, Result};
use super::Header;

pub trait DeserializeHeader<R: Read> {
    fn deserialize_header(&mut self) -> Result<Header>;
}

impl<R: Read> DeserializeHeader<R> for R {
    fn deserialize_header(&mut self) -> Result<Header> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;

        match buf[0] {
            0 => Ok(Header::Unit),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::header::{Header, serialize_header::SerializeHeader};
    use super::DeserializeHeader;

    #[test]
    fn deserialize_header_unit() {
        let mut buf = Vec::new();
        <()>::serialize_header(&mut buf).unwrap();

        assert_eq!(Cursor::new(buf).deserialize_header().unwrap(), Header::Unit);
    }
}
