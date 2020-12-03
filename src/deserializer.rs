use std::io::{BufReader, Read};

use crate::{body::Body, header::Header};

pub fn deserialize<R: Read>(read: R) -> Result<(Header, Body), ()> {
    let mut buf_reader = BufReader::new(read);

    if let Ok(header) = Header::deserialize(&mut buf_reader) {
        if let Ok(body) = Body::deserialize(&header, &mut buf_reader) {
            Ok((header, body))
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{body::Body, header::Header};

    #[test]
    fn deserialize() {
        assert_eq!(super::deserialize(&[0u8, 0] as &[u8]), Ok((Header::Boolean, Body::Boolean(false))));
        assert_eq!(super::deserialize(&[0u8, 1] as &[u8]), Ok((Header::Boolean, Body::Boolean(true))));
        assert_eq!(super::deserialize(&[1u8, 0] as &[u8]), Ok((Header::UInt8, Body::UInt8(0))));
        assert_eq!(super::deserialize(&[1u8, 255] as &[u8]), Ok((Header::UInt8, Body::UInt8(255))));
        assert_eq!(super::deserialize(&[2u8, 0] as &[u8]), Ok((Header::Int8, Body::Int8(0))));
        assert_eq!(super::deserialize(&[[2u8], i8::MIN.to_le_bytes()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MIN))));
        assert_eq!(super::deserialize(&[[2u8], i8::MAX.to_le_bytes()].concat() as &[u8]), Ok((Header::Int8, Body::Int8(i8::MAX))));
    }
}
