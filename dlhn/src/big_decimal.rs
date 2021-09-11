use bigdecimal::Zero;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

use crate::de::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BigDecimal {
    signed_bytes: Vec<u8>,
    scale: i64,
}

impl From<bigdecimal::BigDecimal> for BigDecimal {
    fn from(v: bigdecimal::BigDecimal) -> Self {
        if v.is_zero() {
            Self {
                signed_bytes: Vec::new(),
                scale: 0,
            }
        } else {
            let (bigint, scale) = v.normalized().into_bigint_and_exponent();
            Self {
                signed_bytes: bigint.to_signed_bytes_le(),
                scale,
            }
        }
    }
}

impl Into<bigdecimal::BigDecimal> for BigDecimal {
    fn into(self) -> bigdecimal::BigDecimal {
        bigdecimal::BigDecimal::new(
            num_bigint::BigInt::from_signed_bytes_le(self.signed_bytes.as_ref()),
            self.scale,
        )
    }
}

impl Serialize for BigDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        if self.signed_bytes == [] {
            seq.serialize_element(&0u8)?;
        } else {
            seq.serialize_element(&self.signed_bytes)?;
            seq.serialize_element(&self.scale)?;
        }

        seq.end()
    }
}

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
        let digits = num_bigint::BigInt::from_signed_bytes_le(
            seq.next_element::<Vec<u8>>()?
                .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?
                .as_slice(),
        );
        if digits.is_zero() {
            Ok(BigDecimal {
                signed_bytes: Vec::new(),
                scale: 0,
            })
        } else {
            Ok(BigDecimal {
                signed_bytes: digits.to_signed_bytes_le(),
                scale: seq
                    .next_element::<i64>()?
                    .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?,
            })
        }
    }
}
impl<'de> Deserialize<'de> for BigDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, BigDecimalVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::array::IntoIter;

    use crate::{big_decimal::BigDecimal, de::Deserializer, ser::Serializer};
    use num_bigint::BigInt;
    use serde::{Deserialize, Serialize};

    #[test]
    fn from() {
        let v = BigDecimal::from(bigdecimal::BigDecimal::from(123));
        assert_eq!(
            v,
            BigDecimal {
                signed_bytes: vec![123],
                scale: 0,
            }
        );
    }

    #[test]
    fn into() {
        let v: bigdecimal::BigDecimal = BigDecimal::from(bigdecimal::BigDecimal::from(-123)).into();
        assert_eq!(v, bigdecimal::BigDecimal::from(-123));
    }

    #[test]
    fn serialize() {
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::from(0))),
            [0]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                0
            ))),
            [1, 1, 0]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                -1
            ))),
            [1, 1, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                1
            ))),
            [1, 1, 2]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(10),
                0
            ))),
            [1, 1, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                63
            ))),
            [1, 1, 126]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                64
            ))),
            [1, 1, 128, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                -64
            ))),
            [1, 1, 127]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(1),
                -65
            ))),
            [1, 1, 129, 1]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(i16::MIN),
                0
            ))),
            [2, 0, 128, 0]
        );
        assert_eq!(
            encode_big_decimal(BigDecimal::from(bigdecimal::BigDecimal::new(
                BigInt::from(i16::MAX),
                0
            ))),
            [2, 255, 127, 0]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_decimal(value: BigDecimal) {
            let buf = encode_big_decimal(value.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = BigDecimal::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, value);
        }

        IntoIter::new([
            BigDecimal::from(bigdecimal::BigDecimal::from(0)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), 0)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), -1)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), 1)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), 63)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), 64)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), -64)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(1), -65)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(i16::MIN), 0)),
            BigDecimal::from(bigdecimal::BigDecimal::new(BigInt::from(i16::MAX), 0)),
        ])
        .for_each(assert_big_decimal);
    }

    fn encode_big_decimal(value: BigDecimal) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        value.serialize(&mut serializer).unwrap();
        buf
    }
}
