use anyhow::Result;

use crate::Client;

pub struct Organizations {
    pub client: Client,
}

impl Organizations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Organizations { client }
    }

    /**
     * This function performs a `GET` to the `/organizations` endpoint.
     *
     * List all organizations.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name or id.
     */
    pub async fn get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<crate::types::OrganizationResultsPage> {
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
        let url = format!("/organizations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations` endpoint.
     *
     * Create a new organization.
     */
    pub async fn post(&self, body: &crate::types::ProjectCreate) -> Result<crate::types::User> {
        let url = "/organizations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}` endpoint.
     *
     * Fetch a specific organization
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get_organizations(&self, organization_name: &str) -> Result<crate::types::User> {
        let url = format!(
            "/organizations/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}` endpoint.
     *
     * Update a specific organization.
     *  * TODO-correctness: Is it valid for PUT to accept application/json that's a subset of what the resource actually represents?  If not, is that a problem? (HTTP may require that this be idempotent.)  If so, can we get around that having this be a slightly different content-type (e.g., "application/json-patch")?  We should see what other APIs do.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn put(
        &self,
        organization_name: &str,
        body: &crate::types::OrganizationUpdate,
    ) -> Result<crate::types::User> {
        let url = format!(
            "/organizations/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}` endpoint.
     *
     * Delete a specific organization.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn delete(&self, organization_name: &str) -> Result<()> {
        let url = format!(
            "/organizations/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}` endpoint.
     *
     * Update a specific project.
     *  * TODO-correctness: Is it valid for PUT to accept application/json that's a subset of what the resource actually represents?  If not, is that a problem? (HTTP may require that this be idempotent.)  If so, can we get around that having this be a slightly different content-type (e.g., "application/json-patch")?  We should see what other APIs do.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn projects_put_project(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::ProjectUpdate,
    ) -> Result<crate::types::Project> {
        let url = format!(
            "/organizations/{}/projects/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}