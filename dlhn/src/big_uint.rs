use crate::de::Error;
use num_traits::Zero;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BigUint(Vec<u8>);

impl From<num_bigint::BigUint> for BigUint {
    fn from(v: num_bigint::BigUint) -> Self {
        if v.is_zero() {
            Self(Vec::new())
        } else {
            Self(v.to_bytes_le())
        }
    }
}

impl Into<num_bigint::BigUint> for BigUint {
    fn into(self) -> num_bigint::BigUint {
        num_bigint::BigUint::from_bytes_le(self.0.as_ref())
    }
}

impl Serialize for BigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.0)?;

        seq.end()
    }
}

struct BigUintVisitor;

impl<'de> Visitor<'de> for BigUintVisitor {
    type Value = BigUint;

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
        Ok(BigUint(v))
    }
}

impl<'de> Deserialize<'de> for BigUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(1, BigUintVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::BigUint;
    use crate::{de::Deserializer, ser::Serializer};
    use serde::{Deserialize, Serialize};
    use std::array::IntoIter;

    #[test]
    fn from() {
        let v = BigUint::from(num_bigint::BigUint::from(123u8));
        assert_eq!(v, BigUint(vec![123]),);
    }

    #[test]
    fn into() {
        let v: num_bigint::BigUint = BigUint::from(num_bigint::BigUint::from(123u8)).into();
        assert_eq!(v, num_bigint::BigUint::from(123u8));
    }

    #[test]
    fn serialize() {
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(0u8))),
            [0]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u8::MAX))),
            [1, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u16::MAX))),
            [2, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u16::MAX) + 1u8)),
            [3, 0, 0, 1]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u32::MAX))),
            [4, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u32::MAX) + 1u8)),
            [5, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u64::MAX))),
            [8, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u64::MAX) + 1u8)),
            [9, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u128::MAX))),
            [16, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(num_bigint::BigUint::from(u128::MAX) + 1u8)),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_uint(big_uint: BigUint) {
            let buf = encode_big_uint(big_uint.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = BigUint::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, big_uint);
        }

        IntoIter::new([
            BigUint::from(num_bigint::BigUint::from(0u8)),
            BigUint::from(num_bigint::BigUint::from(u8::MAX)),
            BigUint::from(num_bigint::BigUint::from(u16::MAX)),
            BigUint::from(num_bigint::BigUint::from(u16::MAX) + 1u8),
            BigUint::from(num_bigint::BigUint::from(u32::MAX)),
            BigUint::from(num_bigint::BigUint::from(u32::MAX) + 1u8),
            BigUint::from(num_bigint::BigUint::from(u64::MAX)),
            BigUint::from(num_bigint::BigUint::from(u64::MAX) + 1u8),
            BigUint::from(num_bigint::BigUint::from(u128::MAX)),
            BigUint::from(num_bigint::BigUint::from(u128::MAX) + 1u8),
        ])
        .for_each(assert_big_uint);
    }

    fn encode_big_uint(big_uint: BigUint) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        big_uint.serialize(&mut serializer).unwrap();
        buf
    }
}
