use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todos")]
#[command(about = "A simple CLI app to show a list of data", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // List all items
    List {
        #[arg(short, long, default_value_t = 10)]
        count: usize,
    },
}

pub fn handle_cli_commands() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List { count } => {
            println!("Hi: {}", count);
        }
    }
}
