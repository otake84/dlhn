use std::io::{BufReader, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
    UInt8,
}

impl Header {
    pub fn body_size(&self) -> BodySize{
        match self {
            Header::Boolean => BodySize::Fix(1),
            Header::UInt8 => BodySize::Fix(1),
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Header::Boolean => {
                vec![0]
            }
            Header::UInt8 => {
                vec![1]
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(read: R) -> Result<Header, ()> {
        let mut reader = BufReader::new(read);
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).or(Err(()))?;

        if let Some(first) = buf.first() {
            match first {
                0 => Ok(Header::Boolean),
                1 => Ok(Header::UInt8),
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BodySize {
    Fix(usize),
    Variable,
}

#[cfg(test)]
mod tests {
    use super::Header;

    #[test]
    fn deserialize() {
        assert_eq!(Header::deserialize(&[0u8] as &[u8]), Ok(Header::Boolean));
        assert_eq!(Header::deserialize(&[1u8] as &[u8]), Ok(Header::UInt8));
    }
}
