use crate::header::Header;

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
    UInt8(u8),
}

impl Body {
    pub(crate) fn serialize(&self, header: &Header) -> Result<Vec<u8>, ()> {
        match header {
            Header::Boolean => {
                match self {
                    Body::Boolean(v) => {
                        if *v {
                            Ok(vec![1])
                        } else {
                            Ok(vec![0])
                        }
                    }
                    _ => Err(())
                }
            }
            Header::UInt8 => {
                if let Body::UInt8(v) = self {
                    Ok(v.to_le_bytes().to_vec())
                } else {
                    Err(())
                }
            }
        }
    }
}
