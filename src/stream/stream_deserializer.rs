use crate::{body::Body, header::Header};
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct StreamDeserializer<T> {
    header: Header,
    buf_reader: BufReader<T>,
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

#[cfg(test)]
mod tests {
    use super::StreamDeserializer;
    use crate::{body::Body, header::Header, stream::stream_serializer::StreamSerializer};
    use core::panic;

    #[test]
    fn deserialize() {
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

        if let Ok(mut stream_deserializer) =
            StreamDeserializer::new(stream_serializer.get_ref().as_slice())
        {
            assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(true)));
            assert_eq!(stream_deserializer.deserialize(), Ok(Body::Boolean(false)));
            assert_eq!(stream_deserializer.deserialize(), Err(()));
        } else {
            panic!();
        }
    }
}
