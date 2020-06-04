//! Event Stream responses
//!
//! TODO

use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::pin::Pin;

use crc32fast::Hasher;
use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;

use crate::error::RusotoError;
use crate::request::HttpResponse;
use crate::stream::ByteStream;

/// TODO
pub trait DeserializeEvent: Sized {
    /// TODO
    fn deserialize_event(event_type: &str, data: &[u8]) -> Result<Self, RusotoError<()>>;
}

#[derive(Debug, Eq, PartialEq)]
enum EventStreamParseError {
    UnexpectedEof,
    InvalidCrc,
    InvalidData(&'static str),
}

fn check_crc32(data: &[u8], ref_value: u32) -> Result<(), EventStreamParseError> {
    let mut hasher = Hasher::new();
    hasher.update(data);

    if hasher.finalize() != ref_value {
        Err(EventStreamParseError::InvalidCrc)
    } else {
        Ok(())
    }
}

fn read_slice<'a>(reader: &mut &'a [u8], size: usize) -> Result<&'a [u8], EventStreamParseError> {
    if reader.len() < size {
        return Err(EventStreamParseError::UnexpectedEof);
    }

    let slice = &reader[..size];
    *reader = &reader[size..];
    Ok(slice)
}

fn read_u8(reader: &mut &[u8]) -> Result<u8, EventStreamParseError> {
    let buf = read_slice(reader, std::mem::size_of::<u8>())?.try_into().unwrap();
    Ok(u8::from_be_bytes(buf))
}

fn read_u16(reader: &mut &[u8]) -> Result<u16, EventStreamParseError> {
    let buf = read_slice(reader, std::mem::size_of::<u16>())?.try_into().unwrap();
    Ok(u16::from_be_bytes(buf))
}

fn read_u32(reader: &mut &[u8]) -> Result<u32, EventStreamParseError> {
    let buf = read_slice(reader, std::mem::size_of::<u32>())?.try_into().unwrap();
    Ok(u32::from_be_bytes(buf))
}

fn read_u64(reader: &mut &[u8]) -> Result<u64, EventStreamParseError> {
    let buf = read_slice(reader, std::mem::size_of::<u64>())?.try_into().unwrap();
    Ok(u64::from_be_bytes(buf))
}

impl EventStreamParseError {
    fn eof_as_invalid(self) -> Self {
        match self {
            EventStreamParseError::UnexpectedEof => EventStreamParseError::InvalidData(
                "Malformed event: ended unexpectedly"
            ),
            other => other,
        }
    }
}

impl std::error::Error for EventStreamParseError {}

impl Display for EventStreamParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventStreamParseError::UnexpectedEof => write!(f, "Expected additional data"),
            EventStreamParseError::InvalidCrc => write!(f, "CRC check failed"),
            EventStreamParseError::InvalidData(msg) => write!(f, "{}", msg),
        }
    }
}

impl<T> Into<RusotoError<T>> for EventStreamParseError {
    fn into(self) -> RusotoError<T> {
        RusotoError::ParseError(self.to_string())
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EventStreamHeaderValue<'a> {
    Bool(bool),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    ByteArray(&'a [u8]),
    String(&'a str),
    Timestamp(u64),
    Uuid(&'a [u8; 16]), // don't want to pull the uuid dependency for this, so just u8
}

impl<'a> EventStreamHeaderValue<'a> {
    pub fn parse(reader: &mut &'a [u8]) -> Result<Self, EventStreamParseError> {
        let value_type = read_u8(reader)?;
        let value = match value_type {
            0 => EventStreamHeaderValue::Bool(true),
            1 => EventStreamHeaderValue::Bool(false),
            2 => EventStreamHeaderValue::UInt8(read_u8(reader)?),
            3 => EventStreamHeaderValue::UInt16(read_u16(reader)?),
            4 => EventStreamHeaderValue::UInt32(read_u32(reader)?),
            5 => EventStreamHeaderValue::UInt64(read_u64(reader)?),
            6 => {
                let size = read_u16(reader)? as usize;
                let byte_array = read_slice(reader, size)?;
                EventStreamHeaderValue::ByteArray(byte_array)
            },
            7 => {
                let size = read_u16(reader)? as usize;
                let string_bytes = read_slice(reader, size)?;
                let string = std::str::from_utf8(string_bytes)
                    .or(Err(EventStreamParseError::InvalidData("Header string data is not valid utf-8")))?;
                EventStreamHeaderValue::String(string)
            },
            8 => EventStreamHeaderValue::Timestamp(read_u64(reader)?),
            9 => EventStreamHeaderValue::Uuid(
                read_slice(reader, 16)?.try_into().unwrap()
            ),
            _ => Err(EventStreamParseError::InvalidData("Invalid header value type"))?,
        };
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EventStreamHeader<'a> {
    name: &'a str,
    value: EventStreamHeaderValue<'a>,
}

impl <'a> EventStreamHeader<'a> {
    pub fn parse(reader: &mut &'a [u8]) -> Result<Self, EventStreamParseError> {
        let name_size = read_u8(reader)? as usize;
        let name_bytes = read_slice(reader, name_size)?;
        let name = std::str::from_utf8(name_bytes)
            .or(Err(EventStreamParseError::InvalidData("Header name is not valid utf-8")))?;

        let value = EventStreamHeaderValue::parse(reader)?;

        Ok(EventStreamHeader { name, value })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EventStreamMessage<'a> {
    headers: Vec<EventStreamHeader<'a>>,
    payload: &'a [u8],
}

impl <'a> EventStreamMessage<'a> {
    const HEADER_LENGTH: usize = 12;

    pub fn parse(reader: &mut &'a [u8]) -> Result<Self, EventStreamParseError> {
        let mut event_buf: &[u8] = *reader;

        if reader.len() < Self::HEADER_LENGTH {
            return Err(EventStreamParseError::UnexpectedEof);
        }
        let total_length = read_u32(reader)? as usize;
        if total_length < Self::HEADER_LENGTH {
            return Err(EventStreamParseError::InvalidData("Invalid event total length value"));
        }
        event_buf = &event_buf[..total_length];
        let mut remainder_reader = read_slice(reader, total_length - 4)?;

        Self::parse_complete_event(event_buf, &mut remainder_reader)
            // The entire event is available, EOF is no longer possible with well-formed packets
            .map_err(EventStreamParseError::eof_as_invalid)
    }

    fn parse_complete_event(
        event_buf: &'a [u8],
        remainder_reader: &mut &'a [u8],
    ) -> Result<Self, EventStreamParseError> {
        let headers_length = read_u32(remainder_reader)? as usize;
        let prelude_crc = read_u32(remainder_reader)?;
        check_crc32(&event_buf[..8], prelude_crc)?;

        let mut headers_reader = read_slice(remainder_reader, headers_length)?;
        let mut headers = Vec::with_capacity(3);
        while !headers_reader.is_empty() {
            let header = EventStreamHeader::parse(&mut headers_reader)?;
            headers.push(header);
        }

        if remainder_reader.len() < 4 {
            return Err(EventStreamParseError::InvalidData("Malformed event: unexpected EOF"));
        }
        let payload = read_slice(remainder_reader, remainder_reader.len() - 4)?;
        let payload_crc = read_u32(remainder_reader)?;
        check_crc32(&event_buf[..(event_buf.len() - 4)], payload_crc)?;

        Ok(EventStreamMessage { headers, payload })
    }

    pub fn get_header(&self, name: &str) -> Option<&EventStreamHeader<'a>> {
        self.headers.iter().find(|h| h.name == name)
    }
}

/// Event Stream.
///
/// # Default
///
/// TODO
///
/// # Example
///
/// TODO
#[pin_project]
#[derive(Debug)]
pub struct EventStream<T: DeserializeEvent> {
    #[pin]
    response_body: ByteStream,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeEvent> EventStream<T> {
    /// Create an Event Stream.
    ///
    /// # Default
    ///
    /// TODO
    ///
    /// # Example
    ///
    /// TODO
    pub fn new(response: HttpResponse) -> EventStream<T> {
        EventStream {
            response_body: response.body,
            _phantom: PhantomData {},
        }
    }
}

impl<T: DeserializeEvent> futures::stream::Stream for EventStream<T> {
    type Item = Result<T, RusotoError<()>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO
        let projection = self.project();
        let chunk_option = futures::ready!(Stream::poll_next(projection.response_body, cx));
        match chunk_option {
            Some(chunk_res) => match chunk_res {
                Ok(byte_chunk) => {
                    log::trace!("Got event stream bytes: {:?}", byte_chunk);

                    // TODO
                    let event_msg = EventStreamMessage::parse(&mut &*byte_chunk).unwrap();
                    println!("Parsed event stream event: {:?}", event_msg);

                    let event_type_header = event_msg.get_header(":event-type").unwrap();
                    let event_type = match event_type_header.value {
                        EventStreamHeaderValue::String(s) => s,
                        _ => panic!("Invalid event type value"),
                    };
                    if event_type == "initial-response" {
                        return Poll::Pending;
                    }

                    let parsed_event = T::deserialize_event(event_type, event_msg.payload);
                    Poll::Ready(Some(parsed_event.map_err(RusotoError::from)))
                }
                Err(e) => Poll::Ready(Some(Err(RusotoError::from(e)))),
            },
            None => Poll::Ready(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_response() {
        let data = b"\0\0\0r\0\0\0`\xab\x82\r\x9e\x0b:event-type\x07\0\x10initial-response\r\
            :content-type\x07\0\x1aapplication/x-amz-json-1.1\
            \r:message-type\x07\0\x05event{}\xac\xaek}";

        let event_msg = EventStreamMessage::parse(&mut &data[..]);
        assert_eq!(
            event_msg,
            Ok(EventStreamMessage {
                headers: vec![
                    EventStreamHeader {
                        name: ":event-type",
                        value: EventStreamHeaderValue::String("initial-response"),
                    },
                    EventStreamHeader {
                        name: ":content-type",
                        value: EventStreamHeaderValue::String("application/x-amz-json-1.1"),
                    },
                    EventStreamHeader {
                        name: ":message-type",
                        value: EventStreamHeaderValue::String("event"),
                    },
                ],
                payload: b"{}",
            }),
        );
    }

    #[test]
    fn parse_error_event() {
        let data = b"\0\0\x01\x06\0\0\0pq;\x88P\x0f:exception-type\x07\0\x18\
            KMSAccessDeniedException\r:content-type\x07\0\x1aapplication/x-amz-json-1.1\r\
            :message-type\x07\0\texception{\"message\":\"User AIDAAAAAAAAAAAAAAAAAA is not \
            authorized to decrypt records in stream 666666666666:rusoto-test-tud2Vz6q1V\
            :1590674508\"}\xfc\xd1\x99T";

        let event_msg = EventStreamMessage::parse(&mut &data[..]);
        assert_eq!(
            event_msg,
            Ok(EventStreamMessage {
                headers: vec![
                    EventStreamHeader {
                        name: ":exception-type",
                        value: EventStreamHeaderValue::String("KMSAccessDeniedException"),
                    },
                    EventStreamHeader {
                        name: ":content-type",
                        value: EventStreamHeaderValue::String("application/x-amz-json-1.1"),
                    },
                    EventStreamHeader {
                        name: ":message-type",
                        value: EventStreamHeaderValue::String("exception"),
                    },
                ],
                payload: b"{\"message\":\"User AIDAAAAAAAAAAAAAAAAAA is not \
                    authorized to decrypt records in stream 666666666666:rusoto-test-tud2Vz6q1V\
                    :1590674508\"}",
            }),
        );
    }

    #[test]
    fn invalid_prelude_crc() {
        let data = b"\0\0\0r\0\0\0`\xab\x82\r\x9f\x0b:event-type\x07\0\x10initial-response\r\
            :content-type\x07\0\x1aapplication/x-amz-json-1.1\
            \r:message-type\x07\0\x05event{}\xac\xaek}";

        let event_msg = EventStreamMessage::parse(&mut &data[..]);
        assert_eq!(
            event_msg,
            Err(EventStreamParseError::InvalidCrc),
        );
    }

    #[test]
    fn invalid_message_crc() {
        let data = b"\0\0\0r\0\0\0`\xab\x82\r\x9e\x0b:event-type\x07\0\x10initial-response\r\
            :content-type\x07\0\x1aapplication/x-amz-json-1.1\
            \r:message-type\x07\0\x05event{}\xad\xaek}";

        let event_msg = EventStreamMessage::parse(&mut &data[..]);
        assert_eq!(
            event_msg,
            Err(EventStreamParseError::InvalidCrc),
        );
    }
}
