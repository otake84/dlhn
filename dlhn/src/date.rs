use crate::de::Error;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeTuple,
    Deserialize, Serialize,
};

const DATE_YEAR_OFFSET: i32 = 2000;
const DATE_ORDINAL_OFFSET: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    year: i32,
    ordinal: u16,
}

#[cfg(feature = "time")]
impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Self {
            year: date.year(),
            ordinal: date.ordinal(),
        }
    }
}

#[cfg(feature = "time")]
impl std::convert::TryInto<time::Date> for Date {
    type Error = ();

    fn try_into(self) -> Result<time::Date, Self::Error> {
        time::Date::from_ordinal_date(self.year, self.ordinal).or(Err(()))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&(self.year - DATE_YEAR_OFFSET))?;
        tuple.serialize_element(&(self.ordinal - DATE_ORDINAL_OFFSET))?;
        tuple.end()
    }
}

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
        Ok(Date { year, ordinal })
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, DateVisitor)
    }
}

#[cfg(feature = "time")]
#[cfg(test)]
mod tests {
    use super::Date;
    use crate::{de::Deserializer, prefix_varint::PrefixVarint, ser::Serializer, zigzag::ZigZag};
    use serde::{Deserialize, Serialize};
    use std::convert::TryInto;

    #[test]
    fn from() {
        let date = Date::from(time::Date::from_ordinal_date(2020, 12).unwrap());
        assert_eq!(
            date,
            Date {
                year: 2020,
                ordinal: 12,
            }
        );
    }

    #[test]
    fn try_into() {
        let date = Date::from(time::Date::from_ordinal_date(2020, 12).unwrap());
        let time_date: time::Date = date.try_into().unwrap();
        assert_eq!(time_date, time::Date::from_ordinal_date(2020, 12).unwrap());
    }

    #[test]
    fn serialize_date() {
        assert_eq!(
            serialize(Date::from(time::Date::from_ordinal_date(2000, 1).unwrap())),
            [
                0i32.encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from(time::Date::from_ordinal_date(1936, 1).unwrap())),
            [
                (1936i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from(time::Date::from_ordinal_date(1935, 1).unwrap())),
            [
                (1935i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from(
                time::Date::from_ordinal_date(2063, 128).unwrap()
            )),
            [
                (2063i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                127u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from(
                time::Date::from_ordinal_date(2064, 129).unwrap()
            )),
            [
                (2064i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                128u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from(
                time::Date::from_ordinal_date(2000, 366).unwrap()
            )),
            [
                (2000i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                365u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
    }

    #[test]
    fn deserialize_date() {
        let buf = serialize(Date::from(
            time::Date::from_calendar_date(1970, time::Month::January, 11).unwrap(),
        ));
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        assert_eq!(
            Date::deserialize(&mut deserializer).unwrap(),
            Date::from(time::Date::from_calendar_date(1970, time::Month::January, 11).unwrap())
        );
    }

    fn serialize<T: Serialize>(v: T) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        v.serialize(&mut serializer).unwrap();
        buf
    }
}
