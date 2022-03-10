use crate::de::Error;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserializer, Serializer,
};
use time::{ext::NumericalDuration, OffsetDateTime};

struct OffsetDateTimeVisitor;

impl<'de> Visitor<'de> for OffsetDateTimeVisitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let unix_timestamp = seq
            .next_element::<i64>()?
            .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?;
        let nanosecond = seq
            .next_element::<u32>()?
            .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?;
        Ok(OffsetDateTime::from_unix_timestamp(unix_timestamp)
            .or(Err(de::Error::invalid_value(Unexpected::Seq, &Error::Read)))?
            + (nanosecond as i64).nanoseconds())
    }
}

pub fn serialize<T: Serializer>(
    date_time: &OffsetDateTime,
    serializer: T,
) -> Result<T::Ok, T::Error> {
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
    use crate::{de::Deserializer, prefix_varint::PrefixVarint, ser::Serializer, zigzag::ZigZag};
    use time::{ext::NumericalDuration, OffsetDateTime};

    #[test]
    fn serialize_date_time() {
        fn assert_date_time(date_time: OffsetDateTime) {
            assert_eq!(
                encode_date_time(date_time),
                [
                    date_time
                        .unix_timestamp()
                        .encode_zigzag()
                        .encode_prefix_varint_vec(),
                    date_time.nanosecond().encode_prefix_varint_vec()
                ]
                .concat()
            );
        }

        IntoIterator::into_iter([
            OffsetDateTime::UNIX_EPOCH,
            OffsetDateTime::UNIX_EPOCH + 1.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 999999999.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 1000000000.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH - 100000.days(),
            OffsetDateTime::UNIX_EPOCH + 100000.days(),
            OffsetDateTime::UNIX_EPOCH - 100000.days() - 999999999.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 100000.days() + 1.nanoseconds(),
        ])
        .for_each(assert_date_time);

        assert_eq!(
            encode_date_time(OffsetDateTime::UNIX_EPOCH + 1000000000.nanoseconds()),
            encode_date_time(OffsetDateTime::UNIX_EPOCH + 1.seconds())
        );
    }

    #[test]
    fn deserialize_datetime() {
        fn assert_date_time(date_time: OffsetDateTime) {
            let buf = encode_date_time(date_time);
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(date_time, super::deserialize(&mut deserializer).unwrap());
        }

        IntoIterator::into_iter([
            OffsetDateTime::UNIX_EPOCH,
            OffsetDateTime::UNIX_EPOCH + 1.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 999999999.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 1000000000.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH - 100000.days(),
            OffsetDateTime::UNIX_EPOCH + 100000.days(),
            OffsetDateTime::UNIX_EPOCH - 100000.days() - 999999999.nanoseconds(),
            OffsetDateTime::UNIX_EPOCH + 100000.days() + 1.nanoseconds(),
        ])
        .for_each(assert_date_time);
    }

    fn encode_date_time(date_time: OffsetDateTime) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        super::serialize(&date_time, &mut serializer).unwrap();
        buf
    }
}
