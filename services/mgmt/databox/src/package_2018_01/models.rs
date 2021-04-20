#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCredentialDetails {
    #[serde(rename = "accountName", skip_serializing)]
    pub account_name: Option<String>,
    #[serde(rename = "accountConnectionString", skip_serializing)]
    pub account_connection_string: Option<String>,
    #[serde(rename = "shareCredentialDetails", skip_serializing)]
    pub share_credential_details: Vec<ShareCredentialDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressValidationOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddressValidationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressValidationProperties {
    #[serde(rename = "validationStatus", skip_serializing)]
    pub validation_status: Option<address_validation_properties::ValidationStatus>,
    #[serde(rename = "alternateAddresses", skip_serializing)]
    pub alternate_addresses: Vec<ShippingAddress>,
}
pub mod address_validation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationStatus {
        Valid,
        Invalid,
        Ambiguous,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceNetworkConfiguration {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "macAddress", skip_serializing)]
    pub mac_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmBaseObject {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableSkuRequest {
    #[serde(rename = "transferType")]
    pub transfer_type: available_sku_request::TransferType,
    pub country: String,
    pub location: String,
    #[serde(rename = "skuNames", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_names: Vec<String>,
}
pub mod available_sku_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableSkusResult {
    #[serde(skip_serializing)]
    pub value: Vec<SkuInformation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancellationReason {
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    #[serde(rename = "contactName")]
    pub contact_name: String,
    pub phone: String,
    #[serde(rename = "phoneExtension", default, skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
    #[serde(rename = "notificationPreference", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_preference: Vec<NotificationPreference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyLogDetails {
    #[serde(rename = "copyLogDetailsType")]
    pub copy_log_details_type: copy_log_details::CopyLogDetailsType,
}
pub mod copy_log_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CopyLogDetailsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyProgress {
    #[serde(rename = "storageAccountName", skip_serializing)]
    pub storage_account_name: Option<String>,
    #[serde(rename = "accountId", skip_serializing)]
    pub account_id: Option<String>,
    #[serde(rename = "bytesSentToCloud", skip_serializing)]
    pub bytes_sent_to_cloud: Option<i64>,
    #[serde(rename = "totalBytesToProcess", skip_serializing)]
    pub total_bytes_to_process: Option<i64>,
    #[serde(rename = "filesProcessed", skip_serializing)]
    pub files_processed: Option<i64>,
    #[serde(rename = "totalFilesToProcess", skip_serializing)]
    pub total_files_to_process: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxAccountCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[serde(rename = "accountName", skip_serializing)]
    pub account_name: Option<String>,
    #[serde(rename = "copyLogLink", skip_serializing)]
    pub copy_log_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[serde(rename = "diskSerialNumber", skip_serializing)]
    pub disk_serial_number: Option<String>,
    #[serde(rename = "errorLogLink", skip_serializing)]
    pub error_log_link: Option<String>,
    #[serde(rename = "verboseLogLink", skip_serializing)]
    pub verbose_log_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskCopyProgress {
    #[serde(rename = "serialNumber", skip_serializing)]
    pub serial_number: Option<String>,
    #[serde(rename = "bytesCopied", skip_serializing)]
    pub bytes_copied: Option<i64>,
    #[serde(rename = "percentComplete", skip_serializing)]
    pub percent_complete: Option<i32>,
    #[serde(skip_serializing)]
    pub status: Option<data_box_disk_copy_progress::Status>,
}
pub mod data_box_disk_copy_progress {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        CompletedWithErrors,
        Failed,
        NotReturned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[serde(rename = "preferredDisks", default, skip_serializing_if = "Option::is_none")]
    pub preferred_disks: Option<serde_json::Value>,
    #[serde(rename = "copyProgress", skip_serializing)]
    pub copy_progress: Vec<DataBoxDiskCopyProgress>,
    #[serde(rename = "disksAndSizeDetails", skip_serializing)]
    pub disks_and_size_details: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passkey: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[serde(rename = "diskSecrets", skip_serializing)]
    pub disk_secrets: Vec<DiskSecret>,
    #[serde(rename = "passKey", skip_serializing)]
    pub pass_key: Option<String>,
    #[serde(rename = "isPasskeyUserDefined", skip_serializing)]
    pub is_passkey_user_defined: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyAccountCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[serde(rename = "accountName", skip_serializing)]
    pub account_name: Option<String>,
    #[serde(rename = "copyLogLink", skip_serializing)]
    pub copy_log_link: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[serde(rename = "copyProgress", skip_serializing)]
    pub copy_progress: Vec<CopyProgress>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[serde(rename = "cabinetPodSecrets", skip_serializing)]
    pub cabinet_pod_secrets: Vec<DataBoxHeavySecret>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavySecret {
    #[serde(rename = "deviceSerialNumber", skip_serializing)]
    pub device_serial_number: Option<String>,
    #[serde(rename = "devicePassword", skip_serializing)]
    pub device_password: Option<String>,
    #[serde(rename = "networkConfigurations", skip_serializing)]
    pub network_configurations: Vec<ApplianceNetworkConfiguration>,
    #[serde(rename = "encodedValidationCertPubKey", skip_serializing)]
    pub encoded_validation_cert_pub_key: Option<String>,
    #[serde(rename = "accountCredentialDetails", skip_serializing)]
    pub account_credential_details: Vec<AccountCredentialDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[serde(rename = "copyProgress", skip_serializing)]
    pub copy_progress: Vec<CopyProgress>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataboxJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[serde(rename = "podSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub pod_secrets: Vec<DataBoxSecret>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxSecret {
    #[serde(rename = "deviceSerialNumber", skip_serializing)]
    pub device_serial_number: Option<String>,
    #[serde(rename = "devicePassword", skip_serializing)]
    pub device_password: Option<String>,
    #[serde(rename = "networkConfigurations", skip_serializing)]
    pub network_configurations: Vec<ApplianceNetworkConfiguration>,
    #[serde(rename = "encodedValidationCertPubKey", skip_serializing)]
    pub encoded_validation_cert_pub_key: Option<String>,
    #[serde(rename = "accountCredentialDetails", skip_serializing)]
    pub account_credential_details: Vec<AccountCredentialDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationAccountDetails {
    #[serde(rename = "dataDestinationType")]
    pub data_destination_type: destination_account_details::DataDestinationType,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
pub mod destination_account_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataDestinationType {
        UnknownType,
        StorageAccount,
        ManagedDisk,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationManagedDiskDetails {
    #[serde(flatten)]
    pub destination_account_details: DestinationAccountDetails,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    #[serde(rename = "stagingStorageAccountId")]
    pub staging_storage_account_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationStorageAccountDetails {
    #[serde(flatten)]
    pub destination_account_details: DestinationAccountDetails,
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationToServiceLocationMap {
    #[serde(rename = "destinationLocation", skip_serializing)]
    pub destination_location: Option<String>,
    #[serde(rename = "serviceLocation", skip_serializing)]
    pub service_location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskSecret {
    #[serde(rename = "diskSerialNumber", skip_serializing)]
    pub disk_serial_number: Option<String>,
    #[serde(rename = "bitLockerKey", skip_serializing)]
    pub bit_locker_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[serde(rename = "expectedDataSizeInTeraBytes", default, skip_serializing_if = "Option::is_none")]
    pub expected_data_size_in_tera_bytes: Option<i32>,
    #[serde(rename = "jobStages", skip_serializing)]
    pub job_stages: Vec<JobStages>,
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    #[serde(rename = "shippingAddress")]
    pub shipping_address: ShippingAddress,
    #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<PackageShippingDetails>,
    #[serde(rename = "returnPackage", default, skip_serializing_if = "Option::is_none")]
    pub return_package: Option<PackageShippingDetails>,
    #[serde(rename = "destinationAccountDetails")]
    pub destination_account_details: Vec<DestinationAccountDetails>,
    #[serde(rename = "errorDetails", skip_serializing)]
    pub error_details: Vec<JobErrorDetails>,
    #[serde(rename = "jobDetailsType")]
    pub job_details_type: job_details::JobDetailsType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[serde(rename = "copyLogDetails", skip_serializing)]
    pub copy_log_details: Vec<CopyLogDetails>,
    #[serde(rename = "reverseShipmentLabelSasKey", skip_serializing)]
    pub reverse_shipment_label_sas_key: Option<String>,
    #[serde(rename = "chainOfCustodySasKey", skip_serializing)]
    pub chain_of_custody_sas_key: Option<String>,
}
pub mod job_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobDetailsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobErrorDetails {
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<i32>,
    #[serde(rename = "recommendedAction", skip_serializing)]
    pub recommended_action: Option<String>,
    #[serde(rename = "exceptionMessage", skip_serializing)]
    pub exception_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename = "isCancellable", skip_serializing)]
    pub is_cancellable: Option<bool>,
    #[serde(rename = "isDeletable", skip_serializing)]
    pub is_deletable: Option<bool>,
    #[serde(rename = "isShippingAddressEditable", skip_serializing)]
    pub is_shipping_address_editable: Option<bool>,
    #[serde(skip_serializing)]
    pub status: Option<job_properties::Status>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<JobDetails>,
    #[serde(rename = "cancellationReason", skip_serializing)]
    pub cancellation_reason: Option<String>,
}
pub mod job_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        DeviceOrdered,
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        #[serde(rename = "Failed_IssueReportedAtCustomer")]
        FailedIssueReportedAtCustomer,
        #[serde(rename = "Failed_IssueDetectedAtAzureDC")]
        FailedIssueDetectedAtAzureDc,
        Aborted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: JobProperties,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResourceUpdateParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateJobProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSecrets {
    #[serde(rename = "jobSecretsType")]
    pub job_secrets_type: job_secrets::JobSecretsType,
}
pub mod job_secrets {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobSecretsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStages {
    #[serde(rename = "stageName", skip_serializing)]
    pub stage_name: Option<job_stages::StageName>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "stageStatus", skip_serializing)]
    pub stage_status: Option<job_stages::StageStatus>,
    #[serde(rename = "stageTime", skip_serializing)]
    pub stage_time: Option<String>,
    #[serde(rename = "jobStageDetails", skip_serializing)]
    pub job_stage_details: Option<serde_json::Value>,
    #[serde(rename = "errorDetails", skip_serializing)]
    pub error_details: Vec<JobErrorDetails>,
}
pub mod job_stages {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DeviceOrdered,
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        #[serde(rename = "Failed_IssueReportedAtCustomer")]
        FailedIssueReportedAtCustomer,
        #[serde(rename = "Failed_IssueDetectedAtAzureDC")]
        FailedIssueDetectedAtAzureDc,
        Aborted,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
        SucceededWithErrors,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreference {
    #[serde(rename = "stageName")]
    pub stage_name: notification_preference::StageName,
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
}
pub mod notification_preference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageShippingDetails {
    #[serde(rename = "carrierName", skip_serializing)]
    pub carrier_name: Option<String>,
    #[serde(rename = "trackingId", skip_serializing)]
    pub tracking_id: Option<String>,
    #[serde(rename = "trackingUrl", skip_serializing)]
    pub tracking_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(rename = "preferredDataCenterRegion", default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_data_center_region: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub sku: Sku,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareCredentialDetails {
    #[serde(rename = "shareName", skip_serializing)]
    pub share_name: Option<String>,
    #[serde(rename = "shareType", skip_serializing)]
    pub share_type: Option<share_credential_details::ShareType>,
    #[serde(rename = "userName", skip_serializing)]
    pub user_name: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(rename = "supportedAccessProtocols", skip_serializing)]
    pub supported_access_protocols: Vec<String>,
}
pub mod share_credential_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ShareType {
        UnknownType,
        #[serde(rename = "HCS")]
        Hcs,
        BlockBlob,
        PageBlob,
        AzureFile,
        ManagedDisk,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipmentPickUpRequest {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "shipmentLocation")]
    pub shipment_location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipmentPickUpResponse {
    #[serde(rename = "confirmationNumber", skip_serializing)]
    pub confirmation_number: Option<String>,
    #[serde(rename = "readyByTime", skip_serializing)]
    pub ready_by_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[serde(rename = "streetAddress3", default, skip_serializing_if = "Option::is_none")]
    pub street_address3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    pub country: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "zipExtendedCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_extended_code: Option<String>,
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "addressType", default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<shipping_address::AddressType>,
}
pub mod shipping_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddressType {
        None,
        Residential,
        Commercial,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapacity {
    #[serde(skip_serializing)]
    pub usable: Option<String>,
    #[serde(skip_serializing)]
    pub maximum: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCost {
    #[serde(rename = "meterId", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(rename = "meterType", skip_serializing)]
    pub meter_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing)]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SkuProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuProperties {
    #[serde(rename = "destinationToServiceLocationMap", skip_serializing)]
    pub destination_to_service_location_map: Vec<DestinationToServiceLocationMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[serde(skip_serializing)]
    pub costs: Vec<SkuCost>,
    #[serde(rename = "apiVersions", skip_serializing)]
    pub api_versions: Vec<String>,
    #[serde(rename = "disabledReason", skip_serializing)]
    pub disabled_reason: Option<sku_properties::DisabledReason>,
    #[serde(rename = "disabledReasonMessage", skip_serializing)]
    pub disabled_reason_message: Option<String>,
    #[serde(rename = "requiredFeature", skip_serializing)]
    pub required_feature: Option<String>,
}
pub mod sku_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DisabledReason {
        None,
        Country,
        Region,
        Feature,
        OfferType,
        NoSubscriptionInfo,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnencryptedCredentials {
    #[serde(rename = "jobName", skip_serializing)]
    pub job_name: Option<String>,
    #[serde(rename = "jobSecrets", default, skip_serializing_if = "Option::is_none")]
    pub job_secrets: Option<JobSecrets>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnencryptedCredentialsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UnencryptedCredentials>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateJobDetails {
    #[serde(rename = "contactDetails", default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateJobProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<UpdateJobDetails>,
    #[serde(rename = "destinationAccountDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_account_details: Vec<DestinationAccountDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateAddress {
    #[serde(rename = "shippingAddress")]
    pub shipping_address: ShippingAddress,
    #[serde(rename = "deviceType")]
    pub device_type: validate_address::DeviceType,
}
pub mod validate_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
