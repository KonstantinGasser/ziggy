use axum::Router;
use clap::{Parser, Subcommand};
use tracing::{debug, info};
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
            println!("{}", port);
            // tracing_subscriber::registry()
            //     .with(
            //         tracing_subscriber::EnvFilter::try_from_default_env()
            //             .unwrap_or_else(|_| "counter".into()),
            //     )
            //     .with(fmt::layer())
            //     .init();
            //
            // let router = Router::new();
            //
            // let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
            //     .await
            //     .unwrap();
            // info!("starting http server...");
        }
    }
}
