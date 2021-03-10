use crate::{body::Body, header::Header};
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct StreamDeserializer<T> {
    header: Header,
    buf_reader: BufReader<T>,
}

impl<T> StreamDeserializer<T> {
    pub fn header(&self) -> &Header {
        &self.header
    }
}

impl<T: Read> StreamDeserializer<T> {
    pub fn new(reader: T) -> Result<StreamDeserializer<T>, ()> {
        let mut buf_reader = BufReader::new(reader);
        Ok(StreamDeserializer {
            header: Header::deserialize(&mut buf_reader)?,
            buf_reader,
        })
    }

    pub fn deserialize(&mut self) -> Result<Body, ()> {
        Body::deserialize(&self.header, &mut self.buf_reader)
    }
}

impl<T: Seek> StreamDeserializer<T> {
    pub fn position(&mut self) -> Result<u64, ()> {
        self.buf_reader.seek(SeekFrom::Current(0)).or(Err(()))
    }
}

#[cfg(test)]
mod tests {
    use super::StreamDeserializer;
    use crate::{body::Body, header::Header, stream::stream_serializer::StreamSerializer};
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[test]
    fn deserialize() {
        let mut stream_serializer = new_stream_serializer(Vec::new());

        let mut stream_deserializer =
            StreamDeserializer::new(stream_serializer.writer().as_slice()).unwrap();
        assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(true)));
        assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(false)));
        assert_eq!(stream_deserializer.deserialize(), Err(()));
    }

    #[test]
    fn position() {
        let mut stream_serializer = new_stream_serializer(Cursor::new(Vec::new()));
        let cursor = stream_serializer.writer();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut stream_deserializer = StreamDeserializer::new(cursor).unwrap();

        assert_eq!(stream_deserializer.position(), Ok(1));
        assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(true)));
        assert_eq!(stream_deserializer.position(), Ok(2));
        assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(false)));
        assert_eq!(stream_deserializer.position(), Ok(3));
        assert_eq!(stream_deserializer.deserialize(), Err(()));
        assert_eq!(stream_deserializer.position(), Ok(3));
    }

    fn new_stream_serializer<T: Write>(writer: T) -> StreamSerializer<T> {
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
        stream_serializer
    }
}
