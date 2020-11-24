#[derive(Clone, Debug, PartialEq)]
pub struct Header(Vec<Type>);

impl Header {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn types(&self) -> &[Type] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Bool,
}
