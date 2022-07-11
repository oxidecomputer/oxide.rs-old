use anyhow::Result;

use crate::Client;

pub struct Hardware {
    pub client: Client,
}

impl Hardware {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Hardware { client }
    }

    /**
     * Fetch information about a sled in the system.
     *
     * This function performs a `GET` to the `/hardware/sleds/{sled_id}` endpoint.
     *
     * **Parameters:**
     *
     * * `sled_id: &str` -- The sled's unique ID.
     */
    pub async fn sleds_get_sled(&self, sled_id: &str) -> Result<crate::types::Sled> {
        let url = format!(
            "/hardware/sleds/{}",
            crate::progenitor_support::encode_path(sled_id),
        );

        self.client.get(&url, None).await
    }
}
