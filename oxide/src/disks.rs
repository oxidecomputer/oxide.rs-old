use anyhow::Result;

use crate::Client;

pub struct Disks {
    pub client: Client,
}

impl Disks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Disks { client }
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
     *
     * List disks in a project.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     * * `sort_by: crate::types::NameSortModeAscending` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::Disk>> {
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
            "/organizations/{}/projects/{}/disks?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let resp: crate::types::DiskResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * List disks in a project.
     */
    pub async fn get_all(
        &self,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::Disk>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/disks?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let mut resp: crate::types::DiskResultsPage = self.client.get(&url, None).await?;

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
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
     *
     * Create a disk in a project.
     *  * TODO-correctness See note about instance create.  This should be async.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::DiskCreate,
    ) -> Result<crate::types::Disk> {
        let url = format!(
            "/organizations/{}/projects/{}/disks",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}` endpoint.
     *
     * Fetch a single disk in a project.
     *
     * **Parameters:**
     *
     * * `disk_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get(
        &self,
        disk_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Disk> {
        let url = format!(
            "/organizations/{}/projects/{}/disks/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(disk_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}` endpoint.
     *
     * Delete a disk from a project.
     *
     * **Parameters:**
     *
     * * `disk_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn delete(
        &self,
        disk_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/disks/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(disk_name),
        );

        self.client.delete(&url, None).await
    }
}