use anyhow::Result;

use crate::Client;

pub struct IpPools {
    pub client: Client,
}

impl IpPools {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IpPools { client }
    }

    /**
     * List IP Pools.
     *
     * This function performs a `GET` to the `/ip-pools` endpoint.
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
    ) -> Result<Vec<crate::types::IpPool>> {
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
        let url = format!("/ip-pools?{}", query_);

        let resp: crate::types::IpPoolResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List IP Pools.
     *
     * This function performs a `GET` to the `/ip-pools` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        sort_by: crate::types::NameOrIdSortMode,
    ) -> Result<Vec<crate::types::IpPool>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/ip-pools?{}", query_);

        let mut resp: crate::types::IpPoolResultsPage = self.client.get(&url, None).await?;

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
     * Create a new IP Pool.
     *
     * This function performs a `POST` to the `/ip-pools` endpoint.
     */
    pub async fn post(&self, body: &crate::types::IpPoolCreate) -> Result<crate::types::IpPool> {
        let url = "/ip-pools".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch a single IP Pool.
     *
     * This function performs a `GET` to the `/ip-pools/{pool_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get_pool(&self, pool_name: &str) -> Result<crate::types::IpPool> {
        let url = format!(
            "/ip-pools/{}",
            crate::progenitor_support::encode_path(pool_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update an IP Pool.
     *
     * This function performs a `PUT` to the `/ip-pools/{pool_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn put_pool(
        &self,
        pool_name: &str,
        body: &crate::types::IpPoolUpdate,
    ) -> Result<crate::types::IpPool> {
        let url = format!(
            "/ip-pools/{}",
            crate::progenitor_support::encode_path(pool_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete an IP Pool.
     *
     * This function performs a `DELETE` to the `/ip-pools/{pool_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn delete_pool(&self, pool_name: &str) -> Result<()> {
        let url = format!(
            "/ip-pools/{}",
            crate::progenitor_support::encode_path(pool_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List the ranges of IP addresses within an existing IP Pool.
     *
     * This function performs a `GET` to the `/ip-pools/{pool_name}/ranges` endpoint.
     *
     * Note that ranges are listed sorted by their first address.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     */
    pub async fn ranges_get(
        &self,
        limit: u32,
        page_token: &str,
        pool_name: &str,
    ) -> Result<Vec<crate::types::IpPoolRange>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ip-pools/{}/ranges?{}",
            crate::progenitor_support::encode_path(pool_name),
            query_
        );

        let resp: crate::types::IpPoolRangeResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List the ranges of IP addresses within an existing IP Pool.
     *
     * This function performs a `GET` to the `/ip-pools/{pool_name}/ranges` endpoint.
     *
     * As opposed to `ranges_get`, this function returns all the pages of the request at once.
     *
     * Note that ranges are listed sorted by their first address.
     */
    pub async fn ranges_get_all(&self, pool_name: &str) -> Result<Vec<crate::types::IpPoolRange>> {
        let url = format!(
            "/ip-pools/{}/ranges",
            crate::progenitor_support::encode_path(pool_name),
        );

        let mut resp: crate::types::IpPoolRangeResultsPage = self.client.get(&url, None).await?;

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
     * Add a new range to an existing IP Pool.
     *
     * This function performs a `POST` to the `/ip-pools/{pool_name}/ranges/add` endpoint.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn ranges_add(
        &self,
        pool_name: &str,
        body: &crate::types::IpRange,
    ) -> Result<crate::types::IpPoolRange> {
        let url = format!(
            "/ip-pools/{}/ranges/add",
            crate::progenitor_support::encode_path(pool_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Remove a range from an existing IP Pool.
     *
     * This function performs a `POST` to the `/ip-pools/{pool_name}/ranges/delete` endpoint.
     *
     * **Parameters:**
     *
     * * `pool_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn ranges_delete(&self, pool_name: &str, body: &crate::types::IpRange) -> Result<()> {
        let url = format!(
            "/ip-pools/{}/ranges/delete",
            crate::progenitor_support::encode_path(pool_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
