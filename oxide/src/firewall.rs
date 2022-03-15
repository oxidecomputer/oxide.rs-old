use anyhow::Result;

use crate::Client;

pub struct Firewall {
    pub client: Client,
}

impl Firewall {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Firewall { client }
    }

    /**
     * List firewall rules for a VPC.
     *
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules` endpoint.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn rules_get(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::FirewallRules> {
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
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn rules_put(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::FirewallRuleUpdateParams,
    ) -> Result<crate::types::FirewallRules> {
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
