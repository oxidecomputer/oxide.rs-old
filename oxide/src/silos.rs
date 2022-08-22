use anyhow::Result;

use crate::Client;

pub struct Silos {
    pub client: Client,
}

impl Silos {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Silos { client }
    }

    /**
     * Fetch the current silo's IAM policy.
     *
     * This function performs a `GET` to the `/policy` endpoint.
     */
    pub async fn policy_get(&self) -> Result<crate::types::SiloRolePolicy> {
        let url = "/policy".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update the current silo's IAM policy.
     *
     * This function performs a `PUT` to the `/policy` endpoint.
     */
    pub async fn policy_put(
        &self,
        body: &crate::types::SiloRolePolicy,
    ) -> Result<crate::types::SiloRolePolicy> {
        let url = "/policy".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List silos.
     *
     * This function performs a `GET` to the `/silos` endpoint.
     *
     * Lists silos that are discoverable based on the current permissions.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameOrIdSortMode` -- Supported set of sort modes for scanning by name or id.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameOrIdSortMode,
    ) -> Result<Vec<crate::types::Silo>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/silos?{}", query_);

        let resp: crate::types::SiloResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List silos.
     *
     * This function performs a `GET` to the `/silos` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Lists silos that are discoverable based on the current permissions.
     */
    pub async fn get_all(
        &self,
        sort_by: crate::types::NameOrIdSortMode,
    ) -> Result<Vec<crate::types::Silo>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/silos?{}", query_);

        let mut resp: crate::types::SiloResultsPage = self.client.get(&url, None).await?;

        let mut items = resp.items;
        let mut page = resp.next_page;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?page={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&page={}", url, page), None)
                    .await?;
            }

            items.append(&mut resp.items);

            if !resp.next_page.is_empty() && resp.next_page != page {
                page = resp.next_page.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }

    /**
     * Create a silo.
     *
     * This function performs a `POST` to the `/silos` endpoint.
     */
    pub async fn post(&self, body: &crate::types::SiloCreate) -> Result<crate::types::Silo> {
        let url = "/silos".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch a silo.
     *
     * This function performs a `GET` to the `/silos/{silo_name}` endpoint.
     *
     * Fetch a silo by name.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn get(&self, silo_name: &str) -> Result<crate::types::Silo> {
        let url = format!(
            "/silos/{}",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a silo.
     *
     * This function performs a `DELETE` to the `/silos/{silo_name}` endpoint.
     *
     * Delete a silo by name.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn delete(&self, silo_name: &str) -> Result<()> {
        let url = format!(
            "/silos/{}",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List a silo's IDPs.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/identity-providers` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn get_identity_providers(
        &self,
        limit: u32,
        page_token: &str,
        silo_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::IdentityProvider>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/silos/{}/identity-providers?{}",
            crate::progenitor_support::encode_path(silo_name),
            query_
        );

        let resp: crate::types::IdentityProviderResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List a silo's IDPs.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/identity-providers` endpoint.
     *
     * As opposed to `get_identity_providers`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_identity_providers(
        &self,
        silo_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::IdentityProvider>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/silos/{}/identity-providers?{}",
            crate::progenitor_support::encode_path(silo_name),
            query_
        );

        let mut resp: crate::types::IdentityProviderResultsPage =
            self.client.get(&url, None).await?;

        let mut items = resp.items;
        let mut page = resp.next_page;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?page={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&page={}", url, page), None)
                    .await?;
            }

            items.append(&mut resp.items);

            if !resp.next_page.is_empty() && resp.next_page != page {
                page = resp.next_page.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }

    /**
     * Fetch a silo's IAM policy.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/policy` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn get_policy(&self, silo_name: &str) -> Result<crate::types::SiloRolePolicy> {
        let url = format!(
            "/silos/{}/policy",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a silo's IAM policy.
     *
     * This function performs a `PUT` to the `/silos/{silo_name}/policy` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn put_policy(
        &self,
        silo_name: &str,
        body: &crate::types::SiloRolePolicy,
    ) -> Result<crate::types::SiloRolePolicy> {
        let url = format!(
            "/silos/{}/policy",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Create a SAML IDP.
     *
     * This function performs a `POST` to the `/silos/{silo_name}/saml-identity-providers` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn saml_idp_fetch(
        &self,
        silo_name: &str,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider> {
        let url = format!(
            "/silos/{}/saml-identity-providers",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch a SAML IDP.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/saml-identity-providers/{provider_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `provider_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn saml_idp_create(
        &self,
        provider_name: &str,
        silo_name: &str,
    ) -> Result<crate::types::SamlIdentityProvider> {
        let url = format!(
            "/silos/{}/saml-identity-providers/{}",
            crate::progenitor_support::encode_path(silo_name),
            crate::progenitor_support::encode_path(provider_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::IdSortMode` -- Supported set of sort modes for scanning by id only.
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn users_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::IdSortMode,
    ) -> Result<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let resp: crate::types::UserResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `users_get`, this function returns all the pages of the request at once.
     */
    pub async fn users_get_all(
        &self,
        sort_by: crate::types::IdSortMode,
    ) -> Result<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let mut resp: crate::types::UserResultsPage = self.client.get(&url, None).await?;

        let mut items = resp.items;
        let mut page = resp.next_page;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?page={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&page={}", url, page), None)
                    .await?;
            }

            items.append(&mut resp.items);

            if !resp.next_page.is_empty() && resp.next_page != page {
                page = resp.next_page.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }
}
