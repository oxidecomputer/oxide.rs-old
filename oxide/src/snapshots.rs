use anyhow::Result;

use crate::Client;

pub struct Snapshots {
    pub client: Client,
}

impl Snapshots {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Snapshots { client }
    }

    /**
     * Fetch a snapshot by id.
     *
     * This function performs a `GET` to the `/by-id/snapshots/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: &str`
     */
    pub async fn view(&self, id: &str) -> Result<crate::types::Snapshot> {
        let url = format!(
            "/by-id/snapshots/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
     * List snapshots.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/snapshots` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Snapshot>> {
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
        let url = format!(
            "/organizations/{}/projects/{}/snapshots?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let resp: crate::types::SnapshotResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List snapshots.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/snapshots` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        organization_name: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Snapshot>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/snapshots?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let mut resp: crate::types::SnapshotResultsPage = self.client.get(&url, None).await?;

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
     * Create a snapshot.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/snapshots` endpoint.
     *
     * Creates a point-in-time snapshot from a disk.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::SnapshotCreate,
    ) -> Result<crate::types::Snapshot> {
        let url = format!(
            "/organizations/{}/projects/{}/snapshots",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch a snapshot.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/snapshots/{snapshot_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `snapshot_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn get(
        &self,
        organization_name: &str,
        project_name: &str,
        snapshot_name: &str,
    ) -> Result<crate::types::Snapshot> {
        let url = format!(
            "/organizations/{}/projects/{}/snapshots/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(snapshot_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a snapshot.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/snapshots/{snapshot_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `snapshot_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn delete(
        &self,
        organization_name: &str,
        project_name: &str,
        snapshot_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/snapshots/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(snapshot_name),
        );

        self.client.delete(&url, None).await
    }
}
