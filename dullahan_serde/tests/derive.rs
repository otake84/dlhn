use std::io::Cursor;
use dullahan_serde::header::{Header, serialize_header::SerializeHeader, deserialize_header::DeserializeHeader};
use dullahan_serde_derive::*;

#[test]
fn derive_serialize_header() {
    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        struct Test {
            a: bool,
            b: u8,
            c: Option<u32>,
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [20, 3, 2, 3, 1, 5]);
    }

    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        enum Test {
            A(bool),
            B,
            C(u32),
            D(bool, u8, u32),
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [22, 4, 1, 2, 1, 0, 1, 5, 3, 2, 3, 5]);
    }
}

#[test]
fn deserialize_header() {
    #[allow(dead_code)]
    #[derive(SerializeHeader)]
    struct Test {
        a: (),
        b: bool,
        c: u8,
    }

    let mut buf = Vec::new();
    Test::serialize_header(&mut buf).unwrap();
    let mut cursor = Cursor::new(buf);
    assert_eq!(cursor.deserialize_header().unwrap(), Header::Struct(vec![Header::Unit, Header::Boolean, Header::UInt8]));
}
