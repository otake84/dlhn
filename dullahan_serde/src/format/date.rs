use serde::{Deserializer, Serializer, de::{self, SeqAccess, Unexpected, Visitor}, ser::SerializeSeq};
use time::Date;
use crate::{de::Error, leb128::Leb128, zigzag::ZigZag};

const DATE_YEAR_OFFSET: i32 = 2000;
const DATE_ORDINAL_OFFSET: u16 = 1;

struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
            A: SeqAccess<'de>, {
                let year = seq.next_element::<i32>()?.ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))? + DATE_YEAR_OFFSET;
                let ordinal = seq.next_element::<u16>()?.ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))? + DATE_ORDINAL_OFFSET;
                let date = Date::try_from_yo(year, ordinal).or(Err(de::Error::invalid_value(Unexpected::Seq, &Error::Read)))?;
                Ok(date)
    }
}

pub fn serialize<T: Serializer>(date: &Date, serializer: T) -> Result<T::Ok, T::Error> {
    let year = date.year() - DATE_YEAR_OFFSET;
    let ordinal = date.ordinal() - DATE_ORDINAL_OFFSET;
    let mut seq = serializer.serialize_seq(None)?;
    let (buf, size) = year.encode_zigzag().encode_leb128();
    for e in buf[..size].iter() {
        seq.serialize_element(e)?;
    }
    let (buf, size) = ordinal.encode_leb128();
    for e in buf[..size].iter() {
        seq.serialize_element(e)?;
    }
    seq.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<Date, T::Error> {
    deserializer.deserialize_tuple(2, DateVisitor)
}

#[cfg(test)]
mod tests {
    use dullahan::{body::Body, serializer::serialize_body};
    use serde::{Deserialize, Serialize};
    use time::Date;
    use crate::{de::Deserializer, ser::Serializer};

    #[test]
    fn serialize_date() {
        #[derive(Serialize)]
        struct Test {
            #[serde(with = "crate::format::date")]
            date: Date,
        }

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            date: Date::try_from_ymd(1970, 1, 1).unwrap(),
        };
        body.serialize(&mut serializer).unwrap();
        assert_eq!(buf, serialize_body(&Body::Date(Date::try_from_ymd(1970, 1, 1).unwrap())));
    }

    #[test]
    fn deserialize_date() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            #[serde(with = "crate::format::date")]
            date: Date,
        }

        let buf = serialize(Test {
            date: Date::try_from_ymd(1970, 1, 11).unwrap(),
        });
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = Test::deserialize(&mut deserializer).unwrap();
        assert_eq!(result, Test { date: Date::try_from_ymd(1970, 1, 11).unwrap()})
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
