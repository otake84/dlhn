use dlhn::Serializer;
use iai::main;
use serde::Serialize;
use time::Date;

#[derive(Serialize)]
struct Test {
    #[serde(with = "dlhn::format::date")]
    date: Date,
}

fn serialize() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    let body = Test {
        date: Date::from_ordinal_date(1970, 1).unwrap(),
    };
    body.serialize(&mut serializer).unwrap();
    buf
}

main!(serialize,);
