use crate::de::Error;
use num_bigint::BigInt;
use num_traits::Zero;
use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserializer, Serializer,
};

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
        Ok(BigInt::from_signed_bytes_le(v.as_slice()))
    }
}

pub fn serialize<T: Serializer>(big_int: &BigInt, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;

    if big_int.is_zero() {
        seq.serialize_element(&0u8)?;
    } else {
        seq.serialize_element(&big_int.to_signed_bytes_le())?;
    }

    seq.end()
}

pub fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<BigInt, T::Error> {
    deserializer.deserialize_tuple(1, BigIntVisitor)
}

#[cfg(test)]
mod tests {
    use crate::{de::Deserializer, ser::Serializer};
    use num_bigint::BigInt;
    use std::array::IntoIter;

    #[test]
    fn serilize() {
        assert_eq!(encode_big_int(BigInt::from(0)), [0]);
        assert_eq!(
            encode_big_int(BigInt::from(i8::MIN)),
            [[1], i8::MIN.to_le_bytes()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i8::MAX)),
            [[1], i8::MAX.to_le_bytes()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i16::MIN)),
            [[2].as_ref(), i16::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i16::MAX)),
            [[2].as_ref(), i16::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i16::MIN) - 1),
            [3, 255, 127, 255]
        );
        assert_eq!(encode_big_int(BigInt::from(i16::MAX) + 1), [3, 0, 128, 0]);
        assert_eq!(
            encode_big_int(BigInt::from(i32::MIN)),
            [[4].as_ref(), i32::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i32::MAX)),
            [[4].as_ref(), i32::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i32::MIN) - 1),
            [5, 255, 255, 255, 127, 255]
        );
        assert_eq!(
            encode_big_int(BigInt::from(i32::MAX) + 1),
            [5, 0, 0, 0, 128, 0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(i64::MIN)),
            [[8].as_ref(), i64::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i64::MAX)),
            [[8].as_ref(), i64::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i64::MIN) - 1),
            [9, 255, 255, 255, 255, 255, 255, 255, 127, 255]
        );
        assert_eq!(
            encode_big_int(BigInt::from(i64::MAX) + 1),
            [9, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );
        assert_eq!(
            encode_big_int(BigInt::from(i128::MIN)),
            [[16].as_ref(), i128::MIN.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i128::MAX)),
            [[16].as_ref(), i128::MAX.to_le_bytes().as_ref()].concat()
        );
        assert_eq!(
            encode_big_int(BigInt::from(i128::MIN) - 1),
            [
                17, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
                255
            ]
        );
        assert_eq!(
            encode_big_int(BigInt::from(i128::MAX) + 1),
            [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0]
        );
    }

    #[test]
    fn deserialize() {
        fn assert_big_int(big_int: BigInt) {
            let buf = encode_big_int(big_int.clone());
            let mut reader = buf.as_slice();
            let mut deserializer = Deserializer::new(&mut reader);
            assert_eq!(big_int, super::deserialize(&mut deserializer).unwrap());
        }

        IntoIter::new([
            BigInt::from(0),
            BigInt::from(i8::MIN),
            BigInt::from(i8::MAX),
            BigInt::from(i8::MIN) - 1,
            BigInt::from(i8::MAX) + 1,
            BigInt::from(i16::MIN),
            BigInt::from(i16::MAX),
            BigInt::from(i16::MIN) - 1,
            BigInt::from(i16::MAX) + 1,
            BigInt::from(i32::MIN),
            BigInt::from(i32::MAX),
            BigInt::from(i32::MIN) - 1,
            BigInt::from(i32::MAX) + 1,
            BigInt::from(i64::MIN),
            BigInt::from(i64::MAX),
            BigInt::from(i64::MIN) - 1,
            BigInt::from(i64::MAX) + 1,
            BigInt::from(i128::MIN),
            BigInt::from(i128::MAX),
            BigInt::from(i128::MIN) - 1,
            BigInt::from(i128::MAX) + 1,
        ])
        .for_each(assert_big_int);
    }

    fn encode_big_int(big_int: BigInt) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        super::serialize(&big_int, &mut serializer).unwrap();
        buf
    }
}
