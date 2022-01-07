use anyhow::Result;

use crate::Client;

pub struct Hardware {
    pub client: Client,
}

impl Hardware {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Hardware { client }
    }

    /**
     * This function performs a `GET` to the `/hardware/racks` endpoint.
     *
     * List racks in the system.
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
    pub async fn racks_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::IdSortModeAscending,
    ) -> Result<crate::types::RackResultsPage> {
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
        let url = format!("/hardware/racks?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/hardware/racks/{rack_id}` endpoint.
     *
     * Fetch information about a particular rack.
     *
     * **Parameters:**
     *
     * * `rack_id: &str` -- human-readable free-form text about a resource.
     */
    pub async fn racks_get_rack(&self, rack_id: &str) -> Result<crate::types::Rack> {
        let url = format!(
            "/hardware/racks/{}",
            crate::progenitor_support::encode_path(&rack_id.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn sleds_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::IdSortModeAscending,
    ) -> Result<crate::types::SledResultsPage> {
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

        self.client.get(&url, None).await
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
    pub async fn sleds_get_sled(&self, sled_id: &str) -> Result<crate::types::Sled> {
        let url = format!(
            "/hardware/sleds/{}",
            crate::progenitor_support::encode_path(&sled_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
