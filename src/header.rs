use std::io::{BufReader, Read};

use indexmap::IndexMap;
use integer_encoding::{VarInt, VarIntReader};

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
    UInt,
    UInt8,
    Int,
    Int8,
    Float32,
    Float64,
    String,
    Binary,
    Array(Box<Header>),
    Map(IndexMap<String, Header>),
}

impl Header {
    pub const fn body_size(&self) -> BodySize{
        match self {
            Header::Boolean => BodySize::Fix(1),
            Header::UInt => BodySize::Variable,
            Header::UInt8 => BodySize::Fix(1),
            Header::Int => BodySize::Variable,
            Header::Int8 => BodySize::Fix(1),
            Header::Float32 => BodySize::Fix(4),
            Header::Float64 => BodySize::Fix(8),
            Header::String => BodySize::Variable,
            Header::Binary => BodySize::Variable,
            Header::Array(_) => BodySize::Variable,
            Header::Map(_) => BodySize::Variable,
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
            Header::Float32 => {
                vec![5]
            }
            Header::Float64 => {
                vec![6]
            }
            Header::String => {
                vec![7]
            }
            Header::Binary => {
                vec![8]
            }
            Header::Array(inner) => {
                vec![vec![9], inner.serialize()].concat()
            }
            Header::Map(inner) => {
                vec![vec![10], inner.len().encode_var_vec(), inner.iter().flat_map(|v| [Self::serialize_map_key(v.0), v.1.serialize()].concat()).collect()].concat()
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
            Some(5) => Ok(Header::Float32),
            Some(6) => Ok(Header::Float64),
            Some(7) => Ok(Header::String),
            Some(8) => Ok(Header::Binary),
            Some(9) => {
                let inner = Self::deserialize(buf_reader)?;
                Ok(Header::Array(Box::new(inner)))
            }
            Some(10) => {
                let size = buf_reader.read_varint::<usize>().or(Err(()))?;
                let mut index_map: IndexMap<String, Header> = IndexMap::with_capacity(size);
                for _ in 0..size {
                    index_map.insert(Self::deserialize_map_key(buf_reader)?, Self::deserialize(buf_reader)?);
                }
                Ok(Header::Map(index_map))
            }
            _ => Err(())
        }
    }

    fn serialize_map_key(v: &str) -> Vec<u8> {
        [v.len().encode_var_vec().as_ref(), v.as_bytes()].concat()
    }

    fn deserialize_map_key<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, ()> {
        let size = buf_reader.read_varint::<usize>().or(Err(()))?;
        let mut body_buf = Vec::with_capacity(size);
        unsafe { body_buf.set_len(size); }
        buf_reader.read_exact(&mut body_buf).or(Err(()))?;
        String::from_utf8(body_buf).or(Err(()))
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
    use indexmap::*;

    #[test]
    fn deserialize() {
        assert_eq!(Header::deserialize(&mut BufReader::new([0u8].as_ref())), Ok(Header::Boolean));
        assert_eq!(Header::deserialize(&mut BufReader::new([1u8].as_ref())), Ok(Header::UInt));
        assert_eq!(Header::deserialize(&mut BufReader::new([2u8].as_ref())), Ok(Header::UInt8));
        assert_eq!(Header::deserialize(&mut BufReader::new([3u8].as_ref())), Ok(Header::Int));
        assert_eq!(Header::deserialize(&mut BufReader::new([4u8].as_ref())), Ok(Header::Int8));
        assert_eq!(Header::deserialize(&mut BufReader::new([5u8].as_ref())), Ok(Header::Float32));
        assert_eq!(Header::deserialize(&mut BufReader::new([6u8].as_ref())), Ok(Header::Float64));
        assert_eq!(Header::deserialize(&mut BufReader::new([7u8].as_ref())), Ok(Header::String));
        assert_eq!(Header::deserialize(&mut BufReader::new([8u8].as_ref())), Ok(Header::Binary));
        assert_eq!(Header::deserialize(&mut BufReader::new([9u8, 0].as_ref())), Ok(Header::Array(Box::new(Header::Boolean))));
        assert_eq!(Header::deserialize(&mut BufReader::new([vec![10u8], vec![1], Header::serialize_map_key("test"), Header::Boolean.serialize()].concat().as_slice())), Ok(Header::Map(indexmap!{String::from("test") => Header::Boolean})));
    }
}
