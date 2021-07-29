use dlhn::ser::Serializer;
use iai::main;
use time::Date;
use serde::Serialize;

#[derive(Serialize)]
struct Test {
    #[serde(with = "dlhn::format::date")]
    date: Date,
}

fn serialize() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        date: Date::try_from_ymd(1970, 1, 1).unwrap(),
    };
    body.serialize(&mut serializer).unwrap();
    buf
}

main!(
    serialize,
);
