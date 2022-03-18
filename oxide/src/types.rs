//! The data types sent to and returned from the API client.
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

/**
 * The type of an individual datum of a metric.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
        DatumType::Bool
    }
}
impl std::str::FromStr for DatumType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Bool" {
            return Ok(DatumType::Bool);
        }
        if s == "Bytes" {
            return Ok(DatumType::Bytes);
        }
        if s == "CumulativeF64" {
            return Ok(DatumType::CumulativeF64);
        }
        if s == "CumulativeI64" {
            return Ok(DatumType::CumulativeI64);
        }
        if s == "F64" {
            return Ok(DatumType::F64);
        }
        if s == "HistogramF64" {
            return Ok(DatumType::HistogramF64);
        }
        if s == "HistogramI64" {
            return Ok(DatumType::HistogramI64);
        }
        if s == "I64" {
            return Ok(DatumType::I64);
        }
        if s == "String" {
            return Ok(DatumType::String);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl DatumType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DatumType::Noop)
    }
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for DiskState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Client view of an [`Disk`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Disk {
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
    pub project_id: String,

    /**
     * A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(default)]
    pub size: u64,

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
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DiskCreate {
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
    pub description: String,

    /**
     * A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     */
    #[serde(default)]
    pub size: u64,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DiskResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

/// Error information from a response.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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

/**
 * The source from which a field is derived, the target or metric.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
        FieldSource::Metric
    }
}
impl std::str::FromStr for FieldSource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Metric" {
            return Ok(FieldSource::Metric);
        }
        if s == "Target" {
            return Ok(FieldSource::Target);
        }
        anyhow::bail!("invalid string: {}", s);
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
        FieldType::Bool
    }
}
impl std::str::FromStr for FieldType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Bool" {
            return Ok(FieldType::Bool);
        }
        if s == "I64" {
            return Ok(FieldType::I64);
        }
        if s == "IpAddr" {
            return Ok(FieldType::IpAddr);
        }
        if s == "String" {
            return Ok(FieldType::String);
        }
        if s == "Uuid" {
            return Ok(FieldType::Uuid);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl FieldType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FieldType::Noop)
    }
}

/// The name and type information for a field of a timeseries schema.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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
    #[serde(default, skip_serializing_if = "FieldSource::is_noop")]
    pub source: FieldSource,

    /**
     * The `FieldType` identifies the data type of a target or metric field.
     */
    #[serde(default, skip_serializing_if = "FieldType::is_noop")]
    pub ty: FieldType,
}

/**
 * Supported set of sort modes for scanning by id only.
 *   
 *   Currently, we only support scanning in ascending order.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum IdSortMode {
    #[serde(rename = "id-ascending")]
    IdAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdSortMode::IdAscending => "id-ascending",
            IdSortMode::Noop => "",
            IdSortMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdSortMode {
    fn default() -> IdSortMode {
        IdSortMode::IdAscending
    }
}
impl std::str::FromStr for IdSortMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "id-ascending" {
            return Ok(IdSortMode::IdAscending);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl IdSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdSortMode::Noop)
    }
}

/**
 * Running state of an Instance (primarily: booted or stopped)
 *   
 *   This typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
        InstanceState::Creating
    }
}
impl std::str::FromStr for InstanceState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "creating" {
            return Ok(InstanceState::Creating);
        }
        if s == "destroyed" {
            return Ok(InstanceState::Destroyed);
        }
        if s == "failed" {
            return Ok(InstanceState::Failed);
        }
        if s == "migrating" {
            return Ok(InstanceState::Migrating);
        }
        if s == "rebooting" {
            return Ok(InstanceState::Rebooting);
        }
        if s == "repairing" {
            return Ok(InstanceState::Repairing);
        }
        if s == "running" {
            return Ok(InstanceState::Running);
        }
        if s == "starting" {
            return Ok(InstanceState::Starting);
        }
        if s == "stopped" {
            return Ok(InstanceState::Stopped);
        }
        if s == "stopping" {
            return Ok(InstanceState::Stopping);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl InstanceState {
    pub fn is_noop(&self) -> bool {
        matches!(self, InstanceState::Noop)
    }
}

/// Client view of an [`Instance`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Instance {
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
    #[serde(default)]
    pub memory: u64,

    /**
     * The number of CPUs in an Instance
     */
    #[serde()]
    pub ncpus: u16,

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
    #[serde(default, skip_serializing_if = "InstanceState::is_noop")]
    pub run_state: InstanceState,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_run_state_updated: crate::utils::DisplayOptionDateTime,
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for InstanceNetworkInterfaceAttachment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InstanceCreate {
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
    #[serde(default)]
    pub memory: u64,

    /**
     * The number of CPUs in an Instance
     */
    #[serde()]
    pub ncpus: u16,

    /**
     * Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[header(hidden = true)]
    pub network_interfaces: Option<InstanceNetworkInterfaceAttachment>,
}

/// Migration parameters for an [`Instance`](omicron_common::api::external::Instance)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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

/// Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct NetworkInterfaceCreate {
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InstanceNetworkInterfaceCreate {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub params: Vec<NetworkInterfaceCreate>,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InstanceResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IpNet {
    V4(String),
    V6(String),
}

impl fmt::Display for IpNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for IpNet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum NameOrIdSortMode {
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

impl std::fmt::Display for NameOrIdSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameOrIdSortMode::IdAscending => "id-ascending",
            NameOrIdSortMode::NameAscending => "name-ascending",
            NameOrIdSortMode::NameDescending => "name-descending",
            NameOrIdSortMode::Noop => "",
            NameOrIdSortMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NameOrIdSortMode {
    fn default() -> NameOrIdSortMode {
        NameOrIdSortMode::IdAscending
    }
}
impl std::str::FromStr for NameOrIdSortMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "id-ascending" {
            return Ok(NameOrIdSortMode::IdAscending);
        }
        if s == "name-ascending" {
            return Ok(NameOrIdSortMode::NameAscending);
        }
        if s == "name-descending" {
            return Ok(NameOrIdSortMode::NameDescending);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl NameOrIdSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameOrIdSortMode::Noop)
    }
}

/**
 * Supported set of sort modes for scanning by name only
 *   
 *   Currently, we only support scanning in ascending order.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum NameSortMode {
    #[serde(rename = "name-ascending")]
    NameAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NameSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameSortMode::NameAscending => "name-ascending",
            NameSortMode::Noop => "",
            NameSortMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NameSortMode {
    fn default() -> NameSortMode {
        NameSortMode::NameAscending
    }
}
impl std::str::FromStr for NameSortMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "name-ascending" {
            return Ok(NameSortMode::NameAscending);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl NameSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameSortMode::Noop)
    }
}

/// A `NetworkInterface` represents a virtual network interface device.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct NetworkInterface {
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
    pub description: String,

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
    pub subnet_id: String,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

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

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct NetworkInterfaceResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

/// Client view of an [`Organization`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Organization {
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
    pub description: String,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for an [`Organization`](crate::external_api::views::Organization)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationCreate {
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
    pub description: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<Organization>,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationUpdate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
     * Updateable properties of an [`Organization`](crate::external_api::views::Organization)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
}

/// Client view of a [`Project`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Project {
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
    pub description: String,

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
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for an [`Organization`](crate::external_api::views::Organization)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectCreate {
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
    pub description: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectUpdate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
     * Updateable properties of a [`Project`](crate::external_api::views::Project)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
}

/// Client view of an [`Organization`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Rack {
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
    pub description: String,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct RackResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Role {
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
    pub description: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct RoleResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for RouteDestination {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for RouteTarget {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/**
 * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
 *   
 *   See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum RouterRouteKind {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "vpc_peering")]
    VpcPeering,
    #[serde(rename = "vpc_subnet")]
    VpcSubnet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RouterRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouterRouteKind::Custom => "custom",
            RouterRouteKind::Default => "default",
            RouterRouteKind::VpcPeering => "vpc_peering",
            RouterRouteKind::VpcSubnet => "vpc_subnet",
            RouterRouteKind::Noop => "",
            RouterRouteKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RouterRouteKind {
    fn default() -> RouterRouteKind {
        RouterRouteKind::Custom
    }
}
impl std::str::FromStr for RouterRouteKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "custom" {
            return Ok(RouterRouteKind::Custom);
        }
        if s == "default" {
            return Ok(RouterRouteKind::Default);
        }
        if s == "vpc_peering" {
            return Ok(RouterRouteKind::VpcPeering);
        }
        if s == "vpc_subnet" {
            return Ok(RouterRouteKind::VpcSubnet);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl RouterRouteKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, RouterRouteKind::Noop)
    }
}

/// A route defines a rule that governs where traffic should be sent based on its destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct RouterRoute {
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
    pub description: String,

    #[serde()]
    pub destination: RouteDestination,

    /**
     * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
     *  
     *  See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
     */
    #[serde(default, skip_serializing_if = "RouterRouteKind::is_noop")]
    pub kind: RouterRouteKind,

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
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`RouterRoute`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct RouterRouteCreateParams {
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
    pub description: String,

    #[serde()]
    pub destination: RouteDestination,

    #[serde()]
    pub target: RouteTarget,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct RouterRouteResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<RouterRoute>,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct RouterRouteUpdateParams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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

    #[serde()]
    pub target: RouteTarget,
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for SagaState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for SagaErrorInfo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SagaResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

/// Client view of currently authed user.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Sled {
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
    pub description: String,

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
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SledResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

/// Client view of a Snapshot
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Snapshot {
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
    pub description: String,

    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub disk_id: String,

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
    #[serde(default)]
    pub size: u64,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Snapshot`](omicron_common::api::external::Snapshot)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SnapshotCreate {
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
    pub description: String,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SnapshotResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<Snapshot>,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct TimeseriesSchema {
    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub created: crate::utils::DisplayOptionDateTime,

    /**
     * The type of an individual datum of a metric.
     */
    #[serde(default, skip_serializing_if = "DatumType::is_noop")]
    pub datum_type: DatumType,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct TimeseriesSchemaResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

/// Client view of an [`Organization`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct User {
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
    pub description: String,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct UserResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<User>,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Vpc {
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
    pub ipv6_prefix: String,

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
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcCreate {
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_prefix: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VpcFirewallRuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleAction::Allow => "allow",
            VpcFirewallRuleAction::Deny => "deny",
            VpcFirewallRuleAction::Noop => "",
            VpcFirewallRuleAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleAction {
    fn default() -> VpcFirewallRuleAction {
        VpcFirewallRuleAction::Allow
    }
}
impl std::str::FromStr for VpcFirewallRuleAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "allow" {
            return Ok(VpcFirewallRuleAction::Allow);
        }
        if s == "deny" {
            return Ok(VpcFirewallRuleAction::Deny);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl VpcFirewallRuleAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleDirection {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VpcFirewallRuleDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleDirection::Inbound => "inbound",
            VpcFirewallRuleDirection::Outbound => "outbound",
            VpcFirewallRuleDirection::Noop => "",
            VpcFirewallRuleDirection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleDirection {
    fn default() -> VpcFirewallRuleDirection {
        VpcFirewallRuleDirection::Inbound
    }
}
impl std::str::FromStr for VpcFirewallRuleDirection {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "inbound" {
            return Ok(VpcFirewallRuleDirection::Inbound);
        }
        if s == "outbound" {
            return Ok(VpcFirewallRuleDirection::Outbound);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl VpcFirewallRuleDirection {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleDirection::Noop)
    }
}

/// Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct VpcFirewallRuleFilter {
    /**
     * If present, the sources (if incoming) or destinations (if outgoing) this rule applies to.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hosts: Vec<VpcFirewallRuleHostFilter>,

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
    pub protocols: Vec<VpcFirewallRuleProtocol>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleStatus {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VpcFirewallRuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleStatus::Disabled => "disabled",
            VpcFirewallRuleStatus::Enabled => "enabled",
            VpcFirewallRuleStatus::Noop => "",
            VpcFirewallRuleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleStatus {
    fn default() -> VpcFirewallRuleStatus {
        VpcFirewallRuleStatus::Disabled
    }
}
impl std::str::FromStr for VpcFirewallRuleStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "disabled" {
            return Ok(VpcFirewallRuleStatus::Disabled);
        }
        if s == "enabled" {
            return Ok(VpcFirewallRuleStatus::Enabled);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl VpcFirewallRuleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleTarget {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
    Instance(String),
}

impl fmt::Display for VpcFirewallRuleTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for VpcFirewallRuleTarget {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcFirewallRule {
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
    pub description: String,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleAction::is_noop")]
    pub action: VpcFirewallRuleAction,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleDirection::is_noop")]
    pub direction: VpcFirewallRuleDirection,

    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    #[header(hidden = true)]
    pub filters: VpcFirewallRuleFilter,

    /**
     * The number of CPUs in an Instance
     */
    #[serde()]
    pub priority: u16,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleStatus::is_noop")]
    pub status: VpcFirewallRuleStatus,

    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub targets: Vec<VpcFirewallRuleTarget>,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

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
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleHostFilter {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
    Instance(String),
}

impl fmt::Display for VpcFirewallRuleHostFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for VpcFirewallRuleHostFilter {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/**
 * The protocols that may be specified in a firewall rule's filter
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleProtocol {
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

impl std::fmt::Display for VpcFirewallRuleProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleProtocol::Icmp => "ICMP",
            VpcFirewallRuleProtocol::Tcp => "TCP",
            VpcFirewallRuleProtocol::Udp => "UDP",
            VpcFirewallRuleProtocol::Noop => "",
            VpcFirewallRuleProtocol::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleProtocol {
    fn default() -> VpcFirewallRuleProtocol {
        VpcFirewallRuleProtocol::Icmp
    }
}
impl std::str::FromStr for VpcFirewallRuleProtocol {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ICMP" {
            return Ok(VpcFirewallRuleProtocol::Icmp);
        }
        if s == "TCP" {
            return Ok(VpcFirewallRuleProtocol::Tcp);
        }
        if s == "UDP" {
            return Ok(VpcFirewallRuleProtocol::Udp);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl VpcFirewallRuleProtocol {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleProtocol::Noop)
    }
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcFirewallRuleUpdate {
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
    pub description: String,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleAction::is_noop")]
    pub action: VpcFirewallRuleAction,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleDirection::is_noop")]
    pub direction: VpcFirewallRuleDirection,

    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    #[header(hidden = true)]
    pub filters: VpcFirewallRuleFilter,

    /**
     * The number of CPUs in an Instance
     */
    #[serde()]
    pub priority: u16,

    #[serde(default, skip_serializing_if = "VpcFirewallRuleStatus::is_noop")]
    pub status: VpcFirewallRuleStatus,

    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub targets: Vec<VpcFirewallRuleTarget>,
}

/// Updateable properties of a `Vpc`'s firewall Note that VpcFirewallRules are implicitly created along with a Vpc, so there is no explicit creation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcFirewallRuleUpdateParams {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub rules: Vec<VpcFirewallRuleUpdate>,
}

/// Collection of a [`Vpc`]'s firewall rules
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcFirewallRules {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub rules: Vec<VpcFirewallRule>,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcRouterKind {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VpcRouterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcRouterKind::Custom => "custom",
            VpcRouterKind::System => "system",
            VpcRouterKind::Noop => "",
            VpcRouterKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VpcRouterKind {
    fn default() -> VpcRouterKind {
        VpcRouterKind::Custom
    }
}
impl std::str::FromStr for VpcRouterKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "custom" {
            return Ok(VpcRouterKind::Custom);
        }
        if s == "system" {
            return Ok(VpcRouterKind::System);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl VpcRouterKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcRouterKind::Noop)
    }
}

/// A VPC router defines a series of rules that indicate where traffic should be sent depending on its destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcRouter {
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
    pub description: String,

    #[serde(default, skip_serializing_if = "VpcRouterKind::is_noop")]
    pub kind: VpcRouterKind,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

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

/// Create-time parameters for an [`Organization`](crate::external_api::views::Organization)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcRouterCreate {
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
    pub description: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcRouterResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<VpcRouter>,

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

/// Updateable properties of a [`VpcRouter`](crate::external_api::views::VpcRouter)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcRouterUpdate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
     * Updateable properties of a [`VpcRouter`](crate::external_api::views::VpcRouter)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
}

/// A VPC subnet represents a logical grouping for instances that allows network traffic between them, within a IPv4 subnetwork or optionall an IPv6 subnetwork.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcSubnet {
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
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_block: String,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,

    /**
     * timestamp when this resource was created
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcSubnetCreate {
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
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcSubnetResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<VpcSubnet>,

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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcSubnetUpdate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
}

/// Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcUpdate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
}
