use serde::{Deserializer, Serializer, de::{self, SeqAccess, Unexpected, Visitor}, ser::SerializeSeq};
use time::{NumericalDuration, OffsetDateTime};
use crate::de::Error;

struct OffsetDateTimeVisitor;

impl<'de> Visitor<'de> for OffsetDateTimeVisitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
            A: SeqAccess<'de>, {
                let unix_timestamp = seq.next_element::<i64>()?.ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?;
                let nanosecond = seq.next_element::<u32>()?.ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?;
                Ok(OffsetDateTime::from_unix_timestamp(unix_timestamp) + nanosecond.nanoseconds())
    }
}

pub fn serialize<T: Serializer>(date_time: &OffsetDateTime, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;
    seq.serialize_element(&date_time.unix_timestamp())?;
    seq.serialize_element(&date_time.time().nanosecond())?;
    seq.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<OffsetDateTime, T::Error> {
    deserializer.deserialize_tuple(2, OffsetDateTimeVisitor)
}

#[cfg(test)]
mod tests {
    use std::array::IntoIter;
    use integer_encoding::VarInt;
    use serde::{Serialize, Deserialize};
    use time::{NumericalDuration, OffsetDateTime};
    use crate::{de::Deserializer, ser::Serializer};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "crate::format::date_time")]
        date_time: OffsetDateTime,
    }

    #[test]
    fn serialize_date_time() {
        fn assert_date_time(date_time: OffsetDateTime) {
            assert_eq!(encode_date_time(date_time), [date_time.unix_timestamp().encode_var_vec(), date_time.nanosecond().encode_var_vec()].concat());
        }

        IntoIter::new([
            OffsetDateTime::unix_epoch(),
            OffsetDateTime::unix_epoch() + 1.nanoseconds(),
            OffsetDateTime::unix_epoch() + 999999999.nanoseconds(),
            OffsetDateTime::unix_epoch() + 1000000000.nanoseconds(),
            OffsetDateTime::unix_epoch() - 100000.days(),
            OffsetDateTime::unix_epoch() + 100000.days(),
            OffsetDateTime::unix_epoch() - 100000.days() - 999999999.nanoseconds(),
            OffsetDateTime::unix_epoch() + 100000.days() + 1.nanoseconds(),
        ]).for_each(assert_date_time);

        assert_eq!(encode_date_time(OffsetDateTime::unix_epoch() + 1000000000.nanoseconds()), encode_date_time(OffsetDateTime::unix_epoch() + 1.seconds()));
    }

    #[test]
    fn deserialize_datetime() {
        fn assert_date_time(date_time: OffsetDateTime) {
            let buf = encode_date_time(date_time);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, Test { date_time });
        }

        IntoIter::new([
            OffsetDateTime::unix_epoch(),
            OffsetDateTime::unix_epoch() + 1.nanoseconds(),
            OffsetDateTime::unix_epoch() + 999999999.nanoseconds(),
            OffsetDateTime::unix_epoch() + 1000000000.nanoseconds(),
            OffsetDateTime::unix_epoch() - 100000.days(),
            OffsetDateTime::unix_epoch() + 100000.days(),
            OffsetDateTime::unix_epoch() - 100000.days() - 999999999.nanoseconds(),
            OffsetDateTime::unix_epoch() + 100000.days() + 1.nanoseconds(),
        ]).for_each(assert_date_time);
    }

    fn encode_date_time(date_time: OffsetDateTime) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            date_time,
        };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
