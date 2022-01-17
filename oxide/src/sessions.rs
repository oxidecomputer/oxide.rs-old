use anyhow::Result;

use crate::Client;

pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Sessions {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/session/me` endpoint.
*
* Fetch the user associated with the current session
*/
pub async fn me(
&self,
) -> Result<crate::types::SessionUser> {
let url =
"/session/me".to_string();
self.client.get(&url, None).await
}


}