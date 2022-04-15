use anyhow::Result;

use crate::Client;

pub struct Updates {
    pub client: Client,
}

impl Updates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Updates { client }
    }

    /**
    * Refresh update metadata.
    *
    * This function performs a `POST` to the `/updates/refresh` endpoint.
    */
    pub async fn refresh(&self) -> Result<()> {
        let url = "/updates/refresh".to_string();
        self.client.post(&url, None).await
    }
}
