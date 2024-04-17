use axum::{routing::get, Router};

use crate::external::{external::ExternalApiService, ExternalApi};

pub async fn create_app() -> Router {
    let router = Router::new()
        // get info api
        .route("/", get(get_info));

    router
}

async fn get_info() -> &'static str {
    // TODO
    // api call
    let api_service = ExternalApiService::get_instance();

    let m = match api_service.lock().await.get_info_api().await {
        Ok(info) => info,
        Err(e) => format!("Error: {}", e),
    };

    "dfsdf"
}
