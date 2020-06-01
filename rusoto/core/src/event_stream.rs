//! Event Stream responses
//!
//! TODO

use std::fmt::Display;
use std::io::{Cursor, Read};
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

/// TODO
pub trait DeserializeEvent: Sized {
    /// TODO
    fn deserialize_event(event_type: &str, data: &Bytes) -> Result<Self, RusotoError<()>>;
}

fn read_repr<T, R: Read>(read: &mut R, buf: &mut [u8]) -> std::io::Result<()> {
    let type_size = std::mem::size_of::<T>();
    let exact_buf = &mut buf[..type_size];
    read.read_exact(exact_buf)
}

fn read_u8(read: &mut impl Read) -> std::io::Result<u8> {
    let mut buf = [0; std::mem::size_of::<u8>()];
    read_repr::<u8, _>(read, &mut buf)?;
    Ok(u8::from_be_bytes(buf))
}

#[derive(Debug)]
enum EventStreamError {
    UnexpectedEof,
    InvalidCrc,
    InvalidData(&'static str),
    IoError(std::io::Error),
}

impl std::error::Error for EventStreamError {}

impl Display for EventStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventStreamError::UnexpectedEof => write!(f, "Expected additional data"),
            EventStreamError::InvalidCrc => write!(f, "CRC check failed"),
            EventStreamError::InvalidData(msg) => write!(f, "{}", msg),
            EventStreamError::IoError(io_error) => io_error.fmt(f),
        }
    }
}

impl From<std::io::Error> for EventStreamError {
    fn from(err: Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::UnexpectedEof => EventStreamError::UnexpectedEof,
            _ => EventStreamError::IoError(err),
        }
    }
}

impl<T> Into<RusotoError<T>> for EventStreamError {
    fn into(self) -> RusotoError<T> {
        RusotoError::ParseError(self.to_string())
    }
}

/// A struct representing an eventstream header value.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
enum EventStreamHeaderValue<'a> {
    True,
    False,
    Uint8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    ByteArray(&'a [u8]),
    String(&'a str),
    Timestamp(u64),
    Uuid(&'a [u8; 16]), // don't want to pull the uuid dependency for this, so just u8
}

impl<'a> EventStreamHeaderValue<'a> {
    pub fn parse(data: &mut [u8]) -> Result<Self, EventStreamError> {
        let mut cursor = Cursor::new(data);
        let _value_type: u8 = read_u8(&mut cursor)?;

        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug)]
struct EventStreamHeader<'a> {
    // Header name
    name: &'a str,
    /// Header value
    value: EventStreamHeaderValue<'a>,
}

#[derive(Clone, Debug)]
struct EventStreamMessage<'a> {
    headers: Vec<EventStreamHeader<'a>>,
    payload: &'a [u8],
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
                    println!("Got bytes: {:?}", byte_chunk);
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
                    println!("Got json bytes: {:?}", json_bytes);

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
