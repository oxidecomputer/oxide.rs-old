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
     * Start an OAuth 2.0 Device Authorization Grant.
     *
     * This function performs a `POST` to the `/device/auth` endpoint.
     *
     * This endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted.
     */
    pub async fn device_auth_request(&self) -> Result<()> {
        let url = "/device/auth".to_string();
        self.client.post(&url, None).await
    }

    /**
     * Confirm an OAuth 2.0 Device Authorization Grant.
     *
     * This function performs a `POST` to the `/device/confirm` endpoint.
     *
     * This endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/device/token`.
     */
    pub async fn device_auth_confirm(&self, body: &crate::types::DeviceAuthVerify) -> Result<()> {
        let url = "/device/confirm".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Request a device access token.
     *
     * This function performs a `POST` to the `/device/token` endpoint.
     *
     * This endpoint should be polled by the client until the user code is verified and the grant is confirmed.
     */
    pub async fn device_access_token(&self) -> Result<()> {
        let url = "/device/token".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/login` endpoint.
     */
    pub async fn spoof_login(&self, body: &crate::types::SpoofLoginBody) -> Result<()> {
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
    pub async fn session_me(&self) -> Result<crate::types::User> {
        let url = "/session/me".to_string();
        self.client.get(&url, None).await
    }
}
