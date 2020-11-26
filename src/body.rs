use std::io::{BufReader, Read};

use crate::header::{BodySize, Header};

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
    UInt8(u8),
}

impl Body {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Body::Boolean(v) => {
                if *v {
                    vec![1]
                } else {
                    vec![0]
                }
            }
            Body::UInt8(v) => {
                v.to_le_bytes().to_vec()
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(header: Header, read: R) -> Result<Body, ()> {
        let mut reader = BufReader::new(read);

        if let BodySize::Fix(size) = header.body_size() {
            let mut body_buf = Vec::with_capacity(size);
            unsafe { body_buf.set_len(size); }
            reader.read_exact(&mut body_buf).or(Err(()))?;

            match header {
                Header::Boolean => {
                    match *body_buf.first().unwrap() {
                        0 => Ok(Body::Boolean(false)),
                        1 => Ok(Body::Boolean(true)),
                        _ => Err(()),
                    }
                }
                Header::UInt8 => {
                    Ok(Body::UInt8(*body_buf.first().unwrap()))
                }
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;

    use super::Body;

    #[test]
    fn deserialize() {
        assert_eq!(super::Body::deserialize(Header::Boolean, &[0u8] as &[u8]), Ok(Body::Boolean(false)));
        assert_eq!(super::Body::deserialize(Header::Boolean, &[1u8] as &[u8]), Ok(Body::Boolean(true)));
        assert_eq!(super::Body::deserialize(Header::UInt8, &[0u8] as &[u8]), Ok(Body::UInt8(0)));
        assert_eq!(super::Body::deserialize(Header::UInt8, &[255u8] as &[u8]), Ok(Body::UInt8(255)));
    }
}
