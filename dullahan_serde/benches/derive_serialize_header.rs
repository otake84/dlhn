use iai::main;
use dullahan_serde::header::ser::SerializeHeader;
use dullahan_serde_derive::*;

fn serialize_header_struct() {
    #[allow(dead_code)]
    #[derive(SerializeHeader)]
    struct Test {
        a: bool,
        b: u8,
        c: Option<u32>,
    }

    let mut buf = Vec::new();
    Test::serialize_header(&mut buf).unwrap();
}

fn serialize_header_enum() {
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
}

main!(
    serialize_header_struct,
    serialize_header_enum,
);
