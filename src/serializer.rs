use crate::{body::Body, header::Header};

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    body.serialize(header)
}
