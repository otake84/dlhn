use crate::{body::Body, header::Header, serializer::validate};
use std::io::{BufWriter, Write};

pub struct StreamSerializer<T: Write> {
    header: Header,
    buf_writer: BufWriter<T>,
}

impl<T: Write> StreamSerializer<T> {
    pub fn new(header: Header, write: T) -> Self {
        StreamSerializer {
            header,
            buf_writer: BufWriter::new(write),
        }
    }

    pub fn serialize(&mut self, body: &Body) -> Result<usize, ()> {
        if validate(&self.header, body) {
            let data = body.serialize();
            self.buf_writer.write_all(data.as_slice()).or(Err(()))?;
            Ok(data.len())
        } else {
            Err(())
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.buf_writer.get_ref()
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        self.buf_writer.flush().or(Err(()))
    }
}

#[cfg(test)]
mod tests {
    use super::StreamSerializer;
    use crate::{body::Body, header::Header};

    #[test]
    fn serialize() {
        let writer: Vec<u8> = Vec::new();
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, writer);
        assert_eq!(stream_serializer.serialize(&Body::Boolean(true)), Ok(1));
        assert_eq!(stream_serializer.serialize(&Body::Boolean(false)), Ok(1));
        assert_eq!(stream_serializer.flush(), Ok(()));
        assert_eq!(stream_serializer.get_ref().len(), 2);
        assert_eq!(stream_serializer.get_ref(), &[1, 0]);
    }
}
