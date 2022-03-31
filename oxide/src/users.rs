use anyhow::Result;

use crate::Client;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * List the built-in system users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortMode,
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
     * List the built-in system users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        sort_by: crate::types::NameSortMode,
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

    /**
     * Fetch a specific built-in system user.
     *
     * This function performs a `GET` to the `/users/{user_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `user_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get(&self, user_name: &str) -> Result<crate::types::User> {
        let url = format!(
            "/users/{}",
            crate::progenitor_support::encode_path(user_name),
        );

        self.client.get(&url, None).await
    }
}
