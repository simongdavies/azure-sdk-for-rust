#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateVerificationDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateBodyDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateListDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CertificateDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateWithNonceDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificatePropertiesWithNonce>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessSignatureAuthorizationRule {
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    pub rights: shared_access_signature_authorization_rule::Rights,
}
pub mod shared_access_signature_authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rights {
        RegistryRead,
        RegistryWrite,
        ServiceConnect,
        DeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite")]
        RegistryReadRegistryWrite,
        #[serde(rename = "RegistryRead, ServiceConnect")]
        RegistryReadServiceConnect,
        #[serde(rename = "RegistryRead, DeviceConnect")]
        RegistryReadDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect")]
        RegistryWriteServiceConnect,
        #[serde(rename = "RegistryWrite, DeviceConnect")]
        RegistryWriteDeviceConnect,
        #[serde(rename = "ServiceConnect, DeviceConnect")]
        ServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect")]
        RegistryReadRegistryWriteServiceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, DeviceConnect")]
        RegistryReadRegistryWriteDeviceConnect,
        #[serde(rename = "RegistryRead, ServiceConnect, DeviceConnect")]
        RegistryReadServiceConnectDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryWriteServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryReadRegistryWriteServiceConnectDeviceConnect,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(skip_serializing)]
    pub subject: Option<String>,
    #[serde(skip_serializing)]
    pub expiry: Option<String>,
    #[serde(skip_serializing)]
    pub thumbprint: Option<String>,
    #[serde(rename = "isVerified", skip_serializing)]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing)]
    pub created: Option<String>,
    #[serde(skip_serializing)]
    pub updated: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificatePropertiesWithNonce {
    #[serde(skip_serializing)]
    pub subject: Option<String>,
    #[serde(skip_serializing)]
    pub expiry: Option<String>,
    #[serde(skip_serializing)]
    pub thumbprint: Option<String>,
    #[serde(rename = "isVerified", skip_serializing)]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing)]
    pub created: Option<String>,
    #[serde(skip_serializing)]
    pub updated: Option<String>,
    #[serde(rename = "verificationCode", skip_serializing)]
    pub verification_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubProperties {
    #[serde(rename = "authorizationPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_policies: Vec<SharedAccessSignatureAuthorizationRule>,
    #[serde(rename = "ipFilterRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_filter_rules: Vec<IpFilterRule>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "hostName", skip_serializing)]
    pub host_name: Option<String>,
    #[serde(rename = "eventHubEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_endpoints: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<RoutingProperties>,
    #[serde(rename = "storageEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoints: Option<serde_json::Value>,
    #[serde(rename = "messagingEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub messaging_endpoints: Option<serde_json::Value>,
    #[serde(rename = "enableFileUploadNotifications", default, skip_serializing_if = "Option::is_none")]
    pub enable_file_upload_notifications: Option<bool>,
    #[serde(rename = "cloudToDevice", default, skip_serializing_if = "Option::is_none")]
    pub cloud_to_device: Option<CloudToDeviceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "operationsMonitoringProperties", default, skip_serializing_if = "Option::is_none")]
    pub operations_monitoring_properties: Option<OperationsMonitoringProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<iot_hub_properties::Features>,
}
pub mod iot_hub_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Features {
        None,
        DeviceManagement,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuInfo {
    pub name: iot_hub_sku_info::Name,
    #[serde(skip_serializing)]
    pub tier: Option<iot_hub_sku_info::Tier>,
    pub capacity: i64,
}
pub mod iot_hub_sku_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        F1,
        S1,
        S2,
        S3,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubProperties {
    #[serde(rename = "retentionTimeInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_time_in_days: Option<i64>,
    #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[serde(rename = "partitionIds", skip_serializing)]
    pub partition_ids: Vec<String>,
    #[serde(skip_serializing)]
    pub path: Option<String>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageEndpointProperties {
    #[serde(rename = "sasTtlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub sas_ttl_as_iso8601: Option<String>,
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagingEndpointProperties {
    #[serde(rename = "lockDurationAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[serde(rename = "ttlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudToDeviceProperties {
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(rename = "defaultTtlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub default_ttl_as_iso8601: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<FeedbackProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsMonitoringProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpFilterRule {
    #[serde(rename = "filterName")]
    pub filter_name: String,
    pub action: ip_filter_rule::Action,
    #[serde(rename = "ipMask")]
    pub ip_mask: String,
}
pub mod ip_filter_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Accept,
        Reject,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedbackProperties {
    #[serde(rename = "lockDurationAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[serde(rename = "ttlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<RoutingEndpoints>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<RouteProperties>,
    #[serde(rename = "fallbackRoute", default, skip_serializing_if = "Option::is_none")]
    pub fallback_route: Option<FallbackRouteProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingEndpoints {
    #[serde(rename = "serviceBusQueues", default, skip_serializing_if = "Vec::is_empty")]
    pub service_bus_queues: Vec<RoutingServiceBusQueueEndpointProperties>,
    #[serde(rename = "serviceBusTopics", default, skip_serializing_if = "Vec::is_empty")]
    pub service_bus_topics: Vec<RoutingServiceBusTopicEndpointProperties>,
    #[serde(rename = "eventHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hubs: Vec<RoutingEventHubProperties>,
    #[serde(rename = "storageContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_containers: Vec<RoutingStorageContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusQueueEndpointProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusTopicEndpointProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingEventHubProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingStorageContainerProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "fileNameFormat", default, skip_serializing_if = "Option::is_none")]
    pub file_name_format: Option<String>,
    #[serde(rename = "batchFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub batch_frequency_in_seconds: Option<i32>,
    #[serde(rename = "maxChunkSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_chunk_size_in_bytes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteProperties {
    pub name: String,
    pub source: route_properties::Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
pub mod route_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        DeviceMessages,
        TwinChangeEvents,
        DeviceLifecycleEvents,
        DeviceJobLifecycleEvents,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FallbackRouteProperties {
    pub source: fallback_route_properties::Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
pub mod fallback_route_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        DeviceMessages,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDescription {
    #[serde(flatten)]
    pub resource: Resource,
    pub subscriptionid: String,
    pub resourcegroup: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubProperties>,
    pub sku: IotHubSkuInfo,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessSignatureAuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessSignatureAuthorizationRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(rename = "Code", skip_serializing)]
    pub code: Option<String>,
    #[serde(rename = "HttpStatusCode", skip_serializing)]
    pub http_status_code: Option<String>,
    #[serde(rename = "Message", skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "Details", skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubQuotaMetricInfoListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubQuotaMetricInfo>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryStatistics {
    #[serde(rename = "totalDeviceCount", skip_serializing)]
    pub total_device_count: Option<i64>,
    #[serde(rename = "enabledDeviceCount", skip_serializing)]
    pub enabled_device_count: Option<i64>,
    #[serde(rename = "disabledDeviceCount", skip_serializing)]
    pub disabled_device_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuDescription {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    pub sku: IotHubSkuInfo,
    pub capacity: IotHubCapacity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubCapacity {
    #[serde(skip_serializing)]
    pub minimum: Option<i64>,
    #[serde(skip_serializing)]
    pub maximum: Option<i64>,
    #[serde(skip_serializing)]
    pub default: Option<i64>,
    #[serde(rename = "scaleType", skip_serializing)]
    pub scale_type: Option<iot_hub_capacity::ScaleType>,
}
pub mod iot_hub_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConsumerGroupsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConsumerGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuDescriptionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubSkuDescription>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponse {
    #[serde(rename = "jobId", skip_serializing)]
    pub job_id: Option<String>,
    #[serde(rename = "startTimeUtc", skip_serializing)]
    pub start_time_utc: Option<String>,
    #[serde(rename = "endTimeUtc", skip_serializing)]
    pub end_time_utc: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<job_response::Type>,
    #[serde(skip_serializing)]
    pub status: Option<job_response::Status>,
    #[serde(rename = "failureReason", skip_serializing)]
    pub failure_reason: Option<String>,
    #[serde(rename = "statusMessage", skip_serializing)]
    pub status_message: Option<String>,
    #[serde(rename = "parentJobId", skip_serializing)]
    pub parent_job_id: Option<String>,
}
pub mod job_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "export")]
        Export,
        #[serde(rename = "import")]
        Import,
        #[serde(rename = "backup")]
        Backup,
        #[serde(rename = "readDeviceProperties")]
        ReadDeviceProperties,
        #[serde(rename = "writeDeviceProperties")]
        WriteDeviceProperties,
        #[serde(rename = "updateDeviceConfiguration")]
        UpdateDeviceConfiguration,
        #[serde(rename = "rebootDevice")]
        RebootDevice,
        #[serde(rename = "factoryResetDevice")]
        FactoryResetDevice,
        #[serde(rename = "firmwareUpdate")]
        FirmwareUpdate,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "enqueued")]
        Enqueued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDescriptionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubDescription>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubQuotaMetricInfo {
    #[serde(rename = "Name", skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "CurrentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(rename = "MaxValue", skip_serializing)]
    pub max_value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    #[serde(rename = "Name")]
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubNameAvailabilityInfo {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<iot_hub_name_availability_info::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod iot_hub_name_availability_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDevicesRequest {
    #[serde(rename = "ExportBlobContainerUri")]
    pub export_blob_container_uri: String,
    #[serde(rename = "ExcludeKeys")]
    pub exclude_keys: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportDevicesRequest {
    #[serde(rename = "InputBlobContainerUri")]
    pub input_blob_container_uri: String,
    #[serde(rename = "OutputBlobContainerUri")]
    pub output_blob_container_uri: String,
}
