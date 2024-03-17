use axum::{extract::Extension, routing::get, Router};

use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod counter;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "counter".into()),
        )
        .with(fmt::layer())
        .init();

    info!("starting http server...");

    let app = counter::app::App::new();

    let router = Router::new()
        .route("/", get(handlers::index::get_count))
        .route("/increment", get(handlers::index::increment))
        .route("/decrement", get(handlers::index::decrement))
        .layer(Extension(app));

    let port = 3000_u16;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
