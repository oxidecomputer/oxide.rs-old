// TODO: these all come from here: https://github.com/oxidecomputer/omicron/tree/main/common/src/api/external
//
// And we should import the package instead when its deps are not as annoying to deal with.
pub const ERROR: &str = r#"
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
"#;

pub const IPV4_NET: &str = r##"/// An `Ipv4Net` represents a IPv4 subnetwork, including the address and network mask.
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

    fn json_schema(
        _: &mut schemars::gen::SchemaGenerator,
    ) -> schemars::schema::Schema {
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
}"##;

pub const IPV6_NET: &str = r##"/// An `Ipv6Net` represents a IPv6 subnetwork, including the address and network mask.
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
        self.is_unique_local()
            && self.0.prefix() == Self::VPC_IPV6_PREFIX_LENGTH
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

    fn json_schema(
        _: &mut schemars::gen::SchemaGenerator,
    ) -> schemars::schema::Schema {
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
}"##;

pub const IP_RANGE: &str = r#"#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, JsonSchema)]
pub enum IpRange {
    Ipv4Range,
    #[default]
    Ipv6Range
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
}"#;

pub const IP_NET: &str = r#"/// An `IpNet` represents an IP network, either IPv4 or IPv6.
#[derive(
    Clone, Copy, Debug, Deserialize, PartialEq, Hash, JsonSchema, Serialize,
)]
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
        let net =
            s.parse::<ipnetwork::IpNetwork>().map_err(|e| e.to_string())?;
        match net {
            ipnetwork::IpNetwork::V4(net) => Ok(IpNet::from(Ipv4Net(net))),
            ipnetwork::IpNetwork::V6(net) => Ok(IpNet::from(Ipv6Net(net))),
        }
    }
}
"#;

pub const DISK_SOURCE: &str = r##"#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
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
"##;

pub const DATUM: &str = r##"#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
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
"##;
