use crate::{body::Body, header::Header};

fn validate(header: &Header, body: &Body) -> bool {
    match (header, body) {
        (Header::Boolean, Body::Boolean(_)) => true,
        (Header::UInt, Body::UInt(_)) => true,
        (Header::UInt8, Body::UInt8(_)) => true,
        (Header::Int, Body::Int(_)) => true,
        (Header::Int8, Body::Int8(_)) => true,
        (Header::Float32, Body::Float32(_)) => true,
        (Header::String, Body::String(_)) => true,
        (Header::Array(inner_header), Body::Array(inner_body)) => {
            inner_body.iter().all(|v| validate(inner_header, v))
        },
        _ => false,
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

        let header = Header::UInt;
        assert!(super::validate(&header, &Body::UInt(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::UInt8;
        assert!(super::validate(&header, &Body::UInt8(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int;
        assert!(super::validate(&header, &Body::Int(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int8;
        assert!(super::validate(&header, &Body::Int8(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Float32;
        assert!(super::validate(&header, &Body::Float32(0f32)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::String;
        assert!(super::validate(&header, &Body::String(String::from("test"))));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Array(Box::new(Header::UInt8));
        assert!(super::validate(&header, &Body::Array(vec![Body::UInt8(0)])));
        assert!(!super::validate(&header, &Body::Array(vec![Body::Boolean(true)])));
        assert!(super::validate(&header, &Body::Array(vec![Body::UInt8(0), Body::UInt8(1)])));
        assert!(!super::validate(&header, &Body::Array(vec![Body::UInt8(0), Body::Boolean(true)])));
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
        assert_eq!(super::serialize(&header, &Body::UInt8(0)).unwrap(), [[2], (0u8).to_le_bytes()].concat());
        assert_eq!(super::serialize(&header, &Body::UInt8(255)).unwrap(), [[2], (255u8).to_le_bytes()].concat());
    }
}
