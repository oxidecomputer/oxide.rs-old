use std::{cmp::Ordering, collections::BTreeMap};

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{render_param, struct_name, TypeDetails, TypeId, TypeSpace};

/*
 * Declare named types we know about:
 */
pub fn generate_types(api: &openapiv3::OpenAPI, ts: &mut TypeSpace) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Make sure we don't generate duplicate types.
    let mut seen: BTreeMap<String, bool> = BTreeMap::new();

    a("//! The data types sent to and returned from the API client.");
    a("    use schemars::JsonSchema;");
    a("    use serde::{Serialize, Deserialize};");
    a("    use std::fmt;");
    a("    use tabled::Tabled;");
    a("");

    for te in ts.clone().id_to_entry.values() {
        if let Some(sn) = te.name.as_deref() {
            let sn = struct_name(sn);

            if seen.contains_key(sn.as_str()) {
                continue;
            }

            seen.insert(sn.clone(), true);

            if sn == "Ipv4Net" {
                a(
                    r##"/// An `Ipv4Net` represents a IPv4 subnetwork, including the address and network mask.
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
}"##,
                );
                continue;
            } else if sn == "Ipv6Net" {
                a(
                    r##"/// An `Ipv6Net` represents a IPv6 subnetwork, including the address and network mask.
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
}"##,
                );

                continue;
            }

            match &te.details {
                TypeDetails::Enum(vals, schema_data) => {
                    let mut desc = "".to_string();
                    if let Some(d) = &schema_data.description {
                        desc = d.to_string();
                    }
                    let p = render_param(
                        sn.as_str(),
                        vals,
                        false,
                        &desc,
                        schema_data.default.as_ref(),
                        true,
                    );
                    a(&p);
                }
                TypeDetails::Placeholder(..) => {}
                TypeDetails::OneOf(omap, _) => a(&do_one_of_type(ts, omap, sn)),
                TypeDetails::AnyOf(omap, _) => a(&do_all_of_type(ts, omap, sn)),
                TypeDetails::AllOf(omap, _) => a(&do_all_of_type(ts, omap, sn)),
                TypeDetails::Object(omap, schema_data) => {
                    let mut omap = omap.clone();
                    /*
                     * TODO: This breaks things so ignore for now.
                     * Eventually this should work, we should ignore empty structs.
                    if omap.is_empty() {
                        // Continue early.
                        // We don't care about empty structs.
                        continue;
                    }*/

                    let desc = if let Some(description) = &schema_data.description {
                        format!("/// {}", description.replace('\n', "\n/// "))
                    } else {
                        "".to_string()
                    };

                    if !desc.is_empty() {
                        a(&desc);
                    }

                    // TODO: just make everything a default,
                    // this is gated by the oneof types cooperating.
                    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema,");
                    if sn != "Saga"
                        && sn != "RouterRouteUpdateParams"
                        && sn != "RouterRouteCreateParams"
                        && sn != "Disk"
                        && sn != "RouterRoute"
                    {
                        a("Default,");
                    }
                    if sn != "VpcFirewallRuleFilter" {
                        a("Tabled,");
                    }
                    a(r#")]"#);

                    a(&format!("pub struct {} {{", sn));

                    // If possible we want the order to be id, name, description,
                    // then everything else.
                    // Let's shoot for that.
                    let try_first = vec!["id", "name", "description"];
                    for f in try_first.iter() {
                        if let Some(tid) = omap.get(&f.to_string()) {
                            a(&render_property(ts, tid, f, &desc, &sn)?);
                            omap.remove(&f.to_string());
                        }
                    }

                    for (name, tid) in omap.iter() {
                        a(&render_property(ts, tid, name, &desc, &sn)?);
                    }
                    a("}");
                    a("");
                }
                TypeDetails::Basic(..) => {}
                TypeDetails::Unknown => {}
                TypeDetails::NamedType(..) => {}
                TypeDetails::ComponentSchema(tid, _schema_data) => {
                    a(&format!(
                        "pub type {} = {};",
                        sn,
                        ts.render_type(tid, true)?
                    ));
                }
                TypeDetails::Array(..) => {}
                TypeDetails::Optional(..) => {}
            }
        }
    }

    // Iterate over anything we missed.
    if let Some(components) = &api.components {
        for (_i, (sn, s)) in components.schemas.iter().enumerate() {
            if sn == "Ipv6Net" || sn == "Ipv4Net" {
                continue;
            }

            let id = ts.select(Some(sn.as_str()), s, "")?;

            let rendered = ts.render_type(&id, true)?;

            let et = ts.id_to_entry.get(&id).unwrap();
            let mut desc = "".to_string();
            if let TypeDetails::Basic(_, schema_data) = &et.details {
                desc = if let Some(description) = &schema_data.description {
                    format!("/// {}", description.replace('\n', "\n/// "))
                } else {
                    "".to_string()
                };
            }

            if rendered == "String" || rendered.starts_with('u') || rendered.starts_with('i') {
                if !desc.is_empty() {
                    a(&desc);
                }

                a(&format!("pub type {} = {};", sn, rendered));
            }
        }
    }

    Ok(out.to_string())
}

fn render_property(
    ts: &mut TypeSpace,
    tid: &TypeId,
    name: &str,
    desc: &str,
    sn: &str,
) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    if let Ok(mut rt) = ts.render_type(tid, true) {
        let mut prop = name.trim().to_string();
        if prop == "next" {
            rt = "String".to_string();
        }
        if prop == "ref"
            || prop == "type"
            || prop == "self"
            || prop == "box"
            || prop == "match"
            || prop == "foo"
            || prop == "enum"
            || prop == "const"
            || prop == "use"
        {
            prop = format!("{}_", name);
        } else if name == "$ref" {
            prop = format!("{}_", name.replace('$', ""));
        } else if name == "$type" {
            prop = format!("{}__", name.replace('$', ""));
        } else if name == "+1" {
            prop = "plus_one".to_string()
        } else if name == "-1" {
            prop = "minus_one".to_string()
        } else if name.starts_with('@') {
            prop = name.trim_start_matches('@').to_string();
        } else if name.starts_with('_') {
            prop = name.trim_start_matches('_').to_string();
        }

        // Try to render the docs.
        let p = ts.render_docs(tid);
        if !p.is_empty() && p != desc {
            a("/**");
            a(&p);
            a("*/");
        }

        let te = ts.id_to_entry.get(tid).unwrap();

        // Render the serde string.
        if rt == "String"
            || rt.starts_with("Vec<")
            || rt.starts_with("Option<")
            || rt.starts_with("BTreeMap<")
        {
            a(r#"#[serde(default,"#);
            if rt == "String" {
                a(r#"skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize","#);
            } else if rt.starts_with("Vec<") {
                a(r#"skip_serializing_if = "Vec::is_empty",
                                      deserialize_with = "crate::utils::deserialize_null_vector::deserialize","#);
            } else if rt.starts_with("std::collections::BTreeMap<") {
                a(r#"skip_serializing_if = "std::collections::BTreeMap::is_empty","#);
            } else if rt.starts_with("Option<url::Url") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::deserialize_empty_url::deserialize","#);
            } else if rt.starts_with("Option<chrono::NaiveDate") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_format::deserialize","#);
            } else if rt.starts_with("crate::utils::DisplayOptionDateTime") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize","#);
            } else if rt.starts_with("Option<") {
                a(r#"skip_serializing_if = "Option::is_none","#);
            }
        } else if rt == "bool" {
            if sn.ends_with("Request") {
                // We have a request, we want to make sure our bools are
                // options so we don't have to always provide them.
                a(r#"#[serde(default, skip_serializing_if = "Option::is_none","#);
                rt = "Option<bool>".to_string();
            } else {
                a(r#"#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize","#);
            }
        } else if rt == "i32" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i32",
                                    deserialize_with = "crate::utils::deserialize_null_i32::deserialize","#);
        } else if rt == "i64" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize","#);
        } else if rt == "f32" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f32",
                                    deserialize_with = "crate::utils::deserialize_null_f32::deserialize","#);
        } else if rt == "f64" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize","#);
        } else if rt == "u32" || rt == "u64" {
            a(r#"#[serde(default,"#);
        } else if let TypeDetails::Enum(_, sd) = &te.details {
            // We for sure have a default for every single enum, even
            // if the default is a noop.
            a(r#"#[serde(default,"#);
            // Figure out if its a no op and skip serializing if it is.
            if sd.default.is_none() {
                a(&format!(r#"skip_serializing_if = "{}::is_noop","#, rt));
            }
        } else {
            a(r#"#[serde("#);
        }

        if !prop.ends_with('_') {
            prop = to_snake_case(&prop);
        }

        // DO this again.
        // I know this is shit sue me, but sometimes we change the prop
        // so much it becomes one of these, ie. in the case of shipbob.
        if prop == "ref"
            || prop == "type"
            || prop == "self"
            || prop == "box"
            || prop == "match"
            || prop == "foo"
            || prop == "enum"
            || prop == "const"
            || prop == "use"
        {
            prop = format!("{}_", prop);
        }

        if prop == "ipv_4_block" {
            prop = "ipv4_block".to_string();
        } else if prop == "ipv_6_block" {
            prop = "ipv6_block".to_string();
        } else if prop == "ipv_6_prefix" {
            prop = "ipv6_prefix".to_string();
        } else if prop == "ipv_4_prefix" {
            prop = "ipv4_prefix".to_string();
        }

        // Close the serde string.
        if *name != prop {
            a(&format!(r#"rename = "{}")]"#, name));
        } else if rt == "Page" && prop == "page" || rt.ends_with("Page") {
            a(r#"default)]"#);
        } else {
            a(r#")]"#);
        }

        // Hide things from the table that don't implement display.
        if (rt.starts_with("Vec<")
            || rt.starts_with("Option<InstanceNetwork")
            || rt == "VpcFirewallRuleFilter")
            && sn != "VpcFirewallRuleFilter"
        {
            a(r#"#[header(hidden = true)]"#);
        }

        if prop == "type" {
            println!("{} {}", sn, prop);
        }

        a(&format!("pub {}: {},", prop, rt));
    } else {
        bail!("rendering type {} {:?} failed", name, tid);
    }

    Ok(out.to_string())
}

fn do_one_of_type(
    ts: &mut TypeSpace,
    one_of: &[openapiv3::ReferenceOr<openapiv3::Schema>],
    sn: String,
) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    if sn == "IpNet" {
        return r#"/// An `IpNet` represents an IP network, either IPv4 or IPv6.
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
"#
        .to_string();
    }

    let mut tag = "";
    let mut content = "";
    let mut omap: Vec<crate::TypeId> = Default::default();
    for one in one_of {
        let itid = ts.select(Some(&sn), one, "").unwrap();
        omap.push(itid);
    }

    omap.sort_unstable();
    omap.dedup();

    let mut is_enum = false;

    for itid in omap.iter() {
        // Determine if we can do anything fancy with the resulting enum and flatten it.
        let et = ts.id_to_entry.get(itid).unwrap();

        if let TypeDetails::Object(o, _) = &et.details {
            // Iterate over the properties of the object and try to find a tag.
            for (name, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    is_enum = true;
                    if e.len() == 1 {
                        // We have an enum of one so we can use that as the tag.
                        tag = name;
                        continue;
                    }
                } else if o.len() == 2 {
                    content = name;
                }
            }
        }
    }

    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]");
    if !tag.is_empty() {
        a("#[serde(rename_all = \"lowercase\")]");
        a(&format!("#[serde(tag = \"{}\"", tag));
        if !content.is_empty() {
            a(&format!(", content = \"{}\"", content));
        }
        a(")]");
    }
    a(&format!("pub enum {} {{", sn));

    let mut types_strings: BTreeMap<String, (String, String)> = Default::default();

    let mut prop_types: Vec<(String, String)> = Default::default();

    for tid in omap.iter() {
        let et = ts.id_to_entry.get(tid).unwrap();
        if let TypeDetails::Object(o, _) = &et.details {
            let mut name = String::new();
            for (key, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    if e.len() == 1 {
                        let prop = struct_name(&e[0]);
                        let mut sep = "";
                        // We have an enum of one so we can use that as the tag.
                        if o.len() == 1 {
                            a(&format!("{},", prop));
                        } else if o.len() == 2 {
                            a(&format!("{}(", prop));
                            sep = "(..)";
                        } else if o.len() > 2 {
                            a(&format!("{} {{", prop));
                            sep = "{..}";
                        }
                        types_strings.insert(prop.clone(), (e[0].to_string(), sep.to_string()));
                        name = prop.clone();
                        break;
                    }
                } else if o.len() == 1 {
                    a(&format!("{}(", struct_name(key)));
                    types_strings.insert(key.clone(), (key.to_string(), "(..)".to_string()));
                    name = key.clone();
                }
            }
            for (n, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    if e.len() == 1 {
                        continue;
                    }
                }

                if o.len() <= 2 {
                    let t = ts.render_type(prop, true).unwrap();
                    prop_types.push((name.to_string(), t.to_string()));
                    a(&format!("{},", t));
                } else {
                    let t = ts.render_type(prop, true).unwrap();
                    prop_types.push((name.to_string(), t.to_string()));
                    a(&format!("{}: {},", n, t));
                }
            }

            match o.len().cmp(&2) {
                Ordering::Less => {
                    if !is_enum {
                        a("),");
                    }
                }
                Ordering::Equal => {
                    a("),");
                }
                Ordering::Greater => {
                    a("},");
                }
            }
        }
    }

    a("}");
    a("");

    if !tag.is_empty() && !content.is_empty() {
        // Handle display and to_string differently.
        // Now we need to implement display for the enum.
        a(&format!("impl fmt::Display for {} {{", sn));
        a("fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {");
        a("   let j = serde_json::json!(self);");
        a(&format!(
            "   let tag: String = serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
            tag
        ));
        a(&format!(
            "   let mut content: String = \
             serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
            content
        ));
        a(" if content.is_empty() {");
        a(&format!(
            "let map: std::collections::HashMap<String, String> = \
             serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
            content
        ));
        a("if let Some((_, v)) = map.iter().next() { content = v.to_string(); }");
        a("}");
        a("     write!(f, \"{}={}\",tag, content)");
        a("}");
        a("}");
        a("");

        // Let's implement FromStr for clap so we can use enums there.
        a(&format!("impl std::str::FromStr for {} {{", sn));
        a("type Err = anyhow::Error;");
        a("fn from_str(s: &str) -> Result<Self, Self::Err> {");
        a("    let parts = s.split('=').collect::<Vec<&str>>();");
        a("    if parts.len() != 2 {");
        a(&format!(
            r#"        anyhow::bail!("invalid format for {}, got {{}}", s);"#,
            sn
        ));
        a("    }");
        a("    let tag = parts[0].to_string();");
        a("    let content = parts[1].to_string();");
        a("    let mut j = String::new();");
        for (name, p) in prop_types.iter() {
            a(&format!("if tag == \"{}\" {{", name.to_lowercase()));
            a("j = format!(r#\"{{");
            a(&format!("\"{}\": \"{{}}\",", tag));
            if p == "String" || p == "InstanceNetworkInterfaceCreate" {
                a(&format!("\"{}\": \"{{}}\"", content));
                a("        }}\"#, tag, content);");
            } else {
                a(&format!("\"{}\": {{}}", content));
                a(&format!(
                    "        }}}}\"#, tag, serde_json::json!({}::from_str(&content).unwrap()));",
                    p
                ));
            }
            a("}");
        }
        a("    let result = serde_json::from_str(&j)?;");
        a("    Ok(result)");
        a("}");
        a("}");
    } else {
        // Now we need to implement display for the enum.
        a(&format!("impl fmt::Display for {} {{", sn));
        a("fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {");
        a("   write!(f, \"{}\", serde_json::json!(self))");
        a("}");
        a("}");
        a("");

        // Let's implement FromStr for clap so we can use enums there.
        a(&format!("impl std::str::FromStr for {} {{", sn));
        a("type Err = anyhow::Error;");
        a("fn from_str(s: &str) -> Result<Self, Self::Err> {");
        a("   Ok(serde_json::from_str(s)?)");
        a("}");
        a("}");
    }

    if !tag.is_empty() {
        let mut values: Vec<String> = Vec::new();
        // If we have a tag let's create the variant types.
        a(&format!("impl {} {{", sn));
        a("pub fn variants() -> Vec<String> {");
        a("    vec![");
        for (name, _) in types_strings.iter() {
            let k = name.to_lowercase();
            a(&format!("        \"{}\".to_string(),", k));
            values.push(k.to_string());
        }
        a("    ]");
        a("}");
        a("}");

        // Now we want to render a new enum for this type.
        let render = render_param(
            &format!("{}Type", sn),
            &values,
            false,
            &format!("The types for {}.", sn),
            None,
            false,
        );

        a(&render);
    }

    out
}

fn do_all_of_type(ts: &mut TypeSpace, omap: &[crate::TypeId], sn: String) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Get the description.
    let mut description =
        "All of the following types are flattened into one object:\n\n".to_string();

    for itid in omap {
        let rt = ts.render_type(itid, true).unwrap();
        description.push_str(&format!("- `{}`\n", rt));
    }
    description = format!("/// {}", description.replace('\n', "\n/// "));
    a(&description);

    a("#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone, JsonSchema, Tabled)]");
    a(&format!("pub struct {} {{", sn));
    let mut name_map: BTreeMap<String, String> = Default::default();
    // Becasue we have so many defaults set on our serde types these enums
    // sometimes parse the wrong value. It's better to instead use the functions we
    // inject that force the value to a specific type.
    let mut fns: Vec<String> = Default::default();
    for tid in omap.iter() {
        let name = ts.render_type(tid, true).unwrap();

        let fn_name = if name.starts_with("Vec<") {
            format!(
                "{}Vector",
                name.trim_start_matches("Vec<")
                    .trim_end_matches('>')
                    .replace("serde_json::", "")
            )
        } else if name.starts_with("serde_json") {
            "Value".to_string()
        } else {
            struct_name(&name)
        };

        if !fns.contains(&fn_name) {
            // Try to render the docs.
            let p = ts.render_docs(tid);
            if !p.is_empty() && p != description {
                a("/**");
                a(&p);
                a("*/");
            }

            a("#[serde(flatten)]");
            a(&format!("pub {}: {},", to_snake_case(&fn_name), name));
            name_map.insert(fn_name.to_string(), name.to_string());
            fns.push(fn_name);
        }
    }
    a("}");
    a("");

    out
}
