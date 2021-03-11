use crate::{body::Body, header::Header, serializer::validate};
use std::io::Write;

#[derive(Debug)]
pub struct StreamSerializer<T: Write> {
    header: Header,
    writer: T,
    header_state: HeaderState,
}

impl<T: Write> StreamSerializer<T> {
    pub fn new(header: Header, writer: T) -> Self {
        StreamSerializer {
            header,
            writer,
            header_state: HeaderState::NotWritten,
        }
    }

    pub fn serialize_header(&mut self) -> Result<usize, ()> {
        if self.header_state == HeaderState::NotWritten {
            let data = self.header.serialize();
            self.writer
                .write_all(data.as_slice())
                .map(|_| {
                    self.header_state = HeaderState::Written;
                    data.len()
                })
                .or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn serialize_body(&mut self, body: &Body) -> Result<usize, ()> {
        if validate(&self.header, body) {
            let data = body.serialize();
            self.writer
                .write_all(data.as_slice())
                .map(|_| {
                    if self.header_state != HeaderState::Written {
                        self.header_state = HeaderState::DoNotWrite;
                    }
                    data.len()
                })
                .or(Err(()))
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

#[derive(Clone, Debug, PartialEq)]
enum HeaderState {
    NotWritten,
    Written,
    DoNotWrite,
}

#[cfg(test)]
mod tests {
    use super::StreamSerializer;
    use crate::{body::Body, header::Header};

    #[test]
    fn serialize_header() {
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, Vec::new());
        assert_eq!(stream_serializer.serialize_header(), Ok(1));
    }

    #[test]
    fn serialize_body() {
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, Vec::new());
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
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, Vec::new());
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

    #[test]
    fn should_error_double_serialize_header() {
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, Vec::new());
        assert_eq!(stream_serializer.serialize_header(), Ok(1));
        assert_eq!(stream_serializer.serialize_header(), Err(()));
    }

    #[test]
    fn should_error_serialize_header_after_serialize_body() {
        let mut stream_serializer = StreamSerializer::new(Header::Boolean, Vec::new());
        assert_eq!(
            stream_serializer.serialize_body(&Body::Boolean(true)),
            Ok(1)
        );
        assert_eq!(stream_serializer.serialize_header(), Err(()));
    }
}
