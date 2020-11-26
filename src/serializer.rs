use crate::{body::Body, header::Header};

fn validate(header: &Header, body: &Body) -> bool {
    match header {
        Header::Boolean => {
            if let Body::Boolean(_) = body {
                true
            } else {
                false
            }
        }
        Header::UInt8 => {
            if let Body::UInt8(_) = body {
                true
            } else {
                false
            }
        }
    }
}

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    if !validate(header, body) {
        return Err(())
    }

    let mut serialized_header= header.serialize();
    serialized_header.append(&mut body.serialize());
    Ok(serialized_header)
}

#[cfg(test)]
mod tests {
    use crate::{body::Body, header::Header};

    #[test]
    fn validate() {
        let header = Header::Boolean;
        assert!(super::validate(&header, &Body::Boolean(true)));
        assert!(!super::validate(&header, &Body::UInt8(0)));

        let header = Header::UInt8;
        assert!(super::validate(&header, &Body::UInt8(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));
    }

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
