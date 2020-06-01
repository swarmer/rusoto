//! Event Stream responses
//!
//! TODO

use std::fmt::Display;
use std::io::Read;
use std::marker::PhantomData;
use std::pin::Pin;

use bytes::Bytes;
use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;

use crate::error::RusotoError;
use crate::request::HttpResponse;
use crate::stream::ByteStream;
use serde::export::Formatter;
use futures::io::Error;
use std::convert::TryInto;

/// TODO
pub trait DeserializeEvent: Sized {
    /// TODO
    fn deserialize_event(event_type: &str, data: &Bytes) -> Result<Self, RusotoError<()>>;
}

fn read_repr<T, R: Read>(reader: &mut R, buf: &mut [u8]) -> std::io::Result<()> {
    let type_size = std::mem::size_of::<T>();
    let exact_buf = &mut buf[..type_size];
    reader.read_exact(exact_buf)
}

fn read_u8(reader: &mut impl Read) -> std::io::Result<u8> {
    let mut buf = [0; std::mem::size_of::<u8>()];
    read_repr::<u8, _>(reader, &mut buf)?;
    Ok(u8::from_be_bytes(buf))
}

fn read_u16(reader: &mut impl Read) -> std::io::Result<u16> {
    let mut buf = [0; std::mem::size_of::<u16>()];
    read_repr::<u16, _>(reader, &mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

fn read_u32(reader: &mut impl Read) -> std::io::Result<u32> {
    let mut buf = [0; std::mem::size_of::<u32>()];
    read_repr::<u32, _>(reader, &mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_u64(reader: &mut impl Read) -> std::io::Result<u64> {
    let mut buf = [0; std::mem::size_of::<u64>()];
    read_repr::<u64, _>(reader, &mut buf)?;
    Ok(u64::from_be_bytes(buf))
}

fn read_slice<'a>(reader: &mut &'a [u8], size: usize) -> Result<&'a [u8], EventStreamParseError> {
    if reader.len() < size {
        return Err(EventStreamParseError::UnexpectedEof);
    }

    let slice = &reader[..size];
    *reader = &reader[size..];
    Ok(slice)
}

#[derive(Debug)]
enum EventStreamParseError {
    UnexpectedEof,
    InvalidCrc,
    InvalidData(&'static str),
    IoError(std::io::Error),
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
            EventStreamParseError::IoError(io_error) => io_error.fmt(f),
        }
    }
}

impl From<std::io::Error> for EventStreamParseError {
    fn from(err: Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::UnexpectedEof => EventStreamParseError::UnexpectedEof,
            _ => EventStreamParseError::IoError(err),
        }
    }
}

impl<T> Into<RusotoError<T>> for EventStreamParseError {
    fn into(self) -> RusotoError<T> {
        RusotoError::ParseError(self.to_string())
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
struct EventStreamMessage<'a> {
    headers: Vec<EventStreamHeader<'a>>,
    payload: &'a [u8],
}

impl <'a> EventStreamMessage<'a> {
    pub fn parse(reader: &mut &'a [u8]) -> Result<Self, EventStreamParseError> {
        if reader.len() < 4 {
            return Err(EventStreamParseError::UnexpectedEof);
        }
        let total_length = read_u32(reader)? as usize;
        if total_length < 4 {
            return Err(EventStreamParseError::InvalidData("Invalid event total length value"));
        }
        let mut remainder_reader = read_slice(reader, total_length - 4)?;

        Self::parse_complete_event(&mut remainder_reader)
            // The entire event is available, EOF is no longer possible with well-formed packets
            .map_err(EventStreamParseError::eof_as_invalid)
    }

    fn parse_complete_event(remainder_reader: &mut &'a [u8]) -> Result<Self, EventStreamParseError> {
        let headers_length = read_u32(remainder_reader)? as usize;
        let _prelude_crc = read_u32(remainder_reader)?; // TODO: check

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
        let _payload_crc = read_u32(remainder_reader)?; // TODO: check

        Ok(EventStreamMessage { headers, payload })
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
                    // TODO
                    log::trace!("Got event stream bytes: {:?}", byte_chunk);
                    if byte_chunk
                        .windows(16)
                        .any(move |sub_slice| sub_slice == b"initial-response")
                    {
                        return Poll::Pending;
                    }

                    let json_start = byte_chunk.iter().position(|&c| c == b'{').unwrap();
                    let mut obj_depth = 0;
                    let mut json_end = None;
                    for (i, c) in (&byte_chunk[json_start..]).iter().enumerate() {
                        match c {
                            b'{' => obj_depth += 1,
                            b'}' => obj_depth -= 1,
                            _ => {}
                        }

                        if obj_depth == 0 {
                            json_end = Some(json_start + i);
                            break;
                        }
                    }
                    // let json_end = json_start + byte_chunk.slice(json_start..).iter().position(|&c| c == b'}').unwrap();
                    let json_bytes = byte_chunk.slice(json_start..=json_end.unwrap());

                    // TODO
                    let parsed_event = T::deserialize_event("SubscribeToShardEvent", &json_bytes);
                    Poll::Ready(Some(parsed_event.map_err(RusotoError::from)))
                }
                Err(e) => Poll::Ready(Some(Err(RusotoError::from(e)))),
            },
            None => Poll::Ready(None),
        }
    }
}
