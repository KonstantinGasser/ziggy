use askama::Template;
use std::sync::{Arc, Mutex};

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod conway;
mod http;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Run {
        #[arg(short, long, default_value_t = 200)]
        delay: u64,
    },
    Web {
        #[arg(short, long, default_value_t = 3000)]
        port: u64,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run { delay } => {
            let (width, height) = term_size::dimensions_stdout().unwrap_or((32, 64));

            let mut game = conway::game::Game::new(height, width);

            game.repaint();
            std::thread::sleep(std::time::Duration::from_millis(delay));

            loop {
                game = game.next_cycle();

                game.repaint();
                std::thread::sleep(std::time::Duration::from_millis(delay));
            }
        }
        Command::Web { port } => {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "game_of_life".into()),
                )
                .with(fmt::layer())
                .init();

            let game = conway::game::Game::new(32, 32);
            let router = Router::new()
                .route("/", get(http::handler::index))
                .route("/next", get(http::handler::next_cycle))
                .route("/reset", get(http::handler::reset))
                .route("/flip", get(http::handler::flip))
                .layer(Extension(Arc::new(Mutex::new(game))));

            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
                .await
                .unwrap();

            info!("Started HTTP Server on 0.0.0.0:{port}");
            axum::serve(listener, router).await.unwrap();
        }
    }
}
