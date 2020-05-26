// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Connection credentials. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionCredentials {
    /// <p>The connection token.</p>
    #[serde(rename = "ConnectionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_token: Option<String>,
    /// <p>The expiration of the token.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "Expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateParticipantConnectionRequest {
    /// <p>Participant Token as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContactResponse.html">StartChatContact</a> API response.</p>
    #[serde(rename = "ParticipantToken")]
    pub participant_token: String,
    /// <p>Type of connection information required.</p>
    #[serde(rename = "Type")]
    pub type_: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateParticipantConnectionResponse {
    /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_credentials: Option<ConnectionCredentials>,
    /// <p>Creates the participant's websocket connection.</p>
    #[serde(rename = "Websocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<Websocket>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisconnectParticipantRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisconnectParticipantResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTranscriptRequest {
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The contactId from the current contact chain for which transcript is needed.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>The maximum number of results to return in the page. Default: 10. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The direction from StartPosition from which to retrieve message. Default: BACKWARD when no StartPosition is provided, FORWARD with StartPosition. </p>
    #[serde(rename = "ScanDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_direction: Option<String>,
    /// <p>The sort order for the records. Default: DESCENDING.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filtering option for where to start.</p>
    #[serde(rename = "StartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<StartPosition>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTranscriptResponse {
    /// <p>The initial contact ID for the contact. </p>
    #[serde(rename = "InitialContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of messages in the session.</p>
    #[serde(rename = "Transcript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Vec<Item>>,
}

/// <p>An item - message or event - that has been sent. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Item {
    /// <p>The time when the message or event was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The content of the message or event.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The type of content of the item.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The chat display name of the sender.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The ID of the item.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the sender in the session.</p>
    #[serde(rename = "ParticipantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// <p>The role of the sender. For example, is it a customer, agent, or system.</p>
    #[serde(rename = "ParticipantRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    /// <p>Type of the item: message or event. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendEventRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the participant's connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The content of the event to be sent (for example, message text). This is not yet supported.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p><p>The content type of the request. Supported types are:</p> <ul> <li> <p>application/vnd.amazonaws.connect.event.typing</p> </li> <li> <p>application/vnd.amazonaws.connect.event.connection.acknowledged</p> </li> </ul></p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendEventResponse {
    /// <p>The time when the event was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the response.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendMessageRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The authentication token associated with the connection.</p>
    #[serde(rename = "ConnectionToken")]
    pub connection_token: String,
    /// <p>The content of the message.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The type of the content. Supported types are text/plain.</p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendMessageResponse {
    /// <p>The time when the message was sent.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the message.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>A filtering option for where to start. For example, if you sent 100 messages, start with message 50. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartPosition {
    /// <p>The time in ISO format where to start.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "AbsoluteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    /// <p>The ID of the message or event where to start. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The start position of the most recent message where you want to start. </p>
    #[serde(rename = "MostRecent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent: Option<i64>,
}

/// <p>The websocket for the participant's connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Websocket {
    /// <p>The URL expiration timestamp in ISO date format.</p> <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[serde(rename = "ConnectionExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_expiry: Option<String>,
    /// <p>The URL of the websocket.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Errors returned by CreateParticipantConnection
#[derive(Debug, PartialEq)]
pub enum CreateParticipantConnectionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateParticipantConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateParticipantConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateParticipantConnectionError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateParticipantConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateParticipantConnectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateParticipantConnectionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateParticipantConnectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateParticipantConnectionError {}
/// Errors returned by DisconnectParticipant
#[derive(Debug, PartialEq)]
pub enum DisconnectParticipantError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisconnectParticipantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisconnectParticipantError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisconnectParticipantError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisconnectParticipantError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisconnectParticipantError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisconnectParticipantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisconnectParticipantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisconnectParticipantError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisconnectParticipantError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisconnectParticipantError {}
/// Errors returned by GetTranscript
#[derive(Debug, PartialEq)]
pub enum GetTranscriptError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetTranscriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTranscriptError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetTranscriptError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetTranscriptError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetTranscriptError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTranscriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTranscriptError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetTranscriptError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetTranscriptError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTranscriptError {}
/// Errors returned by SendEvent
#[derive(Debug, PartialEq)]
pub enum SendEventError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SendEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendEventError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SendEventError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(SendEventError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendEventError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendEventError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendEventError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SendEventError::InternalServer(ref cause) => write!(f, "{}", cause),
            SendEventError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendEventError {}
/// Errors returned by SendMessage
#[derive(Debug, PartialEq)]
pub enum SendMessageError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SendMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SendMessageError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(SendMessageError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendMessageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendMessageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendMessageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SendMessageError::InternalServer(ref cause) => write!(f, "{}", cause),
            SendMessageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendMessageError {}
/// Trait representing the capabilities of the Amazon Connect Participant API. Amazon Connect Participant clients implement this trait.
#[async_trait]
pub trait ConnectParticipant {
    /// <p>Creates the participant's connection. Note that ParticipantToken is used for invoking this API instead of ConnectionToken.</p> <p>The participant token is valid for the lifetime of the participant – until the they are part of a contact.</p> <p>The response URL for <code>WEBSOCKET</code> Type has a connect expiry timeout of 100s. Clients must manually connect to the returned websocket URL and subscribe to the desired topic. </p> <p>For chat, you need to publish the following on the established websocket connection:</p> <p> <code>{"topic":"aws/subscribe","content":{"topics":["aws/chat"]}}</code> </p> <p>Upon websocket URL expiry, as specified in the response ConnectionExpiry parameter, clients need to call this API again to obtain a new websocket URL and perform the same steps as before.</p>
    async fn create_participant_connection(
        &self,
        input: CreateParticipantConnectionRequest,
    ) -> Result<CreateParticipantConnectionResponse, RusotoError<CreateParticipantConnectionError>>;

    /// <p>Disconnects a participant. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn disconnect_participant(
        &self,
        input: DisconnectParticipantRequest,
    ) -> Result<DisconnectParticipantResponse, RusotoError<DisconnectParticipantError>>;

    /// <p>Retrieves a transcript of the session. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn get_transcript(
        &self,
        input: GetTranscriptRequest,
    ) -> Result<GetTranscriptResponse, RusotoError<GetTranscriptError>>;

    /// <p>Sends an event. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn send_event(
        &self,
        input: SendEventRequest,
    ) -> Result<SendEventResponse, RusotoError<SendEventError>>;

    /// <p>Sends a message. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> Result<SendMessageResponse, RusotoError<SendMessageError>>;
}
/// A client for the Amazon Connect Participant API.
#[derive(Clone)]
pub struct ConnectParticipantClient {
    client: Client,
    region: region::Region,
}

impl ConnectParticipantClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConnectParticipantClient {
        ConnectParticipantClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectParticipantClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ConnectParticipantClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ConnectParticipantClient {
        ConnectParticipantClient { client, region }
    }
}

#[async_trait]
impl ConnectParticipant for ConnectParticipantClient {
    /// <p>Creates the participant's connection. Note that ParticipantToken is used for invoking this API instead of ConnectionToken.</p> <p>The participant token is valid for the lifetime of the participant – until the they are part of a contact.</p> <p>The response URL for <code>WEBSOCKET</code> Type has a connect expiry timeout of 100s. Clients must manually connect to the returned websocket URL and subscribe to the desired topic. </p> <p>For chat, you need to publish the following on the established websocket connection:</p> <p> <code>{"topic":"aws/subscribe","content":{"topics":["aws/chat"]}}</code> </p> <p>Upon websocket URL expiry, as specified in the response ConnectionExpiry parameter, clients need to call this API again to obtain a new websocket URL and perform the same steps as before.</p>
    async fn create_participant_connection(
        &self,
        input: CreateParticipantConnectionRequest,
    ) -> Result<CreateParticipantConnectionResponse, RusotoError<CreateParticipantConnectionError>>
    {
        let request_uri = "/participant/connection";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.participant_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateParticipantConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateParticipantConnectionError::from_response(response))
        }
    }

    /// <p>Disconnects a participant. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn disconnect_participant(
        &self,
        input: DisconnectParticipantRequest,
    ) -> Result<DisconnectParticipantResponse, RusotoError<DisconnectParticipantError>> {
        let request_uri = "/participant/disconnect";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisconnectParticipantResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisconnectParticipantError::from_response(response))
        }
    }

    /// <p>Retrieves a transcript of the session. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn get_transcript(
        &self,
        input: GetTranscriptRequest,
    ) -> Result<GetTranscriptResponse, RusotoError<GetTranscriptError>> {
        let request_uri = "/participant/transcript";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTranscriptResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTranscriptError::from_response(response))
        }
    }

    /// <p>Sends an event. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn send_event(
        &self,
        input: SendEventRequest,
    ) -> Result<SendEventResponse, RusotoError<SendEventError>> {
        let request_uri = "/participant/event";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendEventResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendEventError::from_response(response))
        }
    }

    /// <p>Sends a message. Note that ConnectionToken is used for invoking this API instead of ParticipantToken.</p>
    async fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> Result<SendMessageResponse, RusotoError<SendMessageError>> {
        let request_uri = "/participant/message";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("participant.connect".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("X-Amz-Bearer", &input.connection_token);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendMessageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendMessageError::from_response(response))
        }
    }
}
