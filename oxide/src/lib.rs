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
//! oxide-api = "0.1.0-rc.13"
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
//! let oxide = Client::new(String::from("api-key"), String::from("host"));
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
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod disks;
pub mod firewall;
pub mod hidden;
pub mod instances;
pub mod metrics;
pub mod organizations;
pub mod projects;
pub mod racks;
pub mod roles;
pub mod routers;
pub mod routes;
pub mod sagas;
pub mod sleds;
pub mod subnets;
#[cfg(test)]
mod tests;
pub mod types;
pub mod users;
#[doc(hidden)]
pub mod utils;
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
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
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

    /// Return a reference to an interface that provides access to racks operations.
    pub fn racks(&self) -> racks::Racks {
        racks::Racks::new(self.clone())
    }

    /// Return a reference to an interface that provides access to sleds operations.
    pub fn sleds(&self) -> sleds::Sleds {
        sleds::Sleds::new(self.clone())
    }

    /// Return a reference to an interface that provides access to organizations operations.
    pub fn organizations(&self) -> organizations::Organizations {
        organizations::Organizations::new(self.clone())
    }

    /// Return a reference to an interface that provides access to disks operations.
    pub fn disks(&self) -> disks::Disks {
        disks::Disks::new(self.clone())
    }

    /// Return a reference to an interface that provides access to projects operations.
    pub fn projects(&self) -> projects::Projects {
        projects::Projects::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users operations.
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    /// Return a reference to an interface that provides access to roles operations.
    pub fn roles(&self) -> roles::Roles {
        roles::Roles::new(self.clone())
    }

    /// Return a reference to an interface that provides access to instances operations.
    pub fn instances(&self) -> instances::Instances {
        instances::Instances::new(self.clone())
    }

    /// Return a reference to an interface that provides access to sagas operations.
    pub fn sagas(&self) -> sagas::Sagas {
        sagas::Sagas::new(self.clone())
    }

    /// Return a reference to an interface that provides access to metrics operations.
    pub fn metrics(&self) -> metrics::Metrics {
        metrics::Metrics::new(self.clone())
    }

    /// Return a reference to an interface that provides access to vpcs operations.
    pub fn vpcs(&self) -> vpcs::Vpcs {
        vpcs::Vpcs::new(self.clone())
    }

    /// Return a reference to an interface that provides access to subnets operations.
    pub fn subnets(&self) -> subnets::Subnets {
        subnets::Subnets::new(self.clone())
    }

    /// Return a reference to an interface that provides access to firewall operations.
    pub fn firewall(&self) -> firewall::Firewall {
        firewall::Firewall::new(self.clone())
    }

    /// Return a reference to an interface that provides access to routers operations.
    pub fn routers(&self) -> routers::Routers {
        routers::Routers::new(self.clone())
    }

    /// Return a reference to an interface that provides access to routes operations.
    pub fn routes(&self) -> routes::Routes {
        routes::Routes::new(self.clone())
    }

    /// Return a reference to an interface that provides access to hidden operations.
    pub fn hidden(&self) -> hidden::Hidden {
        hidden::Hidden::new(self.clone())
    }
}
