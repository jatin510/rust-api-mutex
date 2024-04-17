use async_trait::async_trait;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;

use super::ExternalApi;

pub struct ExternalApiService {
    access_token: Mutex<String>,
    refresh_token: Mutex<String>,
}

impl ExternalApiService {
    pub fn new() -> Self {
        let access_token: &str = "yJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MTMzNTA3ODIsImV4cCI6MTcxMzQzNzE4Mn0.cQc4t5FCz5QOKIH0Pim5fymqnnYcLt4oUZso_jp9OM0";
        let refresh_token :&str= "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MTMzNTA0NDUsImV4cCI6MTcxMzk1NTI0NX0.p--jjgKMKS6JbSW331-aghsnQCDvLaovZmunTrz9DbA";

        ExternalApiService {
            access_token: Mutex::new(access_token.to_string()),
            refresh_token: Mutex::new(refresh_token.to_string()),
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
        let bearer_token = self.access_token.lock().await.clone();
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

        println!("response = {response:?}");
        println!("status: {}", response.status());
        if response.status() == StatusCode::UNAUTHORIZED {
            // spawn a new thread
            // println!("creating new thread");

            // thread::spawn(move || self.get_new_access_token_api());
            drop(bearer_token); // Release the lock before refreshing token
            self.refresh_access_token().await?;
            return self.get_info_api().await; // Retry the request
        }

        Ok("info".to_string())
    }

    async fn refresh_access_token(&self) -> Result<String, String> {
        let client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let refresh_token = self.refresh_token.lock().await.clone();

        let data = format!(
            r#"{{
            "token": "{}"
        }}"#,
            refresh_token
        );

        let json: serde_json::Value = serde_json::from_str(&data).unwrap();

        let request = client
            .request(
                reqwest::Method::POST,
                "http://localhost:3000/refresh-access-token",
            )
            .headers(headers)
            // .body(&json);
            .json(&json);

        let response = request.send().await.unwrap();
        if response.status() == StatusCode::OK {
            let data: serde_json::Value = response.json().await.unwrap();

            let new_access_token = data["accessToken"].as_str().unwrap().to_string();
            let new_refresh_token = data["refreshToken"].as_str().unwrap().to_string();

            self.update_tokens(new_access_token, new_refresh_token)
                .await;
        }

        Ok("access_token".to_string())
    }

    async fn update_tokens(&self, new_access_token: String, new_refresh_token: String) {
        let mut access_token_guard = self.access_token.lock().await;
        let mut refresh_token_guard = self.refresh_token.lock().await;

        *access_token_guard = new_access_token;
        *refresh_token_guard = new_refresh_token;
    }
}
