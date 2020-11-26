#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Boolean(bool),
    UInt8(u8),
}

impl Body {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Body::Boolean(v) => {
                if *v {
                    vec![1]
                } else {
                    vec![0]
                }
            }
            Body::UInt8(v) => {
                v.to_le_bytes().to_vec()
            }
        }
    }
}
