use anyhow::Result;

use crate::Client;

pub struct Session {
    pub client: Client,
}

impl Session {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Session { client }
    }

    /**
     * List SSH public keys.
     *
     * This function performs a `GET` to the `/session/me/sshkeys` endpoint.
     *
     * Lists SSH public keys for the currently authenticated user.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn sshkeys_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::SshKey>> {
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
        let url = format!("/session/me/sshkeys?{}", query_);

        let resp: crate::types::SshKeyResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List SSH public keys.
     *
     * This function performs a `GET` to the `/session/me/sshkeys` endpoint.
     *
     * As opposed to `sshkeys_get`, this function returns all the pages of the request at once.
     *
     * Lists SSH public keys for the currently authenticated user.
     */
    pub async fn sshkeys_get_all(
        &self,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::SshKey>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/session/me/sshkeys?{}", query_);

        let mut resp: crate::types::SshKeyResultsPage = self.client.get(&url, None).await?;

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
     * Create an SSH public key.
     *
     * This function performs a `POST` to the `/session/me/sshkeys` endpoint.
     *
     * Create an SSH public key for the currently authenticated user.
     */
    pub async fn sshkeys_post(
        &self,
        body: &crate::types::SshKeyCreate,
    ) -> Result<crate::types::SshKey> {
        let url = "/session/me/sshkeys".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch an SSH public key.
     *
     * This function performs a `GET` to the `/session/me/sshkeys/{ssh_key_name}` endpoint.
     *
     * Fetch an SSH public key associated with the currently authenticated user.
     *
     * **Parameters:**
     *
     * * `ssh_key_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn sshkeys_get_key(&self, ssh_key_name: &str) -> Result<crate::types::SshKey> {
        let url = format!(
            "/session/me/sshkeys/{}",
            crate::progenitor_support::encode_path(ssh_key_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete an SSH public key.
     *
     * This function performs a `DELETE` to the `/session/me/sshkeys/{ssh_key_name}` endpoint.
     *
     * Delete an SSH public key associated with the currently authenticated user.
     *
     * **Parameters:**
     *
     * * `ssh_key_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn sshkeys_delete_key(&self, ssh_key_name: &str) -> Result<()> {
        let url = format!(
            "/session/me/sshkeys/{}",
            crate::progenitor_support::encode_path(ssh_key_name),
        );

        self.client.delete(&url, None).await
    }
}
