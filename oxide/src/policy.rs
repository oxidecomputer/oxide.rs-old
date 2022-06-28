use anyhow::Result;

use crate::Client;

pub struct Policy {
    pub client: Client,
}

impl Policy {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Policy { client }
    }

    /**
     * Fetch the top-level IAM policy.
     *
     * This function performs a `GET` to the `/policy` endpoint.
     */
    pub async fn get(&self) -> Result<crate::types::FleetRolePolicy> {
        let url = "/policy".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update the top-level IAM policy.
     *
     * This function performs a `PUT` to the `/policy` endpoint.
     */
    pub async fn put(
        &self,
        body: &crate::types::FleetRolePolicy,
    ) -> Result<crate::types::FleetRolePolicy> {
        let url = "/policy".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
