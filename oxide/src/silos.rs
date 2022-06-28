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
     * This function performs a `GET` to the `/silos` endpoint.
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
     * This function performs a `GET` to the `/silos` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
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
     * Create a new silo.
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
     * Fetch a specific silo.
     *
     * This function performs a `GET` to the `/silos/{silo_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get(&self, silo_name: &str) -> Result<crate::types::Silo> {
        let url = format!(
            "/silos/{}",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a specific silo.
     *
     * This function performs a `DELETE` to the `/silos/{silo_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn delete(&self, silo_name: &str) -> Result<()> {
        let url = format!(
            "/silos/{}",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List Silo identity providers.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/identity_providers` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
            "/silos/{}/identity_providers?{}",
            crate::progenitor_support::encode_path(silo_name),
            query_
        );

        let resp: crate::types::IdentityProviderResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List Silo identity providers.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/identity_providers` endpoint.
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
            "/silos/{}/identity_providers?{}",
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
     * Fetch the IAM policy for this Silo.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/policy` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get_policy(&self, silo_name: &str) -> Result<crate::types::SiloRolePolicy> {
        let url = format!(
            "/silos/{}/policy",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update the IAM policy for this Silo.
     *
     * This function performs a `PUT` to the `/silos/{silo_name}/policy` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
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
     * Create a new SAML identity provider for a silo.
     *
     * This function performs a `POST` to the `/silos/{silo_name}/saml_identity_providers` endpoint.
     *
     * **Parameters:**
     *
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn saml_idp_create(
        &self,
        silo_name: &str,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider> {
        let url = format!(
            "/silos/{}/saml_identity_providers",
            crate::progenitor_support::encode_path(silo_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * GET a silo's SAML identity provider.
     *
     * This function performs a `GET` to the `/silos/{silo_name}/saml_identity_providers/{provider_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `provider_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `silo_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn saml_idp_fetch(
        &self,
        provider_name: &str,
        silo_name: &str,
    ) -> Result<crate::types::SamlIdentityProvider> {
        let url = format!(
            "/silos/{}/saml_identity_providers/{}",
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
