use anyhow::Result;

use crate::Client;

pub struct Roles {
    pub client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Roles { client }
    }

    /**
     * This function performs a `GET` to the `/roles` endpoint.
     *
     * List the built-in roles
     *
     * **Parameters:**
     *
     * * `limit: u32` -- A count of bytes, typically used either for memory or storage capacity
     *  
     *  The maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.
     * * `page_token: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get(&self, limit: u32, page_token: &str) -> Result<crate::types::RoleResultsPage> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/roles?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/roles/{role_name}` endpoint.
     *
     * Fetch a specific built-in role
     *
     * **Parameters:**
     *
     * * `role_name: &str` -- human-readable free-form text about a resource.
     */
    pub async fn get_roles(&self, role_name: &str) -> Result<crate::types::Role> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(&role_name.to_string()),
        );

        self.client.get(&url, None).await
    }
}
