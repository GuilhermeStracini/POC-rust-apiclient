use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::ApiError;
use crate::models::Post;

#[derive(Debug, Clone)]
pub struct RestApiClient {
    client: Client,
    base_url: String,
}

impl RestApiClient {
    pub fn new(base_url: &str) -> Self {
        RestApiClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    // GET Request
    pub async fn get_post(&self, post_id: u32) -> Result<Post, ApiError> {
        let url = format!("{}/posts/{}", self.base_url, post_id);
        let response = self.client.get(&url).send().await?.json::<Post>().await?;
        Ok(response)
    }

    // POST Request
    pub async fn create_post(&self, new_post: &Post) -> Result<Post, ApiError> {
        let url = format!("{}/posts", self.base_url);
        let response = self.client.post(&url).json(new_post).send().await?.json::<Post>().await?;
        Ok(response)
    }

    // PUT Request
    pub async fn update_post(&self, post_id: u32, updated_post: &Post) -> Result<Post, ApiError> {
        let url = format!("{}/posts/{}", self.base_url, post_id);
        let response = self.client.put(&url).json(updated_post).send().await?.json::<Post>().await?;
        Ok(response)
    }

    // DELETE Request
    pub async fn delete_post(&self, post_id: u32) -> Result<(), ApiError> {
        let url = format!("{}/posts/{}", self.base_url, post_id);
        self.client.delete(&url).send().await?;
        Ok(())
    }
}