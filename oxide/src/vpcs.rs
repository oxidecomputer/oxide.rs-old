use anyhow::Result;

use crate::Client;

pub struct Vpcs {
    pub client: Client,
}

impl Vpcs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Vpcs { client }
    }

    /**
     * List VPCs in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs` endpoint.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get_page(
        &self,
        limit: u32,
        organization_name: &str,
        page_token: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Vpc>> {
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
            "/organizations/{}/projects/{}/vpcs?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let resp: crate::types::VpcResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List VPCs in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        organization_name: &str,
        project_name: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Vpc>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/vpcs?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            query_
        );

        let mut resp: crate::types::VpcResultsPage = self.client.get(&url, None).await?;

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
     * Create a VPC in a project.
     *
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/vpcs` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::VpcCreate,
    ) -> Result<crate::types::Vpc> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a VPC in a project.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `vpc_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn get(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::Vpc> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a VPC.
     *
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `vpc_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn put(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcUpdate,
    ) -> Result<crate::types::Vpc> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a vpc from a project.
     *
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `vpc_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn delete(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List firewall rules for a VPC.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `vpc_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn firewall_rules_get(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcFirewallRules> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Replace the firewall rules for a VPC.
     *
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `project_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     * * `vpc_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn firewall_rules_put(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcFirewallRuleUpdateParams,
    ) -> Result<crate::types::VpcFirewallRules> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
