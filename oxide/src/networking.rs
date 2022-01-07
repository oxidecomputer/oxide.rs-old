use anyhow::Result;

use crate::Client;

pub struct Networking {
    pub client: Client,
}

impl Networking {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Networking { client }
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs` endpoint.
     *
     * List VPCs in a project.
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
    pub async fn project_vpcs_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
    ) -> Result<crate::types::VpcResultsPage> {
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
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/vpcs` endpoint.
     *
     * Create a VPC in a project.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_vpcs_post(
        &self,
        organization_name: &str,
        project_name: &str,
        body: &crate::types::VpcCreate,
    ) -> Result<crate::types::Vpc> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * Get a VPC in a project.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_vpcs_get_vpc(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::Vpc> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * Update a VPC.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_vpcs_put_vpc(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcUpdate,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}` endpoint.
     *
     * Delete a vpc from a project.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn project_vpcs_delete_vpc(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules` endpoint.
     *
     * List firewall rules for a VPC.
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
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_firewall_rules_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcFirewallRuleResultsPage> {
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
            "/organizations/{}/projects/{}/vpcs/{}/firewall/rules?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules` endpoint.
     *
     * Replace the firewall rules for a VPC
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_firewall_rules_put(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcFirewallRuleUpdate,
    ) -> Result<crate::types::VpcFirewallRule> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers` endpoint.
     *
     * List VPC Custom and System Routers
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
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_routers_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcRouterResultsPage> {
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
            "/organizations/{}/projects/{}/vpcs/{}/routers?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers` endpoint.
     *
     * Create a VPC Router
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_routers_post(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::ProjectCreate,
    ) -> Result<crate::types::VpcRouter> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}` endpoint.
     *
     * Get a VPC Router
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_routers_get_router(
        &self,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcRouter> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}` endpoint.
     *
     * Update a VPC Router
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_routers_put_router(
        &self,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcRouterUpdate,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}` endpoint.
     *
     * Delete a router from its VPC
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_routers_delete_router(
        &self,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes` endpoint.
     *
     * List a Router's routes
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
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn routers_routes_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::RouterRouteResultsPage> {
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
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes` endpoint.
     *
     * Create a VPC Router
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn routers_routes_post(
        &self,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
        body: &crate::types::RouterRouteCreateParams,
    ) -> Result<crate::types::RouterRoute> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}` endpoint.
     *
     * Get a VPC Router route
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `route_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn routers_routes_get_route(
        &self,
        organization_name: &str,
        project_name: &str,
        route_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::RouterRoute> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
            crate::progenitor_support::encode_path(&route_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}` endpoint.
     *
     * Update a Router route
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `route_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn routers_routes_put_route(
        &self,
        organization_name: &str,
        project_name: &str,
        route_name: &str,
        router_name: &str,
        vpc_name: &str,
        body: &crate::types::RouterRouteUpdateParams,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
            crate::progenitor_support::encode_path(&route_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}` endpoint.
     *
     * Delete a route from its router
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `route_name: &str` -- human-readable free-form text about a resource.
     * * `router_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn routers_routes_delete_route(
        &self,
        organization_name: &str,
        project_name: &str,
        route_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&router_name.to_string()),
            crate::progenitor_support::encode_path(&route_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets` endpoint.
     *
     * List subnets in a VPC.
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
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_subnets_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcSubnetResultsPage> {
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
            "/organizations/{}/projects/{}/vpcs/{}/subnets?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets` endpoint.
     *
     * Create a subnet in a VPC.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_subnets_post(
        &self,
        organization_name: &str,
        project_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcSubnetCreate,
    ) -> Result<crate::types::VpcSubnet> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/subnets",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}` endpoint.
     *
     * Get subnet in a VPC.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `subnet_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_subnets_get_subnet(
        &self,
        organization_name: &str,
        project_name: &str,
        subnet_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::VpcSubnet> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&subnet_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}` endpoint.
     *
     * Update a VPC Subnet.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `subnet_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_subnets_put_subnet(
        &self,
        organization_name: &str,
        project_name: &str,
        subnet_name: &str,
        vpc_name: &str,
        body: &crate::types::VpcSubnetUpdate,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&subnet_name.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `DELETE` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}` endpoint.
     *
     * Delete a subnet from a VPC.
     *
     * **Parameters:**
     *
     * * `organization_name: &str` -- human-readable free-form text about a resource.
     * * `project_name: &str` -- human-readable free-form text about a resource.
     * * `subnet_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn vpc_subnets_delete_subnet(
        &self,
        organization_name: &str,
        project_name: &str,
        subnet_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&subnet_name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}/ips` endpoint.
     *
     * List IP addresses on a VPC subnet.
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
     * * `subnet_name: &str` -- human-readable free-form text about a resource.
     * * `vpc_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn subnets_ips_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        subnet_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::NetworkInterfaceResultsPage> {
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
            "/organizations/{}/projects/{}/vpcs/{}/subnets/{}/ips?{}",
            crate::progenitor_support::encode_path(&organization_name.to_string()),
            crate::progenitor_support::encode_path(&project_name.to_string()),
            crate::progenitor_support::encode_path(&vpc_name.to_string()),
            crate::progenitor_support::encode_path(&subnet_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
