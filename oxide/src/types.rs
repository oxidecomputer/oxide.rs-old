//! The data types sent to and returned from the API client.
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "devicePath"
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "projectId"
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "snapshotId"
    )]
    pub snapshot_id: String,
    #[serde()]
    pub state: DiskStateOneOf,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`Disk`]
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
     * Create-time parameters for a [`Disk`]
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "snapshotId"
    )]
    pub snapshot_id: String,
}

/// Parameters for the [`Disk`] to be attached or detached to an instance
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
pub enum StateCreating {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateCreating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateCreating::Creating => "creating",
            StateCreating::Noop => "",
            StateCreating::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateCreating {
    fn default() -> StateCreating {
        StateCreating::Noop
    }
}
impl StateCreating {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateCreating::Noop)
    }
}

/// Disk is being initialized
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskState {
    #[serde(default, skip_serializing_if = "StateCreating::is_noop")]
    pub state: StateCreating,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateDetached {
    #[serde(rename = "detached")]
    Detached,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateDetached {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateDetached::Detached => "detached",
            StateDetached::Noop => "",
            StateDetached::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateDetached {
    fn default() -> StateDetached {
        StateDetached::Noop
    }
}
impl StateDetached {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateDetached::Noop)
    }
}

/// Disk is ready but detached from any Instance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateData {
    #[serde(default, skip_serializing_if = "StateDetached::is_noop")]
    pub state: StateDetached,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateAttaching {
    #[serde(rename = "attaching")]
    Attaching,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateAttaching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateAttaching::Attaching => "attaching",
            StateAttaching::Noop => "",
            StateAttaching::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateAttaching {
    fn default() -> StateAttaching {
        StateAttaching::Noop
    }
}
impl StateAttaching {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateAttaching::Noop)
    }
}

/// Disk is being attached to the given Instance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateDataType {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance: String,
    #[serde(default, skip_serializing_if = "StateAttaching::is_noop")]
    pub state: StateAttaching,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateAttached {
    #[serde(rename = "attached")]
    Attached,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateAttached {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateAttached::Attached => "attached",
            StateAttached::Noop => "",
            StateAttached::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateAttached {
    fn default() -> StateAttached {
        StateAttached::Noop
    }
}
impl StateAttached {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateAttached::Noop)
    }
}

/// Disk is attached to the given Instance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateDataTypeLinks {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance: String,
    #[serde(default, skip_serializing_if = "StateAttached::is_noop")]
    pub state: StateAttached,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateDetaching {
    #[serde(rename = "detaching")]
    Detaching,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateDetaching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateDetaching::Detaching => "detaching",
            StateDetaching::Noop => "",
            StateDetaching::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateDetaching {
    fn default() -> StateDetaching {
        StateDetaching::Noop
    }
}
impl StateDetaching {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateDetaching::Noop)
    }
}

/// Disk is being detached from the given Instance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateDataTypeLinksObject {
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance: String,
    #[serde(default, skip_serializing_if = "StateDetaching::is_noop")]
    pub state: StateDetaching,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateDestroyed {
    #[serde(rename = "destroyed")]
    Destroyed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateDestroyed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateDestroyed::Destroyed => "destroyed",
            StateDestroyed::Noop => "",
            StateDestroyed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateDestroyed {
    fn default() -> StateDestroyed {
        StateDestroyed::Noop
    }
}
impl StateDestroyed {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateDestroyed::Noop)
    }
}

/// Disk has been destroyed
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateDataTypeLinksObjectBlah {
    #[serde(default, skip_serializing_if = "StateDestroyed::is_noop")]
    pub state: StateDestroyed,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateFaulted {
    #[serde(rename = "faulted")]
    Faulted,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateFaulted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateFaulted::Faulted => "faulted",
            StateFaulted::Noop => "",
            StateFaulted::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateFaulted {
    fn default() -> StateFaulted {
        StateFaulted::Noop
    }
}
impl StateFaulted {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateFaulted::Noop)
    }
}

/// Disk is unavailable
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DiskStateDataTypeLinksObjectBlahFoo {
    #[serde(default, skip_serializing_if = "StateFaulted::is_noop")]
    pub state: StateFaulted,
}

/// All of the following types:
///
/// - `DiskState`
/// - `DiskStateData`
/// - `DiskStateDataType`
/// - `DiskStateDataTypeLinks`
/// - `DiskStateDataTypeLinksObject`
/// - `DiskStateDataTypeLinksObjectBlah`
/// - `DiskStateDataTypeLinksObjectBlahFoo`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DiskStateOneOf {
    /**
     * Disk is being initialized
     */
    DiskState(DiskState),
    /**
     * Disk is ready but detached from any Instance
     */
    DiskStateData(DiskStateData),
    /**
     * Disk is being attached to the given Instance
     */
    DiskStateDataType(DiskStateDataType),
    /**
     * Disk is attached to the given Instance
     */
    DiskStateDataTypeLinks(DiskStateDataTypeLinks),
    /**
     * Disk is being detached from the given Instance
     */
    DiskStateDataTypeLinksObject(DiskStateDataTypeLinksObject),
    /**
     * Disk has been destroyed
     */
    DiskStateDataTypeLinksObjectBlah(DiskStateDataTypeLinksObjectBlah),
    /**
     * Disk is unavailable
     */
    DiskStateDataTypeLinksObjectBlahFoo(DiskStateDataTypeLinksObjectBlahFoo),
}

impl DiskStateOneOf {
    pub fn disk_state(&self) -> Option<&DiskState> {
        if let DiskStateOneOf::DiskState(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data(&self) -> Option<&DiskStateData> {
        if let DiskStateOneOf::DiskStateData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data_type(&self) -> Option<&DiskStateDataType> {
        if let DiskStateOneOf::DiskStateDataType(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data_type_links(&self) -> Option<&DiskStateDataTypeLinks> {
        if let DiskStateOneOf::DiskStateDataTypeLinks(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data_type_links_object(&self) -> Option<&DiskStateDataTypeLinksObject> {
        if let DiskStateOneOf::DiskStateDataTypeLinksObject(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data_type_links_object_blah(
        &self,
    ) -> Option<&DiskStateDataTypeLinksObjectBlah> {
        if let DiskStateOneOf::DiskStateDataTypeLinksObjectBlah(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn disk_state_data_type_links_object_blah_foo(
        &self,
    ) -> Option<&DiskStateDataTypeLinksObjectBlahFoo> {
        if let DiskStateOneOf::DiskStateDataTypeLinksObjectBlahFoo(ref_) = self {
            return Some(ref_);
        }
        None
    }
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

/// Client view of a [`User`]
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct User {
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
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "projectId"
    )]
    pub project_id: String,
    /**
     * Running state of an Instance (primarily: booted or stopped)
     *  
     *  This typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle
     */
    #[serde(rename = "runState")]
    pub run_state: InstanceState,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeRunStateUpdated"
    )]
    pub time_run_state_updated: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for an [`Instance`]
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

/// A `NetworkInterface` represents a virtual network interface device.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkInterface {
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
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

/// Create-time parameters for a [`Project`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProjectCreate {
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

/// Updateable properties of an [`Organization`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrganizationUpdate {
    /**
     * Updateable properties of an [`Organization`]
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "organizationId"
    )]
    pub organization_id: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
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

/// Updateable properties of a [`Project`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProjectUpdate {
    /**
     * Updateable properties of a [`Project`]
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

/// Client view of an [`Rack`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Rack {
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
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
pub enum TypeIp {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TypeIp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TypeIp::Ip => "ip",
            TypeIp::Noop => "",
            TypeIp::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TypeIp {
    fn default() -> TypeIp {
        TypeIp::Noop
    }
}
impl TypeIp {
    pub fn is_noop(&self) -> bool {
        matches!(self, TypeIp::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteTarget {
    #[serde(default, skip_serializing_if = "TypeIp::is_noop", rename = "type")]
    pub type_: TypeIp,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TypeVpc {
    #[serde(rename = "vpc")]
    Vpc,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TypeVpc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TypeVpc::Vpc => "vpc",
            TypeVpc::Noop => "",
            TypeVpc::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TypeVpc {
    fn default() -> TypeVpc {
        TypeVpc::Noop
    }
}
impl TypeVpc {
    pub fn is_noop(&self) -> bool {
        matches!(self, TypeVpc::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteDestinationData {
    #[serde(default, skip_serializing_if = "TypeVpc::is_noop", rename = "type")]
    pub type_: TypeVpc,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TypeSubnet {
    #[serde(rename = "subnet")]
    Subnet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TypeSubnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TypeSubnet::Subnet => "subnet",
            TypeSubnet::Noop => "",
            TypeSubnet::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TypeSubnet {
    fn default() -> TypeSubnet {
        TypeSubnet::Noop
    }
}
impl TypeSubnet {
    pub fn is_noop(&self) -> bool {
        matches!(self, TypeSubnet::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcFirewallRuleTarget {
    #[serde(default, skip_serializing_if = "TypeSubnet::is_noop", rename = "type")]
    pub type_: TypeSubnet,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// All of the following types:
///
/// - `RouteTarget`
/// - `RouteDestinationData`
/// - `VpcFirewallRuleTarget`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RouteDestinationOneOf {
    RouteTarget(RouteTarget),
    RouteDestinationData(RouteDestinationData),
    VpcFirewallRuleTarget(VpcFirewallRuleTarget),
}

impl RouteDestinationOneOf {
    pub fn route_destination_data(&self) -> Option<&RouteDestinationData> {
        if let RouteDestinationOneOf::RouteDestinationData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn route_target(&self) -> Option<&RouteTarget> {
        if let RouteDestinationOneOf::RouteTarget(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn vpc_firewall_rule_target(&self) -> Option<&VpcFirewallRuleTarget> {
        if let RouteDestinationOneOf::VpcFirewallRuleTarget(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TypeInstance {
    #[serde(rename = "instance")]
    Instance,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TypeInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TypeInstance::Instance => "instance",
            TypeInstance::Noop => "",
            TypeInstance::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TypeInstance {
    fn default() -> TypeInstance {
        TypeInstance::Noop
    }
}
impl TypeInstance {
    pub fn is_noop(&self) -> bool {
        matches!(self, TypeInstance::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteTargetData {
    #[serde(
        default,
        skip_serializing_if = "TypeInstance::is_noop",
        rename = "type"
    )]
    pub type_: TypeInstance,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TypeInternetGateway {
    #[serde(rename = "internetGateway")]
    InternetGateway,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TypeInternetGateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TypeInternetGateway::InternetGateway => "internetGateway",
            TypeInternetGateway::Noop => "",
            TypeInternetGateway::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TypeInternetGateway {
    fn default() -> TypeInternetGateway {
        TypeInternetGateway::Noop
    }
}
impl TypeInternetGateway {
    pub fn is_noop(&self) -> bool {
        matches!(self, TypeInternetGateway::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouteTargetDataType {
    #[serde(
        default,
        skip_serializing_if = "TypeInternetGateway::is_noop",
        rename = "type"
    )]
    pub type_: TypeInternetGateway,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// All of the following types:
///
/// - `RouteTarget`
/// - `RouteDestinationData`
/// - `VpcFirewallRuleTarget`
/// - `RouteTargetData`
/// - `RouteTargetDataType`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RouteTargetOneOf {
    RouteTarget(RouteTarget),
    RouteDestinationData(RouteDestinationData),
    VpcFirewallRuleTarget(VpcFirewallRuleTarget),
    RouteTargetData(RouteTargetData),
    RouteTargetDataType(RouteTargetDataType),
}

impl RouteTargetOneOf {
    pub fn route_destination_data(&self) -> Option<&RouteDestinationData> {
        if let RouteTargetOneOf::RouteDestinationData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn route_target(&self) -> Option<&RouteTarget> {
        if let RouteTargetOneOf::RouteTarget(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn route_target_data(&self) -> Option<&RouteTargetData> {
        if let RouteTargetOneOf::RouteTargetData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn route_target_data_type(&self) -> Option<&RouteTargetDataType> {
        if let RouteTargetOneOf::RouteTargetDataType(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn vpc_firewall_rule_target(&self) -> Option<&VpcFirewallRuleTarget> {
        if let RouteTargetOneOf::VpcFirewallRuleTarget(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

/// A route defines a rule that governs where traffic should be sent based on its destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterRoute {
    #[serde()]
    pub destination: RouteDestinationOneOf,
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
    /**
     * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
     *  
     *  See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
     */
    #[serde()]
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
    pub target: RouteTargetOneOf,
}

/// Create-time parameters for a [`RouterRoute`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterRouteCreateParams {
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
    pub destination: RouteDestinationOneOf,
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
    pub target: RouteTargetOneOf,
}

/**
 * The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.
 *   
 *   See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RouterRouteKind {
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "VpcPeering")]
    VpcPeering,
    #[serde(rename = "VpcSubnet")]
    VpcSubnet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RouterRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RouterRouteKind::Custom => "Custom",
            RouterRouteKind::Default => "Default",
            RouterRouteKind::VpcPeering => "VpcPeering",
            RouterRouteKind::VpcSubnet => "VpcSubnet",
            RouterRouteKind::Noop => "",
            RouterRouteKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RouterRouteKind {
    fn default() -> RouterRouteKind {
        RouterRouteKind::Noop
    }
}
impl RouterRouteKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, RouterRouteKind::Noop)
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterRouteResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RouterRouteUpdateParams {
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
    pub destination: RouteDestinationOneOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde()]
    pub target: RouteTargetOneOf,
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
    pub state: SagaStateOneOf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorActionFailed {
    #[serde(rename = "actionFailed")]
    ActionFailed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorActionFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorActionFailed::ActionFailed => "actionFailed",
            ErrorActionFailed::Noop => "",
            ErrorActionFailed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorActionFailed {
    fn default() -> ErrorActionFailed {
        ErrorActionFailed::Noop
    }
}
impl ErrorActionFailed {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorActionFailed::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaErrorInfo {
    #[serde(default, skip_serializing_if = "ErrorActionFailed::is_noop")]
    pub error: ErrorActionFailed,
    #[serde()]
    pub source_error: serde_json::Value,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorDeserializeFailed {
    #[serde(rename = "deserializeFailed")]
    DeserializeFailed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorDeserializeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorDeserializeFailed::DeserializeFailed => "deserializeFailed",
            ErrorDeserializeFailed::Noop => "",
            ErrorDeserializeFailed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorDeserializeFailed {
    fn default() -> ErrorDeserializeFailed {
        ErrorDeserializeFailed::Noop
    }
}
impl ErrorDeserializeFailed {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorDeserializeFailed::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaErrorInfoData {
    #[serde(default, skip_serializing_if = "ErrorDeserializeFailed::is_noop")]
    pub error: ErrorDeserializeFailed,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorInjected {
    #[serde(rename = "injectedError")]
    InjectedError,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorInjected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorInjected::InjectedError => "injectedError",
            ErrorInjected::Noop => "",
            ErrorInjected::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorInjected {
    fn default() -> ErrorInjected {
        ErrorInjected::Noop
    }
}
impl ErrorInjected {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorInjected::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaErrorInfoDataType {
    #[serde(default, skip_serializing_if = "ErrorInjected::is_noop")]
    pub error: ErrorInjected,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorSerializeFailed {
    #[serde(rename = "serializeFailed")]
    SerializeFailed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorSerializeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorSerializeFailed::SerializeFailed => "serializeFailed",
            ErrorSerializeFailed::Noop => "",
            ErrorSerializeFailed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorSerializeFailed {
    fn default() -> ErrorSerializeFailed {
        ErrorSerializeFailed::Noop
    }
}
impl ErrorSerializeFailed {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorSerializeFailed::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaErrorInfoDataTypeLinks {
    #[serde(default, skip_serializing_if = "ErrorSerializeFailed::is_noop")]
    pub error: ErrorSerializeFailed,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorSubsagaCreateFailed {
    #[serde(rename = "subsagaCreateFailed")]
    SubsagaCreateFailed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorSubsagaCreateFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorSubsagaCreateFailed::SubsagaCreateFailed => "subsagaCreateFailed",
            ErrorSubsagaCreateFailed::Noop => "",
            ErrorSubsagaCreateFailed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorSubsagaCreateFailed {
    fn default() -> ErrorSubsagaCreateFailed {
        ErrorSubsagaCreateFailed::Noop
    }
}
impl ErrorSubsagaCreateFailed {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorSubsagaCreateFailed::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaErrorInfoDataTypeLinksObject {
    #[serde(default, skip_serializing_if = "ErrorSubsagaCreateFailed::is_noop")]
    pub error: ErrorSubsagaCreateFailed,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// All of the following types:
///
/// - `SagaErrorInfo`
/// - `SagaErrorInfoData`
/// - `SagaErrorInfoDataType`
/// - `SagaErrorInfoDataTypeLinks`
/// - `SagaErrorInfoDataTypeLinksObject`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SagaErrorInfoOneOf {
    SagaErrorInfo(SagaErrorInfo),
    SagaErrorInfoData(SagaErrorInfoData),
    SagaErrorInfoDataType(SagaErrorInfoDataType),
    SagaErrorInfoDataTypeLinks(SagaErrorInfoDataTypeLinks),
    SagaErrorInfoDataTypeLinksObject(SagaErrorInfoDataTypeLinksObject),
}

impl SagaErrorInfoOneOf {
    pub fn saga_error_info(&self) -> Option<&SagaErrorInfo> {
        if let SagaErrorInfoOneOf::SagaErrorInfo(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_error_info_data(&self) -> Option<&SagaErrorInfoData> {
        if let SagaErrorInfoOneOf::SagaErrorInfoData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_error_info_data_type(&self) -> Option<&SagaErrorInfoDataType> {
        if let SagaErrorInfoOneOf::SagaErrorInfoDataType(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_error_info_data_type_links(&self) -> Option<&SagaErrorInfoDataTypeLinks> {
        if let SagaErrorInfoOneOf::SagaErrorInfoDataTypeLinks(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_error_info_data_type_links_object(
        &self,
    ) -> Option<&SagaErrorInfoDataTypeLinksObject> {
        if let SagaErrorInfoOneOf::SagaErrorInfoDataTypeLinksObject(ref_) = self {
            return Some(ref_);
        }
        None
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
pub enum StateRunning {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateRunning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateRunning::Running => "running",
            StateRunning::Noop => "",
            StateRunning::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateRunning {
    fn default() -> StateRunning {
        StateRunning::Noop
    }
}
impl StateRunning {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateRunning::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaState {
    #[serde(default, skip_serializing_if = "StateRunning::is_noop")]
    pub state: StateRunning,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateSucceeded {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateSucceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateSucceeded::Succeeded => "succeeded",
            StateSucceeded::Noop => "",
            StateSucceeded::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateSucceeded {
    fn default() -> StateSucceeded {
        StateSucceeded::Noop
    }
}
impl StateSucceeded {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateSucceeded::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaStateData {
    #[serde(default, skip_serializing_if = "StateSucceeded::is_noop")]
    pub state: StateSucceeded,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StateFailed {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StateFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StateFailed::Failed => "failed",
            StateFailed::Noop => "",
            StateFailed::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StateFailed {
    fn default() -> StateFailed {
        StateFailed::Noop
    }
}
impl StateFailed {
    pub fn is_noop(&self) -> bool {
        matches!(self, StateFailed::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SagaStateDataType {
    #[serde()]
    pub error_info: SagaErrorInfoOneOf,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_node_name: String,
    #[serde(default, skip_serializing_if = "StateFailed::is_noop")]
    pub state: StateFailed,
}

/// All of the following types:
///
/// - `SagaState`
/// - `SagaStateData`
/// - `SagaStateDataType`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SagaStateOneOf {
    SagaState(SagaState),
    SagaStateData(SagaStateData),
    SagaStateDataType(SagaStateDataType),
}

impl SagaStateOneOf {
    pub fn saga_state(&self) -> Option<&SagaState> {
        if let SagaStateOneOf::SagaState(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_state_data(&self) -> Option<&SagaStateData> {
        if let SagaStateOneOf::SagaStateData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn saga_state_data_type(&self) -> Option<&SagaStateDataType> {
        if let SagaStateOneOf::SagaStateDataType(ref_) = self {
            return Some(ref_);
        }
        None
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "serviceAddress"
    )]
    pub service_address: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dnsName"
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "projectId"
    )]
    pub project_id: String,
    /**
     * human-readable free-form text about a resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "systemRouterId"
    )]
    pub system_router_id: String,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeCreated"
    )]
    pub time_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * timestamp when this resource was created
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "timeModified"
    )]
    pub time_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create-time parameters for a [`Vpc`]
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dnsName"
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
    pub name: String,
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcFirewallRule {
    #[serde()]
    pub action: VpcFirewallRuleAction,
    #[serde()]
    pub direction: VpcFirewallRuleDirection,
    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    pub filters: VpcFirewallRuleFilter,
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
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
    pub status: VpcFirewallRuleStatus,
    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub targets: Vec<VpcFirewallRuleTargetOneOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
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
        VpcFirewallRuleAction::Noop
    }
}
impl VpcFirewallRuleAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
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
        VpcFirewallRuleDirection::Noop
    }
}
impl VpcFirewallRuleDirection {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleDirection::Noop)
    }
}

/// Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcFirewallRuleFilter {
    /**
     * If present, the sources (if incoming) or destinations (if outgoing) this rule applies to.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hosts: Vec<RouteTargetOneOf>,
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

/**
 * The protocols that may be specified in a firewall rule's filter
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
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
        VpcFirewallRuleProtocol::Noop
    }
}
impl VpcFirewallRuleProtocol {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleProtocol::Noop)
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcFirewallRuleResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<VpcFirewallRule>,
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
        VpcFirewallRuleStatus::Noop
    }
}
impl VpcFirewallRuleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcFirewallRuleStatus::Noop)
    }
}

/// All of the following types:
///
/// - `RouteDestinationData`
/// - `VpcFirewallRuleTarget`
/// - `RouteTargetData`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum VpcFirewallRuleTargetOneOf {
    RouteDestinationData(RouteDestinationData),
    VpcFirewallRuleTarget(VpcFirewallRuleTarget),
    RouteTargetData(RouteTargetData),
}

impl VpcFirewallRuleTargetOneOf {
    pub fn route_destination_data(&self) -> Option<&RouteDestinationData> {
        if let VpcFirewallRuleTargetOneOf::RouteDestinationData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn route_target_data(&self) -> Option<&RouteTargetData> {
        if let VpcFirewallRuleTargetOneOf::RouteTargetData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn vpc_firewall_rule_target(&self) -> Option<&VpcFirewallRuleTarget> {
        if let VpcFirewallRuleTargetOneOf::VpcFirewallRuleTarget(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

/// A single rule in a VPC firewall
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcFirewallRuleUpdate {
    #[serde()]
    pub action: VpcFirewallRuleAction,
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
    pub direction: VpcFirewallRuleDirection,
    /**
     * Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.
     */
    #[serde()]
    pub filters: VpcFirewallRuleFilter,
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
    pub status: VpcFirewallRuleStatus,
    /**
     * list of sets of instances that the rule applies to
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub targets: Vec<VpcFirewallRuleTargetOneOf>,
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
pub struct VpcRouter {
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
    #[serde()]
    pub kind: VpcRouterKind,
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
        VpcRouterKind::Noop
    }
}
impl VpcRouterKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, VpcRouterKind::Noop)
    }
}

/// A single page of results
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcRouterResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
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

/// Updateable properties of a [`VpcRouter`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcRouterUpdate {
    /**
     * Updateable properties of a [`VpcRouter`]
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
pub struct VpcSubnet {
    /**
     * Client view of a [`User`]
     */
    #[serde()]
    pub identity: User,
    /**
     * The IPv4 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv4_block"
    )]
    pub ipv_4_block: String,
    /**
     * The IPv6 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv6_block"
    )]
    pub ipv_6_block: String,
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

/// Create-time parameters for a [`VpcSubnet`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcSubnetCreate {
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
     * The IPv4 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv4Block"
    )]
    pub ipv_4_block: String,
    /**
     * The IPv6 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv6Block"
    )]
    pub ipv_6_block: String,
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
pub struct VpcSubnetResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
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

/// Updateable properties of a [`VpcSubnet`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcSubnetUpdate {
    /**
     * Updateable properties of a [`VpcSubnet`]
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The IPv4 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv4Block"
    )]
    pub ipv_4_block: String,
    /**
     * The IPv6 subnet CIDR block.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipv6Block"
    )]
    pub ipv_6_block: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Updateable properties of a [`Vpc`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VpcUpdate {
    /**
     * Updateable properties of a [`Vpc`]
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dnsName"
    )]
    pub dns_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
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
