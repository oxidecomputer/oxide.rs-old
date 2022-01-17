use anyhow::Result;

use crate::Client;

pub struct Logouts {
    pub client: Client,
}

impl Logouts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Logouts {
            client,
        }
    }

    /**
* This function performs a `POST` to the `/logout` endpoint.
*/
pub async fn logout(
&self,
) -> Result<()> {
let url =
"/logout".to_string();
self.client.post(&url, None).await
}


}