use crate::{de::Error, leb128::Leb128, zigzag::ZigZag};
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserializer, Serializer,
};
use time::Date;

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
        A: SeqAccess<'de>,
    {
        let year = seq
            .next_element::<i32>()?
            .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?
            + DATE_YEAR_OFFSET;
        let ordinal = seq
            .next_element::<u16>()?
            .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?
            + DATE_ORDINAL_OFFSET;
        let date = Date::from_ordinal_date(year, ordinal)
            .or(Err(de::Error::invalid_value(Unexpected::Seq, &Error::Read)))?;
        Ok(date)
    }
}

pub fn serialize<T: Serializer>(date: &Date, serializer: T) -> Result<T::Ok, T::Error> {
    let year = date.year() - DATE_YEAR_OFFSET;
    let ordinal = date.ordinal() - DATE_ORDINAL_OFFSET;
    let mut seq = serializer.serialize_seq(None)?;
    let mut buf = [0u8; u32::LEB128_BUF_SIZE];
    let size = year.encode_zigzag().encode_leb128(&mut buf);
    for e in buf[..size].iter() {
        seq.serialize_element(e)?;
    }
    let mut buf = [0u8; u16::LEB128_BUF_SIZE];
    let size = ordinal.encode_leb128(&mut buf);
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
    use crate::{de::Deserializer, ser::Serializer};
    use serde::{Deserialize, Serialize};
    use time::{Date, Month};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "crate::format::date")]
        date: Date,
    }

    #[test]
    fn serialize_date() {
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(2000, 1).unwrap()
            }),
            [0, 0]
        );
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(1936, 1).unwrap()
            }),
            [127, 0]
        );
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(1935, 1).unwrap()
            }),
            [129, 1, 0]
        );
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(2063, 128).unwrap()
            }),
            [126, 127]
        );
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(2064, 129).unwrap()
            }),
            [128, 1, 128, 1]
        );
        assert_eq!(
            serialize(Test {
                date: Date::from_ordinal_date(2000, 366).unwrap()
            }),
            [0, 237, 2]
        );
    }

    #[test]
    fn deserialize_date() {
        let buf = serialize(Test {
            date: Date::from_calendar_date(1970, Month::January, 11).unwrap(),
        });
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        let result = Test::deserialize(&mut deserializer).unwrap();
        assert_eq!(
            result,
            Test {
                date: Date::from_calendar_date(1970, Month::January, 11).unwrap()
            }
        )
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
