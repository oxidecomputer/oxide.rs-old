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
#[serde(rename_all = "snake_case")]
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
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["state"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["instance"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["instance"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for DiskState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for DiskState, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "attaching" {
            j = format!(
                r#"{{
"state": "attaching",
"instance": "{}"
        }}"#,
                content
            );
        }
        if tag == "attached" {
            j = format!(
                r#"{{
"state": "attached",
"instance": "{}"
        }}"#,
                content
            );
        }
        if tag == "detaching" {
            j = format!(
                r#"{{
"state": "detaching",
"instance": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl DiskState {
    pub fn variants() -> Vec<String> {
        vec![
            "attached".to_string(),
            "attaching".to_string(),
            "creating".to_string(),
            "destroyed".to_string(),
            "detached".to_string(),
            "detaching".to_string(),
            "faulted".to_string(),
        ]
    }
}
/**
 * The types for DiskState.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum DiskStateType {
    #[serde(rename = "Attached")]
    Attached,
    #[serde(rename = "Attaching")]
    Attaching,
    #[serde(rename = "Creating")]
    Creating,
    #[serde(rename = "Destroyed")]
    Destroyed,
    #[serde(rename = "Detached")]
    Detached,
    #[serde(rename = "Detaching")]
    Detaching,
    #[serde(rename = "Faulted")]
    Faulted,
}

impl std::fmt::Display for DiskStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DiskStateType::Attached => "Attached",
            DiskStateType::Attaching => "Attaching",
            DiskStateType::Creating => "Creating",
            DiskStateType::Destroyed => "Destroyed",
            DiskStateType::Detached => "Detached",
            DiskStateType::Detaching => "Detaching",
            DiskStateType::Faulted => "Faulted",
        }
        .fmt(f)
    }
}

impl Default for DiskStateType {
    fn default() -> DiskStateType {
        DiskStateType::Attached
    }
}
impl std::str::FromStr for DiskStateType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Attached" {
            return Ok(DiskStateType::Attached);
        }
        if s == "Attaching" {
            return Ok(DiskStateType::Attaching);
        }
        if s == "Creating" {
            return Ok(DiskStateType::Creating);
        }
        if s == "Destroyed" {
            return Ok(DiskStateType::Destroyed);
        }
        if s == "Detached" {
            return Ok(DiskStateType::Detached);
        }
        if s == "Detaching" {
            return Ok(DiskStateType::Detaching);
        }
        if s == "Faulted" {
            return Ok(DiskStateType::Faulted);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

/// Client view of an [`Disk`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Disk {
    /**
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub device_path: String,

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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DiskCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * id for snapshot from which the Disk should be created, if any
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * token used to fetch the next page of results (if any)
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

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,

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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * RFC1035-compliant hostname for the Instance.
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
     * id for the project containing this Instance
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    #[serde()]
    pub time_run_state_updated: crate::utils::DisplayOptionDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "params")]
pub enum InstanceNetworkInterfaceAttachment {
    Create(InstanceNetworkInterfaceCreate),
    Default,
    None,
}

impl fmt::Display for InstanceNetworkInterfaceAttachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["params"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["params"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for InstanceNetworkInterfaceAttachment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!(
                "invalid format for InstanceNetworkInterfaceAttachment, got {}",
                s
            );
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "create" {
            j = format!(
                r#"{{
"type": "create",
"params": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl InstanceNetworkInterfaceAttachment {
    pub fn variants() -> Vec<String> {
        vec![
            "create".to_string(),
            "default".to_string(),
            "none".to_string(),
        ]
    }
}
/**
 * The types for InstanceNetworkInterfaceAttachment.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum InstanceNetworkInterfaceAttachmentType {
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "None")]
    None,
}

impl std::fmt::Display for InstanceNetworkInterfaceAttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InstanceNetworkInterfaceAttachmentType::Create => "Create",
            InstanceNetworkInterfaceAttachmentType::Default => "Default",
            InstanceNetworkInterfaceAttachmentType::None => "None",
        }
        .fmt(f)
    }
}

impl Default for InstanceNetworkInterfaceAttachmentType {
    fn default() -> InstanceNetworkInterfaceAttachmentType {
        InstanceNetworkInterfaceAttachmentType::Create
    }
}
impl std::str::FromStr for InstanceNetworkInterfaceAttachmentType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Create" {
            return Ok(InstanceNetworkInterfaceAttachmentType::Create);
        }
        if s == "Default" {
            return Ok(InstanceNetworkInterfaceAttachmentType::Default);
        }
        if s == "None" {
            return Ok(InstanceNetworkInterfaceAttachmentType::None);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

/// Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InstanceCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
     * The IP address for the interface. One will be auto-assigned if not provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subnet_name: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * token used to fetch the next page of results (if any)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// An `IpNet` represents an IP network, either IPv4 or IPv6.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Hash, JsonSchema, Serialize)]
pub enum IpNet {
    V4(Ipv4Net),
    V6(Ipv6Net),
}

impl From<Ipv4Net> for IpNet {
    fn from(n: Ipv4Net) -> IpNet {
        IpNet::V4(n)
    }
}

impl From<std::net::Ipv4Addr> for IpNet {
    fn from(n: std::net::Ipv4Addr) -> IpNet {
        IpNet::V4(Ipv4Net(ipnetwork::Ipv4Network::from(n)))
    }
}

impl From<Ipv6Net> for IpNet {
    fn from(n: Ipv6Net) -> IpNet {
        IpNet::V6(n)
    }
}

impl From<std::net::Ipv6Addr> for IpNet {
    fn from(n: std::net::Ipv6Addr) -> IpNet {
        IpNet::V6(Ipv6Net(ipnetwork::Ipv6Network::from(n)))
    }
}

impl std::fmt::Display for IpNet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpNet::V4(inner) => write!(f, "{}", inner),
            IpNet::V6(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::str::FromStr for IpNet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net = s
            .parse::<ipnetwork::IpNetwork>()
            .map_err(|e| e.to_string())?;
        match net {
            ipnetwork::IpNetwork::V4(net) => Ok(IpNet::from(Ipv4Net(net))),
            ipnetwork::IpNetwork::V6(net) => Ok(IpNet::from(Ipv6Net(net))),
        }
    }
}

/// An `Ipv4Net` represents a IPv4 subnetwork, including the address and network mask.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Ipv4Net(pub ipnetwork::Ipv4Network);

impl Ipv4Net {
    /// Return `true` if this IPv4 subnetwork is from an RFC 1918 private
    /// address space.
    pub fn is_private(&self) -> bool {
        self.0.network().is_private()
    }
}

impl std::ops::Deref for Ipv4Net {
    type Target = ipnetwork::Ipv4Network;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Ipv4Net {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Ipv4Net {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net = s
            .parse::<ipnetwork::IpNetwork>()
            .map_err(|e| e.to_string())?;
        match net {
            ipnetwork::IpNetwork::V4(net) => Ok(Ipv4Net(net)),
            ipnetwork::IpNetwork::V6(..) => Err(format!("IPv6 subnet {} not supported here", s)),
        }
    }
}

impl JsonSchema for Ipv4Net {
    fn schema_name() -> String {
        "Ipv4Net".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(
            schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    title: Some("An IPv4 subnet".to_string()),
                    description: Some("An IPv4 subnet, including prefix and subnet mask".to_string()),
                    examples: vec!["192.168.1.0/24".into()],
                    ..Default::default()
                })),
                instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String))),
                string: Some(Box::new(schemars::schema::StringValidation {
                    // Fully-specified IPv4 address. Up to 15 chars for address, plus slash and up to 2 subnet digits.
                    max_length: Some(18),
                    min_length: None,
                    // Addresses must be from an RFC 1918 private address space
                    pattern: Some(
                        concat!(
                            // 10.x.x.x/8
                            r#"(^(10\.(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9]\.){2}(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9])/(1[0-9]|2[0-8]|[8-9]))$)|"#,
                            // 172.16.x.x/12
                            r#"(^(172\.16\.(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9])\.(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9])/(1[2-9]|2[0-8]))$)|"#,
                            // 192.168.x.x/16
                            r#"(^(192\.168\.(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9])\.(25[0-5]|[1-2][0-4][0-9]|[1-9][0-9]|[0-9])/(1[6-9]|2[0-8]))$)"#,
                        ).to_string(),
                    ),
                })),
                ..Default::default()
            }
        )
    }
}
/// An `Ipv6Net` represents a IPv6 subnetwork, including the address and network mask.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Ipv6Net(pub ipnetwork::Ipv6Network);

impl Ipv6Net {
    /// The length for all VPC IPv6 prefixes
    pub const VPC_IPV6_PREFIX_LENGTH: u8 = 48;

    /// The prefix length for all VPC Sunets
    pub const VPC_SUBNET_IPV6_PREFIX_LENGTH: u8 = 64;

    /// Return `true` if this subnetwork is in the IPv6 Unique Local Address
    /// range defined in RFC 4193, e.g., `fd00:/8`
    pub fn is_unique_local(&self) -> bool {
        // TODO: Delegate to `Ipv6Addr::is_unique_local()` when stabilized.
        self.0.network().octets()[0] == 0xfd
    }

    /// Return `true` if this subnetwork is a valid VPC prefix.
    ///
    /// This checks that the subnet is a unique local address, and has the VPC
    /// prefix length required.
    pub fn is_vpc_prefix(&self) -> bool {
        self.is_unique_local() && self.0.prefix() == Self::VPC_IPV6_PREFIX_LENGTH
    }

    /// Return `true` if this subnetwork is a valid VPC Subnet, given the VPC's
    /// prefix.
    pub fn is_vpc_subnet(&self, vpc_prefix: &Ipv6Net) -> bool {
        self.is_unique_local()
            && self.is_subnet_of(vpc_prefix.0)
            && self.prefix() == Self::VPC_SUBNET_IPV6_PREFIX_LENGTH
    }
}

impl std::ops::Deref for Ipv6Net {
    type Target = ipnetwork::Ipv6Network;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Ipv6Net {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Ipv6Net {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net = s
            .parse::<ipnetwork::IpNetwork>()
            .map_err(|e| e.to_string())?;
        match net {
            ipnetwork::IpNetwork::V4(..) => Err(format!("IPv4 subnet {} not supported here", s)),
            ipnetwork::IpNetwork::V6(net) => Ok(Ipv6Net(net)),
        }
    }
}

impl JsonSchema for Ipv6Net {
    fn schema_name() -> String {
        "Ipv6Net".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(
            schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    title: Some("An IPv6 subnet".to_string()),
                    description: Some("An IPv6 subnet, including prefix and subnet mask".to_string()),
                    examples: vec!["fd12:3456::/64".into()],
                    ..Default::default()
                })),
                instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String))),
                string: Some(Box::new(schemars::schema::StringValidation {
                    // Fully-specified IPv6 address. 4 hex chars per segment, 8 segments, 7
                    // ":"-separators, slash and up to 3 subnet digits
                    max_length: Some(43),
                    min_length: None,
                    pattern: Some(
                        // Conforming to unique local addressing scheme, `fd00::/8`.
                        concat!(
                            r#"^(fd|FD)[0-9a-fA-F]{2}:((([0-9a-fA-F]{1,4}\:){6}[0-9a-fA-F]{1,4})|(([0-9a-fA-F]{1,4}:){1,6}:))/(6[4-9]|[7-9][0-9]|1[0-1][0-9]|12[0-6])$"#,
                        ).to_string(),
                    ),
                })),
                ..Default::default()
            }
        )
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct LoginParams {
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * The Instance to which the interface belongs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance_id: String,

    /**
     * The IP address assigned to this interface.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,

    /**
     * A Media Access Control address, in EUI-48 format
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mac: String,

    /**
     * The subnet to which the interface belongs.
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
     * The VPC to which the interface belongs.
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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for an [`Organization`](crate::external_api::views::Organization)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Project`](crate::external_api::views::Project)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * token used to fetch the next page of results (if any)
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

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
}

/// Client view of an [`Rack`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Rack {
    /**
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
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
     * token used to fetch the next page of results (if any)
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
     * Role names consist of two string components separated by dot (".").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * token used to fetch the next page of results (if any)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "value")]
pub enum RouteDestination {
    Ip(String),
    IpNet(IpNet),
    Vpc(String),
    Subnet(String),
}

impl fmt::Display for RouteDestination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["value"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["value"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for RouteDestination {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for RouteDestination, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "ip" {
            j = format!(
                r#"{{
"type": "ip",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "ip_net" {
            j = format!(
                r#"{{
"type": "ip_net",
"value": {}
        }}"#,
                serde_json::json!(IpNet::from_str(&content).unwrap())
            );
        }
        if tag == "vpc" {
            j = format!(
                r#"{{
"type": "vpc",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "subnet" {
            j = format!(
                r#"{{
"type": "subnet",
"value": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl RouteDestination {
    pub fn variants() -> Vec<String> {
        vec![
            "ip".to_string(),
            "ip_net".to_string(),
            "subnet".to_string(),
            "vpc".to_string(),
        ]
    }
}
/**
 * The types for RouteDestination.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum RouteDestinationType {
    #[serde(rename = "Ip")]
    Ip,
    #[serde(rename = "IpNet")]
    IpNet,
    #[serde(rename = "Subnet")]
    Subnet,
    #[serde(rename = "Vpc")]
    Vpc,
}

impl std::fmt::Display for RouteDestinationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouteDestinationType::Ip => "Ip",
            RouteDestinationType::IpNet => "IpNet",
            RouteDestinationType::Subnet => "Subnet",
            RouteDestinationType::Vpc => "Vpc",
        }
        .fmt(f)
    }
}

impl Default for RouteDestinationType {
    fn default() -> RouteDestinationType {
        RouteDestinationType::Ip
    }
}
impl std::str::FromStr for RouteDestinationType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Ip" {
            return Ok(RouteDestinationType::Ip);
        }
        if s == "IpNet" {
            return Ok(RouteDestinationType::IpNet);
        }
        if s == "Subnet" {
            return Ok(RouteDestinationType::Subnet);
        }
        if s == "Vpc" {
            return Ok(RouteDestinationType::Vpc);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["value"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["value"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for RouteTarget {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for RouteTarget, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "ip" {
            j = format!(
                r#"{{
"type": "ip",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "vpc" {
            j = format!(
                r#"{{
"type": "vpc",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "subnet" {
            j = format!(
                r#"{{
"type": "subnet",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "instance" {
            j = format!(
                r#"{{
"type": "instance",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "inetgw" {
            j = format!(
                r#"{{
"type": "internet_gateway",
"value": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl RouteTarget {
    pub fn variants() -> Vec<String> {
        vec![
            "instance".to_string(),
            "inetgw".to_string(),
            "ip".to_string(),
            "subnet".to_string(),
            "vpc".to_string(),
        ]
    }
}
/**
 * The types for RouteTarget.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum RouteTargetType {
    #[serde(rename = "Instance")]
    Instance,
    #[serde(rename = "InternetGateway")]
    InternetGateway,
    #[serde(rename = "Ip")]
    Ip,
    #[serde(rename = "Subnet")]
    Subnet,
    #[serde(rename = "Vpc")]
    Vpc,
}

impl std::fmt::Display for RouteTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouteTargetType::Instance => "Instance",
            RouteTargetType::InternetGateway => "InternetGateway",
            RouteTargetType::Ip => "Ip",
            RouteTargetType::Subnet => "Subnet",
            RouteTargetType::Vpc => "Vpc",
        }
        .fmt(f)
    }
}

impl Default for RouteTargetType {
    fn default() -> RouteTargetType {
        RouteTargetType::Instance
    }
}
impl std::str::FromStr for RouteTargetType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Instance" {
            return Ok(RouteTargetType::Instance);
        }
        if s == "InternetGateway" {
            return Ok(RouteTargetType::InternetGateway);
        }
        if s == "Ip" {
            return Ok(RouteTargetType::Ip);
        }
        if s == "Subnet" {
            return Ok(RouteTargetType::Subnet);
        }
        if s == "Vpc" {
            return Ok(RouteTargetType::Vpc);
        }
        anyhow::bail!("invalid string: {}", s);
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * The VPC Router to which the route belongs.
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`RouterRoute`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct RouterRouteCreateParams {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * token used to fetch the next page of results (if any)
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
#[serde(rename_all = "snake_case")]
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
impl SagaState {
    pub fn variants() -> Vec<String> {
        vec![
            "failed".to_string(),
            "running".to_string(),
            "succeeded".to_string(),
        ]
    }
}
/**
 * The types for SagaState.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum SagaStateType {
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Succeeded")]
    Succeeded,
}

impl std::fmt::Display for SagaStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SagaStateType::Failed => "Failed",
            SagaStateType::Running => "Running",
            SagaStateType::Succeeded => "Succeeded",
        }
        .fmt(f)
    }
}

impl Default for SagaStateType {
    fn default() -> SagaStateType {
        SagaStateType::Failed
    }
}
impl std::str::FromStr for SagaStateType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Failed" {
            return Ok(SagaStateType::Failed);
        }
        if s == "Running" {
            return Ok(SagaStateType::Running);
        }
        if s == "Succeeded" {
            return Ok(SagaStateType::Succeeded);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Saga {
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
#[serde(rename_all = "snake_case")]
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
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["error"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["message"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["message"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for SagaErrorInfo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for SagaErrorInfo, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "action_failed" {
            j = format!(
                r#"{{
"error": "action_failed",
"message": {}
        }}"#,
                serde_json::json!(serde_json::Value::from_str(&content).unwrap())
            );
        }
        if tag == "deserialize_failed" {
            j = format!(
                r#"{{
"error": "deserialize_failed",
"message": "{}"
        }}"#,
                content
            );
        }
        if tag == "serialize_failed" {
            j = format!(
                r#"{{
"error": "serialize_failed",
"message": "{}"
        }}"#,
                content
            );
        }
        if tag == "subsaga_create_failed" {
            j = format!(
                r#"{{
"error": "subsaga_create_failed",
"message": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl SagaErrorInfo {
    pub fn variants() -> Vec<String> {
        vec![
            "action_failed".to_string(),
            "deserialize_failed".to_string(),
            "injected_error".to_string(),
            "serialize_failed".to_string(),
            "subsaga_create_failed".to_string(),
        ]
    }
}
/**
 * The types for SagaErrorInfo.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum SagaErrorInfoType {
    #[serde(rename = "ActionFailed")]
    ActionFailed,
    #[serde(rename = "DeserializeFailed")]
    DeserializeFailed,
    #[serde(rename = "InjectedError")]
    InjectedError,
    #[serde(rename = "SerializeFailed")]
    SerializeFailed,
    #[serde(rename = "SubsagaCreateFailed")]
    SubsagaCreateFailed,
}

impl std::fmt::Display for SagaErrorInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SagaErrorInfoType::ActionFailed => "ActionFailed",
            SagaErrorInfoType::DeserializeFailed => "DeserializeFailed",
            SagaErrorInfoType::InjectedError => "InjectedError",
            SagaErrorInfoType::SerializeFailed => "SerializeFailed",
            SagaErrorInfoType::SubsagaCreateFailed => "SubsagaCreateFailed",
        }
        .fmt(f)
    }
}

impl Default for SagaErrorInfoType {
    fn default() -> SagaErrorInfoType {
        SagaErrorInfoType::ActionFailed
    }
}
impl std::str::FromStr for SagaErrorInfoType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ActionFailed" {
            return Ok(SagaErrorInfoType::ActionFailed);
        }
        if s == "DeserializeFailed" {
            return Ok(SagaErrorInfoType::DeserializeFailed);
        }
        if s == "InjectedError" {
            return Ok(SagaErrorInfoType::InjectedError);
        }
        if s == "SerializeFailed" {
            return Ok(SagaErrorInfoType::SerializeFailed);
        }
        if s == "SubsagaCreateFailed" {
            return Ok(SagaErrorInfoType::SubsagaCreateFailed);
        }
        anyhow::bail!("invalid string: {}", s);
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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub disk_id: String,

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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Snapshot`](omicron_common::api::external::Snapshot)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SnapshotCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * token used to fetch the next page of results (if any)
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
     * Names are constructed by concatenating the target and metric names with ':'. Target and metric names must be lowercase alphanumeric characters with '_' separating words.
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
     * token used to fetch the next page of results (if any)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// Client view of a [`User`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct User {
    /**
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dns_name: String,

    /**
     * An IPv6 subnet, including prefix and subnet mask
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv6_prefix: String,

    /**
     * id for the project containing this VPC
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,

    /**
     * id for the system router where subnet default routes are registered
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,
}

/// Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleTarget {
    Vpc(String),
    Subnet(String),
    Instance(String),
    Ip(String),
    IpNet(IpNet),
}

impl fmt::Display for VpcFirewallRuleTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["value"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["value"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for VpcFirewallRuleTarget {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for VpcFirewallRuleTarget, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "vpc" {
            j = format!(
                r#"{{
"type": "vpc",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "subnet" {
            j = format!(
                r#"{{
"type": "subnet",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "instance" {
            j = format!(
                r#"{{
"type": "instance",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "ip" {
            j = format!(
                r#"{{
"type": "ip",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "ip_net" {
            j = format!(
                r#"{{
"type": "ip_net",
"value": {}
        }}"#,
                serde_json::json!(IpNet::from_str(&content).unwrap())
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl VpcFirewallRuleTarget {
    pub fn variants() -> Vec<String> {
        vec![
            "instance".to_string(),
            "ip".to_string(),
            "ip_net".to_string(),
            "subnet".to_string(),
            "vpc".to_string(),
        ]
    }
}
/**
 * The types for VpcFirewallRuleTarget.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleTargetType {
    #[serde(rename = "Instance")]
    Instance,
    #[serde(rename = "Ip")]
    Ip,
    #[serde(rename = "IpNet")]
    IpNet,
    #[serde(rename = "Subnet")]
    Subnet,
    #[serde(rename = "Vpc")]
    Vpc,
}

impl std::fmt::Display for VpcFirewallRuleTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleTargetType::Instance => "Instance",
            VpcFirewallRuleTargetType::Ip => "Ip",
            VpcFirewallRuleTargetType::IpNet => "IpNet",
            VpcFirewallRuleTargetType::Subnet => "Subnet",
            VpcFirewallRuleTargetType::Vpc => "Vpc",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleTargetType {
    fn default() -> VpcFirewallRuleTargetType {
        VpcFirewallRuleTargetType::Instance
    }
}
impl std::str::FromStr for VpcFirewallRuleTargetType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Instance" {
            return Ok(VpcFirewallRuleTargetType::Instance);
        }
        if s == "Ip" {
            return Ok(VpcFirewallRuleTargetType::Ip);
        }
        if s == "IpNet" {
            return Ok(VpcFirewallRuleTargetType::IpNet);
        }
        if s == "Subnet" {
            return Ok(VpcFirewallRuleTargetType::Subnet);
        }
        if s == "Vpc" {
            return Ok(VpcFirewallRuleTargetType::Vpc);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcFirewallRule {
    /**
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * the relative priority of this rule
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
     * the VPC to which this rule belongs
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleHostFilter {
    Vpc(String),
    Subnet(String),
    Instance(String),
    Ip(String),
    IpNet(IpNet),
}

impl fmt::Display for VpcFirewallRuleHostFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["value"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["value"].clone()).unwrap_or_default();
            if let Some((_, v)) = map.iter().next() {
                content = v.to_string();
            }
        }
        if tag == "internet_gateway" {
            tag = "inetgw".to_string();
        }
        write!(f, "{}={}", tag, content)
    }
}

impl std::str::FromStr for VpcFirewallRuleHostFilter {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for VpcFirewallRuleHostFilter, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "vpc" {
            j = format!(
                r#"{{
"type": "vpc",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "subnet" {
            j = format!(
                r#"{{
"type": "subnet",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "instance" {
            j = format!(
                r#"{{
"type": "instance",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "ip" {
            j = format!(
                r#"{{
"type": "ip",
"value": "{}"
        }}"#,
                content
            );
        }
        if tag == "ip_net" {
            j = format!(
                r#"{{
"type": "ip_net",
"value": {}
        }}"#,
                serde_json::json!(IpNet::from_str(&content).unwrap())
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl VpcFirewallRuleHostFilter {
    pub fn variants() -> Vec<String> {
        vec![
            "instance".to_string(),
            "ip".to_string(),
            "ip_net".to_string(),
            "subnet".to_string(),
            "vpc".to_string(),
        ]
    }
}
/**
 * The types for VpcFirewallRuleHostFilter.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum VpcFirewallRuleHostFilterType {
    #[serde(rename = "Instance")]
    Instance,
    #[serde(rename = "Ip")]
    Ip,
    #[serde(rename = "IpNet")]
    IpNet,
    #[serde(rename = "Subnet")]
    Subnet,
    #[serde(rename = "Vpc")]
    Vpc,
}

impl std::fmt::Display for VpcFirewallRuleHostFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleHostFilterType::Instance => "Instance",
            VpcFirewallRuleHostFilterType::Ip => "Ip",
            VpcFirewallRuleHostFilterType::IpNet => "IpNet",
            VpcFirewallRuleHostFilterType::Subnet => "Subnet",
            VpcFirewallRuleHostFilterType::Vpc => "Vpc",
        }
        .fmt(f)
    }
}

impl Default for VpcFirewallRuleHostFilterType {
    fn default() -> VpcFirewallRuleHostFilterType {
        VpcFirewallRuleHostFilterType::Instance
    }
}
impl std::str::FromStr for VpcFirewallRuleHostFilterType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Instance" {
            return Ok(VpcFirewallRuleHostFilterType::Instance);
        }
        if s == "Ip" {
            return Ok(VpcFirewallRuleHostFilterType::Ip);
        }
        if s == "IpNet" {
            return Ok(VpcFirewallRuleHostFilterType::IpNet);
        }
        if s == "Subnet" {
            return Ok(VpcFirewallRuleHostFilterType::Subnet);
        }
        if s == "Vpc" {
            return Ok(VpcFirewallRuleHostFilterType::Vpc);
        }
        anyhow::bail!("invalid string: {}", s);
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * the relative priority of this rule
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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
     * The VPC to which the router belongs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_id: String,
}

/// Create-time parameters for a [`VpcRouter`](crate::external_api::views::VpcRouter)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct VpcRouterCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

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
     * token used to fetch the next page of results (if any)
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
     * unique, immutable, system-controlled identifier for each resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * An IPv4 subnet, including prefix and subnet mask
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv4_block: String,

    /**
     * An IPv6 subnet, including prefix and subnet mask
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
     * timestamp when this resource was last modified
     */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
     * The VPC to which the subnet belongs.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
     * An IPv4 subnet, including prefix and subnet mask
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ipv4_block: String,

    /**
     * The IPv6 address range for this subnet.
     *  
     *  It must be allocated from the RFC 4193 Unique Local Address range, with the prefix equal to the parent VPC's prefix. A random `/64` block will be assigned if one is not provided. It must not overlap with any existing subnet in the VPC.
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
     * token used to fetch the next page of results (if any)
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

/// A count of bytes, typically used either for memory or storage capacity
///
/// The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
pub type ByteCount = u64;
/// The number of CPUs in an Instance
pub type InstanceCpuCount = u16;
/// An inclusive-inclusive range of IP ports. The second port may be omitted to represent a single port
pub type L4PortRange = String;
/// A Media Access Control address, in EUI-48 format
pub type MacAddr = String;
/// Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
pub type Name = String;
/// Role names consist of two string components separated by dot (".").
pub type RoleName = String;
/// Names are constructed by concatenating the target and metric names with ':'. Target and metric names must be lowercase alphanumeric characters with '_' separating words.
pub type TimeseriesName = String;
