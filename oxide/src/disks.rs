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
     * Fetch a disk by id.
     *
     * This function performs a `GET` to the `/by-id/disks/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: &str`
     */
    pub async fn view(&self, id: &str) -> Result<crate::types::Disk> {
        let url = format!(
            "/by-id/disks/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
     * List disks.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
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
     * List disks.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
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
     * Create a disk.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/disks` endpoint.
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
     * Fetch a disk.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `disk_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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
     * Delete a disk.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `disk_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
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

    /**
     * Fetch disk metrics.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}/metrics/{metric_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `disk_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `metric_name: crate::types::DiskMetricName`
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.
     * * `end_time: chrono::DateTime<chrono::Utc>` -- An exclusive end time of metrics.
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `start_time: chrono::DateTime<chrono::Utc>` -- An inclusive start time of metrics.
     */
    pub async fn metrics_list(
        &self,
        disk_name: &str,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        limit: u32,
        metric_name: &str,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::Measurement>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_time {
            query_args.push(("end_time".to_string(), date.to_rfc3339()));
        }
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if let Some(date) = start_time {
            query_args.push(("start_time".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/disks/{}/metrics/{}?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(disk_name),
            crate::progenitor_support::encode_path(metric_name),
            query_
        );

        let resp: crate::types::MeasurementResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * Fetch disk metrics.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}/metrics/{metric_name}` endpoint.
     *
     * As opposed to `metrics_list`, this function returns all the pages of the request at once.
     */
    pub async fn metrics_list_all(
        &self,
        disk_name: &str,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        metric_name: &str,
        organization_name: &str,
        project_name: &str,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::Measurement>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_time {
            query_args.push(("end_time".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = start_time {
            query_args.push(("start_time".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/disks/{}/metrics/{}?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(disk_name),
            crate::progenitor_support::encode_path(metric_name),
            query_
        );

        let mut resp: crate::types::MeasurementResultsPage = self.client.get(&url, None).await?;

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
}
