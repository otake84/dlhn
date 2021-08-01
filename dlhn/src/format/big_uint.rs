use crate::de::Error;
use num_bigint::BigUint;
use num_traits::Zero;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserializer, Serializer,
};

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
        Ok(BigUint::from_bytes_le(v.as_slice()))
    }
}

pub fn serialize<T: Serializer>(big_uint: &BigUint, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;

    if big_uint.is_zero() {
        seq.serialize_element(&0u8)?;
    } else {
        seq.serialize_element(&big_uint.to_bytes_le())?;
    }

    seq.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<BigUint, T::Error> {
    deserializer.deserialize_tuple(1, BigUintVisitor)
}

#[cfg(test)]
mod tests {
    use crate::{de::Deserializer, ser::Serializer};
    use num_bigint::BigUint;
    use serde::{Deserialize, Serialize};
    use std::array::IntoIter;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "crate::format::big_uint")]
        big_uint: BigUint,
    }

    #[test]
    fn serilize() {
        assert_eq!(encode_big_uint(BigUint::from(0u8)), [0]);
        assert_eq!(encode_big_uint(BigUint::from(u8::MAX)), [1, 255]);
        assert_eq!(encode_big_uint(BigUint::from(u16::MAX)), [2, 255, 255]);
        assert_eq!(encode_big_uint(BigUint::from(u16::MAX) + 1u8), [3, 0, 0, 1]);
        assert_eq!(
            encode_big_uint(BigUint::from(u32::MAX)),
            [4, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u32::MAX) + 1u8),
            [5, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u64::MAX)),
            [8, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u64::MAX) + 1u8),
            [9, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u128::MAX)),
            [16, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u128::MAX) + 1u8),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_uint(big_uint: BigUint) {
            let buf = encode_big_uint(big_uint.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            let result = Test::deserialize(&mut deserializer).unwrap();
            assert_eq!(result, Test { big_uint });
        }

        IntoIter::new([
            BigUint::from(0u8),
            BigUint::from(u8::MAX),
            BigUint::from(u16::MAX),
            BigUint::from(u16::MAX) + 1u8,
            BigUint::from(u32::MAX),
            BigUint::from(u32::MAX) + 1u8,
            BigUint::from(u64::MAX),
            BigUint::from(u64::MAX) + 1u8,
            BigUint::from(u128::MAX),
            BigUint::from(u128::MAX) + 1u8,
        ])
        .for_each(assert_big_uint);
    }

    fn encode_big_uint(big_uint: BigUint) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test { big_uint };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
