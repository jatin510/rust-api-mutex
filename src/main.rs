use log::*;
use std::net::SocketAddr;

mod app;
mod external;
mod settings;

use settings::SETTINGS;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Initializing....");

    let app = app::create_app().await;

    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    //
}
