use crate::{body::Body, header::Header, serializer::validate};
use std::io::Write;

#[derive(Debug)]
pub struct StreamSerializer<T: Write> {
    header: Header,
    writer: T,
}

impl<T: Write> StreamSerializer<T> {
    pub fn new(header: Header, writer: T) -> Self {
        StreamSerializer { header, writer }
    }

    pub fn serialize_header(&mut self) -> Result<usize, ()> {
        let data = self.header.serialize();
        self.writer
            .write_all(data.as_slice())
            .and(Ok(data.len()))
            .or(Err(()))
    }

    pub fn serialize_body(&mut self, body: &Body) -> Result<usize, ()> {
        if validate(&self.header, body) {
            let data = body.serialize();
            self.writer.write_all(data.as_slice()).or(Err(()))?;
            Ok(data.len())
        } else {
            Err(())
        }
    }

    pub fn writer(&mut self) -> &mut T {
        &mut self.writer
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        self.writer.flush().or(Err(()))
    }
}

#[cfg(test)]
mod tests {
    use super::StreamSerializer;
    use crate::{body::Body, header::Header};

    #[test]
    fn serialize_header() {
        let writer: Vec<u8> = Vec::new();
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, writer);
        assert_eq!(stream_serializer.serialize_header(), Ok(1));
    }

    #[test]
    fn serialize_body() {
        let writer: Vec<u8> = Vec::new();
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, writer);
        assert_eq!(
            stream_serializer.serialize_body(&Body::Boolean(true)),
            Ok(1)
        );
        assert_eq!(
            stream_serializer.serialize_body(&Body::Boolean(false)),
            Ok(1)
        );
        assert_eq!(stream_serializer.flush(), Ok(()));
        assert_eq!(stream_serializer.writer().len(), 2);
        assert_eq!(stream_serializer.writer(), &[1, 0]);
    }

    #[test]
    fn serialize_header_and_then_serialize_body() {
        let writer = Vec::new();
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, writer);
        assert_eq!(stream_serializer.serialize_header(), Ok(1));
        assert_eq!(
            stream_serializer.serialize_body(&Body::Boolean(true)),
            Ok(1)
        );
        assert_eq!(
            stream_serializer.serialize_body(&Body::Boolean(false)),
            Ok(1)
        );
        assert_eq!(stream_serializer.flush(), Ok(()));
        assert_eq!(stream_serializer.writer().len(), 3);
        assert_eq!(stream_serializer.writer(), &[1, 1, 0]);
    }
}
