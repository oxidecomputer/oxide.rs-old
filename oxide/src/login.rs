use anyhow::Result;

use crate::Client;

pub struct Login {
    pub client: Client,
}

impl Login {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Login { client }
    }

    /**
     * Ask the user to login to their identity provider.
     *
     * This function performs a `GET` to the `/login/{silo_name}/{provider_name}` endpoint.
     *
     * Either display a page asking a user for their credentials, or redirect them to their identity provider.
     *
     * **Parameters:**
     *
     * * `provider_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get(&self, provider_name: &str, silo_name: &str) -> Result<()> {
        let url = format!(
            "/login/{}/{}",
            crate::progenitor_support::encode_path(silo_name),
            crate::progenitor_support::encode_path(provider_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Consume some sort of credentials, and authenticate a user.
     *
     * This function performs a `POST` to the `/login/{silo_name}/{provider_name}` endpoint.
     *
     * Either receive a username and password, or some sort of identity provider data (like a SAMLResponse). Use these to set the user's session cookie.
     *
     * **Parameters:**
     *
     * * `provider_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn consume_credentials<B: Into<reqwest::Body>>(
        &self,
        provider_name: &str,
        silo_name: &str,
        body: B,
    ) -> Result<()> {
        let url = format!(
            "/login/{}/{}",
            crate::progenitor_support::encode_path(silo_name),
            crate::progenitor_support::encode_path(provider_name),
        );

        self.client.post(&url, Some(body.into())).await
    }
}
