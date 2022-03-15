//! The data types sent to and returned from the API client.
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/**
 * The type of an individual datum of a metric.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DatumType {
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "Bytes")]
    Bytes,
    #[serde(rename = "CumulativeF64")]
    CumulativeF64,
    #[serde(rename = "CumulativeI64")]
    CumulativeI64,
    #[serde(rename = "F64")]
    F64,
    #[serde(rename = "HistogramF64")]
    HistogramF64,
    #[serde(rename = "HistogramI64")]
    HistogramI64,
    #[serde(rename = "I64")]
    I64,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DatumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DatumType::Bool => "Bool",
            DatumType::Bytes => "Bytes",
            DatumType::CumulativeF64 => "CumulativeF64",
            DatumType::CumulativeI64 => "CumulativeI64",
            DatumType::F64 => "F64",
            DatumType::HistogramF64 => "HistogramF64",
            DatumType::HistogramI64 => "HistogramI64",
            DatumType::I64 => "I64",
            DatumType::String => "String",
            DatumType::Noop => "",
            DatumType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DatumType {
    fn default() -> DatumType {
        DatumType::Noop
    }
}
impl DatumType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DatumType::Noop)
    }
}

/// Client view of an [`Disk`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Disk {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub device_path: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
     * Client view of an [`Disk`]
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub snapshot_id: String,
    #[serde()]
    pub state: DiskState,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
     * Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub snapshot_id: String,
}

/// Parameters for the [`Disk`](omicron_common::api::external::Disk) to be attached or detached to an instance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskIdentifier {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub disk: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Disk>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "state", content = "instance")]
pub enum DiskState {
    Creating,
    Detached,
    Attaching(String),
    Attached(String),
    Detaching(String),
    Destroyed,
    Faulted,
}

impl fmt::Display for DiskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiskState::Attached(..) => write!(f, "attached"),
            DiskState::Attaching(..) => write!(f, "attaching"),
            DiskState::Creating => write!(f, "creating"),
            DiskState::Destroyed => write!(f, "destroyed"),
            DiskState::Detached => write!(f, "detached"),
            DiskState::Detaching(..) => write!(f, "detaching"),
            DiskState::Faulted => write!(f, "faulted"),
        }
    }
}

/// Error information from a response.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Error {
    /**
     * Error information from a response.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_code: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_id: String,
}

/// The name and type information for a field of a timeseries schema.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FieldSchema {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The source from which a field is derived, the target or metric.
     */
    #[serde()]
    pub source: FieldSource,
    /**
     * The `FieldType` identifies the data type of a target or metric field.
     */
    #[serde()]
    pub ty: FieldType,
}

/**
 * The source from which a field is derived, the target or metric.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FieldSource {
    #[serde(rename = "Metric")]
    Metric,
    #[serde(rename = "Target")]
    Target,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FieldSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FieldSource::Metric => "Metric",
            FieldSource::Target => "Target",
            FieldSource::Noop => "",
            FieldSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FieldSource {
    fn default() -> FieldSource {
        FieldSource::Noop
    }
}
impl FieldSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, FieldSource::Noop)
    }
}

/**
 * The `FieldType` identifies the data type of a target or metric field.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FieldType {
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "I64")]
    I64,
    #[serde(rename = "IpAddr")]
    IpAddr,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Uuid")]
    Uuid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FieldType::Bool => "Bool",
            FieldType::I64 => "I64",
            FieldType::IpAddr => "IpAddr",
            FieldType::String => "String",
            FieldType::Uuid => "Uuid",
            FieldType::Noop => "",
            FieldType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FieldType {
    fn default() -> FieldType {
        FieldType::Noop
    }
}
impl FieldType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FieldType::Noop)
    }
}

/**
 * Supported set of sort modes for scanning by id only.
 *
 *   Currently, we only support scanning in ascending order.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IdSortModeAscending {
    #[serde(rename = "id-ascending")]
    IdAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdSortModeAscending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdSortModeAscending::IdAscending => "id-ascending",
            IdSortModeAscending::Noop => "",
            IdSortModeAscending::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdSortModeAscending {
    fn default() -> IdSortModeAscending {
        IdSortModeAscending::Noop
    }
}
impl IdSortModeAscending {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdSortModeAscending::Noop)
    }
}

/// Client view of an [`Instance`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Instance {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hostname: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub memory: i64,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ncpus: i64,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,
    /**
     * Running state of an Instance (primarily: booted or stopped)
     *
     *  This typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle
     */
    #[serde()]
    pub run_state: InstanceState,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_run_state_updated: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InstanceCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hostname: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub memory: i64,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ncpus: i64,
    /**
     * Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<InstanceNetworkInterfaceAttachment>,
}

/// Migration parameters for an [`Instance`](omicron_common::api::external::Instance)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InstanceMigrate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dst_sled_uuid: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "params")]
pub enum InstanceNetworkInterfaceAttachment {
    Create(InstanceNetworkInterfaceCreate),
    Default,
    None,
}

impl fmt::Display for InstanceNetworkInterfaceAttachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstanceNetworkInterfaceAttachment::Create(..) => write!(f, "Create"),
            InstanceNetworkInterfaceAttachment::Default => write!(f, "Default"),
            InstanceNetworkInterfaceAttachment::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InstanceNetworkInterfaceCreate {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub params: Vec<NetworkInterfaceCreate>,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InstanceResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Instance>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/**
 * Running state of an Instance (primarily: booted or stopped)
 *
 *   This typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InstanceState {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "destroyed")]
    Destroyed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "rebooting")]
    Rebooting,
    #[serde(rename = "repairing")]
    Repairing,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InstanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InstanceState::Creating => "creating",
            InstanceState::Destroyed => "destroyed",
            InstanceState::Failed => "failed",
            InstanceState::Migrating => "migrating",
            InstanceState::Rebooting => "rebooting",
            InstanceState::Repairing => "repairing",
            InstanceState::Running => "running",
            InstanceState::Starting => "starting",
            InstanceState::Stopped => "stopped",
            InstanceState::Stopping => "stopping",
            InstanceState::Noop => "",
            InstanceState::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InstanceState {
    fn default() -> InstanceState {
        InstanceState::Noop
    }
}
impl InstanceState {
    pub fn is_noop(&self) -> bool {
        matches!(self, InstanceState::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IpNet {
    V4(String),
    V6(String),
}

impl fmt::Display for IpNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpNet::V4(..) => write!(f, "V4"),
            IpNet::V6(..) => write!(f, "V6"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LoginParams {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

/**
 * Supported set of sort modes for scanning by name or id
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NameSortMode {
    #[serde(rename = "id-ascending")]
    IdAscending,
    #[serde(rename = "name-ascending")]
    NameAscending,
    #[serde(rename = "name-descending")]
    NameDescending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NameSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameSortMode::IdAscending => "id-ascending",
            NameSortMode::NameAscending => "name-ascending",
            NameSortMode::NameDescending => "name-descending",
            NameSortMode::Noop => "",
            NameSortMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NameSortMode {
    fn default() -> NameSortMode {
        NameSortMode::Noop
    }
}
impl NameSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameSortMode::Noop)
    }
}

/**
 * Supported set of sort modes for scanning by name only
 *
 *   Currently, we only support scanning in ascending order.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NameSortModeAscending {
    #[serde(rename = "name-ascending")]
    NameAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NameSortModeAscending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameSortModeAscending::NameAscending => "name-ascending",
            NameSortModeAscending::Noop => "",
            NameSortModeAscending::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NameSortModeAscending {
    fn default() -> NameSortModeAscending {
        NameSortModeAscending::Noop
    }
}
impl NameSortModeAscending {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameSortModeAscending::Noop)
    }
}

/// A `NetworkInterface` represents a virtual network interface device.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkInterface {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance_id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mac: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subnet_id: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

/// Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkInterfaceCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subnet_name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkInterfaceResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<NetworkInterface>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Client view of an [`Rack`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Rack {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`VpcRouter`](omicron_common::api::external::VpcRouter)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrganizationResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Rack>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Updateable properties of an [`Organization`](crate::external_api::views::Organization)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrganizationUpdate {
    /**
     * Updateable properties of an [`Organization`](crate::external_api::views::Organization)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Client view of a [`Project`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Project {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_id: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProjectResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Project>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Updateable properties of a [`Project`](crate::external_api::views::Project)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProjectUpdate {
    /**
     * Updateable properties of a [`Project`](crate::external_api::views::Project)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RackResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Rack>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Client view of a [`Role`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Role {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RoleResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Role>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum RouteDestination {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
}

impl fmt::Display for RouteDestination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RouteDestination::Ip(..) => write!(f, "ip"),
            RouteDestination::IpNet(..) => write!(f, "ip_net"),
            RouteDestination::Subnet(..) => write!(f, "subnet"),
            RouteDestination::Vpc(..) => write!(f, "vpc"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum RouteTarget {
    Ip(String),
    Vpc(String),
    Subnet(String),
    Instance(String),
    InternetGateway(String),
}

impl fmt::Display for RouteTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RouteTarget::Instance(..) => write!(f, "instance"),
            RouteTarget::InternetGateway(..) => write!(f, "internet_gateway"),
            RouteTarget::Ip(..) => write!(f, "ip"),
            RouteTarget::Subnet(..) => write!(f, "subnet"),
            RouteTarget::Vpc(..) => write!(f, "vpc"),
        }
    }
}

/// A route defines a rule that governs where traffic should be sent based on its destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Route {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub destination: RouteDestination,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
     *
     *  See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
     */
    #[serde()]
    pub kind: RouteKind,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub router_id: String,
    #[serde()]
    pub target: RouteTarget,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`RouterRoute`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteCreateParams {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub destination: RouteDestination,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde()]
    pub target: RouteTarget,
}

/**
 * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
 *
 *   See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RouteKind {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "vpc_peering")]
    VpcPeering,
    #[serde(rename = "vpc_subnet")]
    Subnet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouteKind::Custom => "custom",
            RouteKind::Default => "default",
            RouteKind::VpcPeering => "vpc_peering",
            RouteKind::Subnet => "vpc_subnet",
            RouteKind::Noop => "",
            RouteKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RouteKind {
    fn default() -> RouteKind {
        RouteKind::Noop
    }
}
impl RouteKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, RouteKind::Noop)
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Route>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Updateable properties of a [`RouterRoute`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteUpdateParams {
    /**
     * Updateable properties of a [`RouterRoute`]
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub destination: RouteDestination,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde()]
    pub target: RouteTarget,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Saga {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde()]
    pub state: SagaState,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "error", content = "message")]
pub enum SagaErrorInfo {
    ActionFailed(serde_json::Value),
    DeserializeFailed(String),
    InjectedError,
    SerializeFailed(String),
    SubsagaCreateFailed(String),
}

impl fmt::Display for SagaErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SagaErrorInfo::ActionFailed(..) => write!(f, "action_failed"),
            SagaErrorInfo::DeserializeFailed(..) => write!(f, "deserialize_failed"),
            SagaErrorInfo::InjectedError => write!(f, "injected_error"),
            SagaErrorInfo::SerializeFailed(..) => write!(f, "serialize_failed"),
            SagaErrorInfo::SubsagaCreateFailed(..) => write!(f, "subsaga_create_failed"),
        }
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Saga>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "state")]
pub enum SagaState {
    Running,
    Succeeded,
    Failed {
        error_info: SagaErrorInfo,
        error_node_name: String,
    },
}

impl fmt::Display for SagaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SagaState::Failed { .. } => write!(f, "failed"),
            SagaState::Running => write!(f, "running"),
            SagaState::Succeeded => write!(f, "succeeded"),
        }
    }
}

/// Client view of currently authed user.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SessionUser {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// Client view of an [`Sled`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Sled {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub service_address: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SledResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Sled>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// The schema for a timeseries.
///
/// This includes the name of the timeseries, as well as the datum type of its metric and the schema for each field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeseriesSchema {
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The type of an individual datum of a metric.
     */
    #[serde()]
    pub datum_type: DatumType,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub field_schema: Vec<FieldSchema>,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeseries_name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeseriesSchemaResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<TimeseriesSchema>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Rack>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Client view of a [`Vpc`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Vpc {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dns_name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv6_prefix"
    )]
    pub ipv_6_prefix: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub system_router_id: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dns_name: String,
    /**
     * The IPv6 prefix for this VPC.
     *
     *  All IPv6 subnets created from this VPC must be taken from this range, which sould be a Unique Local Address in the range `fd00::/48`. The default VPC Subnet will have the first `/64` range from this prefix.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv6_prefix"
    )]
    pub ipv_6_prefix: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FirewallRule {
    #[serde()]
    pub action: FirewallRuleAction,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub direction: FirewallRuleDirection,
    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    pub filters: FirewallRuleFilter,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde()]
    pub status: FirewallRuleStatus,
    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub targets: Vec<FirewallRuleTarget>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FirewallRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FirewallRuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FirewallRuleAction::Allow => "allow",
            FirewallRuleAction::Deny => "deny",
            FirewallRuleAction::Noop => "",
            FirewallRuleAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FirewallRuleAction {
    fn default() -> FirewallRuleAction {
        FirewallRuleAction::Noop
    }
}
impl FirewallRuleAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, FirewallRuleAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FirewallRuleDirection {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FirewallRuleDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FirewallRuleDirection::Inbound => "inbound",
            FirewallRuleDirection::Outbound => "outbound",
            FirewallRuleDirection::Noop => "",
            FirewallRuleDirection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FirewallRuleDirection {
    fn default() -> FirewallRuleDirection {
        FirewallRuleDirection::Noop
    }
}
impl FirewallRuleDirection {
    pub fn is_noop(&self) -> bool {
        matches!(self, FirewallRuleDirection::Noop)
    }
}

/// Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FirewallRuleFilter {
    /**
     * If present, the sources (if incoming) or destinations (if outgoing) this rule applies to.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hosts: Vec<FirewallRuleHostFilter>,
    /**
     * If present, the destination ports this rule applies to.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ports: Vec<String>,
    /**
     * If present, the networking protocols this rule applies to.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub protocols: Vec<FirewallRuleProtocol>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum FirewallRuleHostFilter {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
    Instance(String),
}

impl fmt::Display for FirewallRuleHostFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FirewallRuleHostFilter::Instance(..) => write!(f, "instance"),
            FirewallRuleHostFilter::Ip(..) => write!(f, "ip"),
            FirewallRuleHostFilter::IpNet(..) => write!(f, "ip_net"),
            FirewallRuleHostFilter::Subnet(..) => write!(f, "subnet"),
            FirewallRuleHostFilter::Vpc(..) => write!(f, "vpc"),
        }
    }
}

/**
 * The protocols that may be specified in a firewall rule's filter
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FirewallRuleProtocol {
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FirewallRuleProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FirewallRuleProtocol::Icmp => "ICMP",
            FirewallRuleProtocol::Tcp => "TCP",
            FirewallRuleProtocol::Udp => "UDP",
            FirewallRuleProtocol::Noop => "",
            FirewallRuleProtocol::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FirewallRuleProtocol {
    fn default() -> FirewallRuleProtocol {
        FirewallRuleProtocol::Noop
    }
}
impl FirewallRuleProtocol {
    pub fn is_noop(&self) -> bool {
        matches!(self, FirewallRuleProtocol::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FirewallRuleStatus {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FirewallRuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FirewallRuleStatus::Disabled => "disabled",
            FirewallRuleStatus::Enabled => "enabled",
            FirewallRuleStatus::Noop => "",
            FirewallRuleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FirewallRuleStatus {
    fn default() -> FirewallRuleStatus {
        FirewallRuleStatus::Noop
    }
}
impl FirewallRuleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, FirewallRuleStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum FirewallRuleTarget {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
    Instance(String),
}

impl fmt::Display for FirewallRuleTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FirewallRuleTarget::Instance(..) => write!(f, "instance"),
            FirewallRuleTarget::Ip(..) => write!(f, "ip"),
            FirewallRuleTarget::IpNet(..) => write!(f, "ip_net"),
            FirewallRuleTarget::Subnet(..) => write!(f, "subnet"),
            FirewallRuleTarget::Vpc(..) => write!(f, "vpc"),
        }
    }
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FirewallRuleUpdate {
    #[serde()]
    pub action: FirewallRuleAction,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub direction: FirewallRuleDirection,
    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    pub filters: FirewallRuleFilter,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A count of bytes, typically used either for memory or storage capacity
     *
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde()]
    pub status: FirewallRuleStatus,
    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub targets: Vec<FirewallRuleTarget>,
}

/// Updateable properties of a `Vpc`'s firewall Note that VpcFirewallRules are implicitly created along with a Vpc, so there is no explicit creation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FirewallRuleUpdateParams {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rules: Vec<FirewallRuleUpdate>,
}

/// Collection of a [`Vpc`]'s firewall rules
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FirewallRules {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rules: Vec<FirewallRule>,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Vpc>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// A VPC router defines a series of rules that indicate where traffic should be sent depending on its destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Router {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde()]
    pub kind: RouterKind,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RouterKind {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RouterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouterKind::Custom => "custom",
            RouterKind::System => "system",
            RouterKind::Noop => "",
            RouterKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RouterKind {
    fn default() -> RouterKind {
        RouterKind::Noop
    }
}
impl RouterKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, RouterKind::Noop)
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Router>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Updateable properties of a [`VpcRouter`](omicron_common::api::external::VpcRouter)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterUpdate {
    /**
     * Updateable properties of a [`VpcRouter`](omicron_common::api::external::VpcRouter)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A VPC subnet represents a logical grouping for instances that allows network traffic between them, within a IPv4 subnetwork or optionall an IPv6 subnetwork.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Subnet {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv4_block: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_block: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

/// Create-time parameters for a [`VpcSubnet`](crate::external_api::views::VpcSubnet)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubnetCreate {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv4_block: String,
    /**
     * The IPv6 prefix for this VPC.
     *
     *  All IPv6 subnets created from this VPC must be taken from this range, which sould be a Unique Local Address in the range `fd00::/48`. The default VPC Subnet will have the first `/64` range from this prefix.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_block: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubnetResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Subnet>,
    /**
     * A single page of results
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Updateable properties of a [`VpcSubnet`](crate::external_api::views::VpcSubnet)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubnetUpdate {
    /**
     * Updateable properties of a [`VpcSubnet`](crate::external_api::views::VpcSubnet)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv4_block: String,
    /**
     * The IPv6 prefix for this VPC.
     *
     *  All IPv6 subnets created from this VPC must be taken from this range, which sould be a Unique Local Address in the range `fd00::/48`. The default VPC Subnet will have the first `/64` range from this prefix.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_block: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcUpdate {
    /**
     * Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dns_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}
