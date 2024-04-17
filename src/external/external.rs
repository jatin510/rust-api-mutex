use async_trait::async_trait;
use reqwest;

use super::ExternalApi;

pub struct ExternalApiService {
    access_token: String,
    refresh_token: String,
}

impl ExternalApiService {
    pub fn new() -> Self {
        ExternalApiService {
            access_token: "hello".to_string(),
            refresh_token: "hello".to_string(),
        }
    }
}

#[async_trait]
impl ExternalApi for ExternalApiService {
    async fn get_new_access_token_api(&self) -> Result<String, String> {
        // Implementation here
        Ok("access_token".to_string())
    }

    async fn get_info_api(&self) -> Result<String, String> {
        // Implementation here

        let response = reqwest::get("https://www.rust-lang.org")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("response = {response:?}");
        Ok("info".to_string())
    }
}
