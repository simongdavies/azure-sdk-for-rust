#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub sku: Sku,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyDescriptionList {
    #[serde(skip_serializing)]
    pub value: Vec<KeyDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyDescription {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<KeyRank>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameter {
    pub name: String,
    pub rank: KeyRank,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTypeDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChannelTypeDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTypeDescription {
    #[serde(rename = "channelType", default, skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "channelDescription", default, skip_serializing_if = "Option::is_none")]
    pub channel_description: Option<String>,
    #[serde(rename = "channelFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub channel_functions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelProperties {
    #[serde(rename = "channelType")]
    pub channel_type: String,
    #[serde(rename = "channelFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub channel_functions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyOnlyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Channel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<CheckNameUnavailableReason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDescriptionList {
    #[serde(skip_serializing)]
    pub value: Vec<SkuDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDescription {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", skip_serializing)]
    pub location_info: Vec<SkuLocationInfoItem>,
    #[serde(skip_serializing)]
    pub restrictions: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuLocationInfoItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyRank {
    PrimaryKey,
    SecondaryKey,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CheckNameUnavailableReason {
    Invalid,
    AlreadyExists,
}
