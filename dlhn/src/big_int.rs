use crate::de::Error;
#[cfg(feature = "num-traits")]
use num_traits::Zero;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BigInt(Vec<u8>);

#[cfg(all(feature = "num-traits", feature = "num-bigint"))]
impl From<num_bigint::BigInt> for BigInt {
    fn from(v: num_bigint::BigInt) -> Self {
        if v.is_zero() {
            Self(Vec::new())
        } else {
            Self(v.to_signed_bytes_le())
        }
    }
}

#[cfg(all(feature = "num-traits", feature = "num-bigint"))]
impl Into<num_bigint::BigInt> for BigInt {
    fn into(self) -> num_bigint::BigInt {
        num_bigint::BigInt::from_signed_bytes_le(self.0.as_ref())
    }
}

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.0)?;

        seq.end()
    }
}

struct BigIntVisitor;

impl<'de> Visitor<'de> for BigIntVisitor {
    type Value = BigInt;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let v = seq
            .next_element::<Vec<u8>>()?
            .ok_or(de::Error::invalid_value(Unexpected::Seq, &Error::Read))?;
        Ok(BigInt(v))
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(1, BigIntVisitor)
    }
}

#[cfg(all(feature = "num-traits", feature = "num-bigint"))]
#[cfg(test)]
mod tests {
    use std::array::IntoIter;

    use crate::{big_int::BigInt, de::Deserializer, ser::Serializer};
    use serde::{Deserialize, Serialize};

    #[test]
    fn from() {
        let v = BigInt::from(num_bigint::BigInt::from(123u8));
        assert_eq!(v, BigInt(vec![123]));
    }

    #[test]
    fn into() {
        let v: num_bigint::BigInt = BigInt::from(num_bigint::BigInt::from(-123)).into();
        assert_eq!(v, num_bigint::BigInt::from(-123));
    }

    #[test]
    fn serilize() {
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(0))),
            [0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i8::MIN))),
            [[1], i8::MIN.to_le_bytes()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i8::MAX))),
            [[1], i8::MAX.to_le_bytes()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i16::MIN))),
            [[2].as_ref(), i16::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i16::MAX))),
            [[2].as_ref(), i16::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i16::MIN) - 1)),
            [3, 255, 127, 255]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i16::MAX) + 1)),
            [3, 0, 128, 0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i32::MIN))),
            [[4].as_ref(), i32::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i32::MAX))),
            [[4].as_ref(), i32::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i32::MIN) - 1)),
            [5, 255, 255, 255, 127, 255]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i32::MAX) + 1)),
            [5, 0, 0, 0, 128, 0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i64::MIN))),
            [[8].as_ref(), i64::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i64::MAX))),
            [[8].as_ref(), i64::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i64::MIN) - 1)),
            [9, 255, 255, 255, 255, 255, 255, 255, 127, 255]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i64::MAX) + 1)),
            [9, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i128::MIN))),
            [[16].as_ref(), i128::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i128::MAX))),
            [[16].as_ref(), i128::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i128::MIN) - 1)),
            [
                17, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
                255
            ]
        );
        assert_eq!(
            encode_big_int(BigInt::from(num_bigint::BigInt::from(i128::MAX) + 1)),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_int(big_int: BigInt) {
            let buf = encode_big_int(big_int.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = BigInt::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, big_int);
        }

        IntoIter::new([
            BigInt::from(num_bigint::BigInt::from(0)),
            BigInt::from(num_bigint::BigInt::from(i8::MIN)),
            BigInt::from(num_bigint::BigInt::from(i8::MAX)),
            BigInt::from(num_bigint::BigInt::from(i8::MIN) - 1),
            BigInt::from(num_bigint::BigInt::from(i8::MAX) + 1),
            BigInt::from(num_bigint::BigInt::from(i16::MIN)),
            BigInt::from(num_bigint::BigInt::from(i16::MAX)),
            BigInt::from(num_bigint::BigInt::from(i16::MIN) - 1),
            BigInt::from(num_bigint::BigInt::from(i16::MAX) + 1),
            BigInt::from(num_bigint::BigInt::from(i32::MIN)),
            BigInt::from(num_bigint::BigInt::from(i32::MAX)),
            BigInt::from(num_bigint::BigInt::from(i32::MIN) - 1),
            BigInt::from(num_bigint::BigInt::from(i32::MAX) + 1),
            BigInt::from(num_bigint::BigInt::from(i64::MIN)),
            BigInt::from(num_bigint::BigInt::from(i64::MAX)),
            BigInt::from(num_bigint::BigInt::from(i64::MIN) - 1),
            BigInt::from(num_bigint::BigInt::from(i64::MAX) + 1),
            BigInt::from(num_bigint::BigInt::from(i128::MIN)),
            BigInt::from(num_bigint::BigInt::from(i128::MAX)),
            BigInt::from(num_bigint::BigInt::from(i128::MIN) - 1),
            BigInt::from(num_bigint::BigInt::from(i128::MAX) + 1),
        ])
        .for_each(assert_big_int);
    }

    fn encode_big_int(big_int: BigInt) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        big_int.serialize(&mut serializer).unwrap();
        buf
    }
}
