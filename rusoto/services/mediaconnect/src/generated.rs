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
/// <p>A request to add outputs to the specified flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddFlowOutputsRequest {
    /// <p>The flow that you want to add outputs to.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>A list of outputs that you want to add.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<AddOutputRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFlowOutputsResponse {
    /// <p>The ARN of the flow that these outputs were added to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The details of the newly added outputs.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

/// <p>The output that you want to add to this flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddOutputRequest {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>A description of the output. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address from which video will be sent to output destinations.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The maximum latency in milliseconds for Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The name of the output. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol to use for the output.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>The remote ID for the Zixi-pull output stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 20) and entitlements (up to 50).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFlowRequest {
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The entitlements that you want to grant on a flow.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<GrantEntitlementRequest>>,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The outputs that you want to add to this flow.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AddOutputRequest>>,
    #[serde(rename = "Source")]
    pub source: SetSourceRequest,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFlowResponse {
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFlowRequest {
    /// <p>The ARN of the flow that you want to delete.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFlowResponse {
    /// <p>The ARN of the flow that was deleted.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the DeleteFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFlowRequest {
    /// <p>The ARN of the flow that you want to describe.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFlowResponse {
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
    #[serde(rename = "Messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Messages>,
}

/// <p>Information about the encryption of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Encryption {
    /// <p>The type of algorithm that is used for the encryption (such as aes128, aes192, or aes256).</p>
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// <p>A 128-bit, 16-byte hex value represented by a 32-character string, to be used with the key for encrypting content. This parameter is not valid for static key encryption.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The value of one of the devices that you configured with your digital rights management (DRM) platform key provider. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The type of key that is used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// <p>The AWS Region that the API Gateway proxy endpoint was created in. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>An identifier for the content. The service sends this value to the key server to identify the current endpoint. The resource ID is also known as the content ID. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The ARN of the role that you created during setup (when you set up AWS Elemental MediaConnect as a trusted entity).</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The ARN of the secret that you created in AWS Secrets Manager to store the encryption key. This parameter is required for static key encryption and is not valid for SPEKE encryption.</p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The URL from the API Gateway proxy that you set up to talk to your key server. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The settings for a flow entitlement.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Entitlement {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the entitlement.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output that is associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The ARN of the entitlement.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The name of the entitlement.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flow using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,
}

/// <p>The settings for a flow, including its source, outputs, and entitlements.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Flow {
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>A description of the flow. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address from which video will be sent to output destinations.</p>
    #[serde(rename = "EgressIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ip: Option<String>,
    /// <p>The entitlements in this flow.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
    /// <p>The Amazon Resource Name (ARN), a unique identifier for any AWS resource, of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The outputs in this flow.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<Output>,
    #[serde(rename = "Source")]
    pub source: Source,
    /// <p>The current status of the flow.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>The entitlements that you want to grant on a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantEntitlementRequest {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the entitlement. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the subscriber or end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output that is associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The name of the entitlement. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flows using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,
}

/// <p>A request to grant entitlements on a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantFlowEntitlementsRequest {
    /// <p>The list of entitlements that you want to grant.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<GrantEntitlementRequest>,
    /// <p>The flow that you want to grant entitlements on.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantFlowEntitlementsResponse {
    /// <p>The entitlements that were just granted.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    /// <p>The ARN of the flow that these entitlements were granted to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEntitlementsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListEntitlements request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 20 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEntitlementsResponse {
    /// <p>A list of entitlements that have been granted to you from other AWS accounts.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<ListedEntitlement>>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFlowsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListFlows request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 10 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListFlows request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListFlows request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFlowsResponse {
    /// <p>A list of flow summaries.</p>
    #[serde(rename = "Flows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<ListedFlow>>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListFlows request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListFlows request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource for which to list the tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A map from tag keys to values. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An entitlement that has been granted to you from other AWS accounts.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedEntitlement {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>The ARN of the entitlement.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The name of the entitlement.</p>
    #[serde(rename = "EntitlementName")]
    pub entitlement_name: String,
}

/// <p>Provides a summary of a flow, including its ARN, Availability Zone, and source type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedFlow {
    /// <p>The Availability Zone that the flow was created in.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>A description of the flow.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The ARN of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of source. This value is either owned (originated somewhere other than an AWS Elemental MediaConnect flow owned by another AWS account) or entitled (originated at an AWS Elemental MediaConnect flow owned by another AWS account).</p>
    #[serde(rename = "SourceType")]
    pub source_type: String,
    /// <p>The current status of the flow.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Messages that provide the state of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Messages {
    /// <p>A list of errors that might have been generated from processes on this flow.</p>
    #[serde(rename = "Errors")]
    pub errors: Vec<String>,
}

/// <p>The settings for an output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Output {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the output.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The address where you want to send the output.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The ARN of the entitlement on the originator&#39;&#39;s flow. This value is relevant only on entitled flows.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The input ARN of the AWS Elemental MediaLive channel. This parameter is relevant only for outputs that were added by creating a MediaLive input.</p>
    #[serde(rename = "MediaLiveInputArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input_arn: Option<String>,
    /// <p>The name of the output. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ARN of the output.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Attributes related to the transport stream that are used in the output.</p>
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveFlowOutputRequest {
    /// <p>The flow that you want to remove an output from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The ARN of the output that you want to remove.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveFlowOutputResponse {
    /// <p>The ARN of the flow that is associated with the output you removed.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The ARN of the output that was removed.</p>
    #[serde(rename = "OutputArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeFlowEntitlementRequest {
    /// <p>The ARN of the entitlement that you want to revoke.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The flow that you want to revoke an entitlement from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeFlowEntitlementResponse {
    /// <p>The ARN of the entitlement that was revoked.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The ARN of the flow that the entitlement was revoked from.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The settings for the source of the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSourceRequest {
    /// <p>The type of encryption that is used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to this flow. The entitlement is set by the flow originator, and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The name of the source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The protocol that is used by the source.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

/// <p>The settings for the source of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Source {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>The type of encryption that is used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to content that comes from another AWS account. The entitlement is set by the content originator and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The IP address that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_ip: Option<String>,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The name of the source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ARN of the source.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>Attributes related to the transport stream that are used in the source.</p>
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFlowRequest {
    /// <p>The ARN of the flow that you want to start.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFlowResponse {
    /// <p>The ARN of the flow that you started.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the StartFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopFlowRequest {
    /// <p>The ARN of the flow that you want to stop.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopFlowResponse {
    /// <p>The ARN of the flow that you stopped.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the StopFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource to which to add tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A map from tag keys to values. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Attributes related to the transport stream that are used in a source or output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Transport {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The protocol that is used by the source or output.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>The remote ID for the Zixi-pull stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource from which to delete tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Information about the encryption of the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEncryption {
    /// <p>The type of algorithm that is used for the encryption (such as aes128, aes192, or aes256).</p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>A 128-bit, 16-byte hex value represented by a 32-character string, to be used with the key for encrypting content. This parameter is not valid for static key encryption.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The value of one of the devices that you configured with your digital rights management (DRM) platform key provider. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The type of key that is used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// <p>The AWS Region that the API Gateway proxy endpoint was created in. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>An identifier for the content. The service sends this value to the key server to identify the current endpoint. The resource ID is also known as the content ID. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The ARN of the role that you created during setup (when you set up AWS Elemental MediaConnect as a trusted entity).</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The ARN of the secret that you created in AWS Secrets Manager to store the encryption key. This parameter is required for static key encryption and is not valid for SPEKE encryption.</p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The URL from the API Gateway proxy that you set up to talk to your key server. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The entitlement fields that you want to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowEntitlementRequest {
    /// <p>A description of the entitlement. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the subscriber or end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    /// <p>The ARN of the entitlement that you want to update.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The flow that is associated with the entitlement that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flow using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowEntitlementResponse {
    #[serde(rename = "Entitlement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<Entitlement>,
    /// <p>The ARN of the flow that this entitlement was granted on.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The fields that you want to update in the output.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowOutputRequest {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>A description of the output. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address where you want to send the output.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    /// <p>The flow that is associated with the output that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The maximum latency in milliseconds for Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The ARN of the output that you want to update.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol to use for the output.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The remote ID for the Zixi-pull stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowOutputResponse {
    /// <p>The ARN of the flow that is associated with the updated output.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
}

/// <p>A request to update the source of a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowSourceRequest {
    /// <p>The type of encryption used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<UpdateEncryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to this flow. The entitlement is set by the flow originator, and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The flow that is associated with the source that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The protocol that is used by the source.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The ARN of the source that you want to update.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowSourceResponse {
    /// <p>The ARN of the flow that you want to update.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The settings for the source of the flow.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// Errors returned by AddFlowOutputs
#[derive(Debug, PartialEq)]
pub enum AddFlowOutputsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    AddFlowOutputs420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl AddFlowOutputsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFlowOutputsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AddFlowOutputs420Exception" => {
                    return RusotoError::Service(AddFlowOutputsError::AddFlowOutputs420(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(AddFlowOutputsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AddFlowOutputsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AddFlowOutputsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AddFlowOutputsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddFlowOutputsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddFlowOutputsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddFlowOutputsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFlowOutputsError::AddFlowOutputs420(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::BadRequest(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::Forbidden(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::NotFound(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFlowOutputsError {}
/// Errors returned by CreateFlow
#[derive(Debug, PartialEq)]
pub enum CreateFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    CreateFlow420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl CreateFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFlowError::BadRequest(err.msg))
                }
                "CreateFlow420Exception" => {
                    return RusotoError::Service(CreateFlowError::CreateFlow420(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateFlowError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFlowError::CreateFlow420(ref cause) => write!(f, "{}", cause),
            CreateFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFlowError {}
/// Errors returned by DeleteFlow
#[derive(Debug, PartialEq)]
pub enum DeleteFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DeleteFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFlowError {}
/// Errors returned by DescribeFlow
#[derive(Debug, PartialEq)]
pub enum DescribeFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DescribeFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFlowError {}
/// Errors returned by GrantFlowEntitlements
#[derive(Debug, PartialEq)]
pub enum GrantFlowEntitlementsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    GrantFlowEntitlements420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl GrantFlowEntitlementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GrantFlowEntitlementsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::Forbidden(err.msg))
                }
                "GrantFlowEntitlements420Exception" => {
                    return RusotoError::Service(
                        GrantFlowEntitlementsError::GrantFlowEntitlements420(err.msg),
                    )
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::TooManyRequests(
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
impl fmt::Display for GrantFlowEntitlementsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrantFlowEntitlementsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::GrantFlowEntitlements420(ref cause) => {
                write!(f, "{}", cause)
            }
            GrantFlowEntitlementsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::NotFound(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GrantFlowEntitlementsError {}
/// Errors returned by ListEntitlements
#[derive(Debug, PartialEq)]
pub enum ListEntitlementsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListEntitlementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEntitlementsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEntitlementsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListEntitlementsError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListEntitlementsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntitlementsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEntitlementsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitlementsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitlementsError {}
/// Errors returned by ListFlows
#[derive(Debug, PartialEq)]
pub enum ListFlowsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFlowsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFlowsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFlowsError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListFlowsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListFlowsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFlowsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListFlowsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListFlowsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListFlowsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFlowsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RemoveFlowOutput
#[derive(Debug, PartialEq)]
pub enum RemoveFlowOutputError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RemoveFlowOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFlowOutputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveFlowOutputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveFlowOutputError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveFlowOutputError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveFlowOutputError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveFlowOutputError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveFlowOutputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveFlowOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFlowOutputError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFlowOutputError {}
/// Errors returned by RevokeFlowEntitlement
#[derive(Debug, PartialEq)]
pub enum RevokeFlowEntitlementError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RevokeFlowEntitlementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeFlowEntitlementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::TooManyRequests(
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
impl fmt::Display for RevokeFlowEntitlementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeFlowEntitlementError::BadRequest(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::Forbidden(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::NotFound(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeFlowEntitlementError {}
/// Errors returned by StartFlow
#[derive(Debug, PartialEq)]
pub enum StartFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl StartFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            StartFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StartFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFlowError {}
/// Errors returned by StopFlow
#[derive(Debug, PartialEq)]
pub enum StopFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl StopFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StopFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            StopFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            StopFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StopFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopFlowError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFlowEntitlement
#[derive(Debug, PartialEq)]
pub enum UpdateFlowEntitlementError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowEntitlementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowEntitlementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::TooManyRequests(
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
impl fmt::Display for UpdateFlowEntitlementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowEntitlementError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowEntitlementError {}
/// Errors returned by UpdateFlowOutput
#[derive(Debug, PartialEq)]
pub enum UpdateFlowOutputError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowOutputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowOutputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowOutputError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowOutputError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowOutputError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowOutputError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowOutputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowOutputError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowOutputError {}
/// Errors returned by UpdateFlowSource
#[derive(Debug, PartialEq)]
pub enum UpdateFlowSourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowSourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowSourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowSourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowSourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowSourceError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowSourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowSourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowSourceError {}
/// Trait representing the capabilities of the AWS MediaConnect API. AWS MediaConnect clients implement this trait.
#[async_trait]
pub trait MediaConnect {
    /// <p>Adds outputs to an existing flow. You can create up to 20 outputs per flow.</p>
    async fn add_flow_outputs(
        &self,
        input: AddFlowOutputsRequest,
    ) -> Result<AddFlowOutputsResponse, RusotoError<AddFlowOutputsError>>;

    /// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 20) and entitlements (up to 50).</p>
    async fn create_flow(
        &self,
        input: CreateFlowRequest,
    ) -> Result<CreateFlowResponse, RusotoError<CreateFlowError>>;

    /// <p>Deletes a flow. Before you can delete a flow, you must stop the flow.</p>
    async fn delete_flow(
        &self,
        input: DeleteFlowRequest,
    ) -> Result<DeleteFlowResponse, RusotoError<DeleteFlowError>>;

    /// <p>Displays the details of a flow. The response includes the flow ARN, name, and Availability Zone, as well as details about the source, outputs, and entitlements.</p>
    async fn describe_flow(
        &self,
        input: DescribeFlowRequest,
    ) -> Result<DescribeFlowResponse, RusotoError<DescribeFlowError>>;

    /// <p>Grants entitlements to an existing flow.</p>
    async fn grant_flow_entitlements(
        &self,
        input: GrantFlowEntitlementsRequest,
    ) -> Result<GrantFlowEntitlementsResponse, RusotoError<GrantFlowEntitlementsError>>;

    /// <p>Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page.</p>
    async fn list_entitlements(
        &self,
        input: ListEntitlementsRequest,
    ) -> Result<ListEntitlementsResponse, RusotoError<ListEntitlementsError>>;

    /// <p>Displays a list of flows that are associated with this account. This request returns a paginated result.</p>
    async fn list_flows(
        &self,
        input: ListFlowsRequest,
    ) -> Result<ListFlowsResponse, RusotoError<ListFlowsError>>;

    /// <p>List all tags on an AWS Elemental MediaConnect resource</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Removes an output from an existing flow. This request can be made only on an output that does not have an entitlement associated with it. If the output has an entitlement, you must revoke the entitlement instead. When an entitlement is revoked from a flow, the service automatically removes the associated output.</p>
    async fn remove_flow_output(
        &self,
        input: RemoveFlowOutputRequest,
    ) -> Result<RemoveFlowOutputResponse, RusotoError<RemoveFlowOutputError>>;

    /// <p>Revokes an entitlement from a flow. Once an entitlement is revoked, the content becomes unavailable to the subscriber and the associated output is removed.</p>
    async fn revoke_flow_entitlement(
        &self,
        input: RevokeFlowEntitlementRequest,
    ) -> Result<RevokeFlowEntitlementResponse, RusotoError<RevokeFlowEntitlementError>>;

    /// <p>Starts a flow.</p>
    async fn start_flow(
        &self,
        input: StartFlowRequest,
    ) -> Result<StartFlowResponse, RusotoError<StartFlowError>>;

    /// <p>Stops a flow.</p>
    async fn stop_flow(
        &self,
        input: StopFlowRequest,
    ) -> Result<StopFlowResponse, RusotoError<StopFlowError>>;

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>You can change an entitlement&#39;s description, subscribers, and encryption. If you change the subscribers, the service will remove the outputs that are are used by the subscribers that are removed.</p>
    async fn update_flow_entitlement(
        &self,
        input: UpdateFlowEntitlementRequest,
    ) -> Result<UpdateFlowEntitlementResponse, RusotoError<UpdateFlowEntitlementError>>;

    /// <p>Updates an existing flow output.</p>
    async fn update_flow_output(
        &self,
        input: UpdateFlowOutputRequest,
    ) -> Result<UpdateFlowOutputResponse, RusotoError<UpdateFlowOutputError>>;

    /// <p>Updates the source of a flow.</p>
    async fn update_flow_source(
        &self,
        input: UpdateFlowSourceRequest,
    ) -> Result<UpdateFlowSourceResponse, RusotoError<UpdateFlowSourceError>>;
}
/// A client for the AWS MediaConnect API.
#[derive(Clone)]
pub struct MediaConnectClient {
    client: Client,
    region: region::Region,
}

impl MediaConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaConnectClient {
        MediaConnectClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaConnectClient {
        MediaConnectClient { client, region }
    }
}

#[async_trait]
impl MediaConnect for MediaConnectClient {
    /// <p>Adds outputs to an existing flow. You can create up to 20 outputs per flow.</p>
    async fn add_flow_outputs(
        &self,
        input: AddFlowOutputsRequest,
    ) -> Result<AddFlowOutputsResponse, RusotoError<AddFlowOutputsError>> {
        let request_uri = format!("/v1/flows/{flow_arn}/outputs", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFlowOutputsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFlowOutputsError::from_response(response))
        }
    }

    /// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 20) and entitlements (up to 50).</p>
    async fn create_flow(
        &self,
        input: CreateFlowRequest,
    ) -> Result<CreateFlowResponse, RusotoError<CreateFlowError>> {
        let request_uri = "/v1/flows";

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFlowError::from_response(response))
        }
    }

    /// <p>Deletes a flow. Before you can delete a flow, you must stop the flow.</p>
    async fn delete_flow(
        &self,
        input: DeleteFlowRequest,
    ) -> Result<DeleteFlowResponse, RusotoError<DeleteFlowError>> {
        let request_uri = format!("/v1/flows/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFlowError::from_response(response))
        }
    }

    /// <p>Displays the details of a flow. The response includes the flow ARN, name, and Availability Zone, as well as details about the source, outputs, and entitlements.</p>
    async fn describe_flow(
        &self,
        input: DescribeFlowRequest,
    ) -> Result<DescribeFlowResponse, RusotoError<DescribeFlowError>> {
        let request_uri = format!("/v1/flows/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFlowError::from_response(response))
        }
    }

    /// <p>Grants entitlements to an existing flow.</p>
    async fn grant_flow_entitlements(
        &self,
        input: GrantFlowEntitlementsRequest,
    ) -> Result<GrantFlowEntitlementsResponse, RusotoError<GrantFlowEntitlementsError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements",
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GrantFlowEntitlementsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GrantFlowEntitlementsError::from_response(response))
        }
    }

    /// <p>Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page.</p>
    async fn list_entitlements(
        &self,
        input: ListEntitlementsRequest,
    ) -> Result<ListEntitlementsResponse, RusotoError<ListEntitlementsError>> {
        let request_uri = "/v1/entitlements";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEntitlementsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEntitlementsError::from_response(response))
        }
    }

    /// <p>Displays a list of flows that are associated with this account. This request returns a paginated result.</p>
    async fn list_flows(
        &self,
        input: ListFlowsRequest,
    ) -> Result<ListFlowsResponse, RusotoError<ListFlowsError>> {
        let request_uri = "/v1/flows";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFlowsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFlowsError::from_response(response))
        }
    }

    /// <p>List all tags on an AWS Elemental MediaConnect resource</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Removes an output from an existing flow. This request can be made only on an output that does not have an entitlement associated with it. If the output has an entitlement, you must revoke the entitlement instead. When an entitlement is revoked from a flow, the service automatically removes the associated output.</p>
    async fn remove_flow_output(
        &self,
        input: RemoveFlowOutputRequest,
    ) -> Result<RemoveFlowOutputResponse, RusotoError<RemoveFlowOutputError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/outputs/{output_arn}",
            flow_arn = input.flow_arn,
            output_arn = input.output_arn
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFlowOutputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFlowOutputError::from_response(response))
        }
    }

    /// <p>Revokes an entitlement from a flow. Once an entitlement is revoked, the content becomes unavailable to the subscriber and the associated output is removed.</p>
    async fn revoke_flow_entitlement(
        &self,
        input: RevokeFlowEntitlementRequest,
    ) -> Result<RevokeFlowEntitlementResponse, RusotoError<RevokeFlowEntitlementError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements/{entitlement_arn}",
            entitlement_arn = input.entitlement_arn,
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RevokeFlowEntitlementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RevokeFlowEntitlementError::from_response(response))
        }
    }

    /// <p>Starts a flow.</p>
    async fn start_flow(
        &self,
        input: StartFlowRequest,
    ) -> Result<StartFlowResponse, RusotoError<StartFlowError>> {
        let request_uri = format!("/v1/flows/start/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartFlowError::from_response(response))
        }
    }

    /// <p>Stops a flow.</p>
    async fn stop_flow(
        &self,
        input: StopFlowRequest,
    ) -> Result<StopFlowResponse, RusotoError<StopFlowError>> {
        let request_uri = format!("/v1/flows/stop/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopFlowError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>You can change an entitlement&#39;s description, subscribers, and encryption. If you change the subscribers, the service will remove the outputs that are are used by the subscribers that are removed.</p>
    async fn update_flow_entitlement(
        &self,
        input: UpdateFlowEntitlementRequest,
    ) -> Result<UpdateFlowEntitlementResponse, RusotoError<UpdateFlowEntitlementError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements/{entitlement_arn}",
            entitlement_arn = input.entitlement_arn,
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowEntitlementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowEntitlementError::from_response(response))
        }
    }

    /// <p>Updates an existing flow output.</p>
    async fn update_flow_output(
        &self,
        input: UpdateFlowOutputRequest,
    ) -> Result<UpdateFlowOutputResponse, RusotoError<UpdateFlowOutputError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/outputs/{output_arn}",
            flow_arn = input.flow_arn,
            output_arn = input.output_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowOutputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowOutputError::from_response(response))
        }
    }

    /// <p>Updates the source of a flow.</p>
    async fn update_flow_source(
        &self,
        input: UpdateFlowSourceRequest,
    ) -> Result<UpdateFlowSourceResponse, RusotoError<UpdateFlowSourceError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/source/{source_arn}",
            flow_arn = input.flow_arn,
            source_arn = input.source_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowSourceError::from_response(response))
        }
    }
}
