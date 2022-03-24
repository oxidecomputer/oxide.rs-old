// TODO: these all come from here: https://github.com/oxidecomputer/omicron/tree/main/common/src/api/external
//
// And we should import the package instead when its deps are not as annoying to deal with.
pub const ERROR: &str = r#"
#[derive(Debug, Deserialize, thiserror::Error, PartialEq, Serialize)]
pub enum Error {
    /// An object needed as part of this operation was not found.
    #[error("Object (of type {lookup_type:?}) not found: {type_name}")]
    ObjectNotFound { type_name: ResourceType, lookup_type: LookupType },
    /// An object already exists with the specified name or identifier.
    #[error("Object (of type {type_name:?}) already exists: {object_name}")]
    ObjectAlreadyExists { type_name: ResourceType, object_name: String },
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
    #[error("Invalid Value: {label}, {message}")]
    InvalidValue { label: String, message: String },
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

/// Indicates how an object was looked up (for an `ObjectNotFound` error)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LookupType {
    /// a specific name was requested
    ByName(String),
    /// a specific id was requested
    ById(uuid::Uuid),
}

impl LookupType {
    /// Returns an ObjectNotFound error appropriate for the case where this
    /// lookup failed
    pub fn into_not_found(self, type_name: ResourceType) -> Error {
        Error::ObjectNotFound { type_name, lookup_type: self }
    }
}

impl From<&str> for LookupType {
    fn from(name: &str) -> Self {
        LookupType::ByName(name.to_owned())
    }
}

impl From<&Name> for LookupType {
    fn from(name: &Name) -> Self {
        LookupType::from(name.as_str())
    }
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

    /// Generates an [`Error::ObjectNotFound`] error for a lookup by object
    /// name.
    pub fn not_found_by_name(type_name: ResourceType, name: &Name) -> Error {
        LookupType::from(name).into_not_found(type_name)
    }

    /// Generates an [`Error::ObjectNotFound`] error for a lookup by object id.
    pub fn not_found_by_id(type_name: ResourceType, id: &uuid::Uuid) -> Error {
        LookupType::ById(*id).into_not_found(type_name)
    }

    /// Generates an [`Error::InternalError`] error with the specific message
    ///
    /// InternalError should be used for operational conditions that should not
    /// happen but that we cannot reasonably handle at runtime (e.g.,
    /// deserializing a value from the database, or finding two records for
    /// something that is supposed to be unique).
    pub fn internal_error(internal_message: &str) -> Error {
        Error::InternalError { internal_message: internal_message.to_owned() }
    }

    /// Generates an [`Error::InvalidRequest`] error with the specific message
    ///
    /// This should be used for failures due possibly to invalid client input
    /// or malformed requests.
    pub fn invalid_request(message: &str) -> Error {
        Error::InvalidRequest { message: message.to_owned() }
    }

    /// Generates an [`Error::ServiceUnavailable`] error with the specific
    /// message
    ///
    /// This should be used for transient failures where the caller might be
    /// expected to retry.  Logic errors or other problems indicating that a
    /// retry would not work should probably be an InternalError (if it's a
    /// server problem) or InvalidRequest (if it's a client problem) instead.
    pub fn unavail(message: &str) -> Error {
        Error::ServiceUnavailable { internal_message: message.to_owned() }
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
