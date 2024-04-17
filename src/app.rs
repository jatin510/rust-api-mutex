use axum::{routing::get, Router};

pub async fn create_app() -> Router {
    let router = Router::new()
        // get info api
        .route("/", get(get_info));

    router
}

async fn get_info() -> &'static str {
    "Hello, World!"
}
