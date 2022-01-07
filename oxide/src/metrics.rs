use anyhow::Result;

use crate::Client;

pub struct Metrics {
    pub client: Client,
}

impl Metrics {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Metrics { client }
    }

    /**
     * This function performs a `GET` to the `/timeseries/schema` endpoint.
     *
     * List all timeseries schema
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     */
    pub async fn timeseries_schema_get(
        &self,
        limit: u32,
        page_token: &str,
    ) -> Result<crate::types::TimeseriesSchemaResultsPage> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/timeseries/schema?{}", query_);

        self.client.get(&url, None).await
    }
}
