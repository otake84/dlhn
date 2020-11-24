#[derive(Clone, Debug, PartialEq)]
pub struct Header(Vec<Type>);

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Bool,
}
