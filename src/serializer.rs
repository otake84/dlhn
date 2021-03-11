use crate::{body::Body, header::Header};

pub(crate) fn validate(header: &Header, body: &Body) -> bool {
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
        (Header::VarUInt16, Body::VarUInt16(_)) => true,
        (Header::VarUInt32, Body::VarUInt32(_)) => true,
        (Header::VarUInt64, Body::VarUInt64(_)) => true,
        (Header::Int8, Body::Int8(_)) => true,
        (Header::Int16, Body::Int16(_)) => true,
        (Header::Int32, Body::Int32(_)) => true,
        (Header::Int64, Body::Int64(_)) => true,
        (Header::VarInt16, Body::VarInt16(_)) => true,
        (Header::VarInt32, Body::VarInt32(_)) => true,
        (Header::VarInt64, Body::VarInt64(_)) => true,
        (Header::Float32, Body::Float32(_)) => true,
        (Header::Float64, Body::Float64(_)) => true,
        (Header::BigUInt, Body::BigUInt(_)) => true,
        (Header::BigInt, Body::BigInt(_)) => true,
        (Header::BigDecimal, Body::BigDecimal(_)) => true,
        (Header::String, Body::String(_)) => true,
        (Header::Binary, Body::Binary(_)) => true,
        (Header::Array(inner_header), Body::Array(inner_body)) => {
            inner_body.iter().all(|v| validate(inner_header, v))
        }
        (Header::Map(inner_header), Body::Map(inner_body)) => {
            inner_header.len() == inner_body.len()
                && inner_body.iter().all(|(k, v)| {
                    if let Some(h) = inner_header.get(k) {
                        validate(h, v)
                    } else {
                        false
                    }
                })
        }
        (Header::DynamicMap(inner_header), Body::DynamicMap(inner_body)) => inner_body
            .iter()
            .all(|(_key, value)| validate(inner_header, value)),
        (Header::Date, Body::Date(_)) => true,
        (Header::DateTime, Body::DateTime(_)) => true,
        (Header::Extension(_), Body::Extension(_)) => true,
        _ => false,
    }
}

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    if !validate(header, body) {
        return Err(());
    }

    Ok(serialize_without_validate(header, body))
}

#[inline]
pub fn serialize_without_validate(header: &Header, body: &Body) -> Vec<u8> {
    let mut buf = header.serialize();
    buf.append(&mut body.serialize());
    buf
}

pub fn serialize_body(body: &Body) -> Vec<u8> {
    body.serialize()
}

#[cfg(test)]
mod tests {
    use crate::{
        body::Body,
        header::{ExtensionCode, Header},
    };
    use bigdecimal::BigDecimal;
    use num_bigint::{BigInt, BigUint};
    use std::collections::BTreeMap;
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

        let header = Header::VarUInt16;
        assert!(super::validate(&header, &Body::VarUInt16(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::VarUInt32;
        assert!(super::validate(&header, &Body::VarUInt32(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::VarUInt64;
        assert!(super::validate(&header, &Body::VarUInt64(0)));
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

        let header = Header::VarInt16;
        assert!(super::validate(&header, &Body::VarInt16(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::VarInt32;
        assert!(super::validate(&header, &Body::VarInt32(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::VarInt64;
        assert!(super::validate(&header, &Body::VarInt64(0)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Float32;
        assert!(super::validate(&header, &Body::Float32(0f32)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Float64;
        assert!(super::validate(&header, &Body::Float64(0f64)));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::BigUInt;
        assert!(super::validate(&header, &Body::BigUInt(BigUint::from(0u8))));
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
            &Body::Binary(vec![0, 1, 2, 3, 255])
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

        let header = Header::Map({
            let mut map = BTreeMap::new();
            map.insert(String::from("test"), Header::Boolean);
            map
        });
        assert!(super::validate(
            &header,
            &Body::Map({
                let mut map = BTreeMap::new();
                map.insert(String::from("test"), Body::Boolean(true));
                map
            })
        ));
        assert!(!super::validate(
            &header,
            &Body::Map({
                let mut map = BTreeMap::new();
                map.insert(String::from("test"), Body::UInt8(0));
                map
            })
        ));
        assert!(!super::validate(
            &header,
            &Body::Map({
                let mut map = BTreeMap::new();
                map.insert(String::from("test"), Body::Boolean(true));
                map.insert(String::from("test2"), Body::UInt8(0));
                map
            })
        ));

        let header = Header::DynamicMap(Box::new(Header::Boolean));
        assert!(super::validate(
            &header,
            &Body::DynamicMap({
                let mut body = BTreeMap::new();
                body.insert(String::from("test"), Body::Boolean(true));
                body
            })
        ));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Date;
        assert!(super::validate(
            &header,
            &Body::Date(Date::try_from_yo(2000, 1).unwrap())
        ));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::DateTime;
        assert!(super::validate(
            &header,
            &Body::DateTime(OffsetDateTime::unix_epoch())
        ));
        assert!(!super::validate(&header, &Body::Boolean(true)));

        let header = Header::Extension(ExtensionCode::Code255);
        assert!(super::validate(&header, &Body::Extension(Vec::new())));
        assert!(!super::validate(&header, &Body::Boolean(true)));
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

    #[test]
    fn serialize_body_boolean() {
        assert_eq!(super::serialize_body(&Body::Boolean(false)), [0]);
        assert_eq!(super::serialize_body(&Body::Boolean(true)), [1]);
    }

    #[test]
    fn serialize_body_uint8() {
        assert_eq!(super::serialize_body(&Body::UInt8(0)), 0u8.to_le_bytes());
        assert_eq!(
            super::serialize_body(&Body::UInt8(255)),
            255u8.to_le_bytes()
        );
    }
}
