use async_trait::async_trait;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Client, StatusCode};

use super::ExternalApi;

pub struct ExternalApiService {
    access_token: String,
    refresh_token: String,
}

static ACCESS_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MTMzNTA3ODIsImV4cCI6MTcxMzQzNzE4Mn0.cQc4t5FCz5QOKIH0Pim5fymqnnYcLt4oUZso_jp9OM0";
static REFRESH_TOKEN :&str= "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MTMzNTA0NDUsImV4cCI6MTcxMzk1NTI0NX0.p--jjgKMKS6JbSW331-aghsnQCDvLaovZmunTrz9DbA";

impl ExternalApiService {
    pub fn new() -> Self {
        ExternalApiService {
            access_token: ACCESS_TOKEN.to_string(),
            refresh_token: REFRESH_TOKEN.to_string(),
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
        let client = Client::new();

        // Create the value for the Authorization header
        let bearer_token = self.access_token.clone();
        let auth_value = format!("Bearer {}", bearer_token);

        // Create a HeaderMap and insert the Authorization header
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_value.parse().unwrap());

        // Make the GET request with the custom headers
        let response = client
            .get("http://localhost:3000/protected")
            .headers(headers)
            .send()
            .await
            .unwrap();

        println!("status: {}", response.status());
        if response.status() == StatusCode::UNAUTHORIZED {
            // spawn a new thread
            println!("creating new thread")
        }

        println!("response = {response:?}");
        Ok("info".to_string())
    }
}
