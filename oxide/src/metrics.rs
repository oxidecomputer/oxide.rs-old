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
     * List all timeseries schema.
     *
     * This function performs a `GET` to the `/timeseries/schema` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     */
    pub async fn timeseries_schema_get(
        &self,
        limit: u32,
        page_token: &str,
    ) -> Result<Vec<crate::types::TimeseriesSchema>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/timeseries/schema?{}", query_);

        let resp: crate::types::TimeseriesSchemaResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List all timeseries schema.
     *
     * This function performs a `GET` to the `/timeseries/schema` endpoint.
     *
     * As opposed to `timeseries_schema_get`, this function returns all the pages of the request at once.
     */
    pub async fn timeseries_schema_get_all(&self) -> Result<Vec<crate::types::TimeseriesSchema>> {
        let url = "/timeseries/schema".to_string();
        let mut resp: crate::types::TimeseriesSchemaResultsPage =
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
}
