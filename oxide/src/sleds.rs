use anyhow::Result;

use crate::Client;

pub struct Sleds {
    pub client: Client,
}

impl Sleds {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sleds { client }
    }

    /**
     * This function performs a `GET` to the `/hardware/sleds` endpoint.
     *
     * List sleds in the system.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     * * `sort_by: crate::types::IdSortModeAscending` -- Supported set of sort modes for scanning by id only.
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn hardware_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::IdSortModeAscending,
    ) -> Result<Vec<crate::types::Sled>> {
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
        let url = format!("/hardware/sleds?{}", query_);

        let resp: crate::types::SledResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/hardware/sleds` endpoint.
     *
     * As opposed to `hardware_get`, this function returns all the pages of the request at once.
     *
     * List sleds in the system.
     */
    pub async fn hardware_get_all(
        &self,
        sort_by: crate::types::IdSortModeAscending,
    ) -> Result<Vec<crate::types::Sled>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/hardware/sleds?{}", query_);

        let mut resp: crate::types::SledResultsPage = self.client.get(&url, None).await?;

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
     * This function performs a `GET` to the `/hardware/sleds/{sled_id}` endpoint.
     *
     * Fetch information about a sled in the system.
     *
     * **Parameters:**
     *
     * * `sled_id: &str` -- human-readable free-form text about a resource.
     */
    pub async fn hardware_get_sleds(&self, sled_id: &str) -> Result<crate::types::Sled> {
        let url = format!(
            "/hardware/sleds/{}",
            crate::progenitor_support::encode_path(sled_id),
        );

        self.client.get(&url, None).await
    }
}