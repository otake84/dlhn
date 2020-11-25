use crate::{body::Body, header::Header};

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    let mut serialized_header= header.serialize();
    match body.serialize(header) {
        Ok(mut value) => {
            serialized_header.append(&mut value);
            Ok(serialized_header)
        }
        Err(_) => Err(())
    }
}
