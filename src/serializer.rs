use crate::{body::Body, header::Header};

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    match header {
        Header::Boolean => {
            match body {
                Body::Boolean(_) => {
                    Ok(vec![1])
                }
            }
        }
    }
}
