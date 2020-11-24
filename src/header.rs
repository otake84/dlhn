#[derive(Clone, Debug, PartialEq)]
pub struct Header(Vec<Type>);

impl Default for Header {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Header {
    pub fn new(types: Vec<Type>) -> Self {
        Self(types)
    }

    pub fn types(&self) -> &[Type] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Bool,
}
