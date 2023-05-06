use dlhn::{DateTime, Deserializer, Serializer};
use iai::main;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    #[serde(with = "dlhn::format::date_time")]
    date_time: OffsetDateTime,
}

fn serialize() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        date_time: OffsetDateTime::UNIX_EPOCH,
    };
    body.serialize(&mut serializer).unwrap();
    buf
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test2 {
    date_time: DateTime,
}

fn serialize2() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test2 {
        date_time: DateTime::from(OffsetDateTime::UNIX_EPOCH),
    };
    body.serialize(&mut serializer).unwrap();
    buf
}

fn deserialize2() -> DateTime {
    let buf = serialize2();
    let mut reader = buf.as_slice();
    let mut deserializer = Deserializer::new(&mut reader);
    DateTime::deserialize(&mut deserializer).unwrap()
}

main!(serialize, serialize2, deserialize2);
