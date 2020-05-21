use bytes::Bytes;
use log::*;
use serde::de::DeserializeOwned;
use serde_json::from_slice;

use super::super::super::request::BufferedHttpResponse;
use super::super::super::RusotoError;

static EMPTY_DICT: Bytes = Bytes::from_static(b"{}");

pub struct ResponsePayload<'a> {
    body: &'a Bytes,
}

impl<'a> ResponsePayload<'a> {
    pub fn new(res: &'a BufferedHttpResponse) -> Self {
        debug!("Response status: {}", res.status);
        Self::from_body(&res.body)
    }

    pub fn from_body(body: &'a Bytes) -> Self {
        let mut processed_body = body;

        // `serde-json` serializes field-less structs as "null", but AWS returns
        // "{}" for a field-less response, so we must check for this result
        // and convert it if necessary.
        if processed_body.is_empty() || processed_body.as_ref() == b"null" {
            processed_body = &EMPTY_DICT;
        }

        debug!("Response body: {:?}", processed_body);

        Self { body: processed_body }
    }

    pub fn deserialize<T: DeserializeOwned, E>(&self) -> Result<T, RusotoError<E>> {
        Ok(from_slice(self.body)?)
    }
}
