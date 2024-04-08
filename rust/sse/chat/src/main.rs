use axum::routing::{get, post};
use std::sync::Arc;
use tokio::{self};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod chat;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chat".into()),
        )
        .with(fmt::layer())
        .init();

    let shared_state = Arc::new(chat::State::new());

    let assets = std::env::current_dir().unwrap();
    let router = axum::Router::new()
        .route("/", get(handler::index))
        .route("/hangout", post(handler::create_hangout))
        .route("/connect/:hangout", get(handler::load_hangout))
        .route("/sse/:hangout", get(handler::connect_to_hangout))
        .route("/chat/message", post(handler::send_message))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets.to_str().unwrap())),
        )
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("starting http server on 127.0.0.1:3000");
    axum::serve(listener, router).await.unwrap()
}
