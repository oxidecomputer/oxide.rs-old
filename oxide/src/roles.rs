use anyhow::Result;

use crate::Client;

pub struct Roles {
    pub client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Roles { client }
    }

    /**
     * This function performs a `GET` to the `/roles` endpoint.
     *
     * List the built-in roles
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get_page(&self, limit: u32, page_token: &str) -> Result<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/roles?{}", query_);

        let resp: crate::types::RoleResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/roles` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * List the built-in roles
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::Role>> {
        let url = "/roles".to_string();
        let mut resp: crate::types::RoleResultsPage = self.client.get(&url, None).await?;

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
     * This function performs a `GET` to the `/roles/{role_name}` endpoint.
     *
     * Fetch a specific built-in role
     *
     * **Parameters:**
     *
     * * `role_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get(&self, role_name: &str) -> Result<crate::types::Role> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(role_name),
        );

        self.client.get(&url, None).await
    }
}
