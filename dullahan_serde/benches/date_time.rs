use dullahan_serde::ser::Serializer;
use iai::main;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    #[serde(with = "dullahan_serde::format::date_time")]
    date_time: OffsetDateTime,
}

fn serialize() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        date_time: OffsetDateTime::unix_epoch(),
    };
    body.serialize(&mut serializer).unwrap();
    buf
}

main!(
    serialize,
);
