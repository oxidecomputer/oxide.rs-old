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
    * List the built-in roles.
    *
    * This function performs a `GET` to the `/roles` endpoint.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
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
    * List the built-in roles.
    *
    * This function performs a `GET` to the `/roles` endpoint.
    *
    * As opposed to `get`, this function returns all the pages of the request at once.
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
    * Fetch a specific built-in role.
    *
    * This function performs a `GET` to the `/roles/{role_name}` endpoint.
    *
    * **Parameters:**
    *
    * * `role_name: &str` -- The built-in role's unique name.
    */
    pub async fn get(&self, role_name: &str) -> Result<crate::types::Role> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(role_name),
        );

        self.client.get(&url, None).await
    }
}
