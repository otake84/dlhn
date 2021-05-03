use std::io::Read;

pub struct Deserializer<'de, R: Read> {
    reader: &'de R,
}

impl<'de, R: Read> Deserializer<'de, R> {
    pub fn new(reader: &'de R) -> Self {
        Deserializer {
            reader,
        }
    }
}
