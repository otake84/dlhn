use crate::de::Error;
use bigdecimal::{BigDecimal, Zero};
use num_bigint::BigInt;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserializer, Serializer,
};

struct BigDecimalVisitor;

impl<'de> Visitor<'de> for BigDecimalVisitor {
    type Value = BigDecimal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let digits = BigInt::from_signed_bytes_le(
            seq.next_element::<Vec<u8>>()?
                .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?
                .as_slice(),
        );
        if digits.is_zero() {
            Ok(BigDecimal::from(digits))
        } else {
            Ok(BigDecimal::new(
                digits,
                seq.next_element::<i64>()?
                    .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?,
            ))
        }
    }
}

pub fn serialize<T: Serializer>(value: &BigDecimal, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;

    if value.is_zero() {
        seq.serialize_element(&0u8)?;
    } else {
        let (bigint, scale) = value.normalized().into_bigint_and_exponent();
        seq.serialize_element(&bigint.to_signed_bytes_le())?;
        seq.serialize_element(&scale)?;
    }

    seq.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<BigDecimal, T::Error> {
    deserializer.deserialize_tuple(2, BigDecimalVisitor)
}

#[cfg(test)]
mod tests {
    use crate::{de::Deserializer, ser::Serializer};
    use bigdecimal::BigDecimal;
    use num_bigint::BigInt;
    use serde::{Deserialize, Serialize};
    use std::array::IntoIter;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "crate::format::big_decimal")]
        value: BigDecimal,
    }

    #[test]
    fn serilize() {
        assert_eq!(encode_big_decimal(BigDecimal::from(0)), [0]);
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), 0)),
            [1, 1, 0]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), -1)),
            [1, 1, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), 1)),
            [1, 1, 2]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(10), 0)),
            [1, 1, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), 63)),
            [1, 1, 126]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), 64)),
            [1, 1, 128, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), -64)),
            [1, 1, 127]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(1), -65)),
            [1, 1, 129, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(i16::MIN), 0)),
            [2, 0, 128, 0]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::new(BigInt::from(i16::MAX), 0)),
            [2, 255, 127, 0]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_decimal(value: BigDecimal) {
            let buf = encode_big_decimal(value.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, Test { value });
        }

        IntoIter::new([
            BigDecimal::from(0),
            BigDecimal::new(BigInt::from(1), 0),
            BigDecimal::new(BigInt::from(1), -1),
            BigDecimal::new(BigInt::from(1), 1),
            BigDecimal::new(BigInt::from(1), 63),
            BigDecimal::new(BigInt::from(1), 64),
            BigDecimal::new(BigInt::from(1), -64),
            BigDecimal::new(BigInt::from(1), -65),
            BigDecimal::new(BigInt::from(i16::MIN), 0),
            BigDecimal::new(BigInt::from(i16::MAX), 0),
        ])
        .for_each(assert_big_decimal);
    }

    fn encode_big_decimal(value: BigDecimal) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test { value };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
