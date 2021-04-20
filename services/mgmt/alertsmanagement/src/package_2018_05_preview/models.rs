#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsManagementErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[serde(skip_serializing)]
    pub severity: Option<alert_properties::Severity>,
    #[serde(rename = "signalType", skip_serializing)]
    pub signal_type: Option<alert_properties::SignalType>,
    #[serde(rename = "alertState", skip_serializing)]
    pub alert_state: Option<alert_properties::AlertState>,
    #[serde(rename = "monitorCondition", skip_serializing)]
    pub monitor_condition: Option<alert_properties::MonitorCondition>,
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
    #[serde(rename = "targetResourceName", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_name: Option<String>,
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(rename = "monitorService", skip_serializing)]
    pub monitor_service: Option<alert_properties::MonitorService>,
    #[serde(rename = "sourceCreatedId", skip_serializing)]
    pub source_created_id: Option<String>,
    #[serde(rename = "smartGroupId", skip_serializing)]
    pub smart_group_id: Option<String>,
    #[serde(rename = "smartGroupingReason", skip_serializing)]
    pub smart_grouping_reason: Option<String>,
    #[serde(rename = "startDateTime", skip_serializing)]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", skip_serializing)]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "lastModifiedUserName", skip_serializing)]
    pub last_modified_user_name: Option<String>,
    #[serde(skip_serializing)]
    pub payload: Option<serde_json::Value>,
}
pub mod alert_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignalType {
        Metric,
        Log,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertState {
        New,
        Acknowledged,
        Closed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorCondition {
        Fired,
        Resolved,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorService {
        Platform,
        #[serde(rename = "Application Insights")]
        ApplicationInsights,
        #[serde(rename = "Log Analytics")]
        LogAnalytics,
        #[serde(rename = "Infrastructure Insights")]
        InfrastructureInsights,
        #[serde(rename = "ActivityLog Administrative")]
        ActivityLogAdministrative,
        #[serde(rename = "ActivityLog Security")]
        ActivityLogSecurity,
        #[serde(rename = "ActivityLog Recommendation")]
        ActivityLogRecommendation,
        #[serde(rename = "ActivityLog Policy")]
        ActivityLogPolicy,
        #[serde(rename = "ActivityLog Autoscale")]
        ActivityLogAutoscale,
        ServiceHealth,
        SmartDetector,
        Zabbix,
        #[serde(rename = "SCOM")]
        Scom,
        Nagios,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertModificationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModificationProperties {
    #[serde(rename = "alertId", skip_serializing)]
    pub alert_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<AlertModificationItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModificationItem {
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<alert_modification_item::ModificationEvent>,
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod alert_modification_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        AlertCreated,
        StateChange,
        MonitorConditionChange,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupModificationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModificationProperties {
    #[serde(rename = "smartGroupId", skip_serializing)]
    pub smart_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<SmartGroupModificationItem>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModificationItem {
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<smart_group_modification_item::ModificationEvent>,
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod smart_group_modification_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        SmartGroupCreated,
        StateChange,
        AlertAdded,
        AlertRemoved,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSummaryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "smartGroupsCount", default, skip_serializing_if = "Option::is_none")]
    pub smart_groups_count: Option<i64>,
    #[serde(rename = "summaryByState", default, skip_serializing_if = "Option::is_none")]
    pub summary_by_state: Option<serde_json::Value>,
    #[serde(rename = "summaryBySeverity", default, skip_serializing_if = "Option::is_none")]
    pub summary_by_severity: Option<alerts_summary_properties::SummaryBySeverity>,
    #[serde(rename = "summaryBySeverityAndMonitorCondition", default, skip_serializing_if = "Option::is_none")]
    pub summary_by_severity_and_monitor_condition: Option<serde_json::Value>,
    #[serde(rename = "summaryByMonitorService", default, skip_serializing_if = "Option::is_none")]
    pub summary_by_monitor_service: Option<serde_json::Value>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
pub mod alerts_summary_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SummaryBySeverity {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sev0: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sev1: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sev2: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sev3: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sev4: Option<serde_json::Value>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryByState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryBySeverityAndMonitorCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sev0: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sev1: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sev2: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sev3: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sev4: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryByMonitorCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fired: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryByMonitorService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<i64>,
    #[serde(rename = "application Insights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<i64>,
    #[serde(rename = "log Analytics", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zabbix: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scom: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nagios: Option<i64>,
    #[serde(rename = "infrastructure Insights", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_insights: Option<i64>,
    #[serde(rename = "activityLog Administrative", default, skip_serializing_if = "Option::is_none")]
    pub activity_log_administrative: Option<i64>,
    #[serde(rename = "activityLog Security", default, skip_serializing_if = "Option::is_none")]
    pub activity_log_security: Option<i64>,
    #[serde(rename = "activityLog Recommendation", default, skip_serializing_if = "Option::is_none")]
    pub activity_log_recommendation: Option<i64>,
    #[serde(rename = "activityLog Policy", default, skip_serializing_if = "Option::is_none")]
    pub activity_log_policy: Option<i64>,
    #[serde(rename = "activityLog Autoscale", default, skip_serializing_if = "Option::is_none")]
    pub activity_log_autoscale: Option<i64>,
    #[serde(rename = "serviceHealth", default, skip_serializing_if = "Option::is_none")]
    pub service_health: Option<i64>,
    #[serde(rename = "smartDetector", default, skip_serializing_if = "Option::is_none")]
    pub smart_detector: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SmartGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupProperties {
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
    #[serde(rename = "smartGroupState", skip_serializing)]
    pub smart_group_state: Option<smart_group_properties::SmartGroupState>,
    #[serde(skip_serializing)]
    pub severity: Option<smart_group_properties::Severity>,
    #[serde(rename = "startDateTime", skip_serializing)]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", skip_serializing)]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "lastModifiedUserName", skip_serializing)]
    pub last_modified_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "monitorServices", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_services: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "monitorConditions", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_conditions: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "alertStates", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_states: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "alertSeverities", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_severities: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
pub mod smart_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SmartGroupState {
        New,
        Acknowledged,
        Closed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupAggregatedProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
