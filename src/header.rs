use std::io::{BufReader, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
    UInt,
    UInt8,
    Int,
    Int8,
    String,
}

impl Header {
    pub fn body_size(&self) -> BodySize{
        match self {
            Header::Boolean => BodySize::Fix(1),
            Header::UInt => BodySize::Variable,
            Header::UInt8 => BodySize::Fix(1),
            Header::Int => BodySize::Variable,
            Header::Int8 => BodySize::Fix(1),
            Header::String => BodySize::Variable,
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Header::Boolean => {
                vec![0]
            }
            Header::UInt => {
                vec![1]
            }
            Header::UInt8 => {
                vec![2]
            }
            Header::Int => {
                vec![3]
            }
            Header::Int8 => {
                vec![4]
            }
            Header::String => {
                vec![5]
            }
        }
    }

    pub(crate) fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Header, ()> {
        let mut buf = [0u8; 1];
        buf_reader.read_exact(&mut buf).or(Err(()))?;

        match buf.first() {
            Some(0) => Ok(Header::Boolean),
            Some(1) => Ok(Header::UInt),
            Some(2) => Ok(Header::UInt8),
            Some(3) => Ok(Header::Int),
            Some(4) => Ok(Header::Int8),
            Some(5) => Ok(Header::String),
            _ => Err(())
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
        assert_eq!(Header::deserialize(&mut BufReader::new(&[1u8] as &[u8])), Ok(Header::UInt));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[2u8] as &[u8])), Ok(Header::UInt8));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[3u8] as &[u8])), Ok(Header::Int));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[4u8] as &[u8])), Ok(Header::Int8));
        assert_eq!(Header::deserialize(&mut BufReader::new(&[5u8] as &[u8])), Ok(Header::String));
    }
}
