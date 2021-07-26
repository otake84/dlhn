use std::io::Cursor;
use dullahan_serde::header::{Header, ser::SerializeHeader, de::DeserializeHeader};
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
        assert_eq!(buf, [22, 4, 2, 0, 5, 19, 3, 2, 3, 5]);
    }

    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        enum Test {
            A(bool),
            B,
            C(u32),
            D {
                a: bool,
                b: u8,
                c: u32
            },
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [22, 4, 2, 0, 5, 20, 3, 2, 3, 5]);
    }
}

#[test]
fn derive_serialize_header_with_skip() {
    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        struct Test {
            a: bool,
            #[serde(skip)]
            b: u8,
            c: Option<u32>,
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [20, 2, 2, 1, 5]);
    }

    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        enum Test {
            A(bool),
            B,
            #[serde(skip)]
            C(u32),
            D(bool, u8, u32),
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [22, 3, 2, 0, 19, 3, 2, 3, 5]);
    }
}

#[test]
fn derive_serialize_header_with_skip_serializing() {
    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        struct Test {
            a: bool,
            #[serde(skip_serializing)]
            b: u8,
            c: Option<u32>,
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [20, 2, 2, 1, 5]);
    }

    {
        #[allow(dead_code)]
        #[derive(SerializeHeader)]
        enum Test {
            A(bool),
            B,
            #[serde(skip_serializing)]
            C(u32),
            D(bool, u8, u32),
        }

        let mut buf = Vec::new();
        Test::serialize_header(&mut buf).unwrap();
        assert_eq!(buf, [22, 3, 2, 0, 19, 3, 2, 3, 5]);
    }
}

#[test]
fn deserialize_header() {
    {
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
        let mut cursor = Cursor::new(buf);
        assert_eq!(cursor.deserialize_header().unwrap(), Header::Enum(vec![Header::Boolean, Header::Unit, Header::UInt32, Header::Tuple(vec![Header::Boolean, Header::UInt8, Header::UInt32])]));
    }
}
