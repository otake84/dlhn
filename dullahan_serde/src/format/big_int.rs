use num_bigint::BigInt;
use num_traits::Zero;
use serde::{Serializer, ser::SerializeSeq};

pub fn serialize<T: Serializer>(big_int: &BigInt, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;

    if big_int.is_zero() {
        seq.serialize_element(&0u8)?;
    } else {
        seq.serialize_element(&big_int.to_signed_bytes_le())?;
    }

    seq.end()
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use serde::Serialize;
    use crate::ser::Serializer;

    #[derive(Debug, PartialEq, Serialize)]
    struct Test {
        #[serde(with = "crate::format::big_int")]
        big_int: BigInt,
    }

    #[test]
    fn serilize() {
        assert_eq!(encode_big_int(BigInt::from(0)), [0]);
        assert_eq!(encode_big_int(BigInt::from(i8::MIN)), [[1], i8::MIN.to_le_bytes()].concat());
        assert_eq!(encode_big_int(BigInt::from(i8::MAX)), [[1], i8::MAX.to_le_bytes()].concat());
        assert_eq!(encode_big_int(BigInt::from(i16::MIN)), [[2].as_ref(), i16::MIN.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i16::MAX)), [[2].as_ref(), i16::MAX.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i16::MIN) - 1), [3, 255, 127, 255]);
        assert_eq!(encode_big_int(BigInt::from(i16::MAX) + 1), [3, 0, 128, 0]);
        assert_eq!(encode_big_int(BigInt::from(i32::MIN)), [[4].as_ref(), i32::MIN.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i32::MAX)), [[4].as_ref(), i32::MAX.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i32::MIN) - 1), [5, 255, 255, 255, 127, 255]);
        assert_eq!(encode_big_int(BigInt::from(i32::MAX) + 1), [5, 0, 0, 0, 128, 0]);
        assert_eq!(encode_big_int(BigInt::from(i64::MIN)), [[8].as_ref(), i64::MIN.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i64::MAX)), [[8].as_ref(), i64::MAX.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i64::MIN) - 1), [9, 255, 255, 255, 255, 255, 255, 255, 127, 255]);
        assert_eq!(encode_big_int(BigInt::from(i64::MAX) + 1), [9, 0, 0, 0, 0, 0, 0, 0, 128, 0]);
        assert_eq!(encode_big_int(BigInt::from(i128::MIN)), [[16].as_ref(), i128::MIN.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i128::MAX)), [[16].as_ref(), i128::MAX.to_le_bytes().as_ref()].concat());
        assert_eq!(encode_big_int(BigInt::from(i128::MIN) - 1), [17, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127, 255]);
        assert_eq!(encode_big_int(BigInt::from(i128::MAX) + 1), [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0]);
    }

    fn encode_big_int(big_int: BigInt) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            big_int,
        };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
