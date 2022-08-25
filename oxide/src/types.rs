//! The data types sent to and returned from the API client.
use std::fmt;

use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "start")]
pub enum BinRangedouble {
    RangeTo(f64),
    Range { end: f64, start: f64 },
    RangeFrom(f64),
}

impl fmt::Display for BinRangedouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["start"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["start"].clone()).unwrap_or_default();
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

impl std::str::FromStr for BinRangedouble {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for BinRangedouble, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "range_to" {
            j = format!(
                r#"{{
"type": "range_to",
"start": {}
        }}"#,
                serde_json::json!(f64::from_str(&content).unwrap())
            );
        }
        if tag == "range" {
            j = format!(
                r#"{{
"type": "range",
"start": {}
        }}"#,
                serde_json::json!(f64::from_str(&content).unwrap())
            );
        }
        if tag == "range" {
            j = format!(
                r#"{{
"type": "range",
"start": {}
        }}"#,
                serde_json::json!(f64::from_str(&content).unwrap())
            );
        }
        if tag == "range_from" {
            j = format!(
                r#"{{
"type": "range_from",
"start": {}
        }}"#,
                serde_json::json!(f64::from_str(&content).unwrap())
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl BinRangedouble {
    pub fn variants() -> Vec<String> {
        vec![
            "range".to_string(),
            "range_from".to_string(),
            "range_to".to_string(),
        ]
    }
}
/**
 * The types for BinRangedouble.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum BinRangedoubleType {
    Range,
    RangeFrom,
    RangeTo,
}

impl std::fmt::Display for BinRangedoubleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BinRangedoubleType::Range => "range",
            BinRangedoubleType::RangeFrom => "range_from",
            BinRangedoubleType::RangeTo => "range_to",
        }
        .fmt(f)
    }
}

impl Default for BinRangedoubleType {
    fn default() -> BinRangedoubleType {
        BinRangedoubleType::Range
    }
}
impl std::str::FromStr for BinRangedoubleType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "range" {
            return Ok(BinRangedoubleType::Range);
        }
        if s == "range_from" {
            return Ok(BinRangedoubleType::RangeFrom);
        }
        if s == "range_to" {
            return Ok(BinRangedoubleType::RangeTo);
        }
        anyhow::bail!("invalid string for BinRangedoubleType: {}", s);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "start")]
pub enum BinRangeint64 {
    RangeTo(i64),
    Range { end: i64, start: i64 },
    RangeFrom(i64),
}

impl fmt::Display for BinRangeint64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["start"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["start"].clone()).unwrap_or_default();
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

impl std::str::FromStr for BinRangeint64 {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for BinRangeint64, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "range_to" {
            j = format!(
                r#"{{
"type": "range_to",
"start": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        if tag == "range" {
            j = format!(
                r#"{{
"type": "range",
"start": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        if tag == "range" {
            j = format!(
                r#"{{
"type": "range",
"start": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        if tag == "range_from" {
            j = format!(
                r#"{{
"type": "range_from",
"start": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl BinRangeint64 {
    pub fn variants() -> Vec<String> {
        vec![
            "range".to_string(),
            "range_from".to_string(),
            "range_to".to_string(),
        ]
    }
}
/**
 * The types for BinRangeint64.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum BinRangeint64Type {
    Range,
    RangeFrom,
    RangeTo,
}

impl std::fmt::Display for BinRangeint64Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BinRangeint64Type::Range => "range",
            BinRangeint64Type::RangeFrom => "range_from",
            BinRangeint64Type::RangeTo => "range_to",
        }
        .fmt(f)
    }
}

impl Default for BinRangeint64Type {
    fn default() -> BinRangeint64Type {
        BinRangeint64Type::Range
    }
}
impl std::str::FromStr for BinRangeint64Type {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "range" {
            return Ok(BinRangeint64Type::Range);
        }
        if s == "range_from" {
            return Ok(BinRangeint64Type::RangeFrom);
        }
        if s == "range_to" {
            return Ok(BinRangeint64Type::RangeTo);
        }
        anyhow::bail!("invalid string for BinRangeint64Type: {}", s);
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Bindouble {
    /**
     * The total count of samples in this bin.
     */
    #[serde(default)]
    pub count: u64,

    #[serde()]
    pub range: BinRangedouble,
}

/// Type storing bin edges and a count of samples within it.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Binint64 {
    /**
     * The total count of samples in this bin.
     */
    #[serde(default)]
    pub count: u64,

    #[serde()]
    pub range: BinRangeint64,
}

/// A cumulative or counter data type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Cumulativedouble {
    #[serde()]
    pub start_time: crate::utils::DisplayOptionDateTime,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

/// A cumulative or counter data type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Cumulativeint64 {
    #[serde()]
    pub start_time: crate::utils::DisplayOptionDateTime,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub value: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum Datum {
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

impl std::fmt::Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Datum::Bool => "bool",
            Datum::Bytes => "bytes",
            Datum::CumulativeF64 => "cumulative_f_64",
            Datum::CumulativeI64 => "cumulative_i_64",
            Datum::F64 => "f_64",
            Datum::HistogramF64 => "histogram_f_64",
            Datum::HistogramI64 => "histogram_i_64",
            Datum::I64 => "i_64",
            Datum::String => "string",
            Datum::Noop => "",
            Datum::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Datum {
    fn default() -> Datum {
        Datum::Bool
    }
}
impl std::str::FromStr for Datum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "bool" {
            return Ok(Datum::Bool);
        }
        if s == "bytes" {
            return Ok(Datum::Bytes);
        }
        if s == "cumulative_f_64" {
            return Ok(Datum::CumulativeF64);
        }
        if s == "cumulative_i_64" {
            return Ok(Datum::CumulativeI64);
        }
        if s == "f_64" {
            return Ok(Datum::F64);
        }
        if s == "histogram_f_64" {
            return Ok(Datum::HistogramF64);
        }
        if s == "histogram_i_64" {
            return Ok(Datum::HistogramI64);
        }
        if s == "i_64" {
            return Ok(Datum::I64);
        }
        if s == "string" {
            return Ok(Datum::String);
        }
        anyhow::bail!("invalid string for Datum: {}", s);
    }
}
impl Datum {
    pub fn is_noop(&self) -> bool {
        matches!(self, Datum::Noop)
    }
}

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
            DatumType::CumulativeF64 => "cumulative_f_64",
            DatumType::CumulativeI64 => "cumulative_i_64",
            DatumType::F64 => "f_64",
            DatumType::HistogramF64 => "histogram_f_64",
            DatumType::HistogramI64 => "histogram_i_64",
            DatumType::I64 => "i_64",
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
        if s == "cumulative_f_64" {
            return Ok(DatumType::CumulativeF64);
        }
        if s == "cumulative_i_64" {
            return Ok(DatumType::CumulativeI64);
        }
        if s == "f_64" {
            return Ok(DatumType::F64);
        }
        if s == "histogram_f_64" {
            return Ok(DatumType::HistogramF64);
        }
        if s == "histogram_i_64" {
            return Ok(DatumType::HistogramI64);
        }
        if s == "i_64" {
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DerEncodedKeyPair {
    /**
     * request signing private key (base64 encoded der file)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub private_key: String,

    /**
     * request signing public certificate (base64 encoded der file)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_cert: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DeviceAccessTokenRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub device_code: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub grant_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DeviceAuthRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct DeviceAuthVerify {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "value")]
pub enum Digest {
    Sha256(String),
}

impl fmt::Display for Digest {
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

impl std::str::FromStr for Digest {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for Digest, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "sha_256" {
            j = format!(
                r#"{{
"type": "sha_256",
"value": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl Digest {
    pub fn variants() -> Vec<String> {
        vec!["sha_256".to_string()]
    }
}
/**
 * The types for Digest.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum DigestType {
    Sha256,
}

impl std::fmt::Display for DigestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DigestType::Sha256 => "sha_256",
        }
        .fmt(f)
    }
}

impl Default for DigestType {
    fn default() -> DigestType {
        DigestType::Sha256
    }
}
impl std::str::FromStr for DigestType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "sha_256" {
            return Ok(DigestType::Sha256);
        }
        anyhow::bail!("invalid string for DigestType: {}", s);
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

/// Client view of a [`Disk`]
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
#[serde(tag = "type")]
pub enum DiskSource {
    Blank { block_size: i64 },
    Snapshot { snapshot_id: String },
    Image { image_id: String },
    GlobalImage { image_id: String },
}

impl fmt::Display for DiskSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();

        let mut value = "image_id";
        if tag == *"blank" {
            value = "block_size";
        };
        if tag == *"snapshot" {
            value = "snapshot_id";
        };

        let mut content: String = match serde_json::from_value(j[value].clone()) {
            Ok(v) => v,
            Err(_) => {
                let int: i64 = serde_json::from_value(j[value].clone()).unwrap_or_default();
                format!("{}", int)
            }
        };
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j[value].clone()).unwrap_or_default();
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
"block_size": {}
        }}"#,
                serde_json::json!(i64::from_str(&content).unwrap())
            );
        }
        if tag == "snapshot" {
            j = format!(
                r#"{{
"type": "snapshot",
"snapshot_id": "{}"
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// OS image distribution
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Distribution {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
     * The version of the distribution (e.g. "3.10" or "18.04")
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

impl fmt::Display for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
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
 * The kind of an external IP address for an instance
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IpKind {
    Ephemeral,
    Floating,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IpKind::Ephemeral => "ephemeral",
            IpKind::Floating => "floating",
            IpKind::Noop => "",
            IpKind::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IpKind {
    fn default() -> IpKind {
        IpKind::Ephemeral
    }
}
impl std::str::FromStr for IpKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ephemeral" {
            return Ok(IpKind::Ephemeral);
        }
        if s == "floating" {
            return Ok(IpKind::Floating);
        }
        anyhow::bail!("invalid string for IpKind: {}", s);
    }
}
impl IpKind {
    pub fn is_noop(&self) -> bool {
        matches!(self, IpKind::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ExternalIp {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,

    /**
     * The kind of an external IP address for an instance
     */
    #[serde(default, skip_serializing_if = "IpKind::is_noop")]
    pub kind: IpKind,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "pool_name")]
pub enum ExternalIpCreate {
    Ephemeral(String),
}

impl fmt::Display for ExternalIpCreate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String =
            serde_json::from_value(j["pool_name"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["pool_name"].clone()).unwrap_or_default();
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

impl std::str::FromStr for ExternalIpCreate {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for ExternalIpCreate, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "ephemeral" {
            j = format!(
                r#"{{
"type": "ephemeral",
"pool_name": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl ExternalIpCreate {
    pub fn variants() -> Vec<String> {
        vec!["ephemeral".to_string()]
    }
}
/**
 * The types for ExternalIpCreate.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum ExternalIpCreateType {
    Ephemeral,
}

impl std::fmt::Display for ExternalIpCreateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ExternalIpCreateType::Ephemeral => "ephemeral",
        }
        .fmt(f)
    }
}

impl Default for ExternalIpCreateType {
    fn default() -> ExternalIpCreateType {
        ExternalIpCreateType::Ephemeral
    }
}
impl std::str::FromStr for ExternalIpCreateType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ephemeral" {
            return Ok(ExternalIpCreateType::Ephemeral);
        }
        anyhow::bail!("invalid string for ExternalIpCreateType: {}", s);
    }
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ExternalIpResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<ExternalIp>,

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
            FieldType::I64 => "i_64",
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
        if s == "i_64" {
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum FleetRole {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FleetRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FleetRole::Admin => "admin",
            FleetRole::Collaborator => "collaborator",
            FleetRole::Viewer => "viewer",
            FleetRole::Noop => "",
            FleetRole::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FleetRole {
    fn default() -> FleetRole {
        FleetRole::Admin
    }
}
impl std::str::FromStr for FleetRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(FleetRole::Admin);
        }
        if s == "collaborator" {
            return Ok(FleetRole::Collaborator);
        }
        if s == "viewer" {
            return Ok(FleetRole::Viewer);
        }
        anyhow::bail!("invalid string for FleetRole: {}", s);
    }
}
impl FleetRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, FleetRole::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FleetRoleAssignment {
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

    #[serde(default, skip_serializing_if = "FleetRole::is_noop")]
    pub role_name: FleetRole,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FleetRolePolicy {
    /**
     * Roles directly assigned on this resource
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<FleetRoleAssignment>,
}

/**
 * Describes what kind of identity is described by an id
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IdentityType {
    SiloGroup,
    SiloUser,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdentityType::SiloGroup => "silo_group",
            IdentityType::SiloUser => "silo_user",
            IdentityType::Noop => "",
            IdentityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityType {
    fn default() -> IdentityType {
        IdentityType::SiloGroup
    }
}
impl std::str::FromStr for IdentityType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "silo_group" {
            return Ok(IdentityType::SiloGroup);
        }
        if s == "silo_user" {
            return Ok(IdentityType::SiloUser);
        }
        anyhow::bail!("invalid string for IdentityType: {}", s);
    }
}
impl IdentityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityType::Noop)
    }
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Image distribution
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub distribution: String,

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
     * Image version
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "id")]
pub enum ImageSource {
    Url(String),
    Snapshot(String),
    YouCanBootAnythingAsLongItsAlpine,
}

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["id"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["id"].clone()).unwrap_or_default();
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

impl std::str::FromStr for ImageSource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for ImageSource, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "url" {
            j = format!(
                r#"{{
"type": "url",
"id": "{}"
        }}"#,
                content
            );
        }
        if tag == "snapshot" {
            j = format!(
                r#"{{
"type": "snapshot",
"id": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl ImageSource {
    pub fn variants() -> Vec<String> {
        vec![
            "snapshot".to_string(),
            "url".to_string(),
            "you_can_boot_anything_as_long_its_alpine".to_string(),
        ]
    }
}
/**
 * The types for ImageSource.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum ImageSourceType {
    Snapshot,
    Url,
    YouCanBootAnythingAsLongItsAlpine,
}

impl std::fmt::Display for ImageSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ImageSourceType::Snapshot => "snapshot",
            ImageSourceType::Url => "url",
            ImageSourceType::YouCanBootAnythingAsLongItsAlpine => {
                "you_can_boot_anything_as_long_its_alpine"
            }
        }
        .fmt(f)
    }
}

impl Default for ImageSourceType {
    fn default() -> ImageSourceType {
        ImageSourceType::Snapshot
    }
}
impl std::str::FromStr for ImageSourceType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "snapshot" {
            return Ok(ImageSourceType::Snapshot);
        }
        if s == "url" {
            return Ok(ImageSourceType::Url);
        }
        if s == "you_can_boot_anything_as_long_its_alpine" {
            return Ok(ImageSourceType::YouCanBootAnythingAsLongItsAlpine);
        }
        anyhow::bail!("invalid string for ImageSourceType: {}", s);
    }
}

/// Create-time parameters for an [`GlobalImage`](crate::external_api::views::GlobalImage)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct GlobalImageCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

    /**
     * OS image distribution
     */
    #[serde()]
    pub distribution: Distribution,

    #[serde()]
    pub source: ImageSource,
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

/// A simple type for managing a histogram metric.
///
/// A histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any "gaps" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an error.
///
/// Example ------- ```rust use oximeter::histogram::{BinRange, Histogram};
///
/// let edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);
///
/// let data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin
///
/// assert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```
///
/// Notes -----
///
/// Histograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.
///
/// The short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Histogramdouble {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub bins: Vec<Bindouble>,

    #[serde(default)]
    pub n_samples: u64,

    #[serde()]
    pub start_time: crate::utils::DisplayOptionDateTime,
}

/// A simple type for managing a histogram metric.
///
/// A histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any "gaps" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an error.
///
/// Example ------- ```rust use oximeter::histogram::{BinRange, Histogram};
///
/// let edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);
///
/// let data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin
///
/// assert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```
///
/// Notes -----
///
/// Histograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.
///
/// The short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Histogramint64 {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub bins: Vec<Binint64>,

    #[serde(default)]
    pub n_samples: u64,

    #[serde()]
    pub start_time: crate::utils::DisplayOptionDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IdentityProviderType {
    Saml,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdentityProviderType::Saml => "saml",
            IdentityProviderType::Noop => "",
            IdentityProviderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityProviderType {
    fn default() -> IdentityProviderType {
        IdentityProviderType::Saml
    }
}
impl std::str::FromStr for IdentityProviderType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "saml" {
            return Ok(IdentityProviderType::Saml);
        }
        anyhow::bail!("invalid string for IdentityProviderType: {}", s);
    }
}
impl IdentityProviderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityProviderType::Noop)
    }
}

/// Client view of an [`IdentityProvider`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IdentityProvider {
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

    #[serde(default, skip_serializing_if = "IdentityProviderType::is_noop")]
    pub provider_type: IdentityProviderType,

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
pub struct IdentityProviderResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<IdentityProvider>,

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
pub enum IdentityProviderTypeSaml {
    Saml,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityProviderTypeSaml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdentityProviderTypeSaml::Saml => "saml",
            IdentityProviderTypeSaml::Noop => "",
            IdentityProviderTypeSaml::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityProviderTypeSaml {
    fn default() -> IdentityProviderTypeSaml {
        IdentityProviderTypeSaml::Saml
    }
}
impl std::str::FromStr for IdentityProviderTypeSaml {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "saml" {
            return Ok(IdentityProviderTypeSaml::Saml);
        }
        anyhow::bail!("invalid string for IdentityProviderTypeSaml: {}", s);
    }
}
impl IdentityProviderTypeSaml {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityProviderTypeSaml::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum IdpMetadataSource {
    Url(String),
    Base64EncodedXml(String),
}

impl fmt::Display for IdpMetadataSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::json!(self);
        let mut tag: String = serde_json::from_value(j["type"].clone()).unwrap_or_default();
        let mut content: String = serde_json::from_value(j["data"].clone()).unwrap_or_default();
        if content.is_empty() {
            let map: std::collections::HashMap<String, String> =
                serde_json::from_value(j["data"].clone()).unwrap_or_default();
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

impl std::str::FromStr for IdpMetadataSource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            anyhow::bail!("invalid format for IdpMetadataSource, got {}", s);
        }
        let tag = parts[0].to_string();
        let content = parts[1].to_string();
        let mut j = String::new();
        if tag == "url" {
            j = format!(
                r#"{{
"type": "url",
"data": "{}"
        }}"#,
                content
            );
        }
        if tag == "base_64_encoded_xml" {
            j = format!(
                r#"{{
"type": "base_64_encoded_xml",
"data": "{}"
        }}"#,
                content
            );
        }
        let result = serde_json::from_str(&j)?;
        Ok(result)
    }
}
impl IdpMetadataSource {
    pub fn variants() -> Vec<String> {
        vec!["base_64_encoded_xml".to_string(), "url".to_string()]
    }
}
/**
 * The types for IdpMetadataSource.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum IdpMetadataSourceType {
    Base64EncodedXml,
    Url,
}

impl std::fmt::Display for IdpMetadataSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IdpMetadataSourceType::Base64EncodedXml => "base_64_encoded_xml",
            IdpMetadataSourceType::Url => "url",
        }
        .fmt(f)
    }
}

impl Default for IdpMetadataSourceType {
    fn default() -> IdpMetadataSourceType {
        IdpMetadataSourceType::Base64EncodedXml
    }
}
impl std::str::FromStr for IdpMetadataSourceType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "base_64_encoded_xml" {
            return Ok(IdpMetadataSourceType::Base64EncodedXml);
        }
        if s == "url" {
            return Ok(IdpMetadataSourceType::Url);
        }
        anyhow::bail!("invalid string for IdpMetadataSourceType: {}", s);
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// Create-time parameters for an [`Image`](crate::external_api::views::Image)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct ImageCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * The disks to be created or attached for this instance.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub disks: Vec<InstanceDiskAttachment>,

    /**
     * The external IP addresses provided to this instance.
     *  
     *  By default, all instances have outbound connectivity, but no inbound connectivity. These external addresses can be used to provide a fixed, known IP address for making inbound connections to the instance.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub external_ips: Vec<ExternalIpCreate>,

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
     * Should this instance be started upon creation; true by default.
     */
    #[serde(default = "crate::utils::bool_true")]
    pub start: bool,

    /**
     * User data for instance initialization systems (such as cloud-init). Must be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and / characters with padding). Maximum 32 KiB unencoded data.
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
    pub dst_sled_id: String,
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

/// Contents of an Instance's serial console buffer.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InstanceSerialConsoleData {
    /**
     * The bytes starting from the requested offset up to either the end of the buffer or the request's `max_bytes`. Provided as a u8 array rather than a string, as it may not be UTF-8.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub data: Vec<u8>,

    /**
     * The absolute offset since boot (suitable for use as `byte_offset` in a subsequent request) of the last byte returned in `data`.
     */
    #[serde(default)]
    pub last_byte_offset: u64,
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

/// Identity-related metadata that's included in nearly all public API objects
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPool {
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
    pub project_id: String,

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

/// Create-time parameters for an IP Pool.
///
/// See [`IpPool`](crate::external_api::views::IpPool)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPoolCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Create-time parameters for an IP Pool.
     *  
     *  See [`IpPool`](crate::external_api::views::IpPool)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization: String,

    /**
     * Create-time parameters for an IP Pool.
     *  
     *  See [`IpPool`](crate::external_api::views::IpPool)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, JsonSchema)]
pub enum IpRange {
    Ipv4Range,
    #[default]
    Ipv6Range,
}

impl fmt::Display for IpRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for IpRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPoolRange {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    #[serde()]
    pub range: IpRange,

    #[serde()]
    pub time_created: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPoolRangeResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<IpPoolRange>,

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

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPoolResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<IpPool>,

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

/// Parameters for updating an IP Pool
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct IpPoolUpdate {
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
/// A non-decreasing IPv4 address range, inclusive of both ends.
///
/// The first address must be less than or equal to the last address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Ipv4Range {
    #[serde()]
    pub first: std::net::Ipv4Addr,

    #[serde()]
    pub last: std::net::Ipv4Addr,
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
/// A non-decreasing IPv6 address range, inclusive of both ends.
///
/// The first address must be less than or equal to the last address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct Ipv6Range {
    #[serde()]
    pub first: std::net::Ipv6Addr,

    #[serde()]
    pub last: std::net::Ipv6Addr,
}

/// A `Measurement` is a timestamped datum from a single metric
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Measurement {
    #[serde()]
    pub datum: Datum,

    #[serde()]
    pub timestamp: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct MeasurementResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<Measurement>,

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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * True if this interface is the primary for the instance to which it's attached.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,

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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subnet_name: String,

    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// Parameters for updating a [`NetworkInterface`](omicron_common::api::external::NetworkInterface).
///
/// Note that modifying IP addresses for an interface is not yet supported, a new interface must be created instead.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct NetworkInterfaceUpdate {
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
     * Make a secondary interface the instance's primary interface.
     *  
     *  If applied to a secondary interface, that interface will become the primary on the next reboot of the instance. Note that this may have implications for routing between instances, as the new primary interface will be on a distinct subnet from the previous primary interface.
     *  
     *  Note that this can only be used to select a new primary interface for an instance. Requests to change the primary interface into a secondary will return an error.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
pub enum OrganizationRole {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrganizationRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrganizationRole::Admin => "admin",
            OrganizationRole::Collaborator => "collaborator",
            OrganizationRole::Viewer => "viewer",
            OrganizationRole::Noop => "",
            OrganizationRole::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrganizationRole {
    fn default() -> OrganizationRole {
        OrganizationRole::Admin
    }
}
impl std::str::FromStr for OrganizationRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(OrganizationRole::Admin);
        }
        if s == "collaborator" {
            return Ok(OrganizationRole::Collaborator);
        }
        if s == "viewer" {
            return Ok(OrganizationRole::Viewer);
        }
        anyhow::bail!("invalid string for OrganizationRole: {}", s);
    }
}
impl OrganizationRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrganizationRole::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationRoleAssignment {
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

    #[serde(default, skip_serializing_if = "OrganizationRole::is_noop")]
    pub role_name: OrganizationRole,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OrganizationRolePolicy {
    /**
     * Roles directly assigned on this resource
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<OrganizationRoleAssignment>,
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
pub enum ProjectRole {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProjectRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProjectRole::Admin => "admin",
            ProjectRole::Collaborator => "collaborator",
            ProjectRole::Viewer => "viewer",
            ProjectRole::Noop => "",
            ProjectRole::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProjectRole {
    fn default() -> ProjectRole {
        ProjectRole::Admin
    }
}
impl std::str::FromStr for ProjectRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(ProjectRole::Admin);
        }
        if s == "collaborator" {
            return Ok(ProjectRole::Collaborator);
        }
        if s == "viewer" {
            return Ok(ProjectRole::Viewer);
        }
        anyhow::bail!("invalid string for ProjectRole: {}", s);
    }
}
impl ProjectRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProjectRole::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectRoleAssignment {
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

    #[serde(default, skip_serializing_if = "ProjectRole::is_noop")]
    pub role_name: ProjectRole,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ProjectRolePolicy {
    /**
     * Roles directly assigned on this resource
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<ProjectRoleAssignment>,
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// Identity-related metadata that's included in nearly all public API objects
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SamlIdentityProvider {
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * service provider endpoint where the response will be sent
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub acs_url: String,

    /**
     * idp's entity id
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub idp_entity_id: String,

    /**
     * optional request signing public certificate (base64 encoded der file)
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_cert: String,

    /**
     * service provider endpoint where the idp should send log out requests
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slo_url: String,

    /**
     * sp's client id
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sp_client_id: String,

    /**
     * customer's technical contact for saml configuration
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub technical_contact_email: String,

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

/// Create-time identity-related parameters
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub struct SamlIdentityProviderCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * service provider endpoint where the response will be sent
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub acs_url: String,

    /**
     * If set, SAML attributes with this name will be considered to denote a user's group membership, where the attribute value(s) should be a comma-separated list of group names.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_attribute_name: String,

    /**
     * idp's entity id
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub idp_entity_id: String,

    #[serde()]
    pub idp_metadata_source: IdpMetadataSource,

    /**
     * optional request signing key pair
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[header(hidden = true)]
    pub signing_keypair: Option<DerEncodedKeyPair>,

    /**
     * service provider endpoint where the idp should send log out requests
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slo_url: String,

    /**
     * sp's client id
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sp_client_id: String,

    /**
     * customer's technical contact for saml configuration
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub technical_contact_email: String,
}

/**
 * How users will be provisioned in a silo during authentication.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum UserProvisionType {
    Fixed,
    Jit,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserProvisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UserProvisionType::Fixed => "fixed",
            UserProvisionType::Jit => "jit",
            UserProvisionType::Noop => "",
            UserProvisionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserProvisionType {
    fn default() -> UserProvisionType {
        UserProvisionType::Fixed
    }
}
impl std::str::FromStr for UserProvisionType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "fixed" {
            return Ok(UserProvisionType::Fixed);
        }
        if s == "jit" {
            return Ok(UserProvisionType::Jit);
        }
        anyhow::bail!("invalid string for UserProvisionType: {}", s);
    }
}
impl UserProvisionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserProvisionType::Noop)
    }
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

    /**
     * How users will be provisioned in a silo during authentication.
     */
    #[serde(default, skip_serializing_if = "UserProvisionType::is_noop")]
    pub user_provision_type: UserProvisionType,
}

/// Create-time parameters for a [`Silo`](crate::external_api::views::Silo)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * If set, this group will be created during Silo creation and granted the "Silo Admin" role. Identity providers can assert that users belong to this group and those users can log in and further initialize the Silo.
     *  
     *  Note that if configuring a SAML based identity provider, group_attribute_name must be set for users to be considered part of a group. See [`SamlIdentityProviderCreate`] for more information.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub admin_group_name: String,

    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub discoverable: bool,

    /**
     * How users will be provisioned in a silo during authentication.
     */
    #[serde(default, skip_serializing_if = "UserProvisionType::is_noop")]
    pub user_provision_type: UserProvisionType,
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
pub enum SiloRole {
    Admin,
    Collaborator,
    Viewer,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SiloRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SiloRole::Admin => "admin",
            SiloRole::Collaborator => "collaborator",
            SiloRole::Viewer => "viewer",
            SiloRole::Noop => "",
            SiloRole::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SiloRole {
    fn default() -> SiloRole {
        SiloRole::Admin
    }
}
impl std::str::FromStr for SiloRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            return Ok(SiloRole::Admin);
        }
        if s == "collaborator" {
            return Ok(SiloRole::Collaborator);
        }
        if s == "viewer" {
            return Ok(SiloRole::Viewer);
        }
        anyhow::bail!("invalid string for SiloRole: {}", s);
    }
}
impl SiloRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, SiloRole::Noop)
    }
}

/// Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloRoleAssignment {
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

    #[serde(default, skip_serializing_if = "SiloRole::is_noop")]
    pub role_name: SiloRole,
}

/// Client view of a [`Policy`], which describes how this resource may be accessed
///
/// Note that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SiloRolePolicy {
    /**
     * Roles directly assigned on this resource
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub role_assignments: Vec<SiloRoleAssignment>,
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// Create-time parameters for a [`Snapshot`](crate::external_api::views::Snapshot)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SnapshotCreate {
    /**
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct SpoofLoginBody {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
     * Human-readable name that can identify the user
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
}

/// Client view of a [`UserBuiltin`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct UserBuiltin {
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
pub struct UserBuiltinResultsPage {
    /**
     * list of items on this page of results
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[header(hidden = true)]
    pub items: Vec<UserBuiltin>,

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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

/// Collection of a Vpc's firewall rules
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(rename_all = "snake_case")]
pub enum DiskMetricName {
    Activated,
    Flush,
    Read,
    ReadBytes,
    Write,
    WriteBytes,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DiskMetricName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DiskMetricName::Activated => "activated",
            DiskMetricName::Flush => "flush",
            DiskMetricName::Read => "read",
            DiskMetricName::ReadBytes => "read_bytes",
            DiskMetricName::Write => "write",
            DiskMetricName::WriteBytes => "write_bytes",
            DiskMetricName::Noop => "",
            DiskMetricName::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DiskMetricName {
    fn default() -> DiskMetricName {
        DiskMetricName::Activated
    }
}
impl std::str::FromStr for DiskMetricName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "activated" {
            return Ok(DiskMetricName::Activated);
        }
        if s == "flush" {
            return Ok(DiskMetricName::Flush);
        }
        if s == "read" {
            return Ok(DiskMetricName::Read);
        }
        if s == "read_bytes" {
            return Ok(DiskMetricName::ReadBytes);
        }
        if s == "write" {
            return Ok(DiskMetricName::Write);
        }
        if s == "write_bytes" {
            return Ok(DiskMetricName::WriteBytes);
        }
        anyhow::bail!("invalid string for DiskMetricName: {}", s);
    }
}
impl DiskMetricName {
    pub fn is_noop(&self) -> bool {
        matches!(self, DiskMetricName::Noop)
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
/// Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
pub type Name = String;
/// Unique name for a saga [`Node`]
///
/// Each node requires a string name that's unique within its DAG.  The name is used to identify its output.  Nodes that depend on a given node (either directly or indirectly) can access the node's output using its name.
pub type NodeName = String;
/// Role names consist of two string components separated by dot (".").
pub type RoleName = String;
/// Names are constructed by concatenating the target and metric names with ':'. Target and metric names must be lowercase alphanumeric characters with '_' separating words.
pub type TimeseriesName = String;
