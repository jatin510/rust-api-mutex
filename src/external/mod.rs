pub mod external;

use async_trait::async_trait;

#[async_trait]
pub trait ExternalApi {
    async fn get_new_access_token_api() -> Result<String, String>;
    async fn get_info_api() -> Result<String, String>;
}
