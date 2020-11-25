#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Boolean,
}

impl Header {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        match self {
            Header::Boolean => {
                vec![0]
            }
        }
    }
}
