#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerBaseProperties {
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[serde(rename = "sshConfiguration")]
    pub ssh_configuration: SshConfiguration,
    #[serde(rename = "dataDisks")]
    pub data_disks: DataDisks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FileServer>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisks {
    #[serde(rename = "diskSizeInGB")]
    pub disk_size_in_gb: i32,
    #[serde(rename = "cachingType", skip_serializing)]
    pub caching_type: Option<data_disks::CachingType>,
    #[serde(rename = "diskCount")]
    pub disk_count: i32,
    #[serde(rename = "storageAccountType")]
    pub storage_account_type: data_disks::StorageAccountType,
}
pub mod data_disks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CachingType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "readonly")]
        Readonly,
        #[serde(rename = "readwrite")]
        Readwrite,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: ResourceId,
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: ResourceId,
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountSettings {
    #[serde(rename = "mountPoint", default, skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    #[serde(rename = "fileServerPublicIP", default, skip_serializing_if = "Option::is_none")]
    pub file_server_public_ip: Option<String>,
    #[serde(rename = "fileServerInternalIP", default, skip_serializing_if = "Option::is_none")]
    pub file_server_internal_ip: Option<String>,
    #[serde(rename = "fileServerType", default, skip_serializing_if = "Option::is_none")]
    pub file_server_type: Option<mount_settings::FileServerType>,
}
pub mod mount_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FileServerType {
        #[serde(rename = "nfs")]
        Nfs,
        #[serde(rename = "glusterfs")]
        Glusterfs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshConfiguration {
    #[serde(rename = "publicIPsToAllow", default, skip_serializing_if = "Vec::is_empty")]
    pub public_i_ps_to_allow: Vec<String>,
    #[serde(rename = "userAccountSettings")]
    pub user_account_settings: UserAccountSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileServerBaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerProperties {
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "sshConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ssh_configuration: Option<SshConfiguration>,
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<DataDisks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
    #[serde(rename = "mountSettings", default, skip_serializing_if = "Option::is_none")]
    pub mount_settings: Option<MountSettings>,
    #[serde(rename = "provisioningStateTransitionTime", skip_serializing)]
    pub provisioning_state_transition_time: Option<String>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<file_server_properties::ProvisioningState>,
}
pub mod file_server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterBaseProperties {
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[serde(rename = "vmPriority", default, skip_serializing_if = "Option::is_none")]
    pub vm_priority: Option<cluster_base_properties::VmPriority>,
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
    #[serde(rename = "nodeSetup", default, skip_serializing_if = "Option::is_none")]
    pub node_setup: Option<NodeSetup>,
    #[serde(rename = "userAccountSettings")]
    pub user_account_settings: UserAccountSettings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
}
pub mod cluster_base_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VmPriority {
        #[serde(rename = "dedicated")]
        Dedicated,
        #[serde(rename = "lowpriority")]
        Lowpriority,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateProperties {
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeallocationOption {
    #[serde(rename = "requeue")]
    Requeue,
    #[serde(rename = "terminate")]
    Terminate,
    #[serde(rename = "waitforjobcompletion")]
    Waitforjobcompletion,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManualScaleSettings>,
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleSettings {
    #[serde(rename = "minimumNodeCount")]
    pub minimum_node_count: i32,
    #[serde(rename = "maximumNodeCount")]
    pub maximum_node_count: i32,
    #[serde(rename = "initialNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub initial_node_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualScaleSettings {
    #[serde(rename = "targetNodeCount")]
    pub target_node_count: i32,
    #[serde(rename = "nodeDeallocationOption", default, skip_serializing_if = "Option::is_none")]
    pub node_deallocation_option: Option<DeallocationOption>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineConfiguration {
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageReference {
    pub publisher: String,
    pub offer: String,
    pub sku: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeStateCounts {
    #[serde(rename = "idleNodeCount")]
    pub idle_node_count: i32,
    #[serde(rename = "runningNodeCount")]
    pub running_node_count: i32,
    #[serde(rename = "preparingNodeCount")]
    pub preparing_node_count: i32,
    #[serde(rename = "unusableNodeCount")]
    pub unusable_node_count: i32,
    #[serde(rename = "leavingNodeCount")]
    pub leaving_node_count: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountSettings {
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[serde(rename = "adminUserSshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_ssh_public_key: Option<String>,
    #[serde(rename = "adminUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeSetup {
    #[serde(rename = "setupTask", default, skip_serializing_if = "Option::is_none")]
    pub setup_task: Option<SetupTask>,
    #[serde(rename = "mountVolumes", default, skip_serializing_if = "Option::is_none")]
    pub mount_volumes: Option<MountVolumes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetupTask {
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentSetting>,
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[serde(rename = "stdOutErrPathPrefix")]
    pub std_out_err_path_prefix: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterBaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "vmPriority", default, skip_serializing_if = "Option::is_none")]
    pub vm_priority: Option<cluster_properties::VmPriority>,
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
    #[serde(rename = "nodeSetup", default, skip_serializing_if = "Option::is_none")]
    pub node_setup: Option<NodeSetup>,
    #[serde(rename = "userAccountSettings", default, skip_serializing_if = "Option::is_none")]
    pub user_account_settings: Option<UserAccountSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", skip_serializing)]
    pub provisioning_state_transition_time: Option<String>,
    #[serde(rename = "allocationState", skip_serializing)]
    pub allocation_state: Option<cluster_properties::AllocationState>,
    #[serde(rename = "allocationStateTransitionTime", skip_serializing)]
    pub allocation_state_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<BatchAiError>,
    #[serde(rename = "currentNodeCount", skip_serializing)]
    pub current_node_count: Option<i32>,
    #[serde(rename = "nodeStateCounts", default, skip_serializing_if = "Option::is_none")]
    pub node_state_counts: Option<NodeStateCounts>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VmPriority {
        #[serde(rename = "dedicated")]
        Dedicated,
        #[serde(rename = "lowpriority")]
        Lowpriority,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "deleting")]
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationState {
        #[serde(rename = "steady")]
        Steady,
        #[serde(rename = "resizing")]
        Resizing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBaseProperties {
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub cluster: ResourceId,
    #[serde(rename = "nodeCount")]
    pub node_count: i32,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    #[serde(rename = "cntkSettings", default, skip_serializing_if = "Option::is_none")]
    pub cntk_settings: Option<CntKsettings>,
    #[serde(rename = "tensorFlowSettings", default, skip_serializing_if = "Option::is_none")]
    pub tensor_flow_settings: Option<TensorFlowSettings>,
    #[serde(rename = "caffeSettings", default, skip_serializing_if = "Option::is_none")]
    pub caffe_settings: Option<CaffeSettings>,
    #[serde(rename = "caffe2Settings", default, skip_serializing_if = "Option::is_none")]
    pub caffe2_settings: Option<Caffe2Settings>,
    #[serde(rename = "chainerSettings", default, skip_serializing_if = "Option::is_none")]
    pub chainer_settings: Option<ChainerSettings>,
    #[serde(rename = "customToolkitSettings", default, skip_serializing_if = "Option::is_none")]
    pub custom_toolkit_settings: Option<CustomToolkitSettings>,
    #[serde(rename = "jobPreparation", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation: Option<JobPreparation>,
    #[serde(rename = "stdOutErrPathPrefix")]
    pub std_out_err_path_prefix: String,
    #[serde(rename = "inputDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub input_directories: Vec<InputDirectory>,
    #[serde(rename = "outputDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub output_directories: Vec<OutputDirectory>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<job_base_properties::Constraints>,
}
pub mod job_base_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Constraints {
        #[serde(rename = "maxWallClockTime", default, skip_serializing_if = "Option::is_none")]
        pub max_wall_clock_time: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobBaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ResourceId>,
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    #[serde(rename = "toolType", default, skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<ToolType>,
    #[serde(rename = "cntkSettings", default, skip_serializing_if = "Option::is_none")]
    pub cntk_settings: Option<CntKsettings>,
    #[serde(rename = "tensorFlowSettings", default, skip_serializing_if = "Option::is_none")]
    pub tensor_flow_settings: Option<TensorFlowSettings>,
    #[serde(rename = "caffeSettings", default, skip_serializing_if = "Option::is_none")]
    pub caffe_settings: Option<CaffeSettings>,
    #[serde(rename = "chainerSettings", default, skip_serializing_if = "Option::is_none")]
    pub chainer_settings: Option<ChainerSettings>,
    #[serde(rename = "customToolkitSettings", default, skip_serializing_if = "Option::is_none")]
    pub custom_toolkit_settings: Option<CustomToolkitSettings>,
    #[serde(rename = "jobPreparation", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation: Option<JobPreparation>,
    #[serde(rename = "stdOutErrPathPrefix", default, skip_serializing_if = "Option::is_none")]
    pub std_out_err_path_prefix: Option<String>,
    #[serde(rename = "inputDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub input_directories: Vec<InputDirectory>,
    #[serde(rename = "outputDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub output_directories: Vec<OutputDirectory>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<job_properties::Constraints>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<job_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", skip_serializing)]
    pub provisioning_state_transition_time: Option<String>,
    #[serde(rename = "executionState", default, skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<job_properties::ExecutionState>,
    #[serde(rename = "executionStateTransitionTime", skip_serializing)]
    pub execution_state_transition_time: Option<String>,
    #[serde(rename = "executionInfo", default, skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<job_properties::ExecutionInfo>,
}
pub mod job_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Constraints {
        #[serde(rename = "maxWallClockTime", default, skip_serializing_if = "Option::is_none")]
        pub max_wall_clock_time: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionState {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "terminating")]
        Terminating,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ExecutionInfo {
        #[serde(rename = "startTime")]
        pub start_time: String,
        #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
        pub end_time: Option<String>,
        #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
        pub exit_code: Option<i32>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub errors: Vec<BatchAiError>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteLoginInformation {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteLoginInformationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RemoteLoginInformation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileProperties {
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileListResult {
    #[serde(skip_serializing)]
    pub value: Vec<File>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    #[serde(rename = "cntk")]
    Cntk,
    #[serde(rename = "tensorflow")]
    Tensorflow,
    #[serde(rename = "caffe")]
    Caffe,
    #[serde(rename = "caffe2")]
    Caffe2,
    #[serde(rename = "chainer")]
    Chainer,
    #[serde(rename = "custom")]
    Custom,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameValuePair {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalDataVolume {
    #[serde(rename = "hostPath")]
    pub host_path: String,
    #[serde(rename = "localPath")]
    pub local_path: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageSourceRegistry {
    #[serde(rename = "serverUrl", default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    pub image: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<PrivateRegistryCredentials>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateRegistryCredentials {
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordSecretReference", default, skip_serializing_if = "Option::is_none")]
    pub password_secret_reference: Option<KeyVaultSecretReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSettings {
    #[serde(rename = "imageSourceRegistry")]
    pub image_source_registry: ImageSourceRegistry,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDirectory {
    pub id: String,
    pub path: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputDirectory {
    pub id: String,
    #[serde(rename = "pathPrefix")]
    pub path_prefix: String,
    #[serde(rename = "pathSuffix", default, skip_serializing_if = "Option::is_none")]
    pub path_suffix: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<output_directory::Type>,
    #[serde(rename = "createNew", default, skip_serializing_if = "Option::is_none")]
    pub create_new: Option<bool>,
}
pub mod output_directory {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "model")]
        Model,
        #[serde(rename = "logs")]
        Logs,
        #[serde(rename = "summary")]
        Summary,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageCredentialsInfo {
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[serde(rename = "accountKeySecretReference", default, skip_serializing_if = "Option::is_none")]
    pub account_key_secret_reference: Option<KeyVaultSecretReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareReference {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "azureFileUrl")]
    pub azure_file_url: String,
    pub credentials: AzureStorageCredentialsInfo,
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[serde(rename = "fileMode", default, skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "directoryMode", default, skip_serializing_if = "Option::is_none")]
    pub directory_mode: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobFileSystemReference {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
    pub credentials: AzureStorageCredentialsInfo,
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerReference {
    #[serde(rename = "fileServer")]
    pub file_server: ResourceId,
    #[serde(rename = "sourceDirectory", default, skip_serializing_if = "Option::is_none")]
    pub source_directory: Option<String>,
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnmanagedFileSystemReference {
    #[serde(rename = "mountCommand")]
    pub mount_command: String,
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountVolumes {
    #[serde(rename = "azureFileShares", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_file_shares: Vec<AzureFileShareReference>,
    #[serde(rename = "azureBlobFileSystems", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_blob_file_systems: Vec<AzureBlobFileSystemReference>,
    #[serde(rename = "fileServers", default, skip_serializing_if = "Vec::is_empty")]
    pub file_servers: Vec<FileServerReference>,
    #[serde(rename = "unmanagedFileSystems", default, skip_serializing_if = "Vec::is_empty")]
    pub unmanaged_file_systems: Vec<UnmanagedFileSystemReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CntKsettings {
    #[serde(rename = "languageType", default, skip_serializing_if = "Option::is_none")]
    pub language_type: Option<String>,
    #[serde(rename = "configFilePath", default, skip_serializing_if = "Option::is_none")]
    pub config_file_path: Option<String>,
    #[serde(rename = "pythonScriptFilePath", default, skip_serializing_if = "Option::is_none")]
    pub python_script_file_path: Option<String>,
    #[serde(rename = "pythonInterpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub python_interpreter_path: Option<String>,
    #[serde(rename = "commandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub command_line_args: Option<String>,
    #[serde(rename = "processCount", default, skip_serializing_if = "Option::is_none")]
    pub process_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaffeSettings {
    #[serde(rename = "configFilePath", default, skip_serializing_if = "Option::is_none")]
    pub config_file_path: Option<String>,
    #[serde(rename = "pythonScriptFilePath", default, skip_serializing_if = "Option::is_none")]
    pub python_script_file_path: Option<String>,
    #[serde(rename = "pythonInterpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub python_interpreter_path: Option<String>,
    #[serde(rename = "commandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub command_line_args: Option<String>,
    #[serde(rename = "processCount", default, skip_serializing_if = "Option::is_none")]
    pub process_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Caffe2Settings {
    #[serde(rename = "pythonScriptFilePath")]
    pub python_script_file_path: String,
    #[serde(rename = "pythonInterpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub python_interpreter_path: Option<String>,
    #[serde(rename = "commandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub command_line_args: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChainerSettings {
    #[serde(rename = "pythonScriptFilePath")]
    pub python_script_file_path: String,
    #[serde(rename = "pythonInterpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub python_interpreter_path: Option<String>,
    #[serde(rename = "commandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub command_line_args: Option<String>,
    #[serde(rename = "processCount", default, skip_serializing_if = "Option::is_none")]
    pub process_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomToolkitSettings {
    #[serde(rename = "commandLine", default, skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobPreparation {
    #[serde(rename = "commandLine")]
    pub command_line: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TensorFlowSettings {
    #[serde(rename = "pythonScriptFilePath")]
    pub python_script_file_path: String,
    #[serde(rename = "pythonInterpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub python_interpreter_path: Option<String>,
    #[serde(rename = "masterCommandLineArgs")]
    pub master_command_line_args: String,
    #[serde(rename = "workerCommandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub worker_command_line_args: Option<String>,
    #[serde(rename = "parameterServerCommandLineArgs", default, skip_serializing_if = "Option::is_none")]
    pub parameter_server_command_line_args: Option<String>,
    #[serde(rename = "workerCount", default, skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    #[serde(rename = "parameterServerCount", default, skip_serializing_if = "Option::is_none")]
    pub parameter_server_count: Option<i32>,
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
pub struct BatchAiError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<NameValuePair>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
