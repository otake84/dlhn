use crate::header::Header;

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
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
                }
            }
        }
    }
}
