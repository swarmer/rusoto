//! Event Stream responses
//!
//! TODO

use std::marker::PhantomData;
use std::pin::Pin;

use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;

use crate::request::HttpResponse;
use crate::stream::ByteStream;

// TODO: remove Default bound!

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
pub struct EventStream<T: Default> {
    #[pin]
    response_body: ByteStream,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Default> EventStream<T> {
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

impl<T: Default> futures::stream::Stream for EventStream<T> {
    type Item = Result<T, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO
        let projection = self.project();
        let chunk_option = futures::ready!(Stream::poll_next(projection.response_body, cx));
        match chunk_option {
            Some(chunk_res) => match chunk_res {
                Ok(_byte_chunk) => {
                    Poll::Ready(Some(Ok(Default::default())))
                },
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            None => Poll::Ready(None)
        }
    }
}
