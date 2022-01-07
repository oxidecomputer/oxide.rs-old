use anyhow::Result;

use crate::Client;

pub struct Logout {
    pub client: Client,
}

impl Logout {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Logout {
            client,
        }
    }

    /**
* This function performs a `POST` to the `/logout` endpoint.
*/
pub async fn get(
&self,
) -> Result<()> {
let url =
"/logout".to_string();
self.client.post(&url, None).await
}


}