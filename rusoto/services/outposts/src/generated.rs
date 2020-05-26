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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOutpostInput {
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SiteId")]
    pub site_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOutpostInput {
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOutpostInstanceTypesInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOutpostInstanceTypesOutput {
    #[serde(rename = "InstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<InstanceTypeItem>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

/// <p>Information about an instance type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceTypeItem {
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOutpostsInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOutpostsOutput {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Outposts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outposts: Option<Vec<Outpost>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSitesInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSitesOutput {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
}

/// <p>Information about an Outpost.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Outpost {
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LifeCycleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_status: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

/// <p>Information about a site.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Site {
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

/// Errors returned by CreateOutpost
#[derive(Debug, PartialEq)]
pub enum CreateOutpostError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
    /// <p>You have exceeded a service quota.</p>
    ServiceQuotaExceeded(String),
}

impl CreateOutpostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOutpostError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOutpostError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateOutpostError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateOutpostError::NotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateOutpostError::ServiceQuotaExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateOutpostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOutpostError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOutpostError {}
/// Errors returned by GetOutpost
#[derive(Debug, PartialEq)]
pub enum GetOutpostError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl GetOutpostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOutpostError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOutpostError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetOutpostError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOutpostError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOutpostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOutpostError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOutpostError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutpostError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutpostError {}
/// Errors returned by GetOutpostInstanceTypes
#[derive(Debug, PartialEq)]
pub enum GetOutpostInstanceTypesError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl GetOutpostInstanceTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOutpostInstanceTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::InternalServer(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOutpostInstanceTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOutpostInstanceTypesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOutpostInstanceTypesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutpostInstanceTypesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutpostInstanceTypesError {}
/// Errors returned by ListOutposts
#[derive(Debug, PartialEq)]
pub enum ListOutpostsError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
}

impl ListOutpostsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOutpostsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListOutpostsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListOutpostsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOutpostsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOutpostsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListOutpostsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOutpostsError {}
/// Errors returned by ListSites
#[derive(Debug, PartialEq)]
pub enum ListSitesError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
}

impl ListSitesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSitesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSitesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListSitesError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSitesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSitesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSitesError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSitesError {}
/// Trait representing the capabilities of the Outposts API. Outposts clients implement this trait.
#[async_trait]
pub trait Outposts {
    /// <p>Creates an Outpost.</p>
    async fn create_outpost(
        &self,
        input: CreateOutpostInput,
    ) -> Result<CreateOutpostOutput, RusotoError<CreateOutpostError>>;

    /// <p>Gets information about the specified Outpost.</p>
    async fn get_outpost(
        &self,
        input: GetOutpostInput,
    ) -> Result<GetOutpostOutput, RusotoError<GetOutpostError>>;

    /// <p>Lists the instance types for the specified Outpost.</p>
    async fn get_outpost_instance_types(
        &self,
        input: GetOutpostInstanceTypesInput,
    ) -> Result<GetOutpostInstanceTypesOutput, RusotoError<GetOutpostInstanceTypesError>>;

    /// <p>List the Outposts for your AWS account.</p>
    async fn list_outposts(
        &self,
        input: ListOutpostsInput,
    ) -> Result<ListOutpostsOutput, RusotoError<ListOutpostsError>>;

    /// <p>Lists the sites for the specified AWS account.</p>
    async fn list_sites(
        &self,
        input: ListSitesInput,
    ) -> Result<ListSitesOutput, RusotoError<ListSitesError>>;
}
/// A client for the Outposts API.
#[derive(Clone)]
pub struct OutpostsClient {
    client: Client,
    region: region::Region,
}

impl OutpostsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OutpostsClient {
        OutpostsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OutpostsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        OutpostsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> OutpostsClient {
        OutpostsClient { client, region }
    }
}

#[async_trait]
impl Outposts for OutpostsClient {
    /// <p>Creates an Outpost.</p>
    async fn create_outpost(
        &self,
        input: CreateOutpostInput,
    ) -> Result<CreateOutpostOutput, RusotoError<CreateOutpostError>> {
        let request_uri = "/outposts";

        let mut request = SignedRequest::new("POST", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateOutpostOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateOutpostError::from_response(response))
        }
    }

    /// <p>Gets information about the specified Outpost.</p>
    async fn get_outpost(
        &self,
        input: GetOutpostInput,
    ) -> Result<GetOutpostOutput, RusotoError<GetOutpostError>> {
        let request_uri = format!("/outposts/{outpost_id}", outpost_id = input.outpost_id);

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetOutpostOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetOutpostError::from_response(response))
        }
    }

    /// <p>Lists the instance types for the specified Outpost.</p>
    async fn get_outpost_instance_types(
        &self,
        input: GetOutpostInstanceTypesInput,
    ) -> Result<GetOutpostInstanceTypesOutput, RusotoError<GetOutpostInstanceTypesError>> {
        let request_uri = format!(
            "/outposts/{outpost_id}/instanceTypes",
            outpost_id = input.outpost_id
        );

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetOutpostInstanceTypesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetOutpostInstanceTypesError::from_response(response))
        }
    }

    /// <p>List the Outposts for your AWS account.</p>
    async fn list_outposts(
        &self,
        input: ListOutpostsInput,
    ) -> Result<ListOutpostsOutput, RusotoError<ListOutpostsError>> {
        let request_uri = "/outposts";

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOutpostsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOutpostsError::from_response(response))
        }
    }

    /// <p>Lists the sites for the specified AWS account.</p>
    async fn list_sites(
        &self,
        input: ListSitesInput,
    ) -> Result<ListSitesOutput, RusotoError<ListSitesError>> {
        let request_uri = "/sites";

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListSitesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSitesError::from_response(response))
        }
    }
}
