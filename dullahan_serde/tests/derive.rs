use dullahan_serde::header::serialize_header::SerializeHeader;
use dullahan_serde_derive::*;

#[test]
fn derive_serialize_header() {
    #[allow(dead_code)]
    #[derive(SerializeHeader)]
    struct Test {
        a: bool,
        b: u8,
        c: Option<u32>,
    }

    let mut buf = Vec::new();
    Test::serialize_header(&mut buf).unwrap();
    assert_eq!(buf, [3, 2, 3, 1, 5]);
}
