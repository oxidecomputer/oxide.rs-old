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
     * List instances in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name or id.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- The project's unique name within the organization.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let resp: crate::types::InstanceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List instances in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        organization_name: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Instance>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
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
     * Create an instance in a project.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- The project's unique name within the organization.
     */
    pub async fn post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::InstanceCreate,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get an instance in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete an instance from a project.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn delete(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List disks attached to this instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name or id.
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn disks_get(
        &self,
        instance_name: &str,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            query_
        );

        let resp: crate::types::DiskResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List disks attached to this instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` endpoint.
     *
     * As opposed to `disks_get`, this function returns all the pages of the request at once.
     */
    pub async fn disks_get_all(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Disk>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/disks?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Migrate an instance to a different propolis-server, possibly on a different sled.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/migrate` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn migrate(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::InstanceMigrate,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/migrate",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List network interfaces attached to this instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name or id.
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn network_interfaces_get(
        &self,
        instance_name: &str,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::NetworkInterface>> {
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
            "/organizations/{}/projects/{}/instances/{}/network-interfaces?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            query_
        );

        let resp: crate::types::NetworkInterfaceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List network interfaces attached to this instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` endpoint.
     *
     * As opposed to `network_interfaces_get`, this function returns all the pages of the request at once.
     */
    pub async fn network_interfaces_get_all(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::NetworkInterface>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/network-interfaces?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            query_
        );

        let mut resp: crate::types::NetworkInterfaceResultsPage =
            self.client.get(&url, None).await?;

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
     * Create a network interface for an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn network_interfaces_post(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::NetworkInterfaceCreate,
    ) -> Result<crate::types::NetworkInterface> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/network-interfaces",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get an interface attached to an instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `interface_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn network_interfaces_get_interface(
        &self,
        instance_name: &str,
        interface_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::NetworkInterface> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            crate::progenitor_support::encode_path(interface_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Detach a network interface from an instance.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `interface_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn network_interfaces_delete_interface(
        &self,
        instance_name: &str,
        interface_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            crate::progenitor_support::encode_path(interface_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Reboot an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/reboot` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn reboot(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/reboot",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client.post(&url, None).await
    }

    /**
     * Boot an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/start` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn start(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/start",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client.post(&url, None).await
    }

    /**
     * Halt an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/stop` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- human-readable free-form text about a resource.
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn stop(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::Instance> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/stop",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        self.client.post(&url, None).await
    }
}
