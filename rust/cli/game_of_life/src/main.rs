use askama::Template;
use serde::{de, Deserialize, Deserializer};
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod conway;

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
                .route("/", get(index))
                .route("/next", get(next_cycle))
                .route("/reset", get(reset))
                .layer(Extension(Arc::new(Mutex::new(game))));

            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
                .await
                .unwrap();

            info!("Started HTTP Server on 0.0.0.0:{port}");
            axum::serve(listener, router).await.unwrap();
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    state: Vec<Vec<Option<()>>>,
}

#[derive(Template)]
#[template(path = "state.html")]
pub struct StateTemplate {
    state: Vec<Vec<Option<()>>>,
}

async fn index(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let state = state.lock().unwrap();

    TemplateResponse(IndexTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

async fn next_cycle(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.0 = state.next_cycle().0;

    TemplateResponse(StateTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

async fn reset(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    state.reset();
    TemplateResponse(StateTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

struct TemplateResponse<T>(pub T);

impl<T> IntoResponse for TemplateResponse<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to parse template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
