use std::io::{BufReader, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
    UInt8,
    Int8,
}

impl Header {
    pub fn body_size(&self) -> BodySize{
        match self {
            Header::Boolean => BodySize::Fix(1),
            Header::UInt8 => BodySize::Fix(1),
            Header::Int8 => BodySize::Fix(1),
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
            Header::Int8 => {
                vec![2]
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Header, ()> {
        let mut buf = [0u8; 1];
        buf_reader.read_exact(&mut buf).or(Err(()))?;

        if let Some(first) = buf.first() {
            match first {
                0 => Ok(Header::Boolean),
                1 => Ok(Header::UInt8),
                2 => Ok(Header::Int8),
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
    use std::io::BufReader;

    use super::Header;

    #[test]
    fn deserialize() {
        assert_eq!(Header::deserialize(&mut BufReader::new(&[0u8] as &[u8])), Ok(Header::Boolean));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[1u8] as &[u8])), Ok(Header::UInt8));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[2u8] as &[u8])), Ok(Header::Int8));
    }
}
