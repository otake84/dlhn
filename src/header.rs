#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
    UInt8,
}

impl Header {
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
}
