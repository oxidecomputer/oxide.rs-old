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
     * Fetch an instance by id.
     *
     * This function performs a `GET` to the `/by-id/instances/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: &str`
     */
    pub async fn view(&self, id: &str) -> Result<crate::types::Instance> {
        let url = format!(
            "/by-id/instances/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
     * Fetch a network interface by id.
     *
     * This function performs a `GET` to the `/by-id/network-interfaces/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: &str`
     */
    pub async fn network_interface_view(&self, id: &str) -> Result<crate::types::NetworkInterface> {
        let url = format!(
            "/by-id/network-interfaces/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
     * List instances.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
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
     * List instances.
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
     * Create an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances` endpoint.
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
     * Fetch an instance.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Delete an instance.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * List an instance's disks.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * List an instance's disks.
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
     * Attach a disk to an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/attach` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Detach a disk from an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/detach` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * List external IP addresses.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/external-ips` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn external_ip_list(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::ExternalIp>> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/external-ips",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        let resp: crate::types::ExternalIpResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List external IP addresses.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/external-ips` endpoint.
     *
     * As opposed to `external_ip_list`, this function returns all the pages of the request at once.
     */
    pub async fn external_ip_list_all(
        &self,
        instance_name: &str,
        organization_name: &str,
        project_name: &str,
    ) -> Result<Vec<crate::types::ExternalIp>> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/external-ips",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
        );

        let mut resp: crate::types::ExternalIpResultsPage = self.client.get(&url, None).await?;

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
     * Migrate an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/migrate` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * List network interfaces.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * List network interfaces.
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
     * Create a network interface.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Fetch a network interface.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `interface_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Update a network interface.
     *
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `interface_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     */
    pub async fn network_interfaces_put_interface(
        &self,
        instance_name: &str,
        interface_name: &str,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::NetworkInterfaceUpdate,
    ) -> Result<crate::types::NetworkInterface> {
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            crate::progenitor_support::encode_path(interface_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a network interface.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}` endpoint.
     *
     * Note that the primary interface for an instance cannot be deleted if there are any secondary interfaces. A new primary interface must be designated first. The primary interface can be deleted if there are no secondary interfaces.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `interface_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Fetch an instance's serial console.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/serial-console` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `from_start: u64` -- Character index in the serial buffer from which to read, counting the bytes output since instance start. If this is not provided, `most_recent` must be provided, and if this \*is\* provided, `most_recent` must \*not\* be provided.
     * * `max_bytes: u64` -- Maximum number of bytes of buffered serial console contents to return. If the requested range runs to the end of the available buffer, the data returned will be shorter than `max_bytes`.
     * * `most_recent: u64` -- Character index in the serial buffer from which to read, counting \*backward\* from the most recently buffered data retrieved from the instance. (See note on `from_start` about mutual exclusivity).
     */
    pub async fn serial_get(
        &self,
        from_start: Option<u64>,
        instance_name: &str,
        max_bytes: Option<u64>,
        most_recent: Option<u64>,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::InstanceSerialConsoleData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(u) = from_start {
            query_args.push(("from_start".to_string(), u.to_string()));
        }
        if let Some(u) = max_bytes {
            query_args.push(("max_bytes".to_string(), u.to_string()));
        }
        if let Some(u) = most_recent {
            query_args.push(("most_recent".to_string(), u.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/instances/{}/serial-console?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(instance_name),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Boot an instance.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/start` endpoint.
     *
     * **Parameters:**
     *
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * * `instance_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
