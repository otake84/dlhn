use crate::{body::Body, header::Header};

fn validate(header: &Header, body: &Body) -> bool {
    match (header, body) {
        (Header::Optional(inner_header), Body::Optional(inner_body)) => {
            if let Some(v) = &**inner_body {
                validate(inner_header, v)
            } else {
                true
            }
        }
        (Header::Boolean, Body::Boolean(_)) => true,
        (Header::UInt8, Body::UInt8(_)) => true,
        (Header::UInt16, Body::UInt16(_)) => true,
        (Header::UInt32, Body::UInt32(_)) => true,
        (Header::UInt64, Body::UInt64(_)) => true,
        (Header::Int8, Body::Int8(_)) => true,
        (Header::Int16, Body::Int16(_)) => true,
        (Header::Int32, Body::Int32(_)) => true,
        (Header::Int64, Body::Int64(_)) => true,
        (Header::Float32, Body::Float32(_)) => true,
        (Header::Float64, Body::Float64(_)) => true,
        (Header::BigInt, Body::BigInt(_)) => true,
        (Header::BigDecimal, Body::BigDecimal(_)) => true,
        (Header::String, Body::String(_)) => true,
        (Header::Binary, Body::Binary(_)) => true,
        (Header::Array(inner_header), Body::Array(inner_body)) => {
            inner_body.iter().all(|v| validate(inner_header, v))
        }
        (Header::Map(inner_header), Body::Map(inner_body)) => {
            inner_body.iter().enumerate().all(|(i, v)| {
                if let Some(h) = inner_header.get_index(i) {
                    validate(h.1, v.1)
                } else {
                    false
                }
            })
        }
        (Header::DynamicMap(inner_header), Body::DynamicMap(inner_body)) => inner_body
            .iter()
            .all(|(_key, value)| validate(inner_header, value)),
        (Header::Timestamp, Body::Timestamp(_)) => true,
        (Header::Date, Body::Date(_)) => true,
        _ => false,
    }
}

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    if !validate(header, body) {
        return Err(());
    }

    let mut serialized_header = header.serialize();
    serialized_header.append(&mut body.serialize());
    Ok(serialized_header)
}

#[cfg(test)]
mod tests {
    use crate::{binary::Binary, body::Body, header::Header};
    use bigdecimal::BigDecimal;
    use indexmap::*;
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use time::{Date, OffsetDateTime};

    #[test]
    fn validate() {
        let header = Header::Optional(Box::new(Header::Boolean));
        assert!(super::validate(&header, &Body::Optional(Box::new(None))));
        assert!(super::validate(
            &header,
            &Body::Optional(Box::new(Some(Body::Boolean(true))))
        ));
        assert!(!super::validate(
            &header,
            &Body::Optional(Box::new(Some(Body::UInt8(0))))
        ));

        let header = Header::Boolean;
        assert!(super::validate(&header, &Body::Boolean(true)));
        assert!(!super::validate(&header, &Body::UInt8(0)));

        let header = Header::UInt8;
        assert!(super::validate(&header, &Body::UInt8(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::UInt16;
        assert!(super::validate(&header, &Body::UInt16(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::UInt32;
        assert!(super::validate(&header, &Body::UInt32(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::UInt64;
        assert!(super::validate(&header, &Body::UInt64(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int8;
        assert!(super::validate(&header, &Body::Int8(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int16;
        assert!(super::validate(&header, &Body::Int16(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int32;
        assert!(super::validate(&header, &Body::Int32(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Int64;
        assert!(super::validate(&header, &Body::Int64(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Float32;
        assert!(super::validate(&header, &Body::Float32(0f32)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Float64;
        assert!(super::validate(&header, &Body::Float64(0f64)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::BigInt;
        assert!(super::validate(&header, &Body::BigInt(BigInt::from(0))));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::BigDecimal;
        assert!(super::validate(
            &header,
            &Body::BigDecimal(BigDecimal::from(0))
        ));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::String;
        assert!(super::validate(
            &header,
            &Body::String(String::from("test"))
        ));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Binary;
        assert!(super::validate(
            &header,
            &Body::Binary(Binary(vec![0, 1, 2, 3, 255]))
        ));

        let header = Header::Array(Box::new(Header::UInt8));
        assert!(super::validate(&header, &Body::Array(vec![Body::UInt8(0)])));
        assert!(!super::validate(
            &header,
            &Body::Array(vec![Body::Boolean(true)])
        ));
        assert!(super::validate(
            &header,
            &Body::Array(vec![Body::UInt8(0), Body::UInt8(1)])
        ));
        assert!(!super::validate(
            &header,
            &Body::Array(vec![Body::UInt8(0), Body::Boolean(true)])
        ));

        let header = Header::Map(indexmap! { String::from("test") => Header::Boolean });
        assert!(super::validate(
            &header,
            &Body::Map(indexmap! { String::from("test") => Body::Boolean(true) })
        ));
        assert!(!super::validate(
            &header,
            &Body::Map(indexmap! { String::from("test") => Body::UInt8(0) })
        ));
        assert!(!super::validate(
            &header,
            &Body::Map(
                indexmap! { String::from("test") => Body::Boolean(true), String::from("test2") => Body::UInt8(0) }
            )
        ));

        let header = Header::DynamicMap(Box::new(Header::Boolean));
        assert!(super::validate(
            &header,
            &Body::DynamicMap({
                let mut body = HashMap::new();
                body.insert(String::from("test"), Body::Boolean(true));
                body
            })
        ));

        let header = Header::Timestamp;
        assert!(super::validate(
            &header,
            &Body::Timestamp(OffsetDateTime::unix_epoch())
        ));

        let header = Header::Date;
        assert!(super::validate(
            &header,
            &Body::Date(Date::try_from_yo(2000, 1).unwrap())
        ));
    }

    #[test]
    fn serialize_boolean() {
        let header = Header::Boolean;
        assert_eq!(
            super::serialize(&header, &Body::Boolean(false)).unwrap(),
            [Header::Boolean.code(), 0]
        );
        assert_eq!(
            super::serialize(&header, &Body::Boolean(true)).unwrap(),
            [Header::Boolean.code(), 1]
        );
    }

    #[test]
    fn serialize_uint8() {
        let header = Header::UInt8;
        assert_eq!(
            super::serialize(&header, &Body::UInt8(0)).unwrap(),
            [[Header::UInt8.code()], (0u8).to_le_bytes()].concat()
        );
        assert_eq!(
            super::serialize(&header, &Body::UInt8(255)).unwrap(),
            [[Header::UInt8.code()], (255u8).to_le_bytes()].concat()
        );
    }
}
