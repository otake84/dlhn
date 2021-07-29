use serde::{Deserialize, Serialize};
use dlhn::{de::Deserializer, ser::Serializer};

#[test]
fn seriaize() {
    #[derive(Serialize)]
    struct Test {
        c: String,
        a: bool,
        b: u8,
    }

    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        c: "test".to_string(),
        a: true,
        b: 123,
    };

    body.serialize(&mut serializer).unwrap();
    body.serialize(&mut serializer).unwrap();

    assert_eq!(buf, [
        [4].as_ref(), "test".as_bytes(), [1].as_ref(), [123].as_ref(),
        [4].as_ref(), "test".as_bytes(), [1].as_ref(), [123].as_ref(),
    ].concat());
}

#[test]
fn deserialize() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    struct Test {
        c: String,
        a: bool,
        b: u8,
    }

    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        c: "test".to_string(),
        a: true,
        b: 123,
    };

    body.serialize(&mut serializer).unwrap();
    body.serialize(&mut serializer).unwrap();

    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);

    assert_eq!(body, Test::deserialize(&mut deserializer).unwrap());
    assert_eq!(body, Test::deserialize(&mut deserializer).unwrap());
    assert!(Test::deserialize(&mut deserializer).is_err());
}
