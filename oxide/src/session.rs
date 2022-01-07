use anyhow::Result;

use crate::Client;

pub struct Session {
    pub client: Client,
}

impl Session {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Session {
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