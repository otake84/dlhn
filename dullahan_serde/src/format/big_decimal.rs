use bigdecimal::{BigDecimal, Zero};
use serde::{Serializer, ser::SerializeSeq};

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

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use num_bigint::BigInt;
    use serde::Serialize;
    use crate::ser::Serializer;

    #[derive(Debug, PartialEq, Serialize)]
    struct Test {
        #[serde(with = "crate::format::big_decimal")]
        value: BigDecimal,
    }

    #[test]
    fn serilize() {
        assert_eq!(encode_big_decimal(BigDecimal::from(0)), [0]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), 0)), [1, 1, 0]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), -1)), [1, 1, 1]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), 1)), [1, 1, 2]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(10), 0)), [1, 1, 1]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), 63)), [1, 1, 126]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), 64)), [1, 1, 128, 1]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), -64)), [1, 1, 127]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(1), -65)), [1, 1, 129, 1]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(i16::MIN), 0)), [2, 0, 128, 0]);
        assert_eq!(encode_big_decimal(BigDecimal::new(BigInt::from(i16::MAX), 0)), [2, 255, 127, 0]);
    }

    fn encode_big_decimal(value: BigDecimal) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        let body = Test {
            value,
        };
        body.serialize(&mut serializer).unwrap();
        buf
    }
}
