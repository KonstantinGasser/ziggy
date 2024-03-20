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
}

const CONFIG_PATH: &'static str = "todos.json";

fn main() {
    let args = Args::parse();

    let store = match todo::store::Store::from_fs(CONFIG_PATH) {
        Ok(store) => store,
        Err(err) => panic!("Initialization of CLI failed: {err}"),
    };

    match args.command {
        Command::List => println!("List of ToDos:\n\n{}", store.format_todos()),
    };
}

