//! Event Stream responses
//!
//! TODO

use std::marker::PhantomData;
use std::pin::Pin;

use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;
use serde::de::DeserializeOwned;

use crate::error::RusotoError;
use crate::proto;
use crate::request::HttpResponse;
use crate::stream::ByteStream;

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
pub struct EventStream<T: DeserializeOwned> {
    #[pin]
    response_body: ByteStream,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> EventStream<T> {
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

impl<T: DeserializeOwned> futures::stream::Stream for EventStream<T> {
    type Item = Result<T, RusotoError<std::io::Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO
        let projection = self.project();
        let chunk_option = futures::ready!(Stream::poll_next(projection.response_body, cx));
        match chunk_option {
            Some(chunk_res) => match chunk_res {
                Ok(byte_chunk) => {
                    // TODO
                    println!("Got bytes: {:?}", byte_chunk);
                    if byte_chunk.windows(16).any(move |sub_slice| sub_slice == b"initial-response") {
                        return Poll::Pending;
                    }

                    let json_start = byte_chunk.iter().position(|&c| c == b'{').unwrap();
                    let json_end = json_start + byte_chunk.slice(json_start..).iter().position(|&c| c == b'}').unwrap();
                    let json_bytes = byte_chunk.slice(json_start..=json_end);
                    println!("Got json bytes: {:?}", json_bytes);

                    let parsed_event = proto::json::ResponsePayload::from_body(&json_bytes)
                        .deserialize::<T, _>();
                    Poll::Ready(Some(parsed_event))
                },
                Err(e) => Poll::Ready(Some(Err(RusotoError::from(e)))),
            },
            None => Poll::Ready(None)
        }
    }
}
