use super::ExternalApi;

struct ExternalApiService {
    access_token: String,
    refresh_token: String,
}

impl ExternalApi for ExternalApiService {
    fn get_new_access_token() -> Result<String, String> {
        // Implementation here
        Ok("access_token".to_string())
    }

    fn get_info() -> Result<String, String> {
        // Implementation here
        Ok("info".to_string())
    }
}
