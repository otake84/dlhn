use serde::{Serializer, ser::SerializeSeq};
use time::OffsetDateTime;

pub fn serialize<T: Serializer>(date_time: &OffsetDateTime, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;
    seq.serialize_element(&date_time.unix_timestamp())?;
    seq.serialize_element(&date_time.time().nanosecond())?;
    seq.end()
}

#[cfg(test)]
mod tests {
    use std::array::IntoIter;
    use integer_encoding::VarInt;
    use serde::Serialize;
    use time::{NumericalDuration, OffsetDateTime};
    use crate::ser::Serializer;

    #[test]
    fn serialize_date_time() {
        #[derive(Debug, PartialEq, Serialize)]
        struct Test {
            #[serde(with = "crate::format::date_time")]
            date_time: OffsetDateTime,
        }

        fn assert_date_time(date_time: OffsetDateTime) {
            let buf = encode_date_time(date_time);

            assert_eq!(buf, [date_time.unix_timestamp().encode_var_vec(), date_time.nanosecond().encode_var_vec()].concat());
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
}
