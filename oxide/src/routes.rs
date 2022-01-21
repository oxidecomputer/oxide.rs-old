use anyhow::Result;

use crate::Client;

pub struct Routes {
    pub client: Client,
}

impl Routes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Routes { client }
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
    pub async fn routers_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<Vec<crate::types::RouterRoute>> {
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
            query_
        );

        let resp: crate::types::RouterRouteResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes` endpoint.
     *
     * As opposed to `routers_get`, this function returns all the pages of the request at once.
     *
     * List a Router's routes
     */
    pub async fn routers_get_all(
        &self,
        sort_by: crate::types::NameSortModeAscending,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<Vec<crate::types::RouterRoute>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes?{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
            query_
        );

        let mut resp: crate::types::RouterRouteResultsPage = self.client.get(&url, None).await?;

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
    pub async fn routers_post(
        &self,
        organization_name: &str,
        project_name: &str,
        router_name: &str,
        vpc_name: &str,
        body: &crate::types::RouterRouteCreateParams,
    ) -> Result<crate::types::RouterRoute> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
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
    pub async fn routers_get_routes(
        &self,
        organization_name: &str,
        project_name: &str,
        route_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<crate::types::RouterRoute> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
            crate::progenitor_support::encode_path(route_name),
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
    pub async fn routers_put(
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
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
            crate::progenitor_support::encode_path(route_name),
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
    pub async fn routers_delete(
        &self,
        organization_name: &str,
        project_name: &str,
        route_name: &str,
        router_name: &str,
        vpc_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            crate::progenitor_support::encode_path(organization_name),
            crate::progenitor_support::encode_path(project_name),
            crate::progenitor_support::encode_path(vpc_name),
            crate::progenitor_support::encode_path(router_name),
            crate::progenitor_support::encode_path(route_name),
        );

        self.client.delete(&url, None).await
    }
}