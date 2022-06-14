use crate::de::Error;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeTuple,
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
    let mut tuple = serializer.serialize_tuple(2)?;
    tuple.serialize_element(&(date.year() - DATE_YEAR_OFFSET))?;
    tuple.serialize_element(&(date.ordinal() - DATE_ORDINAL_OFFSET))?;
    tuple.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<Date, T::Error> {
    deserializer.deserialize_tuple(2, DateVisitor)
}

#[cfg(test)]
mod tests {
    use crate::{Deserializer, PrefixVarint, Serializer, ZigZag};
    use time::{Date, Month};

    #[test]
    fn serialize_date() {
        assert_eq!(
            serialize(Date::from_ordinal_date(2000, 1).unwrap()),
            [
                0i32.encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from_ordinal_date(1936, 1).unwrap()),
            [
                (1936i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from_ordinal_date(1935, 1).unwrap()),
            [
                (1935i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                0u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from_ordinal_date(2063, 128).unwrap()),
            [
                (2063i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                127u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from_ordinal_date(2064, 129).unwrap()),
            [
                (2064i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                128u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
        assert_eq!(
            serialize(Date::from_ordinal_date(2000, 366).unwrap()),
            [
                (2000i32 - 2000).encode_zigzag().encode_prefix_varint_vec(),
                365u16.encode_prefix_varint_vec()
            ]
            .concat(),
        );
    }

    #[test]
    fn deserialize_date() {
        let buf = serialize(Date::from_calendar_date(1970, Month::January, 11).unwrap());
        let mut reader = buf.as_slice();
        let mut deserializer = Deserializer::new(&mut reader);
        assert_eq!(
            Date::from_calendar_date(1970, Month::January, 11).unwrap(),
            super::deserialize(&mut deserializer).unwrap()
        );
    }

    fn serialize(date: Date) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        super::serialize(&date, &mut serializer).unwrap();
        buf
    }
}
