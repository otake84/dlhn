use dlhn::ser::Serializer;
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

main!(serialize,);
