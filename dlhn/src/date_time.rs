use serde::{Deserialize, Serialize};
#[cfg(feature = "time")]
use time::{ext::NumericalDuration, OffsetDateTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTime {
    unix_timestamp: i64,
    nanosecond: u32,
}

#[cfg(feature = "time")]
impl From<OffsetDateTime> for DateTime {
    fn from(date_time: OffsetDateTime) -> Self {
        Self {
            unix_timestamp: date_time.unix_timestamp(),
            nanosecond: date_time.nanosecond(),
        }
    }
}

#[cfg(feature = "time")]
impl std::convert::TryInto<OffsetDateTime> for DateTime {
    type Error = ();

    fn try_into(self) -> Result<OffsetDateTime, Self::Error> {
        OffsetDateTime::from_unix_timestamp(self.unix_timestamp)
            .map(|v| v + (self.nanosecond as i64).nanoseconds())
            .or(Err(()))
    }
}

#[cfg(feature = "time")]
#[cfg(test)]
mod tests {
    use super::DateTime;
    use crate::{Deserializer, PrefixVarint, Serializer, ZigZag};
    use serde::{Deserialize, Serialize};
    use std::convert::TryInto;
    use time::{ext::NumericalDuration, OffsetDateTime};

    #[test]
    fn from() {
        let date_time = DateTime::from(OffsetDateTime::UNIX_EPOCH);
        assert_eq!(
            date_time,
            DateTime {
                unix_timestamp: 0,
                nanosecond: 0
            }
        );
    }

    #[test]
    fn try_into() {
        let date_time = DateTime::from(OffsetDateTime::UNIX_EPOCH);
        let offset_date_time: OffsetDateTime = date_time.try_into().unwrap();
        assert_eq!(offset_date_time, OffsetDateTime::UNIX_EPOCH);
    }

    #[test]
    fn serialize() {
        fn assert_date_time(date_time: OffsetDateTime) {
            assert_eq!(
                encode_date_time(DateTime::from(date_time)),
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
            encode_date_time(DateTime::from(
                OffsetDateTime::UNIX_EPOCH + 1000000000.nanoseconds()
            )),
            encode_date_time(DateTime::from(OffsetDateTime::UNIX_EPOCH + 1.seconds()))
        );
    }

    #[test]
    fn deserialize() {
        fn assert_date_time(date_time: OffsetDateTime) {
            let buf = encode_date_time(DateTime::from(date_time));
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = DateTime::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, DateTime::from(date_time));
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

    fn encode_date_time(date_time: DateTime) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        date_time.serialize(&mut serializer).unwrap();
        buf
    }
}
