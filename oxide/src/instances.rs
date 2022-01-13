use anyhow::Result;

use crate::Client;

pub struct Instances {
    pub client: Client,
}

impl Instances {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Instances { client }
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * List instances in a project.
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
    pub async fn project_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::Instance>> {
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
            "/organizations/{}/projects/{}/instances?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            query_
        );

        let resp: crate::types::InstanceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * As opposed to `project_get`, this function returns all the pages of the request at once.
     *
     * List instances in a project.
     */
    pub async fn project_get_all(
        &self,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::Instance>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            query_
        );

        let mut resp: crate::types::InstanceResultsPage = self.client.get(&url, None).await?;

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
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * Create an instance in a project.
     *  * TODO-correctness This is supposed to be async.  Is that right?  We can create the instance immediately -- it's just not booted yet.  Maybe the boot operation is what's a separate operation_id.  What about the response code (201 Created vs 202 Accepted)?  Is that orthogonal?  Things can return a useful response, including an operation id, with either response code.  Maybe a "reboot" operation would return a 202 Accepted because there's no actual resource created?
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::InstanceCreate,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * Get an instance in a project.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_get_instances(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * Delete an instance from a project.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_delete(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` endpoint.
     *
     * List disks attached to this instance.
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
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn disks_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        instance_name: &str,
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
            "/organizations/{}/projects/{}/instances/{}/disks?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
            query_
        );

        let resp: crate::types::DiskResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` endpoint.
     *
     * As opposed to `disks_get`, this function returns all the pages of the request at once.
     *
     * List disks attached to this instance.
     */
    pub async fn disks_get_all(
        &self,
        sort_by: crate::types::NameSortModeAscending,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::Disk>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/disks?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
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
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/attach` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn disks_attach(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::DiskIdentifier,
    ) -> Result<crate::types::Disk> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/disks/attach",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/detach` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn disks_detach(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::DiskIdentifier,
    ) -> Result<crate::types::Disk> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/disks/detach",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/reboot` endpoint.
     *
     * Reboot an instance.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_reboot(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/reboot",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/start` endpoint.
     *
     * Boot an instance.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_start(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/start",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/stop` endpoint.
     *
     * Halt an instance.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_stop(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/stop",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&instance_name.to_string()),
        );

        self.client.post(&url, None).await
    }
}
