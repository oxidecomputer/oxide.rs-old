//! A fully generated, opinionated API client library for Oxide.
//!
//! [![docs.rs](https://docs.rs/oxide-api/badge.svg)](https://docs.rs/oxide-api)
//!
//! ## API Details
//!
//! API for interacting with the Oxide control plane
//!
//!
//!
//! ### Contact
//!
//!
//! | url | email |
//! |----|----|
//! | <https://oxide.computer> | api@oxide.computer |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Oxide OpenAPI
//! specs](https://github.com/oxidecomputer/omicron) based on API spec version `0.0.1`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! oxide-api = "0.1.0-rc.34"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use oxide_api::Client;
//!
//! let oxide = Client::new(
//!     String::from("api-key"),
//!     String::from("host"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `OXIDE_TOKEN`
//! - `OXIDE_HOST`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use oxide_api::Client;
//!
//! let oxide = Client::new_from_env();
//! ```
//!
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Virtual disks are used to store instance-local data which includes the operating system.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod disks;
/// Firewall operation controls the flow of network data into a VPC.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod firewall;
/// TODO operations that will not ship to customers.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod hidden;
/// Images are read-only Virtual Disks that may be used to boot Virtual Machines.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod images;
/// Images are read-only Virtual Disks that may be used to boot Virtual Machines. These images are scoped globally.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod images_global;
/// Virtual machine instances are the basic unit of computation. These operations are used for provisioning, controlling, and destroying instances.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod instances;
/// Metrics provide insight into the operation of the Oxide deployment. These include telemetry on hardware and software components that can be used to understand the current state as well as to diagnose issues.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod metrics;
/// Organizations represent a subset of users and projects in an Oxide deployment.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod organizations;
/// Projects are a grouping of associated resources such as instances and disks within an organization for purposes of billing and access control.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod projects;
/// These operations pertain to hardware inventory and management. Racks are the unit of expansion of an Oxide deployment. Racks are in turn composed of sleds, switches, power supplies, and a cabled backplane.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod racks;
/// Roles are a component of Identity and Access Management (IAM) that allow a user or agent account access to additional permissions.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod roles;
/// Routers direct the flow of network traffic into, out of, and within a VPC via routes.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod routers;
/// Routes define router policy.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod routes;
/// Sagas are the abstraction used to represent multi-step operations within the Oxide deployment. These operations can be used to query saga status and report errors.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod sagas;
/// Silos represent a logical partition of users and resources.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod silos;
/// This tag should be moved into hardware.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod sleds;
/// Snapshots of Virtual Disks at a particular point in time.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod snapshots;
/// Public SSH keys for an individual user.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod sshkeys;
/// This tag should be moved into a generic network tag.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod subnets;
#[cfg(test)]
mod tests;
pub mod types;
/// This tag should be moved into a operations tag.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod updates;
/// This tag should be moved into an IAM tag.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod users;
#[doc(hidden)]
pub mod utils;
/// A Virtual Private Cloud (VPC) is an isolated network environment that should probaby be moved into a more generic networking tag.
///
///FROM: http://oxide.computer/docs/#xxx
pub mod vpcs;

use anyhow::{anyhow, Error, Result};

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::env;

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    token: String,

    client: reqwest::Client,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T, H>(token: T, host: H) -> Self
    where
        T: ToString,
        H: ToString,
    {
        let client = reqwest::Client::builder().build();
        match client {
            Ok(c) => Client {
                host: host.to_string(),
                token: token.to_string(),

                client: c,
            },
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Create a new Client struct from environment variables: OXIDE_TOKEN and OXIDE_HOST.
    pub fn new_from_env() -> Self {
        let token = env::var("OXIDE_TOKEN").expect("must set OXIDE_TOKEN");
        let host = env::var("OXIDE_HOST").expect("must set OXIDE_HOST");

        Client::new(token, host)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method.clone(), url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = body {
            log::debug!(
                "body: {:?}",
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            );
            req = req.body(body);
        }
        log::debug!("request: {:?}", &req);
        Ok(req)
    }

    pub async fn response_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let req = self.request_raw(method, uri, body).await?;
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.response_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error: anyhow::Error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                // Parse the error as the error type.
                match serde_json::from_slice::<crate::types::ErrorResponse>(&response_body) {
                    Ok(resp) => {
                        let e: crate::types::Error = resp.into();
                        e.into()
                    }
                    Err(_) => {
                        anyhow!(
                            "code: {}, error: {:?}",
                            status,
                            String::from_utf8_lossy(&response_body),
                        )
                    }
                }
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
            .await
    }

    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
    }

    /// Virtual disks are used to store instance-local data which includes the operating system.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn disks(&self) -> disks::Disks {
        disks::Disks::new(self.clone())
    }

    /// Firewall operation controls the flow of network data into a VPC.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn firewall(&self) -> firewall::Firewall {
        firewall::Firewall::new(self.clone())
    }

    /// TODO operations that will not ship to customers.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn hidden(&self) -> hidden::Hidden {
        hidden::Hidden::new(self.clone())
    }

    /// Images are read-only Virtual Disks that may be used to boot Virtual Machines.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn images(&self) -> images::Images {
        images::Images::new(self.clone())
    }

    /// Images are read-only Virtual Disks that may be used to boot Virtual Machines. These images are scoped globally.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn images_global(&self) -> images_global::ImagesGlobal {
        images_global::ImagesGlobal::new(self.clone())
    }

    /// Virtual machine instances are the basic unit of computation. These operations are used for provisioning, controlling, and destroying instances.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn instances(&self) -> instances::Instances {
        instances::Instances::new(self.clone())
    }

    /// Metrics provide insight into the operation of the Oxide deployment. These include telemetry on hardware and software components that can be used to understand the current state as well as to diagnose issues.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn metrics(&self) -> metrics::Metrics {
        metrics::Metrics::new(self.clone())
    }

    /// Organizations represent a subset of users and projects in an Oxide deployment.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn organizations(&self) -> organizations::Organizations {
        organizations::Organizations::new(self.clone())
    }

    /// Projects are a grouping of associated resources such as instances and disks within an organization for purposes of billing and access control.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn projects(&self) -> projects::Projects {
        projects::Projects::new(self.clone())
    }

    /// These operations pertain to hardware inventory and management. Racks are the unit of expansion of an Oxide deployment. Racks are in turn composed of sleds, switches, power supplies, and a cabled backplane.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn racks(&self) -> racks::Racks {
        racks::Racks::new(self.clone())
    }

    /// Roles are a component of Identity and Access Management (IAM) that allow a user or agent account access to additional permissions.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn roles(&self) -> roles::Roles {
        roles::Roles::new(self.clone())
    }

    /// Routers direct the flow of network traffic into, out of, and within a VPC via routes.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn routers(&self) -> routers::Routers {
        routers::Routers::new(self.clone())
    }

    /// Routes define router policy.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn routes(&self) -> routes::Routes {
        routes::Routes::new(self.clone())
    }

    /// Sagas are the abstraction used to represent multi-step operations within the Oxide deployment. These operations can be used to query saga status and report errors.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn sagas(&self) -> sagas::Sagas {
        sagas::Sagas::new(self.clone())
    }

    /// Silos represent a logical partition of users and resources.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn silos(&self) -> silos::Silos {
        silos::Silos::new(self.clone())
    }

    /// This tag should be moved into hardware.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn sleds(&self) -> sleds::Sleds {
        sleds::Sleds::new(self.clone())
    }

    /// Snapshots of Virtual Disks at a particular point in time.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn snapshots(&self) -> snapshots::Snapshots {
        snapshots::Snapshots::new(self.clone())
    }

    /// Public SSH keys for an individual user.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn sshkeys(&self) -> sshkeys::Sshkeys {
        sshkeys::Sshkeys::new(self.clone())
    }

    /// This tag should be moved into a generic network tag.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn subnets(&self) -> subnets::Subnets {
        subnets::Subnets::new(self.clone())
    }

    /// This tag should be moved into a operations tag.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn updates(&self) -> updates::Updates {
        updates::Updates::new(self.clone())
    }

    /// This tag should be moved into an IAM tag.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    /// A Virtual Private Cloud (VPC) is an isolated network environment that should probaby be moved into a more generic networking tag.
    ///
    ///FROM: http://oxide.computer/docs/#xxx
    pub fn vpcs(&self) -> vpcs::Vpcs {
        vpcs::Vpcs::new(self.clone())
    }
}
