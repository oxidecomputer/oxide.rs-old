use anyhow::Result;

use crate::Client;

pub struct Hidden {
    pub client: Client,
}

impl Hidden {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Hidden { client }
    }

    /**
    * This function performs a `POST` to the `/login` endpoint.
    */
    pub async fn spoof_login(&self, body: &crate::types::LoginParams) -> Result<()> {
        let url = "/login".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `POST` to the `/logout` endpoint.
    */
    pub async fn logout(&self) -> Result<()> {
        let url = "/logout".to_string();
        self.client.post(&url, None).await
    }

    /**
    * Fetch the user associated with the current session.
    *
    * This function performs a `GET` to the `/session/me` endpoint.
    */
    pub async fn session_me(&self) -> Result<crate::types::SessionUser> {
        let url = "/session/me".to_string();
        self.client.get(&url, None).await
    }
}
