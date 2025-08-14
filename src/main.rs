mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init { name } => commands::handle_init(name),
        Commands::Generate(generate) => commands::handle_generate(&generate.command),
    };

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
