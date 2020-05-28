//! Event Stream responses
//!
//! TODO

use std::marker::PhantomData;
use std::pin::Pin;

use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;
use serde::de::Deserializer;

use crate::error::RusotoError;
use crate::request::HttpResponse;
use crate::stream::ByteStream;

/// TODO
pub trait DeserializeEvent: Sized {
    /// TODO
    fn deserialize_event<'de, D: Deserializer<'de>>(
        event_type: &str,
        deserializer: D,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>;
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
                    let mut deserializer = serde_json::Deserializer::from_slice(&json_bytes);
                    let parsed_event =
                        T::deserialize_event("SubscribeToShardEvent", &mut deserializer);
                    // let parsed_event = proto::json::ResponsePayload::from_body(&Bytes::from(extended_json))
                    //     .deserialize::<T, _>();
                    Poll::Ready(Some(parsed_event.map_err(RusotoError::from)))
                }
                Err(e) => Poll::Ready(Some(Err(RusotoError::from(e)))),
            },
            None => Poll::Ready(None),
        }
    }
}
