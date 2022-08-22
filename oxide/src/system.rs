use anyhow::Result;

use crate::Client;

pub struct System {
    pub client: Client,
}

impl System {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        System { client }
    }

    /**
     * List built-in users.
     *
     * This function performs a `GET` to the `/system/user` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn user_list(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::UserBuiltin>> {
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
        let url = format!("/system/user?{}", query_);

        let resp: crate::types::UserBuiltinResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List built-in users.
     *
     * This function performs a `GET` to the `/system/user` endpoint.
     *
     * As opposed to `user_list`, this function returns all the pages of the request at once.
     */
    pub async fn user_list_all(
        &self,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::UserBuiltin>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/system/user?{}", query_);

        let mut resp: crate::types::UserBuiltinResultsPage = self.client.get(&url, None).await?;

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
     * Fetch a built-in user.
     *
     * This function performs a `GET` to the `/system/user/{user_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `user_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn user_view(&self, user_name: &str) -> Result<crate::types::UserBuiltin> {
        let url = format!(
            "/system/user/{}",
            crate::progenitor_support::encode_path(user_name),
        );

        self.client.get(&url, None).await
    }
}
