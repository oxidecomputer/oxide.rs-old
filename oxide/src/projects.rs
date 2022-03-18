use anyhow::Result;

use crate::Client;

pub struct Projects {
    pub client: Client,
}

impl Projects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Projects { client }
    }

    /**
     * List all projects.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameOrIdSortMode` -- Supported set of sort modes for scanning by name or id.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        sort_by: crate::types::NameOrIdSortMode,
    ) -> Result<Vec<crate::types::Project>> {
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
            "/organizations/{}/projects?{}",
            crate::progenitor_support::encode_path(organization_name),
            query_
        );

        let resp: crate::types::ProjectResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List all projects.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        organization_name: &str,
        sort_by: crate::types::NameOrIdSortMode,
    ) -> Result<Vec<crate::types::Project>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects?{}",
            crate::progenitor_support::encode_path(organization_name),
            query_
        );

        let mut resp: crate::types::ProjectResultsPage = self.client.get(&url, None).await?;

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
     * Create a new project.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn post(
        &self,
        organization_name: &str,
        body: &crate::types::ProjectCreate,
    ) -> Result<crate::types::Project> {
        let url = format!(
            "/organizations/{}/projects",
            crate::progenitor_support::encode_path(organization_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Fetch a specific project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- The project's unique name within the organization.
     */
    pub async fn get(
        &self,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Project> {
        let url = format!(
            "/organizations/{}/projects/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a specific project.
     *
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- The project's unique name within the organization.
     */
    pub async fn put(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::ProjectUpdate,
    ) -> Result<crate::types::Project> {
        let url = format!(
            "/organizations/{}/projects/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a specific project.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- The project's unique name within the organization.
     */
    pub async fn delete(&self, organization_name: &str, project_name: &str) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client.delete(&url, None).await
    }
}
