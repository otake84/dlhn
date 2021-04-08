use crate::{body::Body, header::Header};

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    header: Header,
    body: Body,
}
