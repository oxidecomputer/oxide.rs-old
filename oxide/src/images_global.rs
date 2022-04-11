use anyhow::Result;

use crate::Client;

pub struct ImagesGlobal {
    pub client: Client,
}

impl ImagesGlobal {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ImagesGlobal { client }
    }

    /**
     * List global images.
     *
     * This function performs a `GET` to the `/images` endpoint.
     *
     * Returns a list of all the global images. Global images are returned sorted by creation date, with the most recent images appearing first.
     *
     * **Parameters:**
     *
     * * `limit: u32` -- Maximum number of items returned by a single call.
     * * `page_token: &str` -- Token returned by previous call to retreive the subsequent page.
     * * `sort_by: crate::types::NameSortMode` -- Supported set of sort modes for scanning by name only
     *  
     *  Currently, we only support scanning in ascending order.
     */
    pub async fn images_get(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Image>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/images?{}", query_);

        let resp: crate::types::ImageResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * List global images.
     *
     * This function performs a `GET` to the `/images` endpoint.
     *
     * As opposed to `images_get`, this function returns all the pages of the request at once.
     *
     * Returns a list of all the global images. Global images are returned sorted by creation date, with the most recent images appearing first.
     */
    pub async fn images_get_all(
        &self,
        sort_by: crate::types::NameSortMode,
    ) -> Result<Vec<crate::types::Image>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/images?{}", query_);

        let mut resp: crate::types::ImageResultsPage = self.client.get(&url, None).await?;

        let mut items = resp.items;
        let mut page = resp.next_page;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?page={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&page={}", url, page), None)
                    .await?;
            }

            items.append(&mut resp.items);

            if !resp.next_page.is_empty() && resp.next_page != page {
                page = resp.next_page.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }

    /**
     * Create a global image.
     *
     * This function performs a `POST` to the `/images` endpoint.
     *
     * Create a new global image. This image can then be used by any user as a base for instances.
     */
    pub async fn images_post(
        &self,
        body: &crate::types::ImageCreate,
    ) -> Result<crate::types::Image> {
        let url = "/images".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a global image.
     *
     * This function performs a `GET` to the `/images/{image_name}` endpoint.
     *
     * Returns the details of a specific global image.
     *
     * **Parameters:**
     *
     * * `image_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn images_get_image(&self, image_name: &str) -> Result<crate::types::Image> {
        let url = format!(
            "/images/{}",
            crate::progenitor_support::encode_path(image_name),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a global image.
     *
     * This function performs a `DELETE` to the `/images/{image_name}` endpoint.
     *
     * Permanently delete a global image. This operation cannot be undone. Any instances using the global image will continue to run, however new instances can not be created with this image.
     *
     * **Parameters:**
     *
     * * `image_name: &str` -- Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'.
     */
    pub async fn images_delete_image(&self, image_name: &str) -> Result<()> {
        let url = format!(
            "/images/{}",
            crate::progenitor_support::encode_path(image_name),
        );

        self.client.delete(&url, None).await
    }
}
