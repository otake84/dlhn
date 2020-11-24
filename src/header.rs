#[derive(Clone, Debug, PartialEq)]
pub struct Header(Vec<Type>);

impl Header {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Bool,
}
