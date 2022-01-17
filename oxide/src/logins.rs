use anyhow::Result;

use crate::Client;

pub struct Logins {
    pub client: Client,
}

impl Logins {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Logins {
            client,
        }
    }

    /**
* This function performs a `POST` to the `/login` endpoint.
*/
pub async fn spoof(
&self,
body: &crate::types::LoginParams
) -> Result<()> {
let url =
"/login".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
}


}