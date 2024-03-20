use anyhow::Context;
use clap::{Parser, Subcommand};

mod todo;
/// Simple program to greet a person
#[derive(Parser, Debug)]
// #[command()]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List,
    Create {
        #[clap(short = 't', long)]
        title: String,
    },
}

const CONFIG_PATH: &'static str = "todos.json";

fn main() {
    let args = Args::parse();

    let mut store = match todo::store::Store::from_fs(CONFIG_PATH) {
        Ok(store) => store,
        Err(err) => panic!("Initialization of CLI failed: {err}"),
    };

    match args.command {
        Command::List => println!("List of ToDos:\n\n{}", store.format_todos()),
        Command::Create { title } => {
            let td = todo::store::ToDo::new(title);
            store.add_todo(td);

            let _ = store.write_fs(CONFIG_PATH).context("write created ToDo");
        }
    };
}

