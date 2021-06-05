use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Serializer, ser::SerializeSeq};

pub fn serialize<T: Serializer>(big_uint: &BigUint, serializer: T) -> Result<T::Ok, T::Error> {
    let mut seq = serializer.serialize_seq(None)?;

    if big_uint.is_zero() {
        seq.serialize_element(&0u8)?;
    } else {
        seq.serialize_element(&big_uint.to_bytes_le())?;
    }

    seq.end()
}


#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use serde::Serialize;
    use crate::ser::Serializer;

    #[derive(Serialize)]
    struct Test {
        #[serde(with = "crate::format::big_uint")]
        big_uint: BigUint,
    }

    #[test]
    fn serilize() {
        assert_eq!(encode_big_uint(BigUint::from(0u8)), [0]);
        assert_eq!(encode_big_uint(BigUint::from(u8::MAX)), [1, 255]);
        assert_eq!(
            encode_big_uint(BigUint::from(u16::MAX)),
            [2, 255, 255]
        );
        assert_eq!(
            encode_big_uint(BigUint::from(u16::MAX) + 1u8),
            [3, 0, 0, 1]
        );
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

    fn encode_big_uint(big_uint: BigUint) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            big_uint,
        };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
