use crate::{body::Body, header::Header};

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    let mut serialized_header= header.serialize();
    match body.serialize(header) {
        Ok(mut value) => {
            serialized_header.append(&mut value);
            Ok(serialized_header)
        }
        Err(_) => Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{body::Body, header::Header};

    #[test]
    fn serialize_boolean() {
        let header = Header::Boolean;
        assert_eq!(super::serialize(&header, &Body::Boolean(false)).unwrap(), [0, 0]);
        assert_eq!(super::serialize(&header, &Body::Boolean(true)).unwrap(), [0, 1]);
    }

    #[test]
    fn serialize_uint8() {
        let header = Header::UInt8;
        assert_eq!(super::serialize(&header, &Body::UInt8(0)).unwrap(), [[1], (0 as u8).to_le_bytes()].concat());
        assert_eq!(super::serialize(&header, &Body::UInt8(255)).unwrap(), [[1], (255 as u8).to_le_bytes()].concat());
    }
}
