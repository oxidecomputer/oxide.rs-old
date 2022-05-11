//! The data types sent to and returned from the API client.
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use tabled::Tabled;

/**
* The type of an individual datum of a metric.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum DatumType {
    Bool,
    Bytes,
    CumulativeF64,
    CumulativeI64,
    F64,
    HistogramF64,
    HistogramI64,
    I64,
    String,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DatumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DatumType::Bool => "bool",
            DatumType::Bytes => "bytes",
            DatumType::CumulativeF64 => "cumulative_f64",
            DatumType::CumulativeI64 => "cumulative_i64",
            DatumType::F64 => "f64",
            DatumType::HistogramF64 => "histogram_f64",
            DatumType::HistogramI64 => "histogram_i64",
            DatumType::I64 => "i64",
            DatumType::String => "string",
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
        if s == "bool" {
            return Ok(DatumType::Bool);
        }
        if s == "bytes" {
            return Ok(DatumType::Bytes);
        }
        if s == "cumulative_f64" {
            return Ok(DatumType::CumulativeF64);
        }
        if s == "cumulative_i64" {
            return Ok(DatumType::CumulativeI64);
        }
        if s == "f64" {
            return Ok(DatumType::F64);
        }
        if s == "histogram_f64" {
            return Ok(DatumType::HistogramF64);
        }
        if s == "histogram_i64" {
            return Ok(DatumType::HistogramI64);
        }
        if s == "i64" {
            return Ok(DatumType::I64);
        }
        if s == "string" {
            return Ok(DatumType::String);
        }
        anyhow::bail!("invalid string for DatumType: {}", s);
    }
}
impl DatumType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DatumType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Digest {
    Sha256(String),
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for Digest {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
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
#[serde(rename_all = "snake_case")]
pub enum DiskStateType {
    Attached,
    Attaching,
    Creating,
    Destroyed,
    Detached,
    Detaching,
    Faulted,
}

impl std::fmt::Display for DiskStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DiskStateType::Attached => "attached",
            DiskStateType::Attaching => "attaching",
            DiskStateType::Creating => "creating",
            DiskStateType::Destroyed => "destroyed",
            DiskStateType::Detached => "detached",
            DiskStateType::Detaching => "detaching",
            DiskStateType::Faulted => "faulted",
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
        if s == "attached" {
            return Ok(DiskStateType::Attached);
        }
        if s == "attaching" {
            return Ok(DiskStateType::Attaching);
        }
        if s == "creating" {
            return Ok(DiskStateType::Creating);
        }
        if s == "destroyed" {
            return Ok(DiskStateType::Destroyed);
        }
        if s == "detached" {
            return Ok(DiskStateType::Detached);
        }
        if s == "detaching" {
            return Ok(DiskStateType::Detaching);
        }
        if s == "faulted" {
            return Ok(DiskStateType::Faulted);
        }
        anyhow::bail!("invalid string for DiskStateType: {}", s);
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

    /**
    * A count of bytes, typically used either for memory or storage capacity
    *  
    *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
    */
    #[serde(default)]
    pub block_size: u64,

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
    pub image_id: String,

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "image_id")]
pub enum DiskSource {
    Blank(i64),
    Snapshot(String),
    Image(String),
    GlobalImage(String),
}

impl fmt::Display for DiskSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["image_id"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["image_id"].clone()).unwrap_or_default();
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

impl std::str::FromStr for DiskSource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for DiskSource, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "blank" {
            j = format!(
                r#"{{
"type": "blank",
"image_id": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        if tag == "snapshot" {
            j = format!(
                r#"{{
"type": "snapshot",
"image_id": "{}"
        }}"#,
                content
            );
        }
        if tag == "image" {
            j = format!(
                r#"{{
"type": "image",
"image_id": "{}"
        }}"#,
                content
            );
        }
        if tag == "global_image" {
            j = format!(
                r#"{{
"type": "global_image",
"image_id": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl DiskSource {
    pub fn variants() -> Vec<String> {
        vec![
            "blank".to_string(),
            "global_image".to_string(),
            "image".to_string(),
            "snapshot".to_string(),
        ]
    }
}
/**
* The types for DiskSource.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum DiskSourceType {
    Blank,
    GlobalImage,
    Image,
    Snapshot,
}

impl std::fmt::Display for DiskSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DiskSourceType::Blank => "blank",
            DiskSourceType::GlobalImage => "global_image",
            DiskSourceType::Image => "image",
            DiskSourceType::Snapshot => "snapshot",
        }
        .fmt(f)
    }
}

impl Default for DiskSourceType {
    fn default() -> DiskSourceType {
        DiskSourceType::Blank
    }
}
impl std::str::FromStr for DiskSourceType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "blank" {
            return Ok(DiskSourceType::Blank);
        }
        if s == "global_image" {
            return Ok(DiskSourceType::GlobalImage);
        }
        if s == "image" {
            return Ok(DiskSourceType::Image);
        }
        if s == "snapshot" {
            return Ok(DiskSourceType::Snapshot);
        }
        anyhow::bail!("invalid string for DiskSourceType: {}", s);
    }
}

/// Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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

    #[serde()]
    pub disk_source: DiskSource,

    /**
    * A count of bytes, typically used either for memory or storage capacity
    *  
    *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
    */
    #[serde(default)]
    pub size: u64,
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
    pub name: String,
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

#[derive(Debug, Deserialize, thiserror::Error, PartialEq, Serialize)]
pub enum Error {
    /// An object needed as part of this operation was not found.
    #[error("Object Not Found: {message}")]
    ObjectNotFound { message: String },
    /// An object already exists with the specified name or identifier.
    #[error("Object Already Exists: {message}")]
    ObjectAlreadyExists { message: String },
    /// The request was well-formed, but the operation cannot be completed given
    /// the current state of the system.
    #[error("Invalid Request: {message}")]
    InvalidRequest { message: String },
    /// Authentication credentials were required but either missing or invalid.
    /// The HTTP status code is called "Unauthorized", but it's more accurate to
    /// call it "Unauthenticated".
    #[error("Missing or invalid credentials")]
    Unauthenticated { internal_message: String },
    /// The specified input field is not valid.
    #[error("Invalid Value: {message}")]
    InvalidValue { message: String },
    /// The request is not authorized to perform the requested operation.
    #[error("Forbidden")]
    Forbidden,

    /// The system encountered an unhandled operational error.
    #[error("Internal Error: {internal_message}")]
    InternalError { internal_message: String },
    /// The system (or part of it) is unavailable.
    #[error("Service Unavailable: {internal_message}")]
    ServiceUnavailable { internal_message: String },
    /// Method Not Allowed
    #[error("Method Not Allowed: {internal_message}")]
    MethodNotAllowed { internal_message: String },
}

impl Error {
    /// Returns whether the error is likely transient and could reasonably be
    /// retried
    pub fn retryable(&self) -> bool {
        match self {
            Error::ServiceUnavailable { .. } => true,

            Error::ObjectNotFound { .. }
            | Error::ObjectAlreadyExists { .. }
            | Error::Unauthenticated { .. }
            | Error::InvalidRequest { .. }
            | Error::InvalidValue { .. }
            | Error::Forbidden
            | Error::MethodNotAllowed { .. }
            | Error::InternalError { .. } => false,
        }
    }
}

impl From<ErrorResponse> for Error {
    /// Converts an `Error` error into an `HttpError`.  This defines how
    /// errors that are represented internally using `Error` are ultimately
    /// exposed to clients over HTTP.
    fn from(error: ErrorResponse) -> Error {
        if error.error_code == "ObjectNotFound" {
            return Error::ObjectNotFound {
                message: error.message,
            };
        }

        if error.error_code == "ObjectAlreadyExists" {
            return Error::ObjectAlreadyExists {
                message: error.message,
            };
        }

        if error.error_code == "Unauthorized" {
            return Error::Unauthenticated {
                internal_message: error.message,
            };
        }

        if error.error_code == "InvalidRequest" {
            return Error::InvalidRequest {
                message: error.message,
            };
        }

        if error.error_code == "InvalidValue" {
            return Error::InvalidValue {
                message: error.message,
            };
        }

        if error.error_code == "Forbidden" {
            return Error::Forbidden;
        }

        if error.error_code == "MethodNotAllowed" {
            return Error::MethodNotAllowed {
                internal_message: error.message,
            };
        }

        if error.error_code == "ServiceUnavailable" {
            return Error::ServiceUnavailable {
                internal_message: error.message,
            };
        }

        Error::InternalError {
            internal_message: error.message,
        }
    }
}

/// Identifies a type of API resource
#[derive(
    Clone,
    Copy,
    Debug,
    serde_with::DeserializeFromStr,
    Display,
    Eq,
    FromStr,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::SerializeDisplay,
)]
#[display(style = "kebab-case")]
pub enum ResourceType {
    Fleet,
    Organization,
    Project,
    Dataset,
    Disk,
    Instance,
    NetworkInterface,
    Rack,
    Sled,
    SagaDbg,
    Volume,
    Vpc,
    VpcFirewallRule,
    VpcSubnet,
    VpcRouter,
    RouterRoute,
    Oximeter,
    MetricProducer,
    Role,
    User,
    Zpool,
}

/// Error information from a response.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ErrorResponse {
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
#[serde(rename_all = "snake_case")]
pub enum FieldSource {
    Metric,
    Target,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FieldSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FieldSource::Metric => "metric",
            FieldSource::Target => "target",
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
        if s == "metric" {
            return Ok(FieldSource::Metric);
        }
        if s == "target" {
            return Ok(FieldSource::Target);
        }
        anyhow::bail!("invalid string for FieldSource: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Bool,
    I64,
    IpAddr,
    String,
    Uuid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FieldType::Bool => "bool",
            FieldType::I64 => "i64",
            FieldType::IpAddr => "ip_addr",
            FieldType::String => "string",
            FieldType::Uuid => "uuid",
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
        if s == "bool" {
            return Ok(FieldType::Bool);
        }
        if s == "i64" {
            return Ok(FieldType::I64);
        }
        if s == "ip_addr" {
            return Ok(FieldType::IpAddr);
        }
        if s == "string" {
            return Ok(FieldType::String);
        }
        if s == "uuid" {
            return Ok(FieldType::Uuid);
        }
        anyhow::bail!("invalid string for FieldType: {}", s);
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

/// Client view of global Images
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct GlobalImage {
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
    * A count of bytes, typically used either for memory or storage capacity
    *  
    *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
    */
    #[serde(default)]
    pub block_size: u64,

    /**
    * Hash of the image contents, if applicable
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[header(hidden = true)]
    pub digest: Option<Digest>,

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

    /**
    * URL source of this image, if any
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,

    /**
    * Version of this, if any
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct GlobalImageResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<GlobalImage>,

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

/**
* Describes what kind of identity is described by an id
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IdentityType {
    SiloUser,
    UserBuiltin,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdentityType::SiloUser => "silo_user",
            IdentityType::UserBuiltin => "user_builtin",
            IdentityType::Noop => "",
            IdentityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityType {
    fn default() -> IdentityType {
        IdentityType::SiloUser
    }
}
impl std::str::FromStr for IdentityType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "silo_user" {
            return Ok(IdentityType::SiloUser);
        }
        if s == "user_builtin" {
            return Ok(IdentityType::UserBuiltin);
        }
        anyhow::bail!("invalid string for IdentityType: {}", s);
    }
}
impl IdentityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityType::Noop)
    }
}

/// Client view of project Images
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Image {
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
    * A count of bytes, typically used either for memory or storage capacity
    *  
    *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
    */
    #[serde(default)]
    pub block_size: u64,

    /**
    * Hash of the image contents, if applicable
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[header(hidden = true)]
    pub digest: Option<Digest>,

    /**
    * The project the disk belongs to
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
    * timestamp when this resource was last modified
    */
    #[serde()]
    pub time_modified: crate::utils::DisplayOptionDateTime,

    /**
    * URL source of this image, if any
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,

    /**
    * Version of this, if any
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ImageSource {
    Url(String),
    Snapshot(String),
}

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for ImageSource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Create-time parameters for an [`Image`](omicron_common::api::external::Image)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct ImageCreate {
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
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub block_size: i64,

    #[serde()]
    pub source: ImageSource,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ImageResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<Image>,

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

/**
* Running state of an Instance (primarily: booted or stopped)
*   
*   This typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum InstanceState {
    Creating,
    Destroyed,
    Failed,
    Migrating,
    Rebooting,
    Repairing,
    Running,
    Starting,
    Stopped,
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
        anyhow::bail!("invalid string for InstanceState: {}", s);
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
#[serde(tag = "type", content = "name")]
pub enum InstanceDiskAttachment {
    Create {
        description: String,
        disk_source: DiskSource,
        name: String,
        size: u64,
    },
    Attach(String),
}

impl fmt::Display for InstanceDiskAttachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["name"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["name"].clone()).unwrap_or_default();
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

impl std::str::FromStr for InstanceDiskAttachment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for InstanceDiskAttachment, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "create" {
            j = format!(
                r#"{{
"type": "create",
"name": "{}"
        }}"#,
                content
            );
        }
        if tag == "create" {
            j = format!(
                r#"{{
"type": "create",
"name": {}
        }}"#,
                serde_json::json!(DiskSource::from_str(&content).unwrap())
            );
        }
        if tag == "create" {
            j = format!(
                r#"{{
"type": "create",
"name": "{}"
        }}"#,
                content
            );
        }
        if tag == "create" {
            j = format!(
                r#"{{
"type": "create",
"name": {}
        }}"#,
                serde_json::json!(u64::from_str(&content).unwrap())
            );
        }
        if tag == "attach" {
            j = format!(
                r#"{{
"type": "attach",
"name": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl InstanceDiskAttachment {
    pub fn variants() -> Vec<String> {
        vec!["attach".to_string(), "create".to_string()]
    }
}
/**
* The types for InstanceDiskAttachment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiskAttachmentType {
    Attach,
    Create,
}

impl std::fmt::Display for InstanceDiskAttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InstanceDiskAttachmentType::Attach => "attach",
            InstanceDiskAttachmentType::Create => "create",
        }
        .fmt(f)
    }
}

impl Default for InstanceDiskAttachmentType {
    fn default() -> InstanceDiskAttachmentType {
        InstanceDiskAttachmentType::Attach
    }
}
impl std::str::FromStr for InstanceDiskAttachmentType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "attach" {
            return Ok(InstanceDiskAttachmentType::Attach);
        }
        if s == "create" {
            return Ok(InstanceDiskAttachmentType::Create);
        }
        anyhow::bail!("invalid string for InstanceDiskAttachmentType: {}", s);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "params")]
pub enum InstanceNetworkInterfaceAttachment {
    Create(Vec<NetworkInterfaceCreate>),
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
#[serde(rename_all = "snake_case")]
pub enum InstanceNetworkInterfaceAttachmentType {
    Create,
    Default,
    None,
}

impl std::fmt::Display for InstanceNetworkInterfaceAttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InstanceNetworkInterfaceAttachmentType::Create => "create",
            InstanceNetworkInterfaceAttachmentType::Default => "default",
            InstanceNetworkInterfaceAttachmentType::None => "none",
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
        if s == "create" {
            return Ok(InstanceNetworkInterfaceAttachmentType::Create);
        }
        if s == "default" {
            return Ok(InstanceNetworkInterfaceAttachmentType::Default);
        }
        if s == "none" {
            return Ok(InstanceNetworkInterfaceAttachmentType::None);
        }
        anyhow::bail!(
            "invalid string for InstanceNetworkInterfaceAttachmentType: {}",
            s
        );
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

    /**
    * Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub disks: Vec<InstanceDiskAttachment>,

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

    /**
    * Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_data: String,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationRoles {
    Admin,
    Collaborator,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrganizationRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrganizationRoles::Admin => "admin",
            OrganizationRoles::Collaborator => "collaborator",
            OrganizationRoles::Noop => "",
            OrganizationRoles::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrganizationRoles {
    fn default() -> OrganizationRoles {
        OrganizationRoles::Admin
    }
}
impl std::str::FromStr for OrganizationRoles {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(OrganizationRoles::Admin);
        }
        if s == "collaborator" {
            return Ok(OrganizationRoles::Collaborator);
        }
        anyhow::bail!("invalid string for OrganizationRoles: {}", s);
    }
}
impl OrganizationRoles {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrganizationRoles::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationRolesRoleAssignment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub identity_id: String,

    /**
    * Describes what kind of identity is described by an id
    */
    #[serde(default, skip_serializing_if = "IdentityType::is_noop")]
    pub identity_type: IdentityType,

    #[serde(default, skip_serializing_if = "OrganizationRoles::is_noop")]
    pub role_name: OrganizationRoles,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationRolesPolicy {
    /**
    * Roles directly assigned on this resource
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<OrganizationRolesRoleAssignment>,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum ProjectRoles {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProjectRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProjectRoles::Admin => "admin",
            ProjectRoles::Collaborator => "collaborator",
            ProjectRoles::Viewer => "viewer",
            ProjectRoles::Noop => "",
            ProjectRoles::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProjectRoles {
    fn default() -> ProjectRoles {
        ProjectRoles::Admin
    }
}
impl std::str::FromStr for ProjectRoles {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(ProjectRoles::Admin);
        }
        if s == "collaborator" {
            return Ok(ProjectRoles::Collaborator);
        }
        if s == "viewer" {
            return Ok(ProjectRoles::Viewer);
        }
        anyhow::bail!("invalid string for ProjectRoles: {}", s);
    }
}
impl ProjectRoles {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProjectRoles::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectRolesRoleAssignment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub identity_id: String,

    /**
    * Describes what kind of identity is described by an id
    */
    #[serde(default, skip_serializing_if = "IdentityType::is_noop")]
    pub identity_type: IdentityType,

    #[serde(default, skip_serializing_if = "ProjectRoles::is_noop")]
    pub role_name: ProjectRoles,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectRolesPolicy {
    /**
    * Roles directly assigned on this resource
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<ProjectRolesRoleAssignment>,
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
#[serde(rename_all = "snake_case")]
pub enum RouteDestinationType {
    Ip,
    IpNet,
    Subnet,
    Vpc,
}

impl std::fmt::Display for RouteDestinationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouteDestinationType::Ip => "ip",
            RouteDestinationType::IpNet => "ip_net",
            RouteDestinationType::Subnet => "subnet",
            RouteDestinationType::Vpc => "vpc",
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
        if s == "ip" {
            return Ok(RouteDestinationType::Ip);
        }
        if s == "ip_net" {
            return Ok(RouteDestinationType::IpNet);
        }
        if s == "subnet" {
            return Ok(RouteDestinationType::Subnet);
        }
        if s == "vpc" {
            return Ok(RouteDestinationType::Vpc);
        }
        anyhow::bail!("invalid string for RouteDestinationType: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum RouteTargetType {
    Instance,
    InternetGateway,
    Ip,
    Subnet,
    Vpc,
}

impl std::fmt::Display for RouteTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouteTargetType::Instance => "instance",
            RouteTargetType::InternetGateway => "internet_gateway",
            RouteTargetType::Ip => "ip",
            RouteTargetType::Subnet => "subnet",
            RouteTargetType::Vpc => "vpc",
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
        if s == "instance" {
            return Ok(RouteTargetType::Instance);
        }
        if s == "internet_gateway" {
            return Ok(RouteTargetType::InternetGateway);
        }
        if s == "ip" {
            return Ok(RouteTargetType::Ip);
        }
        if s == "subnet" {
            return Ok(RouteTargetType::Subnet);
        }
        if s == "vpc" {
            return Ok(RouteTargetType::Vpc);
        }
        anyhow::bail!("invalid string for RouteTargetType: {}", s);
    }
}

/**
* The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
*   
*   See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum RouterRouteKind {
    Custom,
    Default,
    VpcPeering,
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
        anyhow::bail!("invalid string for RouterRouteKind: {}", s);
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

    /**
    * The VPC Router to which the route belongs.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vpc_router_id: String,
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
#[serde(rename_all = "snake_case")]
pub enum SagaStateType {
    Failed,
    Running,
    Succeeded,
}

impl std::fmt::Display for SagaStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SagaStateType::Failed => "failed",
            SagaStateType::Running => "running",
            SagaStateType::Succeeded => "succeeded",
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
        if s == "failed" {
            return Ok(SagaStateType::Failed);
        }
        if s == "running" {
            return Ok(SagaStateType::Running);
        }
        if s == "succeeded" {
            return Ok(SagaStateType::Succeeded);
        }
        anyhow::bail!("invalid string for SagaStateType: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum SagaErrorInfoType {
    ActionFailed,
    DeserializeFailed,
    InjectedError,
    SerializeFailed,
    SubsagaCreateFailed,
}

impl std::fmt::Display for SagaErrorInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SagaErrorInfoType::ActionFailed => "action_failed",
            SagaErrorInfoType::DeserializeFailed => "deserialize_failed",
            SagaErrorInfoType::InjectedError => "injected_error",
            SagaErrorInfoType::SerializeFailed => "serialize_failed",
            SagaErrorInfoType::SubsagaCreateFailed => "subsaga_create_failed",
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
        if s == "action_failed" {
            return Ok(SagaErrorInfoType::ActionFailed);
        }
        if s == "deserialize_failed" {
            return Ok(SagaErrorInfoType::DeserializeFailed);
        }
        if s == "injected_error" {
            return Ok(SagaErrorInfoType::InjectedError);
        }
        if s == "serialize_failed" {
            return Ok(SagaErrorInfoType::SerializeFailed);
        }
        if s == "subsaga_create_failed" {
            return Ok(SagaErrorInfoType::SubsagaCreateFailed);
        }
        anyhow::bail!("invalid string for SagaErrorInfoType: {}", s);
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

/// Client view of a ['Silo']
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Silo {
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
    * A silo where discoverable is false can be retrieved only by its id - it will not be part of the "list all silos" output.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub discoverable: bool,

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

/// Create-time parameters for a [`Silo`](crate::external_api::views::Silo)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloCreate {
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
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub discoverable: bool,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<Silo>,

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
#[serde(rename_all = "snake_case")]
pub enum SiloRoles {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SiloRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SiloRoles::Admin => "admin",
            SiloRoles::Collaborator => "collaborator",
            SiloRoles::Viewer => "viewer",
            SiloRoles::Noop => "",
            SiloRoles::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SiloRoles {
    fn default() -> SiloRoles {
        SiloRoles::Admin
    }
}
impl std::str::FromStr for SiloRoles {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(SiloRoles::Admin);
        }
        if s == "collaborator" {
            return Ok(SiloRoles::Collaborator);
        }
        if s == "viewer" {
            return Ok(SiloRoles::Viewer);
        }
        anyhow::bail!("invalid string for SiloRoles: {}", s);
    }
}
impl SiloRoles {
    pub fn is_noop(&self) -> bool {
        matches!(self, SiloRoles::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloRolesRoleAssignment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub identity_id: String,

    /**
    * Describes what kind of identity is described by an id
    */
    #[serde(default, skip_serializing_if = "IdentityType::is_noop")]
    pub identity_type: IdentityType,

    #[serde(default, skip_serializing_if = "SiloRoles::is_noop")]
    pub role_name: SiloRoles,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloRolesPolicy {
    /**
    * Roles directly assigned on this resource
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<SiloRolesRoleAssignment>,
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

/// Client view of a [`SshKey`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SshKey {
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
    * SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,

    /**
    * The user to whom this key belongs
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub silo_user_id: String,

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

/// Create-time parameters for an [`SshKey`](crate::external_api::views::SshKey)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SshKeyCreate {
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
    * SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SshKeyResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<SshKey>,

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
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleAction {
    Allow,
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
        anyhow::bail!("invalid string for VpcFirewallRuleAction: {}", s);
    }
}
impl VpcFirewallRuleAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleDirection {
    Inbound,
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
        anyhow::bail!("invalid string for VpcFirewallRuleDirection: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleStatus {
    Disabled,
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
        anyhow::bail!("invalid string for VpcFirewallRuleStatus: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleTargetType {
    Instance,
    Ip,
    IpNet,
    Subnet,
    Vpc,
}

impl std::fmt::Display for VpcFirewallRuleTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleTargetType::Instance => "instance",
            VpcFirewallRuleTargetType::Ip => "ip",
            VpcFirewallRuleTargetType::IpNet => "ip_net",
            VpcFirewallRuleTargetType::Subnet => "subnet",
            VpcFirewallRuleTargetType::Vpc => "vpc",
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
        if s == "instance" {
            return Ok(VpcFirewallRuleTargetType::Instance);
        }
        if s == "ip" {
            return Ok(VpcFirewallRuleTargetType::Ip);
        }
        if s == "ip_net" {
            return Ok(VpcFirewallRuleTargetType::IpNet);
        }
        if s == "subnet" {
            return Ok(VpcFirewallRuleTargetType::Subnet);
        }
        if s == "vpc" {
            return Ok(VpcFirewallRuleTargetType::Vpc);
        }
        anyhow::bail!("invalid string for VpcFirewallRuleTargetType: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleHostFilterType {
    Instance,
    Ip,
    IpNet,
    Subnet,
    Vpc,
}

impl std::fmt::Display for VpcFirewallRuleHostFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleHostFilterType::Instance => "instance",
            VpcFirewallRuleHostFilterType::Ip => "ip",
            VpcFirewallRuleHostFilterType::IpNet => "ip_net",
            VpcFirewallRuleHostFilterType::Subnet => "subnet",
            VpcFirewallRuleHostFilterType::Vpc => "vpc",
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
        if s == "instance" {
            return Ok(VpcFirewallRuleHostFilterType::Instance);
        }
        if s == "ip" {
            return Ok(VpcFirewallRuleHostFilterType::Ip);
        }
        if s == "ip_net" {
            return Ok(VpcFirewallRuleHostFilterType::IpNet);
        }
        if s == "subnet" {
            return Ok(VpcFirewallRuleHostFilterType::Subnet);
        }
        if s == "vpc" {
            return Ok(VpcFirewallRuleHostFilterType::Vpc);
        }
        anyhow::bail!("invalid string for VpcFirewallRuleHostFilterType: {}", s);
    }
}

/**
* The protocols that may be specified in a firewall rule's filter
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum VpcFirewallRuleProtocol {
    Icmp,
    Tcp,
    Udp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VpcFirewallRuleProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VpcFirewallRuleProtocol::Icmp => "icmp",
            VpcFirewallRuleProtocol::Tcp => "tcp",
            VpcFirewallRuleProtocol::Udp => "udp",
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
        if s == "icmp" {
            return Ok(VpcFirewallRuleProtocol::Icmp);
        }
        if s == "tcp" {
            return Ok(VpcFirewallRuleProtocol::Tcp);
        }
        if s == "udp" {
            return Ok(VpcFirewallRuleProtocol::Udp);
        }
        anyhow::bail!("invalid string for VpcFirewallRuleProtocol: {}", s);
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
#[serde(rename_all = "snake_case")]
pub enum VpcRouterKind {
    Custom,
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
        anyhow::bail!("invalid string for VpcRouterKind: {}", s);
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

/**
* Supported set of sort modes for scanning by id only.
*   
*   Currently, we only support scanning in ascending order.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IdSortMode {
    IdAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdSortMode::IdAscending => "id_ascending",
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
        if s == "id_ascending" {
            return Ok(IdSortMode::IdAscending);
        }
        anyhow::bail!("invalid string for IdSortMode: {}", s);
    }
}
impl IdSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdSortMode::Noop)
    }
}

/**
* Supported set of sort modes for scanning by name only
*   
*   Currently, we only support scanning in ascending order.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum NameSortMode {
    NameAscending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NameSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameSortMode::NameAscending => "name_ascending",
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
        if s == "name_ascending" {
            return Ok(NameSortMode::NameAscending);
        }
        anyhow::bail!("invalid string for NameSortMode: {}", s);
    }
}
impl NameSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameSortMode::Noop)
    }
}

/**
* Supported set of sort modes for scanning by name or id
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum NameOrIdSortMode {
    IdAscending,
    NameAscending,
    NameDescending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NameOrIdSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NameOrIdSortMode::IdAscending => "id_ascending",
            NameOrIdSortMode::NameAscending => "name_ascending",
            NameOrIdSortMode::NameDescending => "name_descending",
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
        if s == "id_ascending" {
            return Ok(NameOrIdSortMode::IdAscending);
        }
        if s == "name_ascending" {
            return Ok(NameOrIdSortMode::NameAscending);
        }
        if s == "name_descending" {
            return Ok(NameOrIdSortMode::NameDescending);
        }
        anyhow::bail!("invalid string for NameOrIdSortMode: {}", s);
    }
}
impl NameOrIdSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, NameOrIdSortMode::Noop)
    }
}

pub type BlockSize = i64;
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
