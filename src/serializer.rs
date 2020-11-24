use crate::{body::Body, header::Header};

pub fn serialize(header: &Header, body: &Body) -> Result<Vec<u8>, ()> {
    match header {
        Header::Bool => {
            match body {
                Body::Bool(_) => {
                    Ok(vec![1])
                }
            }
        }
    }
}
